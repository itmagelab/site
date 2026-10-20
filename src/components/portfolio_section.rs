use yew::prelude::*;

use crate::content::{PortfolioContent, get_portfolio_content};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub language: String,
}

#[function_component(PortfolioSection)]
pub fn portfolio_section(props: &Props) -> Html {
    let portfolio_content: &PortfolioContent = get_portfolio_content(&props.language);

    html! {
        <section id="portfolio" class="relative px-6 py-24 sm:py-32">
            <div class="mx-auto max-w-6xl">
                <span class="font-mono text-sm tracking-[0.4em] text-emerald-400">{ "02" }</span>
                <h2 class="mt-4 text-4xl font-bold tracking-tight text-white sm:text-5xl">
                    { &portfolio_content.title }
                </h2>

                <div class="mt-14">
                    <h3 class="text-sm font-semibold tracking-[0.3em] text-slate-500 uppercase">
                        { &portfolio_content.projects_title }
                    </h3>

                    <div class="mt-8 grid gap-6 lg:grid-cols-2">
                        {
                            portfolio_content.projects.iter().map(|project| {
                                html! {
                                    <article class="group flex h-full flex-col rounded-3xl border border-slate-800 bg-slate-900/50 p-7 transition duration-300 hover:-translate-y-1 hover:border-emerald-400/40 hover:bg-slate-900">
                                        <div class="flex items-start justify-between gap-4">
                                            <h4 class="text-xl font-semibold leading-snug text-white">
                                                { &project.name }
                                            </h4>
                                            {
                                                if let Some(link) = &project.link {
                                                    html! {
                                                        <a
                                                            href={link.clone()}
                                                            target="_blank"
                                                            rel="noopener noreferrer"
                                                            class="inline-flex shrink-0 items-center gap-2 rounded-full border border-slate-700 px-4 py-2 text-xs font-semibold whitespace-nowrap text-slate-300 transition hover:border-emerald-400/50 hover:text-emerald-300"
                                                        >
                                                            <i class="fas fa-external-link-alt"></i>
                                                            <span>{ &portfolio_content.project_link_label }</span>
                                                        </a>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </div>

                                        <ul class="mt-5 flex-1 space-y-3 text-slate-400">
                                            {
                                                project.description.iter().map(|desc| {
                                                    html! {
                                                        <li class="flex gap-3">
                                                            <span class="mt-2.5 h-1.5 w-1.5 shrink-0 rounded-full bg-emerald-400/60"></span>
                                                            <span class="leading-relaxed">
                                                                { Html::from_html_unchecked(AttrValue::from(desc.clone())) }
                                                            </span>
                                                        </li>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </ul>

                                        <div class="mt-6 flex flex-wrap gap-2 border-t border-slate-800 pt-5">
                                            {
                                                project.technologies.iter().map(|tech| {
                                                    html! {
                                                        <span class="rounded-full border border-emerald-400/20 bg-emerald-400/5 px-3 py-1 font-mono text-xs text-emerald-300">
                                                            { tech }
                                                        </span>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>
                                    </article>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>

                <div class="mt-20">
                    <h3 class="text-sm font-semibold tracking-[0.3em] text-slate-500 uppercase">
                        { &portfolio_content.skills_title }
                    </h3>

                    <div class="mt-8 grid gap-x-10 gap-y-7 md:grid-cols-2">
                        {
                            portfolio_content.skills.iter().map(|skill| {
                                html! {
                                    <div>
                                        <div class="mb-2 flex items-baseline justify-between gap-4">
                                            <span class="font-medium text-slate-200">
                                                { &skill.name }
                                            </span>
                                            <span class="font-mono text-sm text-emerald-300">
                                                { format!("{}%", skill.level) }
                                            </span>
                                        </div>
                                        <div class="h-2 w-full overflow-hidden rounded-full bg-slate-800">
                                            <div
                                                class="h-full rounded-full bg-gradient-to-r from-emerald-500 to-teal-300"
                                                style={format!("width: {}%", skill.level)}
                                            ></div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </section>
    }
}
