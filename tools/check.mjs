//! Проверка собранного сайта: тексты на месте, ссылки относительные, файлы есть.

import { readFile, stat } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

import { loadContent, LANGUAGES } from "./content.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const dist = join(root, "dist");

const failures = [];
const check = (condition, message) => {
    if (!condition) failures.push(message);
};

const esc = (value) =>
    String(value)
        .replaceAll("&", "&amp;")
        .replaceAll("<", "&lt;")
        .replaceAll(">", "&gt;")
        .replaceAll('"', "&quot;")
        .replaceAll("'", "&#39;");

/**
 * Собирает все видимые тексты контента.
 *
 * Пропускаются поля, которые сейчас не выводятся на страницу:
 * - `logo` — логотип в первом экране не используется;
 * - `icon` — имена иконок Font Awesome, их заменили встроенные SVG;
 * - `project_link_label` — подпись появляется только у проектов со ссылкой,
 *    а сейчас ссылок нет ни у одного проекта.
 */
const UNUSED_FIELDS = new Set(["logo", "icon", "project_link_label"]);

function texts(content, skip = UNUSED_FIELDS) {
    const result = [];
    const walk = (value, key) => {
        if (key && skip.has(key)) return;
        if (typeof value === "string") {
            if (value.trim() !== "") result.push(value);
            return;
        }
        if (Array.isArray(value)) return value.forEach((item) => walk(item));
        if (value && typeof value === "object") {
            for (const [childKey, child] of Object.entries(value)) walk(child, childKey);
        }
    };
    walk(content);
    return result;
}

const exists = async (path) => {
    try {
        return (await stat(path)).isFile();
    } catch {
        return false;
    }
};

async function main() {
    const content = loadContent(root);
    const pages = { ru: "index.html", en: "en/index.html" };

    for (const lang of LANGUAGES) {
        const file = join(dist, pages[lang]);
        check(await exists(file), `Нет собранной страницы dist/${pages[lang]}`);

        const html = await readFile(file, "utf8");

        check(html.includes(`<html lang="${lang}">`), `Неверный язык документа в ${pages[lang]}`);
        check(html.includes(`<title>${esc(content[lang].hero.title)}`), `Нет заголовка в ${pages[lang]}`);
        check(html.includes('name="description"'), `Нет описания в ${pages[lang]}`);
        check(!/\shref="\/[^"]/.test(html), `Абсолютные ссылки в ${pages[lang]} ломают GitHub Pages`);

        for (const text of texts(content[lang])) {
            const need = esc(text);
            check(html.includes(need), `В ${pages[lang]} потерян текст: ${text.slice(0, 60)}…`);
        }
    }

    const ruHtml = await readFile(join(dist, "index.html"), "utf8");
    const enHtml = await readFile(join(dist, "en/index.html"), "utf8");
    check(ruHtml.includes('href="en/"'), "Со страницы RU нет ссылки на EN");
    check(enHtml.includes('href="../"'), "Со страницы EN нет ссылки на RU");

    for (const file of [
        "assets/main.css",
        "assets/main.js",
        "assets/photo.jpg",
        "assets/favicon.svg",
        "en/index.html",
    ]) {
        check(await exists(join(dist, file)), `Нет файла dist/${file}`);
    }

    const css = await readFile(join(dist, "assets/main.css"), "utf8");
    check(!css.includes("/static/"), "В стилях остались абсолютные пути /static/");

    if (failures.length > 0) {
        console.error(`Проверка не пройдена (${failures.length}):`);
        for (const failure of failures) console.error(`  - ${failure}`);
        process.exitCode = 1;
        return;
    }

    console.log(
        `Проверка пройдена: 2 страницы, ${texts(content.ru).length + texts(content.en).length} текстовых фрагментов на месте.`,
    );
}

main().catch((error) => {
    console.error(`Проверка не удалась: ${error.message}`);
    process.exitCode = 1;
});
