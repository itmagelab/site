use yew::prelude::*;

mod components;
use components::about_section::AboutSection;
use components::hero_section::HeroSection;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <HeroSection />
            <AboutSection />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
