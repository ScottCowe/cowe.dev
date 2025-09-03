use yew::prelude::*;

use crate::{app::Route, components::navbar::NavbarComponent};

#[function_component]
pub fn Home() -> Html {
    html! {
        <div>
            <NavbarComponent current_page={Route::Home}/>
            <h1>{ "Home" }</h1>
        </div>
    }
}
