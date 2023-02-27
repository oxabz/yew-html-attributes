use yew::prelude::*;
use yew_html_attributes::prelude::*;

#[has_html_attributes(element = "input", exclude = "text")]
#[derive(Debug, Clone, PartialEq, Default, Properties, HasHtmlAttributes)]
pub struct InputProps{}

#[function_component(Input)]
pub fn input(props:&InputProps) -> Html {
  let node_ref = use_node_ref();

  use_attributes!(node_ref, props);

  html! {
      <>
          <input ref={node_ref} type="text" />
      </>
  }
}
