use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_lang: String,
    pub on_change: Callback<String>,
}

#[function_component(LanguageSwitcher)]
pub fn language_switcher(props: &Props) -> Html {
    let on_ru = {
        let on_change = props.on_change.clone();
        Callback::from(move |_| on_change.emit("ru".to_string()))
    };

    let on_en = {
        let on_change = props.on_change.clone();
        Callback::from(move |_| on_change.emit("en".to_string()))
    };

    html! {
        <div class="fixed top-4 right-4 z-50 flex gap-2 bg-gray-800 rounded-lg p-2 shadow-lg">
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
}
