use stylist::{Style, css};
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::app::{Route, Theme};
use crate::types::blogposts::BlogpostData;

#[derive(Properties, PartialEq)]
pub struct BlogpostListProps {
    pub posts: Vec<BlogpostData>,
}

#[function_component]
pub fn BlogpostListComponent(props: &BlogpostListProps) -> Html {
    let theme = use_context::<Theme>().expect("No context found");
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

            let post_date = post.created_on.format("%d/%m/%Y").to_string();

            let tags: String = match post.tags.clone() {
                None => "".to_string(),
                Some(tags) => tags
                    .iter()
                    .map(|tag| format!("#{} ", tag))
                    .fold(String::new(), |acc, tag| acc + &tag),
            };

            html! {
                <div class={classes!("post")} {onclick}>
                    <h1 class={classes!("post-title")}>{ &post.title }</h1>
                    <div class={classes!("post-info")}>
                        <p class={classes!("post-date")}>{ post_date }</p>
                        <p class={classes!("post-tags")}>{ tags }</p>
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
        color: ${text}; 

        a {
            text-decoration: none;
            color: ${text}; 
        }

        .post {
            margin: 8px;
            background-color: ${bg};
            padding: 8px;
        }

        .post h1 {
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

        .post-tags {
            text-align: right;
        }
        "#,
        bg = theme.primary,
        text = theme.tertiary,
    );

    let style = Style::new(style_str).expect("Failed to create style");
    let style_name = format!("{}", style.get_class_name());

    html! {
        <div class={classes!(style_name)}>
            { posts }
        </div>
    }
}
