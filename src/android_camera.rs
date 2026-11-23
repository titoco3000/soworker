//! The phone's camera through the NDK Camera2 API (docs/android-plan.md, 5.4): a capture session that repeats one
//! request into an `AImageReader` (YUV 4:2:0, 640x480), read by `media::camera_thread`, which turns and publishes the
//! pictures. Everything here is created and dropped on that one thread.
use ndk_sys as ffi;
use std::ffi::CStr;
use std::os::raw::{c_int, c_void};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};

// ndk-sys declares the functions; nothing links the library they live in
#[link(name = "camera2ndk")]
extern "C" {}

/// Set by the device callbacks when the camera went away (another app took it, or an error).
static LOST: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn on_disconnected(_ctx: *mut c_void, _dev: *mut ffi::ACameraDevice) {
    LOST.store(true, Ordering::Relaxed);
}

unsafe extern "C" fn on_error(_ctx: *mut c_void, _dev: *mut ffi::ACameraDevice, err: c_int) {
    eprintln!("camera: device error {err}");
    LOST.store(true, Ordering::Relaxed);
}

unsafe extern "C" fn on_session(_ctx: *mut c_void, _s: *mut ffi::ACameraCaptureSession) {}

/// One picture's planes, as the reader hands them out: Y, then U and V with their own strides (a pixel stride of 2
/// means U and V are interleaved).
pub struct Planes<'a> {
    pub y: &'a [u8],
    pub u: &'a [u8],
    pub v: &'a [u8],
    pub y_row: usize,
    pub uv_row: usize,
    pub uv_pixel: usize,
    pub w: usize,
    pub h: usize,
}

pub struct Camera {
    mgr: *mut ffi::ACameraManager,
    device: *mut ffi::ACameraDevice,
    reader: *mut ffi::AImageReader,
    container: *mut ffi::ACaptureSessionOutputContainer,
    output: *mut ffi::ACaptureSessionOutput,
    target: *mut ffi::ACameraOutputTarget,
    request: *mut ffi::ACaptureRequest,
    session: *mut ffi::ACameraCaptureSession,
    /// Degrees the sensor is turned from the phone's natural orientation.
    pub sensor: u32,
    pub front: bool,
}

fn check(what: &str, s: ffi::camera_status_t) -> Result<(), String> {
    if s == ffi::camera_status_t::ACAMERA_OK {
        Ok(())
    } else {
        Err(format!("camera: {what} failed ({})", s.0))
    }
}

fn check_media(what: &str, s: ffi::media_status_t) -> Result<(), String> {
    if s == ffi::media_status_t::AMEDIA_OK {
        Ok(())
    } else {
        Err(format!("camera: {what} failed ({})", s.0))
    }
}

impl Camera {
    /// Opens the front (or back) camera and starts it delivering `w` x `h` pictures. Needs the CAMERA permission.
    pub fn open(front: bool, w: i32, h: i32) -> Result<Camera, String> {
        LOST.store(false, Ordering::Relaxed);
        let mut cam = Camera {
            mgr: unsafe { ffi::ACameraManager_create() },
            device: null_mut(),
            reader: null_mut(),
            container: null_mut(),
            output: null_mut(),
            target: null_mut(),
            request: null_mut(),
            session: null_mut(),
            sensor: 0,
            front,
        };
        if cam.mgr.is_null() {
            return Err("camera: no camera service".into());
        }
        // SAFETY: every pointer passed comes from the NDK calls just before it, and lives in `cam` (freed in Drop).
        unsafe {
            let (id, sensor) = cam.pick(front)?;
            cam.sensor = sensor;
            check_media("image reader", ffi::AImageReader_new(w, h, ffi::AIMAGE_FORMATS::AIMAGE_FORMAT_YUV_420_888.0 as i32, 4, &mut cam.reader))?;
            let mut window = null_mut();
            check_media("reader window", ffi::AImageReader_getWindow(cam.reader, &mut window))?;
            let mut dev_cb = ffi::ACameraDevice_StateCallbacks { context: null_mut(), onDisconnected: Some(on_disconnected), onError: Some(on_error) };
            check("open", ffi::ACameraManager_openCamera(cam.mgr, id.as_ptr(), &mut dev_cb, &mut cam.device))?;
            check("outputs", ffi::ACaptureSessionOutputContainer_create(&mut cam.container))?;
            check("output", ffi::ACaptureSessionOutput_create(window, &mut cam.output))?;
            check("add output", ffi::ACaptureSessionOutputContainer_add(cam.container, cam.output))?;
            check("target", ffi::ACameraOutputTarget_create(window, &mut cam.target))?;
            check("request", ffi::ACameraDevice_createCaptureRequest(cam.device, ffi::ACameraDevice_request_template::TEMPLATE_RECORD, &mut cam.request))?;
            check("add target", ffi::ACaptureRequest_addTarget(cam.request, cam.target))?;
            let fps = [15i32, 30];
            let _ = ffi::ACaptureRequest_setEntry_i32(cam.request, ffi::acamera_metadata_tag::ACAMERA_CONTROL_AE_TARGET_FPS_RANGE.0, 2, fps.as_ptr());
            let session_cb = ffi::ACameraCaptureSession_stateCallbacks { context: null_mut(), onClosed: Some(on_session), onReady: Some(on_session), onActive: Some(on_session) };
            check("session", ffi::ACameraDevice_createCaptureSession(cam.device, cam.container, &session_cb, &mut cam.session))?;
            let mut req = cam.request;
            check("repeat", ffi::ACameraCaptureSession_setRepeatingRequest(cam.session, null_mut(), 1, &mut req, null_mut()))?;
            eprintln!("camera: {} camera {:?} started, sensor at {sensor}°", if front { "front" } else { "back" }, id);
        }
        Ok(cam)
    }

