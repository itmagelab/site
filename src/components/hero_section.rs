use yew::prelude::*;

use crate::content::{HeroContent, get_hero_content};
use crate::utils::dom::scroll_to_id;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub language: String,
}

#[function_component(HeroSection)]
pub fn hero_section(props: &Props) -> Html {
    let hero_content: &HeroContent = get_hero_content(&props.language);

    let scroll_to_portfolio = Callback::from(move |_| scroll_to_id("portfolio"));
    let scroll_to_contact = Callback::from(move |_| scroll_to_id("contact"));

    html! {
        <section class="relative flex min-h-screen items-center justify-center px-6 py-24">
            <div class="mx-auto w-full max-w-4xl text-center">
                <span class="inline-flex items-center gap-3 rounded-full border border-emerald-400/25 bg-emerald-400/10 px-4 py-1.5 font-mono text-[0.7rem] font-medium tracking-[0.25em] text-emerald-300 uppercase">
                    <span class="h-1.5 w-1.5 animate-pulse rounded-full bg-emerald-400"></span>
                    { &hero_content.badge }
                </span>

                <h1 class="mt-8 font-sansation text-5xl font-bold tracking-tight sm:text-7xl md:text-8xl">
                    <span class="bg-gradient-to-r from-emerald-300 via-teal-200 to-emerald-400 bg-clip-text text-transparent">
                        { &hero_content.title }
                    </span>
                </h1>

                <p class="mt-6 font-mono text-base text-slate-400 sm:text-lg md:text-xl">
                    { &hero_content.subtitle }
                    <span class="ml-2 inline-block animate-blink text-emerald-400">{ "▌" }</span>
                </p>

                <div class="mt-12 flex flex-col items-center justify-center gap-4 sm:flex-row">
                    <button
                        type="button"
                        onclick={scroll_to_portfolio}
                        class="group inline-flex w-full items-center justify-center gap-3 rounded-full bg-gradient-to-r from-emerald-400 to-teal-400 px-8 py-4 text-base font-semibold text-slate-950 shadow-lg shadow-emerald-500/20 transition duration-300 hover:-translate-y-0.5 hover:shadow-xl hover:shadow-emerald-500/30 focus:outline-none focus-visible:ring-2 focus-visible:ring-emerald-300 sm:w-auto"
                    >
                        { &hero_content.cta_portfolio }
                        <i class="fas fa-arrow-down transition duration-300 group-hover:translate-y-1"></i>
                    </button>

                    <button
                        type="button"
                        onclick={scroll_to_contact}
                        class="group inline-flex w-full items-center justify-center gap-3 rounded-full border border-slate-700 bg-slate-900/60 px-8 py-4 text-base font-semibold text-slate-200 backdrop-blur transition duration-300 hover:-translate-y-0.5 hover:border-emerald-400/50 hover:text-emerald-300 focus:outline-none focus-visible:ring-2 focus-visible:ring-emerald-300/60 sm:w-auto"
                    >
                        { &hero_content.cta_contact }
                        <i class="fas fa-paper-plane transition duration-300 group-hover:translate-x-1"></i>
                    </button>
                </div>
            </div>

            <div class="pointer-events-none absolute bottom-8 left-1/2 hidden -translate-x-1/2 md:block">
                <i class="fas fa-chevron-down animate-float text-xl text-slate-600"></i>
            </div>
        </section>
    }
}
