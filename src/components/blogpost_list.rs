use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::Route;
use crate::types::blogposts::BlogpostData;

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
                <Link<Route> to={Route::Blogpost { id: post.id.clone() }}>{ &post.title }</Link<Route>>
            }
        })
        .collect::<Html>();

    html! { posts }
}
