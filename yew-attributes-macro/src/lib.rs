pub use yew_attributes_macro_derive::has_attributes;
pub use yew_attributes_macro_derive::use_attributes;

pub mod prelude {
  pub use crate::has_attributes;
  pub use crate::use_attributes;
  pub use wasm_bindgen::closure::Closure;
  pub use wasm_bindgen::JsCast;
}