//! Рендеринг HTML-страниц сайта из контента.
//!
//! Модуль не знает ничего о файловой системе: на вход приходит проверенный
//! контент, на выходе — готовая статическая страница. Тексты экранируются,
//! поэтому контент нельзя случайно превратить в разметку.

import { icons } from "./icons.mjs";

/** Служебные подписи интерфейса (не контент сайта). */
export const UI = {
    ru: {
        skip: "Перейти к содержанию",
        nav: "Основная навигация",
        lang: "Выбор языка",
        sections: "Разделы страницы",
        links: "Быстрые ссылки",
        photo: "Фотография",
        levels: { core: "основной инструмент", strong: "уверенно", working: "рабочий уровень" },
    },
    en: {
        skip: "Skip to content",
        nav: "Main navigation",
        lang: "Language selection",
        sections: "Page sections",
        links: "Quick links",
        photo: "Photo",
        levels: { core: "core tool", strong: "confident", working: "working level" },
    },
};

const META = {
    ru: "DevOps, SRE, Linux, UNIX, системное администрирование",
    en: "DevOps, SRE, Linux, UNIX, system administration",
};

const esc = (value) =>
    String(value)
        .replaceAll("&", "&amp;")
        .replaceAll("<", "&lt;")
        .replaceAll(">", "&gt;")
        .replaceAll('"', "&quot;")
        .replaceAll("'", "&#39;");

const telHref = (phone) => `tel:${String(phone).replace(/[^\d+]/g, "")}`;

