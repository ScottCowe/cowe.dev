use stylist::{Style, css};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::app::Route;
use crate::types::blogposts::BlogpostData;

#[derive(Properties, PartialEq)]
pub struct BlogpostListProps {
    pub posts: Vec<BlogpostData>,
}

#[function_component]
pub fn BlogpostListComponent(props: &BlogpostListProps) -> Html {
    let navigator = use_navigator().unwrap();

    let posts = props
        .posts
        .iter()
        .map(|post| {
            let navigator = navigator.clone();
            let page = Route::Blogpost {
                id: post.id.clone(),
            };
            let onclick = Callback::from(move |_| navigator.push(&page));

            html! {
                <div class={classes!("post")} {onclick}>
                    <h1 class={classes!("post-title")}>{ &post.title }</h1>
                    <div class={classes!("post-info")}>
                        <p class={classes!("post-tags")}>{ "#tags" }</p>
                        <p class={classes!("post-date")}>{ "Posted on: " }</p>
                    </div>
                </div>
            }
        })
        .collect::<Html>();

    let style_str = css!(
        r#"
        margin: auto;
        width: 40%;
        padding: 40px 0px 0px 0px;

        a {
            text-decoration: none;
            color: black;
        }

        .post {
            border: 3px solid red;
            margin: 8px;
        }
        
        .post-title {
            margin: 8px 0px 4px 8px;
        }

        .post-info {
            margin: 4px 8px 0px 8px;
            display: flex;
        }

        p {
            flex: 1;
        }

        .post-date {
            text-align: right;
        }
        "#
    );

    let style = Style::new(style_str).expect("Failed to create style");
    let style_name = format!("{}", style.get_class_name());

    html! {
        <div class={classes!(style_name)}>
            { posts }
        </div>
    }
}
