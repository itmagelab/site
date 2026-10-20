//! Небольшое дополнение к статической странице: мягкое появление блоков,
//! «оживление» полос навыков и подсветка текущего раздела в меню.
//!
//! Без JavaScript сайт полностью читается: стили и контент уже в HTML,
//! скрипт только добавляет анимации. Класс `js` ставится именно здесь, поэтому
//! при сбое скрипта блоки остаются видимыми, а не скрытыми.

document.documentElement.classList.add("js");

const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;


function observeSections() {
    const sections = document.querySelectorAll("section[id]");
    const links = new Map(
        [...document.querySelectorAll("[data-nav]")].map((link) => [link.dataset.nav, link]),
    );
    let current = null;

    const pick = () => {
        let active = null;
        const line = window.scrollY + window.innerHeight * 0.35;
        sections.forEach((section) => {
            if (section.offsetTop <= line) active = section.id;
        });

        if (active === current) return;
        current = active;
        links.forEach((link, id) => {
            if (id === active) link.setAttribute("aria-current", "true");
            else link.removeAttribute("aria-current");
        });
    };

    pick();
    window.addEventListener("scroll", pick, { passive: true });
    window.addEventListener("resize", pick);
}

if (reduceMotion) {
    document.querySelectorAll("[data-reveal], [data-skill]").forEach((item) => {
        item.classList.add("is-visible");
    });
} else {
    const observer = new IntersectionObserver(
        (entries) => {
            entries.forEach((entry) => {
                if (!entry.isIntersecting) return;
                entry.target.classList.add("is-visible");
                observer.unobserve(entry.target);
            });
        },
        { rootMargin: "0px 0px -12% 0px", threshold: 0.15 },
    );

    document.querySelectorAll("[data-reveal], [data-skill]").forEach((item) => observer.observe(item));
}

observeSections();
