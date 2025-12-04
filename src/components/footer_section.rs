use yew::prelude::*;

use crate::content::{get_footer_content, FooterContent};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub language: String,
}

#[function_component(FooterSection)]
pub fn footer_section(props: &Props) -> Html {
    let footer_content: &FooterContent = get_footer_content(&props.language);

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
}

