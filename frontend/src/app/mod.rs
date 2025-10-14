use stylist::{
    css,
    yew::{Global, styled_component},
};
use yew::prelude::*;
use yew_router::prelude::*;

pub mod blog;
pub mod notfound_page;

use blog::{blogpost_page::BlogpostPage, blogposts_page::BlogpostListPage};

use notfound_page::NotFound;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
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
        Route::Blog | Route::Index => html! { <BlogpostListPage /> },
        Route::Blogpost { id } => html! { <BlogpostPage id={id} /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[derive(Clone, PartialEq)]
pub struct Theme {
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
}

#[styled_component(GlobalStyle)]
fn global_style() -> Html {
    let theme = use_context::<Theme>().expect("context smth smth");

    html! {
        <Global css={css!(
            r#"
            body {
                background-color: ${bg};
                font-family: "Lucida Console", "Courier New", monospace;
            }
            "#,
            bg = theme.secondary,
        )} />
    }
}

#[styled_component(App)]
pub fn app() -> Html {
    let theme = Theme {
        primary: "#222831".to_owned(),
        secondary: "#393E46".to_owned(),
        tertiary: "#DFD0B8".to_owned(),
    };

    let ctx = use_state(|| theme);

    html! {
        <ContextProvider<Theme> context={(*ctx).clone()}>
            <GlobalStyle />
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<Theme>>
    }
}
