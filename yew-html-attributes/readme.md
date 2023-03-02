# Yew html attributes

Yew html attributes is a macro crate that allow you to easily add standard html attributes to your component and to then pass them to a child.

## Usage : 

To add the html attributes to your props just add the `#[has_html_attributes]` before your props and derive `HasHtmlAttributes`.

To then pass them along use the `use_attributes!` macro with a reference to the child html element and the props refererence.

```rust
use yew::prelude::*;
use yew_attributes_macro::prelude::*;

#[has_html_attributes]
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
If you want element to recieve all it's associated attributes provide the `element` parameter in `has_html_attributes`

```rust, no_run
use yew::prelude::*;
use yew_attributes_macro::prelude::*;

#[has_html_attributes(element=input)]
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

### The `exclude` parameter

You might want some to remove some attributes to avoid overwritting some of your component internal logic.
To do so you have to add the `exclude` parameter to the `has_html_attributes` with a list of all the attributes you want to exclude

```rust
use yew::prelude::*;
use yew_attributes_macro::prelude::*;

#[has_html_attributes(exclude="oninput,onclick")]
#[derive(Debug, Clone, PartialEq, Default, Properties, HasHtmlAt    Finished dev [unoptimized + debuginfo] target(s) in 1.76stributes)]
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

### The `invisible` parameter

By default the macro add the attributes common to visible html elements. If you want to only have the attributes common to all html element use the invisible parameter

```rust
use yew::prelude::*;
use yew_attributes_macro::prelude::*;

#[has_html_attributes(invisible=false)]
#[derive(Debug, Clone, PartialEq, Default, Properties, HasHtmlAttributes)]
pub struct ScriptProps{}

#[function_component(Script)]
pub fn script(props:&ScriptProps) -> Html {
  let node_ref = use_node_ref();

  use_attributes!(node_ref, props);

  html! {
      <>
          <script ref={node_ref} />
      </>
  }
}

```