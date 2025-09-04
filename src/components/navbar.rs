use stylist::{Style, css};
use yew::{Html, Properties, classes, html, prelude::function_component};
use yew_router::prelude::Link;

use crate::app::Route;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub current_page: Route,
}

#[function_component]
pub fn NavbarComponent(props: &NavbarProps) -> Html {
    let style_str = css!(
        r#"
        margin: auto;
        width: 50%;
        text-align: center;
        padding: 20px 0px 20px 0px;

        a {
            margin: auto;
            text-decoration: none;
            color: black;
            padding: 20px;
        }
        "#
    );

    let style = Style::new(style_str).expect("Could not create style");
    let style_name = format!("{}", style.get_class_name()); // A bit hacky ig but whatever

    html! {
        <div class={classes!(style_name)}>
            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
            <Link<Route> to={Route::Blog}>{ "Blog" }</Link<Route>>
        </div>
    }
}
