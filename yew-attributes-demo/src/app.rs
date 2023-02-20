use crate::input::Input;
use web_sys::console::log_1;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
  let onclick = Callback::from(|_: Event| log_1(&"Clicked!".into()));
  html! {
      <main>
          <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
          <h1>{ "Hello World!" }</h1>
          <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
          <Input class={"test"} onclick={onclick}/>
      </main>
  }
}
