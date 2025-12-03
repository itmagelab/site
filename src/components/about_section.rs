use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Deserialize, Clone, PartialEq)]
pub struct WorkExperience {
    pub year: String,
    pub position: String,
    pub company: String,
    pub description: Vec<String>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct AboutContent {
    pub photo: String,
    pub title: String,
    pub description: String,
    pub skills: Vec<String>,
    pub work_experience_title: String,
    pub work_experience: Vec<WorkExperience>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct LangContent {
    pub about: AboutContent,
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

#[function_component(AboutSection)]
pub fn about_section(props: &Props) -> Html {
    let content = use_state(|| None::<AboutContent>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    // Загрузка данных из YAML
    {
        let content = content.clone();
        let error = error.clone();
        let loading = loading.clone();

        let language = props.language.clone();
        use_effect_with(props.language.clone(), move |_| {
            spawn_local(async move {
                match load_content(&language).await {
                    Ok(data) => {
                        let about_content = if language == "ru" {
                            data.ru.about
                        } else {
                            data.en.about
                        };
                        content.set(Some(about_content));
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
            <section class="min-h-screen bg-white font-sansation flex items-center justify-center">
                <div class="text-center">
                    <div class="text-2xl text-gray-800 animate-pulse">{ "Loading..." }</div>
                </div>
            </section>
        };
    }

    if let Some(err) = (*error).as_ref() {
        return html! {
            <section class="min-h-screen bg-white font-sansation flex items-center justify-center">
                <div class="text-center">
                    <div class="text-2xl text-red-600 mb-4">{ "Error loading content" }</div>
                    <div class="text-sm text-gray-700">{ err }</div>
                </div>
            </section>
        };
    }

    if let Some(about_content) = (*content).as_ref() {
        html! {
            <section class="min-h-screen bg-white font-sansation py-16 px-4 flex items-center">
                <div class="max-w-6xl mx-auto w-full">
                    <div class="bg-green-200 rounded-lg p-8 md:p-12 shadow-lg">
                        <div class="flex flex-col md:flex-row gap-8 md:gap-12 items-center md:items-start mb-12">
                            // Текст слева
                            <div class="flex-1 text-center md:text-left">
                                <h2 class="text-4xl sm:text-5xl md:text-6xl font-bold mb-6 text-gray-800">
                                    { &about_content.title }
                                </h2>
                                <p class="text-lg sm:text-xl md:text-2xl text-gray-700 mb-8 leading-relaxed">
                                    { &about_content.description }
                                </p>
                                <ul class="space-y-4 text-left">
                                    {
                                        about_content.skills.iter().map(|item| {
                                            html! {
                                                <li class="flex items-start gap-3">
                                                    <i class="fas fa-check-circle text-gray-800 text-xl mt-1 flex-shrink-0"></i>
                                                    <span class="text-lg sm:text-xl text-gray-700">{ item }</span>
                                                </li>
                                            }
                                        }).collect::<Html>()
                                    }
                                </ul>
                            </div>

                            // Фото справа
                            <div class="w-48 h-48 sm:w-64 sm:h-64 md:w-80 md:h-80 flex-shrink-0">
                                <img
                                    src={about_content.photo.clone()}
                                    alt={about_content.title.clone()}
                                    class="w-full h-full object-cover rounded-lg shadow-xl border-4 border-gray-800"
                                />
                            </div>
                        </div>

                        // Опыт работы по годам
                        <div class="border-t-2 border-gray-800 pt-8">
                            <h3 class="text-3xl sm:text-4xl md:text-5xl font-bold mb-8 text-gray-800 text-center md:text-left">
                                { &about_content.work_experience_title }
                            </h3>
                            <div class="space-y-6">
                                {
                                    about_content.work_experience.iter().map(|exp| {
                                        html! {
                                            <div class="flex flex-col sm:flex-row gap-4 sm:gap-6">
                                                <div class="text-2xl sm:text-3xl font-bold text-gray-800 sm:w-32 flex-shrink-0 text-center sm:text-left">
                                                    { &exp.year }
                                                </div>
                                                <div class="flex-1">
                                                    <h4 class="text-xl sm:text-2xl font-bold text-gray-800 mb-2">
                                                        { &exp.position }
                                                    </h4>
                                                    <p class="text-lg sm:text-xl text-gray-700 mb-2 font-semibold">
                                                        { &exp.company }
                                                    </p>
                                                    <ul class="list-disc list-inside text-base sm:text-lg text-gray-600 leading-relaxed space-y-1">
                                                        {
                                                            exp.description.iter().map(|desc| {
                                                                html! {
                                                                    <li>{ Html::from_html_unchecked(AttrValue::from(desc.clone())) }</li>
                                                                }
                                                            }).collect::<Html>()
                                                        }
                                                    </ul>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        }
    } else {
        html! {
            <section class="min-h-screen bg-white font-sansation flex items-center justify-center">
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
