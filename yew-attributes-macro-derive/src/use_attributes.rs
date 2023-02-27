use proc_macro2::{TokenStream, Ident};
use quote::quote;

use crate::utils::{get_all_attributes};

pub(crate) fn generate_set_instructions(attrs: &[Ident]) -> Vec<TokenStream> {
  // Create a vector of fields
  let mut instructions = Vec::new();
  let attr_dict = get_all_attributes();
  for attr in attrs {
    let name = attr.to_string();
    let name = if name == "typ" {
      "type"
    } else {
      &name
    };
    let typ = attr_dict.get(name).expect(&format!("attribute {name} is not a known html attribute"));

    if typ == "String" {
      let instruction = quote!(
        if let Some(value) = &self.#attr {
          node.set_attribute(#name,value).expect("yew-attributes panicked [ code : 00A0A ]");
        } else {
          node.remove_attribute(#name).expect("yew-attributes panicked [ code : 00A0A ]");;
        }
      );
      instructions.push(instruction);
    } else if typ == "Callback<Event>" {
      let fnid = syn::Ident::new(&format!("set_{}", name), proc_macro2::Span::call_site());
      let instruction = quote!(
        if let Some(callback) = &self.#attr {
          let callback = callback.clone();
          let closure = wasm_bindgen::closure::Closure::<dyn Fn(Event)>::wrap(
            Box::new(move |e: Event| callback.emit(e))
          );
          node.#fnid(Some(wasm_bindgen::JsCast::unchecked_ref(closure.as_ref())));
          listeners.push(closure);
        } else {
          node.#fnid(None);
        }
      );
      instructions.push(instruction);
    }

  }

  instructions
}

pub(crate) fn generate_unset_instructions(attrs: &[Ident]) -> Vec<TokenStream> {
  // Create a vector of fields
  let mut instructions = Vec::new();
  let attr_dict = get_all_attributes();
  for attr in attrs{
    let name = attr.to_string();
    let typ = attr_dict.get(&name).expect(&format!("attribute {name} is not a known html attribute"));
    if typ == "Callback<Event>" {
      let fnid = syn::Ident::new(&format!("set_{}", name), proc_macro2::Span::call_site());
      let instruction = quote!(
        node.#fnid(None);
      );
      instructions.push(instruction);
    }
  }
  instructions
}
