use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Deserialize, Clone, PartialEq)]
pub struct SocialNetwork {
    pub name: String,
    pub url: String,
    pub icon: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct ContactContent {
    pub title: String,
    pub phone: String,
    pub phone_label: String,
    pub email: String,
    pub email_label: String,
    pub social_title: String,
    pub social_networks: Vec<SocialNetwork>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct LangContent {
    pub contact: ContactContent,
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

#[function_component(ContactSection)]
pub fn contact_section(props: &Props) -> Html {
    let content = use_state(|| None::<ContactContent>);
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
                        let contact_content = if language == "ru" {
                            data.ru.contact
                        } else {
                            data.en.contact
                        };
                        content.set(Some(contact_content));
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
            <section class="min-h-screen bg-gray-800 font-sansation flex items-center justify-center">
                <div class="text-center">
                    <div class="text-2xl text-green-300 animate-pulse">{ "Loading..." }</div>
                </div>
            </section>
        };
    }

    if let Some(err) = (*error).as_ref() {
        return html! {
            <section class="min-h-screen bg-gray-800 font-sansation flex items-center justify-center">
                <div class="text-center">
                    <div class="text-2xl text-red-400 mb-4">{ "Error loading content" }</div>
                    <div class="text-sm text-green-300">{ err }</div>
                </div>
            </section>
        };
    }

    if let Some(contact_content) = (*content).as_ref() {
        html! {
            <section class="min-h-screen bg-gray-800 font-sansation py-16 px-4 flex items-center">
                <div class="max-w-4xl mx-auto w-full text-center">
                    <h2 class="text-4xl sm:text-5xl md:text-6xl font-bold mb-12 text-green-300">
                        { &contact_content.title }
                    </h2>

                    // Контактная информация
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mb-16">
                        // Телефон
                        <div class="bg-gray-900 rounded-lg p-8 border-2 border-green-300 hover:border-green-400 transition-colors">
                            <div class="text-green-300 mb-4">
                                <i class="fas fa-phone text-5xl"></i>
                            </div>
                            <h3 class="text-2xl font-bold text-green-300 mb-4">
                                { &contact_content.phone_label }
                            </h3>
                            <a
                                href={format!("tel:{}", contact_content.phone.replace(&[' ', '(', ')', '-'][..], ""))}
                                class="text-xl text-gray-300 hover:text-green-300 transition-colors"
                            >
                                { &contact_content.phone }
                            </a>
                        </div>

                        // Email
                        <div class="bg-gray-900 rounded-lg p-8 border-2 border-green-300 hover:border-green-400 transition-colors">
                            <div class="text-green-300 mb-4">
                                <i class="fas fa-envelope text-5xl"></i>
                            </div>
                            <h3 class="text-2xl font-bold text-green-300 mb-4">
                                { &contact_content.email_label }
                            </h3>
                            <a
                                href={format!("mailto:{}", contact_content.email)}
                                class="text-xl text-gray-300 hover:text-green-300 transition-colors break-all"
                            >
                                { &contact_content.email }
                            </a>
                        </div>
                    </div>

                    // Социальные сети
                    <div>
                        <h3 class="text-3xl font-bold text-green-300 mb-8">
                            { &contact_content.social_title }
                        </h3>
                        <div class="flex justify-center gap-6 flex-wrap">
                            {
                                contact_content.social_networks.iter().map(|social| {
                                    html! {
                                        <a
                                            href={social.url.clone()}
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="bg-gray-900 rounded-lg p-6 border-2 border-green-300 hover:border-green-400 hover:bg-gray-700 transition-all flex flex-col items-center gap-3 min-w-[120px]"
                                        >
                                            <i class={format!("{} text-5xl text-green-300", social.icon)}></i>
                                            <span class="text-lg font-semibold text-gray-300">{ &social.name }</span>
                                        </a>
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
            <section class="min-h-screen bg-gray-800 font-sansation flex items-center justify-center">
                <div class="text-center">
                    <div class="text-2xl text-green-300">{ "No content available" }</div>
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
