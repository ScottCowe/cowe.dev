use crate::types::blogposts::Blogpost;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlogpostProps {
    pub post: Blogpost,
}

#[function_component]
pub fn BlogpostComponent(props: &BlogpostProps) -> Html {
    html! {
        <h1>{ props.post.title.clone() }</h1>
    }
}
