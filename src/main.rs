use yew::prelude::*;

mod components;
use components::hero_section::HeroSection;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="min-h-screen">
            <HeroSection />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
