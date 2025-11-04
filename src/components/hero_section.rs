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
pub struct Content {
    pub hero: HeroContent,
}

#[function_component(HeroSection)]
pub fn hero_section() -> Html {
    let content = use_state(|| None::<HeroContent>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    // Загрузка данных из YAML
    {
        let content = content.clone();
        let error = error.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                match load_content().await {
                    Ok(data) => {
                        content.set(Some(data.hero));
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
            <section class="flex items-center justify-center min-h-screen bg-green-200 font-sansation p-4">
                <div class="flex flex-col md:flex-row items-center gap-6 md:gap-8 w-full max-w-6xl">
                    <div class="w-24 h-24 sm:w-32 sm:h-32 md:w-48 md:h-48 bg-gray-800 rounded-lg flex-shrink-0 flex items-center justify-center">
                        <i class="fas fa-terminal text-green-300 text-4xl sm:text-5xl md:text-8xl"></i>
                    </div>
                    <div class="text-center md:text-left">
                        <h1 class="text-4xl sm:text-5xl md:text-6xl lg:text-8xl font-bold mb-4 md:mb-6 tracking-wider text-gray-800 break-words">
                            { &hero_content.title }
                        </h1>
                        <p class="text-lg sm:text-xl md:text-2xl lg:text-3xl text-gray-700 break-words">
                            { &hero_content.subtitle }
                            <span class="ml-2 animate-pulse">{ "▋" }</span>
                        </p>
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

async fn load_content() -> Result<Content, String> {
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
