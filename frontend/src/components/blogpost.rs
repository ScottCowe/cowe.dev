use crate::{app::Theme, types::blogposts::Blogpost};
use stylist::{Style, css};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BlogpostProps {
    pub post: Blogpost,
}

#[function_component]
pub fn BlogpostComponent(props: &BlogpostProps) -> Html {
    let content = match props.post.format.as_str() {
        "plaintext" => html! {
            <p>{ props.post.content.clone() }</p>
        },
        "html" => Html::from_html_unchecked(props.post.content.clone().into()),
        // TODO: Implement markdown
        _ => html! { "Internal fuckup" },
    };

    let theme = use_context::<Theme>().expect("Failed to context smth smth");

    let style_str = css!(
        r#"
            width: 50%;
            margin: auto;
            margin-top: 40px;
            padding: 10px;
            color: ${text};
            background-color: ${bg};

            .post-title {
                margin: 10px 0px 0px 0px;
            }

            .post-info {
                display: flex;
            }

            .post-info p {
                flex: 1;
            }

            .post-tags p {
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

    let post_date = props
        .post
        .created_on
        .format("Written on %d/%m/%Y at %H:%M")
        .to_string();

    let tags: String = match props.post.tags.clone() {
        None => "".to_string(),
        Some(tags) => tags
            .iter()
            .map(|tag| format!("#{} ", tag))
            .fold(String::new(), |acc, tag| acc + &tag),
    };

    html! {
        <div class={classes!(style_name)}>
            <h1 class={classes!("post-title")}>{ props.post.title.clone() }</h1>
            <div class={classes!("post-info")}>
                <p class={classes!("post-date")}>{ post_date }</p>
                <p class={classes!("post-tags")}>{ tags }</p>
            </div>
            { content }
        </div>
    }
}
