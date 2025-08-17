use yew::prelude::*;

use common::types::blogposts::BlogpostData;

#[derive(Properties, PartialEq)]
pub struct BlogpostListProps {
    pub posts: Vec<BlogpostData>,
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
