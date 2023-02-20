mod app;
mod input;

use app::App;

fn main() {
  yew::Renderer::<App>::new().render();
}
