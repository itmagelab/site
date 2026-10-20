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
    },
    en: {
        skip: "Skip to content",
        nav: "Main navigation",
        lang: "Language selection",
        sections: "Page sections",
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

function renderDescription(item) {
    if (typeof item === "string") return esc(item);
    const link = `<a class="project__link" href="${esc(item.link.url)}" target="_blank" rel="noopener noreferrer">` +
        `${esc(item.link.label)}${icons.external({ size: 14 })}</a>`;
    return `<span class="project__lead">${link}<span class="prose">${esc(item.text)}</span></span>`;
}

function renderHero(hero) {
    return `
            <section class="hero" id="hero">
                <div class="shell hero__inner">
                    <p class="badge">
                        <span class="badge__dot" aria-hidden="true"></span>${esc(hero.badge)}
                    </p>
                    <h1 class="hero__title">${esc(hero.title)}</h1>
                    <p class="hero__subtitle">
                        ${esc(hero.subtitle)}<span class="cursor" aria-hidden="true">▌</span>
                    </p>
                    <div class="hero__actions">
                        <a class="btn btn--primary" href="#portfolio">
                            ${esc(hero.cta_portfolio)}${icons.arrowDown({ size: 18 })}
                        </a>
                        <a class="btn btn--ghost" href="#contact">
                            ${esc(hero.cta_contact)}${icons.send({ size: 18 })}
                        </a>
                    </div>
                </div>
                <a class="scroll-cue" href="#about" aria-hidden="true" tabindex="-1">${icons.chevronDown({ size: 22 })}</a>
            </section>`;
}

function renderAbout(about, base) {
    const facts = about.skills
        .map(
            (skill) => `
                        <li class="fact">
                            ${icons.terminal({ size: 18 })}
                            <span>${esc(skill)}</span>
                        </li>`,
        )
        .join("");

    const experience = about.work_experience
        .map(
            (job) => `
                        <li class="timeline__item">
                            <div class="timeline__top">
                                <h4 class="timeline__position">${esc(job.position)}</h4>
                                <span class="pill">${esc(job.year)}</span>
                            </div>
                            <p class="timeline__company">${esc(job.company)}</p>
                            <ul class="bullets">
${job.description.map((line) => `                                <li>${esc(line)}</li>`).join("\n")}
                            </ul>
                        </li>`,
        )
        .join("");

    return `
            <section class="section" id="about">
                <div class="shell">
                    <div class="about">
                        <div class="about__text">
                            <p class="section__index">01</p>
                            <h2 class="section__title">${esc(about.title)}</h2>
                            <p class="prose">${esc(about.description)}</p>
                        </div>
                        <figure class="about__photo" data-reveal>
                            <img
                                src="${esc(asset(base, about.photo))}"
                                alt="${esc(about.title)}"
                                loading="lazy"
                                decoding="async"
                            />
                        </figure>
                    </div>

                    <ul class="facts" data-reveal>${facts}
                    </ul>

                    <h3 class="subsection__title" data-reveal>${esc(about.work_experience_title)}</h3>
                    <ol class="timeline" data-reveal>${experience}
                    </ol>
                </div>
            </section>`;
}

function renderPortfolio(portfolio) {
    const projects = portfolio.projects
        .map(
            (project, index) => `
                        <article class="project" data-reveal>
                            <header class="project__head">
                                <h4 class="project__name">
                                    <span class="project__index">${String(index + 1).padStart(2, "0")}</span>
                                    ${esc(project.name)}
                                </h4>${
                                    project.link
                                        ? `
                                <a class="project__cta" href="${esc(project.link)}" target="_blank" rel="noopener noreferrer">
                                    ${esc(portfolio.project_link_label)}${icons.external({ size: 14 })}
                                </a>`
                                        : ""
                                }
                            </header>
                            <ul class="bullets bullets--project">
${project.description.map((line) => `                                <li>${renderDescription(line)}</li>`).join("\n")}
                            </ul>
                            <ul class="techs">
${project.technologies.map((tech) => `                                <li class="chip">${esc(tech)}</li>`).join("\n")}
                            </ul>
                        </article>`,
        )
        .join("");

    const skills = portfolio.skills
        .map(
            (skill) => `
                        <li class="skill" data-skill style="--level: ${skill.level}%">
                            <div class="skill__row">
                                <span class="skill__name">${esc(skill.name)}</span>
                                <span class="skill__value">${skill.level}%</span>
                            </div>
                            <div class="skill__track" role="img" aria-label="${esc(skill.name)}: ${skill.level}%">
                                <span class="skill__bar"></span>
                            </div>
                        </li>`,
        )
        .join("");

    return `
            <section class="section" id="portfolio">
                <div class="shell">
                    <p class="section__index">02</p>
                    <h2 class="section__title">${esc(portfolio.title)}</h2>

                    <h3 class="subsection__title" data-reveal>${esc(portfolio.projects_title)}</h3>
                    <div class="projects">${projects}
                    </div>

                    <h3 class="subsection__title" data-reveal>${esc(portfolio.skills_title)}</h3>
                    <ul class="skills" data-reveal>${skills}
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
                                <span>${esc(social.name)}</span>${icons.external({ size: 16 })}
                            </a>
                        </li>`,
        )
        .join("");

    return `
            <section class="section" id="contact">
                <div class="shell">
                    <p class="section__index">03</p>
                    <h2 class="section__title">${esc(contact.title)}</h2>

                    <div class="contacts" data-reveal>
                        <a class="contact" href="${esc(telHref(contact.phone))}">
                            <span class="contact__icon" aria-hidden="true">${icons.phone({ size: 22 })}</span>
                            <span class="contact__label">${esc(contact.phone_label)}</span>
                            <span class="contact__value">${esc(contact.phone)}</span>
                        </a>
                        <a class="contact" href="mailto:${esc(contact.email)}">
                            <span class="contact__icon" aria-hidden="true">${icons.mail({ size: 22 })}</span>
                            <span class="contact__label">${esc(contact.email_label)}</span>
                            <span class="contact__value">${esc(contact.email)}</span>
                        </a>
                    </div>

                    <h3 class="subsection__title" data-reveal>${esc(contact.social_title)}</h3>
                    <ul class="socials" data-reveal>${socials}
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
                `<a class="nav__link" href="${item.href}" data-nav="${item.href.slice(1)}">${esc(item.label)}</a>`,
        )
        .join("\n                        ");

    const language = (code, href) =>
        `<a class="lang__item${code === lang ? " is-active" : ""}" href="${href}" hreflang="${code}"${
            code === lang ? ' aria-current="true"' : ""
        }>${code.toUpperCase()}</a>`;

    return `
        <header class="topbar">
            <div class="shell topbar__inner">
                <a class="brand" href="${base === "" ? "./" : base}">
                    <span class="brand__mark" aria-hidden="true"></span>
                    <span class="brand__text">${esc(content.hero.title)}</span>
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
        <meta name="theme-color" content="#0c0b10" media="(prefers-color-scheme: dark)" />
        <meta name="theme-color" content="#f4f0e8" media="(prefers-color-scheme: light)" />
        <meta prefix="og: https://ogp.me/ns#" property="og:type" content="website" />
        <meta prefix="og: https://ogp.me/ns#" property="og:title" content="${esc(title)}" />
        <meta prefix="og: https://ogp.me/ns#" property="og:description" content="${esc(description)}" />
        <meta name="twitter:card" content="summary" />
        <link rel="icon" href="${base}assets/favicon.svg" type="image/svg+xml" />
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
        <link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap"
        />
        <link rel="stylesheet" href="${base}assets/main.css" />
    </head>
    <body id="top">
        <a class="skip-link" href="#main">${esc(UI[lang].skip)}</a>
        <div class="backdrop" aria-hidden="true">
            <span class="backdrop__grid"></span>
            <span class="backdrop__orb backdrop__orb--one"></span>
            <span class="backdrop__orb backdrop__orb--two"></span>
            <span class="backdrop__orb backdrop__orb--three"></span>
        </div>
${renderHeader(lang, page, ruHref, enHref, base)}
        <main id="main" aria-label="${esc(UI[lang].sections)}">
${renderHero(page.hero)}
${renderAbout(page.about, base)}
${renderPortfolio(page.portfolio)}
${renderContact(page.contact)}
        </main>
        <footer class="footer">
            <div class="shell footer__inner">
                <div>
                    <p class="footer__copyright">${esc(page.footer.copyright)}</p>
                    <p class="footer__credits">${esc(page.footer.credits)}</p>
                </div>
                <a class="btn btn--ghost btn--small" href="#top">
                    ${esc(page.footer.back_to_top)}${icons.arrowUp({ size: 16 })}
                </a>
            </div>
        </footer>
        <script type="module" src="${base}assets/main.js"></script>
    </body>
</html>
`;
}
