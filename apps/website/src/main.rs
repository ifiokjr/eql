mod app;

use app::App;

pub fn main() {
  yew::Renderer::<App>::new().render();
}
