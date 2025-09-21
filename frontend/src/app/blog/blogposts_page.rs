use crate::app::Route;
use crate::components::blogpost_list::BlogpostListComponent;
use crate::components::navbar::NavbarComponent;
use crate::types::blogposts::BlogpostData;
use gloo_net::http::Request;
use yew::prelude::*;

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

    html! {
        <>
            <NavbarComponent current_page={Route::Blog}/>
            <BlogpostListComponent posts={(*posts).clone()} />
        </>
    }
}
