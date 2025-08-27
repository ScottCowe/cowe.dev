use gloo_net::http::Request;
use yew::prelude::*;

use crate::components::blogpost::BlogpostComponent;
use crate::components::blogpost_list::BlogpostListComponent;
use crate::types::blogposts::{Blogpost, BlogpostData};

#[function_component]
pub fn BlogpostListPage() -> Html {
    let posts = use_state(|| vec![]);

    {
        let posts = posts.clone();

        use_effect_with((), move |_| {
            let posts = posts.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let fetched_posts: Vec<BlogpostData> = Request::get("/api/posts")
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
    let post = use_state(|| Blogpost {
        id: "unloaded".to_string(),
        title: "unloaded".to_string(),
        format: "unloaded".to_string(),
        content: "unloaded".to_string(),
    });
    let url = format!("/api/posts/{}", props.id);

    {
        let post = post.clone();
        let url = url.clone();

        use_effect_with((), move |_| {
            let post = post.clone();
            let url = url.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let fetched_post: Blogpost = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                post.set(fetched_post);
            });
            || ()
        });
    }

    html! { <BlogpostComponent post={(*post).clone()} /> }
}
