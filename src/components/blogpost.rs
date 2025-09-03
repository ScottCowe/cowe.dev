use crate::types::blogposts::Blogpost;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlogpostProps {
    pub post: Blogpost,
}

#[function_component]
pub fn BlogpostComponent(props: &BlogpostProps) -> Html {
    let content = match props.post.format.as_str() {
        "plaintext" => html! {
            <p>{ props.post.content.clone() }</p>
        },
        "html" => Html::from_html_unchecked(props.post.content.clone().into()),
        // TODO: Implement markdown
        _ => html! {
            <h1>{ "Internal fuckup " }</h1>
        },
    };

    html! {
        <div>
            <h1>{ props.post.title.clone() }</h1>
            { content }
        </div>
    }
}
