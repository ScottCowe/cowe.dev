use yew::prelude::*;
use yew_router::prelude::*;

pub mod blog;
pub mod home;

use blog::{BlogpostListPage, BlogpostPage};
use home::Home;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    BlogPost { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Blog => html! { <BlogpostListPage /> },
        Route::BlogPost { id } => html! { <BlogpostPage id={id} /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
