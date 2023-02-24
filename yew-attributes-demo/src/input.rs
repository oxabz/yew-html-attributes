use yew::prelude::*;
use yew_attributes_macro::prelude::*;

#[has_attributes(exclude = "type")]
#[derive(Debug, Clone, PartialEq, Default, Properties, HasHtmlAttributes)]
pub struct Props {}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
  let node_ref = use_node_ref();

  use_attributes!(node_ref, props);
  
  html! {
    <>
      <p>{format!("{props:#?}")}</p>
      <input ref={node_ref} type="text" />
    </>
  }
}
