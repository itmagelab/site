//! Тесты контента и рендеринга: `node --test tools/`.

import { test } from "node:test";
import assert from "node:assert/strict";

import { loadContent, validateContent } from "./content.mjs";
import { renderPage } from "./render.mjs";

const content = loadContent();

test("контент содержит обе языковые версии целиком", () => {
    for (const lang of ["ru", "en"]) {
        const page = content[lang];
        assert.ok(page.hero.title.length > 0);
        assert.ok(page.about.work_experience.length > 0);
        assert.ok(page.portfolio.projects.length > 0);
        assert.ok(page.portfolio.skills.length > 0);
        assert.ok(page.contact.email.length > 0);
        assert.ok(page.footer.back_to_top.length > 0);
    }
});

test("у языков одинаковый объём контента", () => {
    assert.equal(content.ru.portfolio.projects.length, content.en.portfolio.projects.length);
    assert.equal(content.ru.portfolio.skills.length, content.en.portfolio.skills.length);
    assert.equal(content.ru.about.work_experience.length, content.en.about.work_experience.length);
    assert.equal(content.ru.contact.social_networks.length, content.en.contact.social_networks.length);
});

test("пустое поле в контенте ломает проверку", () => {
    const broken = structuredClone(content);
    broken.ru.hero.badge = "";
    assert.throws(() => validateContent(broken), /ru\.hero\.badge/);
});

test("расхождение языков по количеству проектов ломает проверку", () => {
    const broken = structuredClone(content);
    broken.en.portfolio.projects.pop();
    assert.throws(() => validateContent(broken), /проектов/);
});

test("страница содержит тексты и корректный язык", () => {
    const html = renderPage({ lang: "ru", content });
    assert.ok(html.includes('<html lang="ru">'));
    assert.ok(html.includes(content.ru.hero.title));
    assert.ok(html.includes(content.ru.contact.email));
    assert.ok(html.includes('href="en/"'));
});

test("английская страница использует относительные пути", () => {
    const html = renderPage({ lang: "en", content, base: "../" });
    assert.ok(html.includes('href="../assets/main.css"'));
    assert.ok(html.includes('href="../"'));
    assert.ok(html.includes(content.en.footer.copyright));
});

test("тексты контента экранируются и не становятся разметкой", () => {
    const html = renderPage({ lang: "ru", content });
    assert.ok(!html.includes("<script>alert"), "нет неэкранированных вставок");
    const injected = structuredClone(content);
    injected.ru.hero.subtitle = '<img src=x onerror="alert(1)">';
    const dirty = renderPage({ lang: "ru", content: injected });
    assert.ok(dirty.includes("&lt;img src=x"));
    assert.ok(!dirty.includes("<img src=x"));
});

test("незаполненный контент не собирается в страницу", () => {
    assert.throws(() => validateContent({}), /ru/);
});
