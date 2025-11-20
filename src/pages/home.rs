use crate::components::about_section::AboutSection;
use crate::components::contact_section::ContactSection;
use crate::components::footer_section::FooterSection;
use crate::components::hero_section::HeroSection;
use crate::components::language_switcher::LanguageSwitcher;
use crate::components::portfolio_section::PortfolioSection;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub lang: String,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    html! {
        <div class="flex flex-col min-h-screen">
            <LanguageSwitcher current_lang={props.lang.clone()} />
            <HeroSection language={props.lang.clone()} />
            <AboutSection language={props.lang.clone()} />
            <PortfolioSection language={props.lang.clone()} />
            <ContactSection language={props.lang.clone()} />
            <FooterSection language={props.lang.clone()} />
        </div>
    }
}
