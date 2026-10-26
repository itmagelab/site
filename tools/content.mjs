//! Загрузка и проверка контента сайта.
//!
//! Контент живёт в `content/content.json` — едином источнике правды для обоих
//! языков. Здесь он читается, проверяется на обязательные поля и согласованность
//! между языками, чтобы сборка падала с понятной ошибкой, а не отдавала пустые
//! блоки на сайт.

import { readFileSync } from "node:fs";

export const LANGUAGES = ["ru", "en"];

const REQUIRED_SECTIONS = {
    hero: ["title", "subtitle", "badge", "cta_portfolio", "cta_contact"],
    about: ["photo", "title", "description", "skills", "work_experience_title", "work_experience"],
    portfolio: ["title", "projects_title", "skills_title", "projects", "skills"],
    contact: ["title", "phone", "phone_label", "email", "email_label", "social_title", "social_networks"],
    footer: ["copyright", "credits", "credits_url", "back_to_top"],
};

const filled = (value) => (Array.isArray(value) ? value.length > 0 : isFilled(value));
const isFilled = (value) => typeof value === "string" && value.trim() !== "";
const assert = (condition, message) => {
    if (!condition) throw new Error(message);
};

/** Проверяет структуру контента и полноту обоих языков. */
export function validateContent(content) {
    for (const lang of LANGUAGES) {
        assert(content[lang], `Отсутствует секция языка «${lang}»`);

        for (const [section, fields] of Object.entries(REQUIRED_SECTIONS)) {
            assert(content[lang][section], `Пустая секция «${lang}.${section}»`);
            for (const field of fields) {
                assert(
                    filled(content[lang][section][field]),
                    `Пустое поле «${lang}.${section}.${field}»`,
                );
            }
        }

        const { about, portfolio, contact } = content[lang];
        assert(about.work_experience.length > 0, `Пустая история работы для «${lang}»`);
        assert(portfolio.projects.length > 0, `Пустой список проектов для «${lang}»`);
        assert(portfolio.skills.length > 0, `Пустой список навыков для «${lang}»`);
        assert(contact.social_networks.length > 0, `Пустой список соцсетей для «${lang}»`);

        for (const experience of about.work_experience) {
            for (const field of ["year", "position", "company"]) {
                assert(isFilled(experience[field]), `Пустое поле «${lang}.about.work_experience.${field}»`);
            }
            assert(experience.description.length > 0, `Пустое описание опыта для «${lang}»`);
        }

        for (const project of portfolio.projects) {
            assert(isFilled(project.name), `Пустое имя проекта для «${lang}»`);
            assert(project.description.length > 0, `Пустое описание проекта «${project.name}»`);
        }

        for (const skill of portfolio.skills) {
            assert(isFilled(skill.name), `Пустое имя навыка для «${lang}»`);
            assert(
                Number.isInteger(skill.level) && skill.level >= 0 && skill.level <= 100,
                `Некорректный уровень навыка «${skill.name}» для «${lang}»`,
            );
        }
    }

    const [first, second] = LANGUAGES.map((lang) => content[lang]);
    assert(
        first.portfolio.projects.length === second.portfolio.projects.length,
        "У языков разное количество проектов",
    );
    assert(first.portfolio.skills.length === second.portfolio.skills.length, "У языков разное количество навыков");
    assert(
        first.about.work_experience.length === second.about.work_experience.length,
        "У языков разное количество мест работы",
    );
    assert(
        first.contact.social_networks.length === second.contact.social_networks.length,
        "У языков разное количество соцсетей",
    );

    return content;
}

/** Читает и проверяет `content/content.json`. */
export function loadContent(root = process.cwd()) {
    const path = `${root}/content/content.json`;
    let raw;
    try {
        raw = readFileSync(path, "utf8");
    } catch (cause) {
        throw new Error(`Не удалось прочитать ${path}: ${cause.message}`, { cause });
    }

    let parsed;
    try {
        parsed = JSON.parse(raw);
    } catch (cause) {
        throw new Error(`Некорректный JSON в ${path}: ${cause.message}`, { cause });
    }

    return validateContent(parsed);
}
