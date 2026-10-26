//! Встроенные SVG-иконки: без внешних иконных шрифтов и CDN.
//!
//! Единственная иконка — «ссылка наружу»: она поясняет, что переход уведёт
//! с сайта. Остальные смыслы несут типографика и линейки, поэтому набор
//! намеренно короткий.

const svg = (body, { size = 20 } = {}) =>
    `<svg class="icon" viewBox="0 0 24 24" width="${size}" height="${size}" fill="none" ` +
    `stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" ` +
    `aria-hidden="true" focusable="false">${body}</svg>`;

export const icons = {
    external: (opts) =>
        svg(
            '<path d="M15 3h6v6"/><path d="M10.5 13.5 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>',
            opts,
        ),
};
