mod app;
mod item;
mod add_item;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
