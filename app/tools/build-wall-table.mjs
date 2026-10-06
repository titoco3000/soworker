#!/usr/bin/env node
// Builds app/data/walls.json (assetId -> {name, height}) from the wall catalog responses in a recorder run.
// Usage: node app/tools/build-wall-table.mjs recorder/logs/<run>
import { readFileSync, writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const run = process.argv[2];
const L = readFileSync(join(run, 'events.jsonl'), 'utf8').trim().split('\n').map((l) => JSON.parse(l));
const out = {};
for (const e of L) {
  const id = e.k === 'res.body' && e.url.match(/eyekia\/api\/v1\/wall\/([^?/]+)/)?.[1];
  if (!id) continue;
  const j = JSON.parse(e.text ?? readFileSync(join(run, e.file), 'utf8'));
  out[id] = { name: j.name, height: j.height };
}
const file = join(dirname(fileURLToPath(import.meta.url)), '..', 'data', 'walls.json');
writeFileSync(file, JSON.stringify(out, null, 1));
console.log(Object.keys(out).length, 'walls ->', file);
