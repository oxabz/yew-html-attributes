#![doc = include_str!("../../readme.md")]
use wasm_bindgen::prelude::Closure;
use web_sys::Event;
pub use yew_html_attributes_macro_derive::HasHtmlAttributes;
pub use yew_html_attributes_macro_derive::has_html_attributes;
pub use yew_html_attributes_macro_derive::use_attributes;

/// The module that expopse everything you need to use the crate
pub mod prelude {
  pub use crate::has_html_attributes;
  pub use crate::use_attributes;
  pub use crate::HasHtmlAttributes;
}

/**
 * This trait is used internaly by the use_attributes macro to set and unset the attributes on the html element. 
 * 
 * it has to be public but I would not recommend using this trait directly.
*/
pub trait HasHtmlAttributes {
  fn set_attributes(&self, node: &web_sys::HtmlElement)-> Vec<Closure<dyn Fn(Event)>>;
  fn unset_attributes(&self, node: &web_sys::HtmlElement);
}