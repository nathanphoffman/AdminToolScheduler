import { writeFile, mkdir } from 'fs/promises';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const dir = dirname(fileURLToPath(import.meta.url));
const BASE = 'https://raw.githubusercontent.com/nathanphoffman/NewWeb/main/setup';

// runtime files only — never touches .md content, README.txt, or the user's package.json
const FILES = [
  { url: `${BASE}/index.html`, dest: join(dir, 'index.html') },
  { url: `${BASE}/engine/build/pkg/engine_bg.wasm`, dest: join(dir, 'engine', 'build', 'pkg', 'engine_bg.wasm') },
  { url: `${BASE}/server.js`, dest: join(dir, 'server.js') },
  { url: `${BASE}/update.js`, dest: join(dir, 'update.js') },
];

async function update() {
  for (const file of FILES) {
    console.log(`→ fetching ${file.url}`);
    const res = await fetch(file.url);

    if (!res.ok) throw new Error(`failed to fetch ${file.url}: ${res.status} ${res.statusText}`);
    await mkdir(dirname(file.dest), { recursive: true });
    await writeFile(file.dest, Buffer.from(await res.arrayBuffer()));
    console.log(`✓ updated ${file.dest}`);
  }

  console.log('newweb engine updated to latest.');
}

update().catch((err) => {
  console.error('Update failed:', err.message);
  process.exit(1);
});
