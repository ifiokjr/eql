use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
  let counter = use_state(|| 0);
  let onclick = {
    let counter = counter.clone();
    move |_event: MouseEvent| {
      let value = *counter + 1;
      counter.set(value);
    }
  };

  html! {
      <main>
        <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
        <h1>{"Awesome"}</h1>
        <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        <div>
          <button onclick={onclick.clone()} ondblclick={onclick}>{ "Click me!" }</button>
          <p>{*counter}</p>
          <p>{*counter}</p>
        </div>
      </main>
  }
}
