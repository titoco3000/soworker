// Memory of the page in Chrome under different flag sets. Usage: node app/test/mem.mjs <port>
import { spawn } from 'node:child_process';
import { readFileSync, readdirSync, rmSync } from 'node:fs';
const port = process.argv[2] ?? 4840;
const variants = {
  baseline: [],
  'no-gpu': ['--disable-gpu'],
  'lean': ['--disable-gpu', '--disable-extensions', '--disable-background-networking', '--disable-sync', '--disable-default-apps', '--disable-component-update', '--no-first-run', '--mute-audio'],
  'lean+one-process-per-site': ['--disable-gpu', '--disable-extensions', '--disable-background-networking', '--disable-sync', '--disable-component-update', '--disable-features=IsolateOrigins,site-per-process,Translate,MediaRouter,OptimizationHints', '--process-per-site', '--renderer-process-limit=1'],
  'lean+single-process': ['--disable-gpu', '--disable-extensions', '--disable-background-networking', '--disable-sync', '--disable-component-update', '--disable-features=Translate,MediaRouter,OptimizationHints', '--single-process', '--no-zygote'],
};
const kb = (t, k) => Number(new RegExp(`^${k}:\\s+(\\d+) kB`, 'm').exec(t)?.[1] ?? 0);
for (const [name, flags] of Object.entries(variants)) {
  const prof = `/tmp/claude-1000/memprof-${name.replace(/\W/g, '_')}`;
  rmSync(prof, { recursive: true, force: true });
  const c = spawn('google-chrome-stable', ['--headless=new', '--no-sandbox', `--user-data-dir=${prof}`, ...flags, `http://127.0.0.1:${port}/`], { stdio: 'ignore' });
  await new Promise((r) => setTimeout(r, 7000));
  let total = 0, rows = {};
  for (const pid of readdirSync('/proc')) {
    if (!/^\d+$/.test(pid)) continue;
    try {
      const cmd = readFileSync(`/proc/${pid}/cmdline`, 'utf8');
      if (!cmd.includes(prof)) continue;
      const type = /--type=([\w-]+)/.exec(cmd)?.[1] ?? 'browser';
      const pss = kb(readFileSync(`/proc/${pid}/smaps_rollup`, 'utf8'), 'Pss') / 1024;
      total += pss; rows[type] = (rows[type] ?? 0) + pss;
    } catch {}
  }
  console.log(name.padEnd(28), total.toFixed(0).padStart(4), 'MB  ', Object.entries(rows).map(([k, v]) => `${k} ${v.toFixed(0)}`).join(' | '));
  c.kill('SIGKILL'); await new Promise((r) => setTimeout(r, 1500));
}
process.exit(0);
