# Yew html attributes

Yew html attributes is a macro crate that allow you to easily had standard html attributes to your component and to then pass them to a children.

> WARNING: This crate is a proof of concept and should be avoided in production, as it is not stable and untested

## Usage : 

To add the hml attributes to your props just add the `#[has_attributes]` before your props.

To then pass them along use the `use_attributes!` macro with a reference to the children html element and the props struct.

```rs
use yew::prelude::*;
use yew_attributes_macro::prelude::*;

#[has_attributes]
#[derive(Debug, Clone, PartialEq, Default, Properties)]
pub struct Props {}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
  let node_ref = use_node_ref();

  use_attributes!(node_ref, props);

  html! {
    <>
      <input ref={node_ref} type="text" />
    </>
  }
}

```