    /// The id of the camera facing the right way (else the first one) and its sensor orientation.
    unsafe fn pick(&self, front: bool) -> Result<(std::ffi::CString, u32), String> {
        let mut list: *mut ffi::ACameraIdList = null_mut();
        check("list", ffi::ACameraManager_getCameraIdList(self.mgr, &mut list))?;
        let ids: Vec<std::ffi::CString> = (0..(*list).numCameras as usize).map(|i| CStr::from_ptr(*(*list).cameraIds.add(i)).to_owned()).collect();
        ffi::ACameraManager_deleteCameraIdList(list);
        let want = if front { 0 } else { 1 }; // ACAMERA_LENS_FACING_FRONT, _BACK
        let mut first = None;
        for id in ids {
            let mut meta = null_mut();
            if ffi::ACameraManager_getCameraCharacteristics(self.mgr, id.as_ptr(), &mut meta) != ffi::camera_status_t::ACAMERA_OK {
                continue;
            }
            let read = |tag: ffi::acamera_metadata_tag| -> Option<i32> {
                let mut e: ffi::ACameraMetadata_const_entry = std::mem::zeroed();
                (ffi::ACameraMetadata_getConstEntry(meta, tag.0, &mut e) == ffi::camera_status_t::ACAMERA_OK && e.count > 0).then(|| match tag {
                    ffi::acamera_metadata_tag::ACAMERA_LENS_FACING => *e.data.u8_ as i32,
                    _ => *e.data.i32_,
                })
            };
            let facing = read(ffi::acamera_metadata_tag::ACAMERA_LENS_FACING);
            let sensor = read(ffi::acamera_metadata_tag::ACAMERA_SENSOR_ORIENTATION).unwrap_or(0).rem_euclid(360) as u32;
            ffi::ACameraMetadata_free(meta);
            if facing == Some(want) {
                return Ok((id, sensor));
            }
            first.get_or_insert((id, sensor));
        }
        first.ok_or_else(|| "camera: this phone has no camera".into())
    }

    pub fn lost(&self) -> bool {
        LOST.load(Ordering::Relaxed)
    }

    /// Runs `f` on the newest picture, if one came since the last call.
    pub fn latest<R>(&mut self, f: impl FnOnce(&Planes) -> R) -> Option<R> {
        // SAFETY: the image and its planes stay valid until AImage_delete, after `f` returned.
        unsafe {
            let mut img = null_mut();
            if ffi::AImageReader_acquireLatestImage(self.reader, &mut img) != ffi::media_status_t::AMEDIA_OK || img.is_null() {
                return None;
            }
            let plane = |i: c_int| -> Option<(&[u8], usize, usize)> {
                let (mut data, mut len, mut row, mut px) = (null_mut(), 0, 0, 0);
                if ffi::AImage_getPlaneData(img, i, &mut data, &mut len) != ffi::media_status_t::AMEDIA_OK || data.is_null() {
                    return None;
                }
                ffi::AImage_getPlaneRowStride(img, i, &mut row);
                ffi::AImage_getPlanePixelStride(img, i, &mut px);
                Some((std::slice::from_raw_parts(data, len.max(0) as usize), row.max(0) as usize, px.max(1) as usize))
            };
            let (mut w, mut h) = (0, 0);
            ffi::AImage_getWidth(img, &mut w);
            ffi::AImage_getHeight(img, &mut h);
            let out = match (plane(0), plane(1), plane(2)) {
                (Some((y, y_row, _)), Some((u, uv_row, uv_pixel)), Some((v, _, _))) => Some(f(&Planes { y, u, v, y_row, uv_row, uv_pixel, w: w as usize & !1, h: h as usize & !1 })),
                _ => None,
            };
            ffi::AImage_delete(img);
            out
        }
    }
}

impl Drop for Camera {
    fn drop(&mut self) {
        // SAFETY: each pointer is null or was made by the matching NDK call; freed once, in reverse order.
        unsafe {
            if !self.session.is_null() {
                ffi::ACameraCaptureSession_stopRepeating(self.session);
                ffi::ACameraCaptureSession_close(self.session);
            }
            if !self.request.is_null() {
                ffi::ACaptureRequest_free(self.request);
            }
            if !self.target.is_null() {
                ffi::ACameraOutputTarget_free(self.target);
            }
            if !self.device.is_null() {
                ffi::ACameraDevice_close(self.device);
            }
            if !self.output.is_null() {
                ffi::ACaptureSessionOutput_free(self.output);
            }
            if !self.container.is_null() {
                ffi::ACaptureSessionOutputContainer_free(self.container);
            }
            if !self.reader.is_null() {
                ffi::AImageReader_delete(self.reader);
            }
            ffi::ACameraManager_delete(self.mgr);
        }
    }
}
