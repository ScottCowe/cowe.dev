use stylist::{Style, css};
use yew::{Html, Properties, classes, html, prelude::function_component, use_context};
use yew_router::prelude::Link;

use crate::app::{Route, Theme};

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub current_page: Route,
}

#[function_component]
pub fn NavbarComponent(_props: &NavbarProps) -> Html {
    let theme = use_context::<Theme>().expect("Could not find context");

    let style_str = css!(
        r#"
        margin: auto;
        width: 100%;
        text-align: center;
        padding: 20px 0px 20px 0px;
        background-color: ${bg}; 

        a {
            margin: auto;
            text-decoration: none;
            padding: 20px;
            color: ${text};
        } 
        "#,
        bg = theme.primary,
        text = theme.tertiary,
    );

    let style = Style::new(style_str).expect("Could not create style");
    let style_name = format!("{}", style.get_class_name()); // A bit hacky ig but whatever

    html! {
        <div class={classes!(style_name)}>
            <Link<Route> to={Route::Blog}>{ "Blog" }</Link<Route>>
            <a href={ "https://github.com/ScottCowe" }>{ "Github" }</a>
        </div>
    }
}
