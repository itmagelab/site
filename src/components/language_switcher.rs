use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_lang: String,
}

#[function_component(LanguageSwitcher)]
pub fn language_switcher(props: &Props) -> Html {
    let navigator = use_navigator();

    let on_ru = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            if let Some(navigator) = navigator.as_ref() {
                navigator.push(&Route::Ru);
            }
        })
    };

    let on_en = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            if let Some(navigator) = navigator.as_ref() {
                navigator.push(&Route::En);
            }
        })
    };

    let button_class = |is_active: bool| {
        let base = "rounded-full px-4 py-1.5 font-mono text-xs font-bold tracking-[0.2em] transition duration-300";
        if is_active {
            format!("{base} bg-emerald-400 text-slate-950 shadow shadow-emerald-500/30")
        } else {
            format!("{base} text-slate-400 hover:text-emerald-300")
        }
    };

    html! {
        <div class="fixed top-5 right-5 z-50">
            <div class="flex items-center gap-1 rounded-full border border-slate-700/80 bg-slate-900/80 p-1 shadow-lg shadow-black/40 backdrop-blur">
                <button type="button" onclick={on_ru} class={button_class(props.current_lang == "ru")}>
                    { "RU" }
                </button>
                <button type="button" onclick={on_en} class={button_class(props.current_lang == "en")}>
                    { "EN" }
                </button>
            </div>
        </div>
    }
}
