use crate::app::{Route, Theme};
use crate::components::navbar::NavbarComponent;
use stylist::{Style, css};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFound)]
pub fn not_found_page() -> Html {
    let theme = use_context::<Theme>().expect("Could not find context");

    let style_str = css!(
        r#"
        width: 50%;
        margin: auto;
        margin-top: 40px;
        padding: 10px;
        color: ${text};
        background-color: ${bg};

        .text {
            text-align: center;
        }
        "#,
        bg = theme.primary,
        text = theme.tertiary,
    );

    let style = Style::new(style_str).expect("Could not create style");
    let style_name = format!("{}", style.get_class_name());

    html! {
        <>
            <NavbarComponent current_page={Route::NotFound} />
            <div class={classes!(style_name)}>
                <h1 class={classes!("text")}>{ "404" }</h1>
                <p class={classes!("text")}>{ "Page not found" }</p>
            </div>
        </>
    }
}
