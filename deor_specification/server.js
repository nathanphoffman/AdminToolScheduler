import { statSync } from 'fs';
import { extname, join, normalize } from 'path';

const dir = import.meta.dir;
const PORT = 8080;

const MIME = {
  '.html': 'text/html',
  '.js':   'application/javascript',
  '.wasm': 'application/wasm',
  '.md':   'text/plain',
  '.svg':  'image/svg+xml',
};

// true for a real top-level browser navigation (address bar, link click, refresh) —
// false for the SPA's own fetch() calls, which never set Sec-Fetch-Mode: navigate
// and don't ask for text/html
function isNavigationRequest(req) {
  const mode = req.headers.get('sec-fetch-mode');

  if (mode) return mode === 'navigate';

  return (req.headers.get('accept') || '').includes('text/html');
}

function fileType(path) {
  try {
    const stat = statSync(path);

    if (stat.isFile()) return 'file';

    if (stat.isDirectory()) return 'dir';

    return null;
  } catch {
    // a missing or unreadable path simply isn't servable, so report "neither"
    return null;
  }
}

function serveFile(path) {
  return new Response(Bun.file(path), {
    headers: {
      'Content-Type': MIME[extname(path)] || 'application/octet-stream',
      'Cache-Control': 'no-store',
    },
  });
}

function notFound() {
  return new Response('not found', { status: 404, headers: { 'Cache-Control': 'no-store' } });
}

const server = Bun.serve({
  port: PORT,
  fetch(req) {
    const url = new URL(req.url);
    let pathname = decodeURIComponent(url.pathname);

    if (pathname.length > 1 && pathname.endsWith('/')) pathname = pathname.slice(0, -1);

    // resolve within `dir` only — reject any path that escapes it (e.g. "..")
    const filePath = normalize(join(dir, pathname));

    if (filePath !== dir && !filePath.startsWith(dir + '/')) {
      return notFound();
    }

    if (pathname === '/') return serveFile(join(dir, 'index.html'));

    const navigating = isNavigationRequest(req);

    // direct navigation to a raw .md file — redirect to the pretty, extension-less path.
    // Cache-Control is required here: browsers cache a bare 301 indefinitely, and since this
    // URL is also fetched internally (non-navigating) for page content, a cached redirect would
    // silently hijack that fetch too and permanently break the page until the cache is cleared.
    if (pathname.endsWith('.md') && navigating) {
      return new Response(null, {
        status: 301,
        headers: { Location: pathname.slice(0, -3), 'Cache-Control': 'no-store' },
      });
    }

    // real file on disk (assets, or the SPA's own fetch() of a .md file) — serve as-is
    if (fileType(filePath) === 'file') return serveFile(filePath);

    // pretty path with no extension — only a real top-level navigation gets the SPA shell;
    // the app's own internal fetch()es must always ask for the .md file directly, so one
    // reaching here without it means a broken link/include, not a page to render
    if (navigating && fileType(filePath + '.md') === 'file') return serveFile(join(dir, 'index.html'));

    return notFound();
  },
});

console.log(`newweb → ${server.url}`);
