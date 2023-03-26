mod components;
mod entities;

use crate::components::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