/** Приводит путь из контента к относительному пути внутри страницы. */
const asset = (base, path) => base + String(path).replace(/^\.?\//, "");

/**
 * Переводит числовой уровень владения в слова.
 *
 * Проценты у навыков ничего не измеряют, поэтому в разметку попадает
 * словесная оценка, а исходное число остаётся только в контенте.
 */
function levelWord(lang, level) {
    const levels = UI[lang].levels;
    if (level >= 95) return levels.core;
    if (level >= 90) return levels.strong;
    return levels.working;
}

function renderDescription(item) {
    if (typeof item === "string") return esc(item);
    const link =
        `<a class="project__link" href="${esc(item.link.url)}" target="_blank" rel="noopener noreferrer">` +
        `${esc(item.link.label)}${icons.external({ size: 13 })}</a>`;
    return `<span class="project__lead">${link}<span class="prose">${esc(item.text)}</span></span>`;
}

/** Пункты списков внутри проектов и мест работы разделены коротким штрихом. */
function renderLines(lines) {
    return lines
        .map((line) => `\n                            <li>${renderDescription(line)}</li>`)
        .join("");
}

function renderHero(hero, lang) {
    return `
            <section class="hero" id="hero">
                <div class="shell hero__masthead">
                    <div>
                        <h1 class="hero__title">${esc(hero.title)}</h1>
                        <p class="hero__subtitle">${esc(hero.subtitle)}</p>
                    </div>
                    <p class="label hero__badge">${esc(hero.badge)}</p>
                </div>
                <div class="shell">
                    <nav class="hero__actions" aria-label="${esc(UI[lang].links)}">
                        <a href="#portfolio">${esc(hero.cta_portfolio)}</a>
                        <a href="#contact">${esc(hero.cta_contact)}</a>
                    </nav>
                </div>
            </section>`;
}

function renderAbout(about, lang, person, base) {
    const facts = about.skills
        .map((skill) => `\n                        <li>${esc(skill)}</li>`)
        .join("");

    const jobs = about.work_experience
        .map(
            (job) => `
                    <li class="job">
                        <p class="job__period">${esc(job.year)}</p>
                        <div>
                            <h4 class="job__position">${esc(job.position)}</h4>
                            <p class="job__company">${esc(job.company)}</p>
                            <ul class="job__list">${renderLines(job.description)}
                            </ul>
                        </div>
                    </li>`,
        )
        .join("");

    return `
            <section class="section" id="about">
                <div class="shell">
                    <h2 class="section__title">${esc(about.title)}</h2>
                    <div class="about">
                        <p class="about__text">${esc(about.description)}</p>
                        <figure class="about__photo">
                            <img
                                src="${esc(asset(base, about.photo))}"
                                alt="${esc(UI[lang].photo)}: ${esc(person)}"
                                loading="lazy"
                                decoding="async"
                            />
                        </figure>
                    </div>

                    <ul class="facts">${facts}
                    </ul>

                    <h3 class="subsection-title subsection-title--spaced">${esc(about.work_experience_title)}</h3>
                    <ol class="jobs">${jobs}
                    </ol>
                </div>
            </section>`;
}

function renderPortfolio(portfolio) {
    const projects = portfolio.projects
        .map(
            (project) => `
                    <article class="project">
                        <header class="project__head">
                            <h4 class="project__name">${esc(project.name)}</h4>${
                                project.link
                                    ? `
                            <a class="project__cta" href="${esc(project.link)}" target="_blank" rel="noopener noreferrer">
                                ${esc(portfolio.project_link_label)}${icons.external({ size: 13 })}
                            </a>`
                                    : ""
                            }
                        </header>
                        <ul class="project__list">${renderLines(project.description)}
                        </ul>
                        <ul class="techs">
${project.technologies.map((tech) => `                            <li>${esc(tech)}</li>`).join("\n")}
                        </ul>
                    </article>`,
        )
        .join("");

    const skills = portfolio.skills
        .map(
            (skill) => `
                        <li class="skill">
                            <span>${esc(skill.name)}</span>
                            <span class="skill__level">${levelWord(portfolio.lang, skill.level)}</span>
                        </li>`,
        )
        .join("");

    return `
            <section class="section" id="portfolio">
                <div class="shell">
                    <h2 class="section__title">${esc(portfolio.title)}</h2>

                    <h3 class="subsection-title">${esc(portfolio.projects_title)}</h3>${projects}


                    <h3 class="subsection-title subsection-title--spaced">${esc(portfolio.skills_title)}</h3>
                    <ul class="skills">${skills}
                    </ul>
                </div>
            </section>`;
}

function renderContact(contact) {
    const socials = contact.social_networks
        .map(
            (social) => `
                        <li>
                            <a class="social" href="${esc(social.url)}" target="_blank" rel="noopener noreferrer">
                                ${esc(social.name)}${icons.external({ size: 13 })}
                            </a>
                        </li>`,
        )
        .join("");

    return `
            <section class="section" id="contact">
                <div class="shell">
                    <h2 class="section__title">${esc(contact.title)}</h2>

                    <div class="contacts">
                        <a class="contact" href="${esc(telHref(contact.phone))}">
                            <span class="label">${esc(contact.phone_label)}</span>
                            <span class="contact__value">${esc(contact.phone)}</span>
                        </a>
                        <a class="contact" href="mailto:${esc(contact.email)}">
                            <span class="label">${esc(contact.email_label)}</span>
                            <span class="contact__value">${esc(contact.email)}</span>
                        </a>
                    </div>

                    <h3 class="subsection-title subsection-title--spaced">${esc(contact.social_title)}</h3>
                    <ul class="socials">${socials}
                    </ul>
                </div>
            </section>`;
}

function renderHeader(lang, content, ruHref, enHref, base) {
    const nav = [
        { href: "#about", label: content.about.title },
        { href: "#portfolio", label: content.portfolio.title },
        { href: "#contact", label: content.contact.title },
    ]
        .map(
            (item) =>
                `<a href="${item.href}" data-nav="${item.href.slice(1)}">${esc(item.label)}</a>`,
        )
        .join("\n                        ");

    const language = (code, href) =>
        `<a href="${href}" hreflang="${code}"${code === lang ? ' aria-current="true"' : ""}>${code.toUpperCase()}</a>`;

    return `
        <header class="header">
            <div class="shell header__inner">
                <a class="brand" href="${base === "" ? "./" : base}">
                    <span>${esc(content.hero.title)}</span>
                </a>
                <nav class="nav" aria-label="${esc(UI[lang].nav)}">
                        ${nav}
                </nav>
                <div class="lang" role="group" aria-label="${esc(UI[lang].lang)}">
                    ${language("ru", ruHref)}
                    ${language("en", enHref)}
                </div>
            </div>
        </header>`;
}

/**
 * Рендерит готовую страницу сайта.
 * @param {object} options
 * @param {string} options.lang код языка страницы (`ru` или `en`)
 * @param {object} options.content проверенный контент обоих языков
 * @param {string} [options.base] префикс относительных путей (`""` или `"../"`)
 */
export function renderPage({ lang, content, base = "" }) {
    const page = content[lang];
    const ruHref = lang === "ru" ? "./" : "../";
    const enHref = lang === "en" ? "./" : "en/";
    const title = `${page.hero.title} — ${page.hero.subtitle}`;
    const description = `${page.hero.title} — ${page.hero.subtitle}. ${page.hero.badge}.`;

    return `<!doctype html>
<html lang="${lang}">
    <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <title>${esc(title)}</title>
        <meta name="description" content="${esc(description)}" />
        <meta name="keywords" content="${esc(META[lang])}" />
        <meta name="author" content="${esc(page.hero.subtitle)}" />
        <meta name="theme-color" content="#f6f3ec" media="(prefers-color-scheme: light)" />
        <meta name="theme-color" content="#131210" media="(prefers-color-scheme: dark)" />
        <meta prefix="og: https://ogp.me/ns#" property="og:type" content="profile" />
        <meta prefix="og: https://ogp.me/ns#" property="og:title" content="${esc(title)}" />
        <meta prefix="og: https://ogp.me/ns#" property="og:description" content="${esc(description)}" />
        <meta name="twitter:card" content="summary" />
        <link rel="icon" href="${base}assets/favicon.svg" type="image/svg+xml" />
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
        <link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500&family=IBM+Plex+Sans:wght@400;600&display=swap"
        />
        <link rel="stylesheet" href="${base}assets/main.css" />
    </head>
    <body id="top">
        <a class="skip-link" href="#main">${esc(UI[lang].skip)}</a>
${renderHeader(lang, page, ruHref, enHref, base)}
        <main id="main" aria-label="${esc(UI[lang].sections)}">
${renderHero(page.hero, lang)}
${renderAbout(page.about, lang, page.hero.subtitle, base)}
${renderPortfolio({ ...page.portfolio, lang })}
${renderContact(page.contact)}
        </main>
        <footer class="footer">
            <div class="shell footer__inner">
                <div>
                    <p>${esc(page.footer.copyright)}</p>
                    <p class="footer__credits">${esc(page.footer.credits)}</p>
                </div>
                <a href="#top">${esc(page.footer.back_to_top)}</a>
            </div>
        </footer>
        <script type="module" src="${base}assets/main.js"></script>
    </body>
</html>
`;
}
