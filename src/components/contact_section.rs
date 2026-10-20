use yew::prelude::*;

use crate::content::{ContactContent, get_contact_content};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub language: String,
}

#[function_component(ContactSection)]
pub fn contact_section(props: &Props) -> Html {
    let contact_content: &ContactContent = get_contact_content(&props.language);

    html! {
        <section id="contact" class="relative px-6 py-24 sm:py-32">
            <div class="mx-auto max-w-4xl">
                <span class="font-mono text-sm tracking-[0.4em] text-emerald-400">{ "03" }</span>
                <h2 class="mt-4 text-4xl font-bold tracking-tight text-white sm:text-5xl">
                    { &contact_content.title }
                </h2>

                <div class="mt-12 grid gap-6 sm:grid-cols-2">
                    <a
                        href={format!("tel:{}", contact_content.phone.replace(&[' ', '(', ')', '-'][..], ""))}
                        class="group flex flex-col items-start gap-5 rounded-3xl border border-slate-800 bg-slate-900/50 p-8 transition duration-300 hover:-translate-y-1 hover:border-emerald-400/40 hover:bg-slate-900"
                    >
                        <span class="flex h-14 w-14 items-center justify-center rounded-2xl bg-emerald-400/10 text-emerald-300 ring-1 ring-emerald-400/20">
                            <i class="fas fa-phone text-lg"></i>
                        </span>
                        <span class="text-xs font-semibold tracking-[0.3em] text-slate-500 uppercase">
                            { &contact_content.phone_label }
                        </span>
                        <span class="text-xl font-semibold text-white transition group-hover:text-emerald-300">
                            { &contact_content.phone }
                        </span>
                    </a>

                    <a
                        href={format!("mailto:{}", contact_content.email)}
                        class="group flex flex-col items-start gap-5 rounded-3xl border border-slate-800 bg-slate-900/50 p-8 transition duration-300 hover:-translate-y-1 hover:border-emerald-400/40 hover:bg-slate-900"
                    >
                        <span class="flex h-14 w-14 items-center justify-center rounded-2xl bg-emerald-400/10 text-emerald-300 ring-1 ring-emerald-400/20">
                            <i class="fas fa-envelope text-lg"></i>
                        </span>
                        <span class="text-xs font-semibold tracking-[0.3em] text-slate-500 uppercase">
                            { &contact_content.email_label }
                        </span>
                        <span class="text-xl font-semibold break-all text-white transition group-hover:text-emerald-300">
                            { &contact_content.email }
                        </span>
                    </a>
                </div>

                <div class="mt-16">
                    <h3 class="text-xs font-semibold tracking-[0.3em] text-slate-500 uppercase">
                        { &contact_content.social_title }
                    </h3>
                    <div class="mt-6 flex flex-wrap gap-4">
                        {
                            contact_content.social_networks.iter().map(|social| {
                                html! {
                                    <a
                                        href={social.url.clone()}
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="inline-flex items-center gap-3 rounded-2xl border border-slate-800 bg-slate-900/50 px-6 py-4 transition duration-300 hover:-translate-y-1 hover:border-emerald-400/40 hover:bg-slate-900"
                                    >
                                        <i class={format!("{} text-2xl text-emerald-300", social.icon)}></i>
                                        <span class="font-medium text-slate-200">{ &social.name }</span>
                                    </a>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </section>
    }
}
