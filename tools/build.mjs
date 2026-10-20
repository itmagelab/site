//! Сборка статического сайта: `dist/` полностью пересоздаётся на каждом запуске.

import { cp, mkdir, rm, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

import { loadContent } from "./content.mjs";
import { renderPage } from "./render.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const dist = join(root, "dist");

const PAGES = [
    { lang: "ru", file: "index.html", base: "" },
    { lang: "en", file: "en/index.html", base: "../" },
];

const STATIC_FILES = [
    ["src/main.css", "assets/main.css"],
    ["src/main.js", "assets/main.js"],
    ["assets/photo.jpg", "assets/photo.jpg"],
    ["assets/favicon.svg", "assets/favicon.svg"],
];

async function build() {
    const content = loadContent(root);

    await rm(dist, { recursive: true, force: true });
    await mkdir(join(dist, "assets"), { recursive: true });
    await mkdir(join(dist, "en"), { recursive: true });

    for (const page of PAGES) {
        const html = renderPage({ lang: page.lang, content, base: page.base });
        const target = join(dist, page.file);
        await mkdir(dirname(target), { recursive: true });
        await writeFile(target, html, "utf8");
        console.log(`  страница ${page.file} (${page.lang}) — ${html.length} байт`);
    }

    for (const [from, to] of STATIC_FILES) {
        await cp(join(root, from), join(dist, to));
        console.log(`  файл      ${to}`);
    }

    console.log(`Сборка готова: ${dist}`);
}

build().catch((error) => {
    console.error(`Сборка не удалась: ${error.message}`);
    process.exitCode = 1;
});
