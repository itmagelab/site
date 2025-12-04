use yew::prelude::*;

use crate::content::{get_portfolio_content, PortfolioContent};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub language: String,
}

#[function_component(PortfolioSection)]
pub fn portfolio_section(props: &Props) -> Html {
    let portfolio_content: &PortfolioContent = get_portfolio_content(&props.language);

    html! {
        <section class="min-h-screen bg-green-200 font-sansation py-16 px-4 flex items-center">
            <div class="max-w-6xl mx-auto w-full">
                <h2 class="text-4xl sm:text-5xl md:text-6xl font-bold mb-12 text-gray-800 text-center">
                    { &portfolio_content.title }
                </h2>

                // Проекты
                <div class="mb-16">
                    <h3 class="text-3xl sm:text-4xl font-bold mb-8 text-gray-800">
                        { &portfolio_content.projects_title }
                    </h3>
                    <div class="space-y-6">
                        {
                            portfolio_content.projects.iter().map(|project| {
                                html! {
                                    <div class="bg-white border-2 border-gray-800 rounded-lg p-6 shadow-lg">
                                        <div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-4 mb-4">
                                            <h4 class="text-2xl font-bold text-gray-800">
                                                { &project.name }
                                            </h4>
                                            {
                                                if let Some(link) = &project.link {
                                                    html! {
                                                        <a
                                                            href={link.clone()}
                                                            target="_blank"
                                                            class="inline-flex items-center gap-2 text-gray-700 hover:text-gray-900 transition-colors"
                                                        >
                                                            <i class="fas fa-external-link-alt"></i>
                                                            <span class="text-sm font-semibold">{ "View Project" }</span>
                                                        </a>
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </div>
                                        <ul class="list-disc list-inside text-lg text-gray-700 mb-4">
                                            {
                                                project.description.iter().map(|desc| {
                                                    html! {
                                                        <li>{ Html::from_html_unchecked(AttrValue::from(desc.clone())) }</li>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </ul>
                                        <div class="flex flex-wrap gap-2">
                                            {
                                                project.technologies.iter().map(|tech| {
                                                    html! {
                                                        <span class="px-3 py-1 bg-gray-800 text-green-300 rounded-full text-sm font-semibold">
                                                            { tech }
                                                        </span>
                                                    }
                                                }).collect::<Html>()
                                            }
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>

                // Навыки
                <div>
                    <h3 class="text-3xl sm:text-4xl font-bold mb-8 text-gray-800">
                        { &portfolio_content.skills_title }
                    </h3>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        {
                            portfolio_content.skills.iter().map(|skill| {
                                html! {
                                    <div>
                                        <div class="flex justify-between items-center mb-2">
                                            <span class="text-lg font-semibold text-gray-800">
                                                { &skill.name }
                                            </span>
                                            <span class="text-lg font-bold text-gray-700">
                                                { format!("{}%", skill.level) }
                                            </span>
                                        </div>
                                        <div class="w-full bg-gray-300 rounded-full h-4 overflow-hidden border-2 border-gray-800">
                                            <div
                                                class="bg-gray-800 h-full transition-all duration-300"
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

