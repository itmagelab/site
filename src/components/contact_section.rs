use yew::prelude::*;

use crate::content::{get_contact_content, ContactContent};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub language: String,
}

#[function_component(ContactSection)]
pub fn contact_section(props: &Props) -> Html {
    let contact_content: &ContactContent = get_contact_content(&props.language);

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
}

