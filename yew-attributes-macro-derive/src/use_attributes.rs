use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::get_attributes;

pub(crate) fn generate_set_instructions() -> Vec<TokenStream> {
  // Create a vector of fields
  let mut instructions = Vec::new();
  for (name, typ) in get_attributes().iter() {
    let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
    if typ == "String" {
      let instruction = quote!(
          if let Some(#ident) = &self.#ident {
              node.set_attribute(#name, #ident).expect("set_attribute failed");
          }
      );
      instructions.push(instruction);
    } else if typ == "Callback<Event>" {
      let fnid = syn::Ident::new(&format!("set_{}", name), proc_macro2::Span::call_site());
      let instruction = quote!(
        if let Some(#ident) = &self.#ident {
          let listener = wasm_bindgen::closure::Closure::<dyn Fn(Event)>::wrap(Box::new({
            let #ident = #ident.clone();
            move |e: Event| {
              #ident.emit(e)
            }
          }));
          node.#fnid(Some(wasm_bindgen::JsCast::unchecked_ref(listener.as_ref())));
          listeners.push(listener);
        }else {
          node.#fnid(None);
        }
      );
      instructions.push(instruction);
    } else {
      panic!("There shouldnt be any other type of than String and Callback<Event>")
    }
  }
  instructions
}

pub(crate) fn generate_unset_instructions() -> Vec<TokenStream> {
  // Create a vector of fields
  let mut instructions = Vec::new();
  for (name, typ) in get_attributes().iter() {
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
