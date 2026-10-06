// Memory of every process whose command line mentions the given Chrome profile dir (Linux /proc, PSS).
import { readFileSync, readdirSync } from 'node:fs';

const kb = (txt, key) => Number(new RegExp(`^${key}:\\s+(\\d+) kB`, 'm').exec(txt)?.[1] ?? 0) * 1024;

export function chromeMemory(profileDir) {
  try {
    let pss = 0, n = 0;
    const byType = {};
    for (const pid of readdirSync('/proc')) {
      if (!/^\d+$/.test(pid)) continue;
      try {
        if (!readFileSync(`/proc/${pid}/cmdline`, 'utf8').includes(profileDir)) continue;
        const p = kb(readFileSync(`/proc/${pid}/smaps_rollup`, 'utf8'), 'Pss');
        const type = /--type=([\w-]+)/.exec(readFileSync(`/proc/${pid}/cmdline`, 'utf8'))?.[1] ?? 'browser';
        byType[type] = (byType[type] ?? 0) + p;
        pss += p;
        n++;
      } catch {}
    }
    return n ? { chromePss: pss, chromeProcs: n, chromeByType: byType } : {};
  } catch {
    return {};
  }
}
