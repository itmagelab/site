use yew::prelude::*;

use crate::content::{AboutContent, get_about_content};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub language: String,
}

#[function_component(AboutSection)]
pub fn about_section(props: &Props) -> Html {
    let about_content: &AboutContent = get_about_content(&props.language);

    html! {
        <section id="about" class="relative px-6 py-24 sm:py-32">
            <div class="mx-auto max-w-6xl">
                <div class="grid items-center gap-12 lg:grid-cols-[1.3fr_1fr] lg:gap-16">
                    <div>
                        <span class="font-mono text-sm tracking-[0.4em] text-emerald-400">{ "01" }</span>
                        <h2 class="mt-4 text-4xl font-bold tracking-tight text-white sm:text-5xl">
                            { &about_content.title }
                        </h2>
                        <p class="mt-6 text-lg leading-relaxed whitespace-pre-line text-slate-400">
                            { &about_content.description }
                        </p>
                    </div>

                    <div class="mx-auto w-full max-w-sm">
                        <div class="rounded-[2rem] bg-gradient-to-br from-emerald-400/60 via-teal-400/20 to-transparent p-[2px] shadow-2xl shadow-emerald-500/10">
                            <img
                                src={about_content.photo.clone()}
                                alt={about_content.title.clone()}
                                class="aspect-square w-full rounded-[2rem] object-cover"
                            />
                        </div>
                    </div>
                </div>

                <div class="mt-16 grid gap-4 sm:grid-cols-2">
                    {
                        about_content.skills.iter().map(|item| {
                            html! {
                                <div class="flex items-start gap-4 rounded-2xl border border-slate-800 bg-slate-900/50 p-5 transition duration-300 hover:border-emerald-400/40 hover:bg-slate-900">
                                    <i class="fas fa-terminal mt-1 text-emerald-400"></i>
                                    <span class="leading-relaxed text-slate-300">{ item }</span>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>

                <div class="mt-24">
                    <h3 class="text-2xl font-semibold tracking-tight text-white sm:text-3xl">
                        { &about_content.work_experience_title }
                    </h3>

                    <ol class="relative mt-10 space-y-12 border-l border-slate-800 pl-6 sm:pl-8">
                        {
                            about_content.work_experience.iter().map(|exp| {
                                html! {
                                    <li class="relative">
                                        <span class="absolute top-3 -left-6 h-3 w-3 -translate-x-1/2 rounded-full bg-emerald-400 ring-4 ring-emerald-400/15 sm:-left-8"></span>
                                        <div class="flex flex-col gap-3 sm:flex-row sm:items-baseline sm:justify-between sm:gap-6">
                                            <h4 class="text-xl font-semibold text-white">
                                                { &exp.position }
                                            </h4>
                                            <span class="inline-flex w-fit shrink-0 items-center rounded-full border border-emerald-400/25 bg-emerald-400/10 px-3 py-1 font-mono text-xs text-emerald-300">
                                                { &exp.year }
                                            </span>
                                        </div>
                                        <p class="mt-2 font-medium text-emerald-400/90">
                                            { &exp.company }
                                        </p>
                                        <ul class="mt-4 space-y-3 text-slate-400">
                                            {
                                                exp.description.iter().map(|desc| {
                                                    html! {
                                                        <li class="flex gap-3">
                                                            <span class="mt-2.5 h-1.5 w-1.5 shrink-0 rounded-full bg-slate-600"></span>
                                                            <span class="leading-relaxed">
                                                                { Html::from_html_unchecked(AttrValue::from(desc.clone())) }
                                                            </span>
                                                        </li>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </ul>
                                    </li>
                                }
                            }).collect::<Html>()
                        }
                    </ol>
                </div>
            </div>
        </section>
    }
}
