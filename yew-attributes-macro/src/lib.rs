use wasm_bindgen::prelude::Closure;
use web_sys::Event;
pub use yew_attributes_macro_derive::HasHtmlAttributes;
pub use yew_attributes_macro_derive::has_attributes;
pub use yew_attributes_macro_derive::use_attributes;

pub mod prelude {
  pub use crate::has_attributes;
  pub use crate::use_attributes;
  pub use crate::HasHtmlAttributes;
}

pub trait HasHtmlAttributes {
  fn set_attributes(&self, node: &web_sys::HtmlElement)-> Vec<Closure<dyn Fn(Event)>>;
  fn unset_attributes(&self, node: &web_sys::HtmlElement);
}