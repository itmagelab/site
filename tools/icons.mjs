//! Набор встроенных SVG-иконок: без внешних иконных шрифтов и CDN.

const svg = (body, { size = 20, extra = "" } = {}) =>
    `<svg class="icon${extra}" viewBox="0 0 24 24" width="${size}" height="${size}" fill="none" ` +
    `stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round" ` +
    `aria-hidden="true" focusable="false">${body}</svg>`;

export const icons = {
    arrowDown: (opts) => svg('<path d="M12 5v14"/><path d="m19 12-7 7-7-7"/>', opts),
    arrowUp: (opts) => svg('<path d="M12 19V5"/><path d="m5 12 7-7 7 7"/>', opts),
    send: (opts) => svg('<path d="m22 2-7 20-4-9-9-4Z"/><path d="M22 2 11 13"/>', opts),
    phone: (opts) =>
        svg(
            '<path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.8 19.8 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6A19.8 19.8 0 0 1 2.12 4.2 2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72c.13.96.36 1.9.7 2.81a2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45c.91.34 1.85.57 2.81.7A2 2 0 0 1 22 16.92Z"/>',
            opts,
        ),
    mail: (opts) =>
        svg('<rect x="2" y="4" width="20" height="16" rx="3"/><path d="m3 7 9 6 9-6"/>', opts),
    terminal: (opts) => svg('<path d="m4 17 6-6-6-6"/><path d="M12 19h8"/>', opts),
    external: (opts) =>
        svg(
            '<path d="M15 3h6v6"/><path d="M10.5 13.5 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>',
            opts,
        ),
    chevronDown: (opts) => svg('<path d="m6 9 6 6 6-6"/>', opts),
    spark: (opts) => svg('<path d="M12 3v18"/><path d="M3 12h18"/><path d="m6 6 12 12"/><path d="m18 6-12 12"/>', opts),
};
