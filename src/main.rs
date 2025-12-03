use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

use pages::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/ru")]
    Ru,
    #[at("/en")]
    En,
    #[at("/")]
    Root,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Ru => html! { <Home lang="ru" /> },
        Route::En => html! { <Home lang="en" /> },
        Route::Root => html! { <Redirect<Route> to={Route::Ru}/> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
