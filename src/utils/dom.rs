//! Вспомогательные функции для работы с DOM.

use web_sys::window;

/// Прокручивает страницу к элементу с указанным `id`.
///
/// Если окно, документ или элемент недоступны, функция ничего не делает.
pub fn scroll_to_id(id: &str) {
    let Some(element) = window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id(id))
    else {
        return;
    };

    element.scroll_into_view();
}

/// Прокручивает страницу в самое начало.
pub fn scroll_to_top() {
    if let Some(window) = window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }
}
