//! Локальный предпросмотр собранного сайта: `npm run serve`.
//! Зависимостей нет — только стандартная библиотека Node.js.

import { createReadStream } from "node:fs";
import { stat } from "node:fs/promises";
import { createServer } from "node:http";
import { fileURLToPath } from "node:url";
import { dirname, extname, join, normalize } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..", "dist");
const port = Number(process.env.PORT ?? 8080);

const TYPES = {
    ".html": "text/html; charset=utf-8",
    ".css": "text/css; charset=utf-8",
    ".js": "text/javascript; charset=utf-8",
    ".json": "application/json; charset=utf-8",
    ".svg": "image/svg+xml",
    ".jpg": "image/jpeg",
    ".jpeg": "image/jpeg",
    ".png": "image/png",
    ".ico": "image/x-icon",
};

async function resolve(urlPath) {
    const clean = normalize(decodeURIComponent(urlPath.split("?")[0])).replace(/^(\.\.[/\\])+/, "");
    for (const candidate of [join(root, clean), join(root, clean, "index.html")]) {
        try {
            const info = await stat(candidate);
            if (info.isFile()) return candidate;
        } catch {
            // пробуем следующий вариант
        }
    }
    return null;
}

createServer(async (request, response) => {
    const file = await resolve(request.url ?? "/");
    if (!file) {
        response.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
        response.end("404 — страница не найдена. Сначала выполните npm run build.");
        return;
    }
    response.writeHead(200, { "content-type": TYPES[extname(file)] ?? "application/octet-stream" });
    createReadStream(file).pipe(response);
}).listen(port, () => {
    console.log(`Сайт доступен на http://localhost:${port} (каталог dist/)`);
});
