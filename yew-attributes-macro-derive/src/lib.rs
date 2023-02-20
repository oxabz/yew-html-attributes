mod has_attributes;
mod use_attributes;
mod utils;

extern crate proc_macro;
use has_attributes::transform_struct;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};
use use_attributes::{generate_set_instructions, generate_unset_instructions};

/// Adds the standard html attributes to the Properties struct
#[proc_macro_attribute]
pub fn has_attributes(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  // Parse the input tokens into a syntax tree
  let args = parse_macro_input!(attr as AttributeArgs);
  if !args.is_empty() {
    panic!("use_attributes does not take any arguments");
  }
  let input: DeriveInput = syn::parse(item).unwrap();
  let mut output = input;
  match &mut output.data {
    syn::Data::Struct(strct) => {
      transform_struct(strct);
    }
    _ => panic!("use_attributes can only be used on structs"),
  }
  quote!(#output).into()
}

/// Create a hook that use the html attributes created by has_attributes to pass them to a given html element
#[proc_macro]
pub fn use_attributes(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  // Parse the input tokens
  let input = parse_macro_input!(item as AttributeArgs);
  if input.len() != 2 {
    panic!("use_attributes takes 2 arguments");
  }
  let node_ref = match &input[0] {
    syn::NestedMeta::Meta(syn::Meta::Path(path)) => path.get_ident().unwrap(),
    _ => panic!("use_attributes first argument must be a path"),
  };
  let props = match &input[1] {
    syn::NestedMeta::Meta(syn::Meta::Path(path)) => path.get_ident().unwrap(),
    _ => panic!("use_attributes second argument must be a path"),
  };

  let instructions = generate_set_instructions().into_iter();
  let unset_instructions = generate_unset_instructions().into_iter();
  quote!(
    use_effect_with_deps(|(node_ref, props)|{
      let node = node_ref.cast::<web_sys::HtmlElement>().unwrap();
      let mut listeners: Vec<wasm_bindgen::closure::Closure::<dyn Fn(Event)>> = vec![];
      #(#instructions)*
      move || {
        #(#unset_instructions)*
        drop(listeners);
      }
    }
    , (#node_ref.clone(), #props.clone()))
  )
  .into()
}
