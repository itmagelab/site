use yew::prelude::*;

use crate::content::{FooterContent, get_footer_content};
use crate::utils::dom::scroll_to_top;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub language: String,
}

#[function_component(FooterSection)]
pub fn footer_section(props: &Props) -> Html {
    let footer_content: &FooterContent = get_footer_content(&props.language);

    let back_to_top = Callback::from(move |_| scroll_to_top());

    html! {
        <footer class="relative border-t border-slate-800 px-6 py-12">
            <div class="mx-auto flex max-w-6xl flex-col items-center gap-6 text-center sm:flex-row sm:justify-between sm:text-left">
                <div>
                    <p class="text-slate-400">
                        { &footer_content.copyright }
                    </p>
                    <p class="mt-1 font-mono text-xs text-slate-600">
                        { &footer_content.credits }
                    </p>
                </div>

                <button
                    type="button"
                    onclick={back_to_top}
                    class="group inline-flex items-center gap-3 rounded-full border border-slate-700 bg-slate-900/60 px-5 py-3 text-sm font-medium text-slate-300 transition duration-300 hover:-translate-y-0.5 hover:border-emerald-400/50 hover:text-emerald-300 focus:outline-none focus-visible:ring-2 focus-visible:ring-emerald-300/60"
                >
                    { &footer_content.back_to_top }
                    <i class="fas fa-arrow-up transition duration-300 group-hover:-translate-y-0.5"></i>
                </button>
            </div>
        </footer>
    }
}
