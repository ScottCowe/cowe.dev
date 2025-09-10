use yew::prelude::*;
use yew_router::prelude::*;

pub mod blog;

use blog::{blogpost_page::BlogpostPage, blogposts_page::BlogpostListPage};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    Blogpost { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Blog => html! { <BlogpostListPage /> },
        Route::Blogpost { id } => html! { <BlogpostPage id={id} /> },
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
