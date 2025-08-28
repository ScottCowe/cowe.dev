use crate::types::blogposts::Blogpost;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlogpostProps {
    pub post: Blogpost,
}

#[function_component]
pub fn BlogpostComponent(props: &BlogpostProps) -> Html {
    match props.post.format.as_str() {
        "plaintext" => html! {
            <div>
                <h1>{ props.post.title.clone() }</h1>
                <p>{ props.post.content.clone() }</p>
            </div>
        },
        "html" => html! {
            <div>
                <h1>{ props.post.title.clone() }</h1>
                { Html::from_html_unchecked(props.post.content.clone().into()) }
            </div>
        },
        // TODO: Implement markdown
        _ => html! {
            <h1>{ "Internal server fuckup" }</h1>
        },
    }
}
