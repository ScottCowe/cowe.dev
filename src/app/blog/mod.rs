use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component]
pub fn Blog() -> Html {
    html! {
        <h1>{ "Blog" }</h1>
    }
}

#[function_component]
pub fn BlogPost(props: &Props) -> Html {
    html! {
        <h1>{ format!("Blog: {}", props.id.clone()) }</h1>
    }
}
