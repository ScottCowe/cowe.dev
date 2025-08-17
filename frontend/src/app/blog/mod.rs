use gloo_net::http::Request;
use yew::prelude::*;

use crate::components::blogpost_list::BlogpostListComponent;
use common::types::blogposts::BlogpostData;

#[function_component]
pub fn BlogpostListPage() -> Html {
    let posts = use_state(|| vec![]);

    {
        let posts = posts.clone();

        use_effect_with((), move |_| {
            let posts = posts.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let fetched_posts: Vec<BlogpostData> = Request::get("/api/post")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                posts.set(fetched_posts);
            });
            || ()
        });
    }

    html! { <BlogpostListComponent posts={(*posts).clone()} /> }
}

#[derive(Properties, PartialEq)]
pub struct BlogpostPageProps {
    pub id: String,
}

#[function_component]
pub fn BlogpostPage(props: &BlogpostPageProps) -> Html {
    html! {
        <h1>{ format!("Blog: {}", props.id.clone()) }</h1>
    }
}
