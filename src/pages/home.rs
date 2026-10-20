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
        <div class="relative min-h-screen bg-slate-950 font-sans text-slate-300 antialiased">
            <div aria-hidden="true" class="pointer-events-none fixed inset-0 overflow-hidden">
                <div class="absolute inset-0 bg-[radial-gradient(circle,rgba(148,163,184,0.10)_1px,transparent_1px)] bg-[length:32px_32px]"></div>
                <div class="absolute -top-32 left-1/2 h-[30rem] w-[30rem] -translate-x-1/2 rounded-full bg-emerald-500/10 blur-3xl"></div>
                <div class="absolute top-1/3 -right-40 h-[26rem] w-[26rem] rounded-full bg-teal-500/10 blur-3xl"></div>
                <div class="absolute -bottom-40 -left-32 h-[26rem] w-[26rem] rounded-full bg-emerald-500/5 blur-3xl"></div>
            </div>

            <LanguageSwitcher current_lang={props.lang.clone()} />
            <HeroSection language={props.lang.clone()} />
            <AboutSection language={props.lang.clone()} />
            <PortfolioSection language={props.lang.clone()} />
            <ContactSection language={props.lang.clone()} />
            <FooterSection language={props.lang.clone()} />
        </div>
    }
}
