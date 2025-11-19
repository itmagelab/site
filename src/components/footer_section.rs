use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Deserialize, Clone, PartialEq)]
pub struct FooterContent {
    pub copyright: String,
    pub credits: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct LangContent {
    pub footer: FooterContent,
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

#[function_component(FooterSection)]
pub fn footer_section(props: &Props) -> Html {
    let content = use_state(|| None::<FooterContent>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| true);

    {
        let content = content.clone();
        let error = error.clone();
        let loading = loading.clone();
        let language = props.language.clone();

        use_effect_with(props.language.clone(), move |_| {
            spawn_local(async move {
                match load_content().await {
                    Ok(data) => {
                        let footer_content = if language == "ru" {
                            data.ru.footer
                        } else {
                            data.en.footer
                        };
                        content.set(Some(footer_content));
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
            <footer class="bg-gray-900 text-white py-8 mt-auto">
                <div class="container mx-auto px-4 text-center">
                    <div class="animate-pulse">{ "Loading..." }</div>
                </div>
            </footer>
        };
    }

    if let Some(err) = (*error).as_ref() {
        return html! {
            <footer class="bg-gray-900 text-white py-8 mt-auto">
                <div class="container mx-auto px-4 text-center text-red-500">
                    { format!("Error: {}", err) }
                </div>
            </footer>
        };
    }

    if let Some(footer_content) = (*content).as_ref() {
        html! {
            <footer class="bg-gray-900 text-gray-400 py-8 mt-auto font-sansation">
                <div class="container mx-auto px-4 text-center">
                    <div class="mb-2">
                        { &footer_content.copyright }
                    </div>
                    <div class="text-sm text-gray-600">
                        { &footer_content.credits }
                    </div>
                </div>
            </footer>
        }
    } else {
        html! { <></> }
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
