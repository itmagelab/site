use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_lang: String,
    pub on_change: Callback<String>,
}

#[function_component(LanguageSwitcher)]
pub fn language_switcher(props: &Props) -> Html {
    let is_open = use_state(|| false);

    let toggle_open = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let on_ru = {
        let on_change = props.on_change.clone();
        let is_open = is_open.clone();
        Callback::from(move |_| {
            on_change.emit("ru".to_string());
            is_open.set(false);
        })
    };

    let on_en = {
        let on_change = props.on_change.clone();
        let is_open = is_open.clone();
        Callback::from(move |_| {
            on_change.emit("en".to_string());
            is_open.set(false);
        })
    };

    html! {
        <div class="fixed top-4 right-4 z-50">
            // Toggle button
            <button
                onclick={toggle_open}
                class="bg-gray-800 text-green-300 px-4 py-2 rounded-lg shadow-lg font-semibold hover:bg-gray-700 transition-colors"
            >
                { &props.current_lang.to_uppercase() }
                <i class="fas fa-globe ml-2"></i>
            </button>

            // Language options (shown when open)
            if *is_open {
                <div class="absolute top-12 right-0 flex flex-col gap-2 bg-gray-800 rounded-lg p-2 shadow-lg">
                    <button
                        onclick={on_ru}
                        class={format!(
                            "px-4 py-2 rounded font-semibold transition-colors {}",
                            if props.current_lang == "ru" {
                                "bg-green-300 text-gray-800"
                            } else {
                                "bg-gray-700 text-green-300 hover:bg-gray-600"
                            }
                        )}
                    >
                        { "RU" }
                    </button>
                    <button
                        onclick={on_en}
                        class={format!(
                            "px-4 py-2 rounded font-semibold transition-colors {}",
                            if props.current_lang == "en" {
                                "bg-green-300 text-gray-800"
                            } else {
                                "bg-gray-700 text-green-300 hover:bg-gray-600"
                            }
                        )}
                    >
                        { "EN" }
                    </button>
                </div>
            }
        </div>
    }
}
