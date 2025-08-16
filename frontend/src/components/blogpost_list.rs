use yew::prelude::*;

use crate::types::blogposts::*;

#[derive(Properties, PartialEq)]
pub struct BlogpostListProps {
    pub posts: Vec<Blogpost>,
}

#[function_component]
pub fn BlogpostListComponent(props: &BlogpostListProps) -> Html {
    let posts = props
        .posts
        .iter()
        .map(|post| {
            html! {
                <h1>{ format!("{} {}", post.id, post.title)}</h1>
            }
        })
        .collect::<Html>();

    html! { posts }
}
