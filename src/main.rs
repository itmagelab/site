use yew::prelude::*;

mod components;
use components::about_section::AboutSection;
use components::contact_section::ContactSection;
use components::hero_section::HeroSection;
use components::language_switcher::LanguageSwitcher;
use components::portfolio_section::PortfolioSection;
use components::footer_section::FooterSection;

#[function_component(App)]
fn app() -> Html {
    let language = use_state(|| "ru".to_string());

    let on_language_change = {
        let language = language.clone();
        Callback::from(move |new_lang: String| {
            language.set(new_lang);
        })
    };

    html! {
        <div class="flex flex-col min-h-screen">
            <LanguageSwitcher
                current_lang={(*language).clone()}
                on_change={on_language_change}
            />
            <HeroSection language={(*language).clone()} />
            <AboutSection language={(*language).clone()} />
            <PortfolioSection language={(*language).clone()} />
            <ContactSection language={(*language).clone()} />
            <FooterSection language={(*language).clone()} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
