use app::App;

pub mod app;
pub mod components;
pub mod types;

fn main() {
    yew::Renderer::<App>::new().render();
}
