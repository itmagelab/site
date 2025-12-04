use yew::prelude::*;

use crate::content::{get_hero_content, HeroContent};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub language: String,
}

#[function_component(HeroSection)]
pub fn hero_section(props: &Props) -> Html {
    let hero_content: &HeroContent = get_hero_content(&props.language);

    html! {
        <section class="bg-green-200 font-sansation py-6 px-4 shadow-md">
            <div class="flex flex-row items-center justify-between gap-6 w-full max-w-6xl mx-auto">
                <div class="flex items-center gap-4">
                    <div class="w-12 h-12 sm:w-16 sm:h-16 bg-gray-800 rounded-lg flex-shrink-0 flex items-center justify-center">
                        <i class="fas fa-terminal text-green-300 text-2xl sm:text-3xl"></i>
                    </div>
                    <div class="text-left">
                        <h1 class="text-2xl sm:text-3xl md:text-4xl font-bold tracking-wider text-gray-800">
                            { &hero_content.title }
                        </h1>
                        <p class="text-sm sm:text-base md:text-lg text-gray-700">
                            { &hero_content.subtitle }
                            <span class="ml-2 animate-pulse">{ "▋" }</span>
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

