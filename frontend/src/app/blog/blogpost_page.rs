use chrono::NaiveDateTime;
use gloo_net::http::Request;
use yew::prelude::*;

use crate::app::Route;
use crate::components::blogpost::BlogpostComponent;
use crate::components::navbar::NavbarComponent;
use crate::types::blogposts::Blogpost;

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
        created_on: NaiveDateTime::from_timestamp(0, 0), // ik its deprecated, idgaf, i'll fix later
        updated_on: None,
        tags: None,
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

    html! {
        <>
            <NavbarComponent current_page={Route::Blogpost { id: (*post).clone().id }} />
            <BlogpostComponent post={(*post).clone()} />
        </>
    }
}
