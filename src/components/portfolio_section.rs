use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub link: Option<String>,
    pub technologies: Vec<String>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Skill {
    pub name: String,
    pub level: u8,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct PortfolioContent {
    pub title: String,
    pub projects_title: String,
    pub skills_title: String,
    pub projects: Vec<Project>,
    pub skills: Vec<Skill>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct LangContent {
    pub portfolio: PortfolioContent,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Content {
    pub ru: LangContent,
    pub en: LangContent,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub language: String,
}

#[function_component(PortfolioSection)]
pub fn portfolio_section(props: &Props) -> Html {
    let content = use_state(|| None::<PortfolioContent>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let content = content.clone();
        let error = error.clone();
        let loading = loading.clone();

        let language = props.language.clone();
        use_effect_with(props.language.clone(), move |_| {
            spawn_local(async move {
                match load_content(&language).await {
                    Ok(data) => {
                        let portfolio_content = if language == "ru" {
                            data.ru.portfolio
                        } else {
                            data.en.portfolio
                        };
                        content.set(Some(portfolio_content));
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    if *loading {
        return html! {
            <section class="min-h-screen bg-green-200 font-sansation flex items-center justify-center">
                <div class="text-center">
                    <div class="text-2xl text-gray-800 animate-pulse">{ "Loading..." }</div>
                </div>
            </section>
        };
    }

    if let Some(err) = (*error).as_ref() {
        return html! {
            <section class="min-h-screen bg-green-200 font-sansation flex items-center justify-center">
                <div class="text-center">
                    <div class="text-2xl text-red-600 mb-4">{ "Error loading content" }</div>
                    <div class="text-sm text-gray-700">{ err }</div>
                </div>
            </section>
        };
    }

    if let Some(portfolio_content) = (*content).as_ref() {
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
                                            <p class="text-lg text-gray-700 mb-4">
                                                { &project.description }
                                            </p>
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
    } else {
        html! {
            <section class="min-h-screen bg-green-200 font-sansation flex items-center justify-center">
                <div class="text-center">
                    <div class="text-2xl text-gray-800">{ "No content available" }</div>
                </div>
            </section>
        }
    }
}

async fn load_content(_language: &str) -> Result<Content, String> {
    let response = Request::get("/static/content.yaml")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch content: {}", e))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response text: {}", e))?;

    serde_yaml::from_str(&text).map_err(|e| format!("Failed to parse YAML: {}", e))
}
