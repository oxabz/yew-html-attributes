# Yew html attributes

Yew html attributes is a macro crate that allow you to easily add standard html attributes to your component and to then pass them to a child.

## Usage : 

To add the html attributes to your props just add the `#[has_attributes]` before your props and derive `HasHtmlAttributes`.

To then pass them along use the `use_attributes!` macro with a reference to the child html element and the props refererence.

```rs
use yew::prelude::*;
use yew_attributes_macro::prelude::*;

#[has_attributes]
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

```

### The `element` parameters

By default the macro only adds the attributes common to all html element. 
If you want element to recieve all it's associated attributes provide the `element` parameter in `has_attributes`