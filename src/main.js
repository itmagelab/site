//! Единственный скрипт страницы: подсветка текущего раздела в меню.
//!
//! Скрипт необязательный: без JavaScript страница читается полностью, ничего
//! не скрыто и не спрятано. Никаких «проявлений» блоков при прокрутке —
//! текст виден сразу, как в документе.

const sections = [...document.querySelectorAll("main section[id]")];
const links = new Map(
    [...document.querySelectorAll("[data-nav]")].map((link) => [link.dataset.nav, link]),
);

let current = null;
let queued = false;

function pick() {
    queued = false;

    // Текущим считается раздел, который уже прошёл верхнюю границу экрана.
    const line = window.scrollY + Math.min(window.innerHeight * 0.3, 240);
    let active = null;
    sections.forEach((section) => {
        if (section.offsetTop <= line) active = section.id;
    });

    if (active === current) return;
    current = active;
    links.forEach((link, id) => {
        if (id === active) link.setAttribute("aria-current", "true");
        else link.removeAttribute("aria-current");
    });
}

function schedule() {
    if (queued) return;
    queued = true;
    window.requestAnimationFrame(pick);
}

if (sections.length > 0 && links.size > 0) {
    pick();
    window.addEventListener("scroll", schedule, { passive: true });
    window.addEventListener("resize", schedule);
}
