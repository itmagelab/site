use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Deserialize, Clone, PartialEq)]
pub struct HeroContent {
    pub title: String,
    pub subtitle: String,
    pub logo: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct LangContent {
    pub hero: HeroContent,
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

#[function_component(HeroSection)]
pub fn hero_section(props: &Props) -> Html {
    let content = use_state(|| None::<HeroContent>);
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
                        let hero_content = if language == "ru" {
                            data.ru.hero
                        } else {
                            data.en.hero
                        };
                        content.set(Some(hero_content));
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
            <section class="flex items-center justify-center min-h-screen bg-green-100 font-sansation">
                <div class="text-center">
                    <div class="text-2xl text-gray-800 animate-pulse">{ "Loading..." }</div>
                </div>
            </section>
        };
    }

    if let Some(err) = (*error).as_ref() {
        return html! {
            <section class="flex items-center justify-center min-h-screen bg-green-100 font-sansation">
                <div class="text-center">
                    <div class="text-2xl text-red-600 mb-4">{ "Error loading content" }</div>
                    <div class="text-sm text-gray-700">{ err }</div>
                </div>
            </section>
        };
    }

    if let Some(hero_content) = (*content).as_ref() {
        html! {
            <section class="bg-green-200 font-sansation py-6 px-4 shadow-md">
                <div class="flex flex-row items-center justify-between gap-6 w-full max-w-6xl mx-auto">
                    <div class="flex items-center gap-4">
                        <div class="w-12 h-12 sm:w-16 sm:h-16 bg-gray-800 rounded-lg flex-shrink-0 flex items-center justify-center">
                            <i class="fas fa-terminal text-green-300 text-2xl sm:text-3xl"></i>
                        </div>
                        <div class="text-left">
                            <h1 class="text-2xl sm:text-3xl md:text-4xl font-bold tracking-wider text-gray-800">
                                { &hero_content.title }
                            </h1>
                            <p class="text-sm sm:text-base md:text-lg text-gray-700">
                                { &hero_content.subtitle }
                                <span class="ml-2 animate-pulse">{ "▋" }</span>
                            </p>
                        </div>
                    </div>
                </div>
            </section>
        }
    } else {
        html! {
            <section class="flex items-center justify-center min-h-screen bg-green-100 font-sansation">
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
