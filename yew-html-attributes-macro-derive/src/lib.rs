mod has_attributes;
mod use_attributes;
mod utils;

extern crate proc_macro;

use has_attributes::transform_struct;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, token::Pound, Attribute, AttributeArgs, DeriveInput, NestedMeta};
use use_attributes::{generate_set_instructions, generate_unset_instructions};

/// Parse the has_attributes macro arguments
fn parse_has_attributes_args(args: Vec<NestedMeta>) -> (bool, Option<String>, Vec<String>) {
  let mut excluded = vec![];
  let mut visible = true;
  let mut element = None;
  for arg in args {
    if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = arg {
      if nv.path.is_ident("exclude") {
        if let syn::Lit::Str(lit) = &nv.lit {
          let ex = lit.value();
          excluded = ex.split(",").map(String::from).collect();
        } else {
          panic!("exclude argument expects a string")
        }
      }
      if nv.path.is_ident("invisble") {
        if let syn::Lit::Bool(lit) = &nv.lit {
          let lit = lit.value();
          visible = !lit;
        } else {
          panic!("invisble argument expects a boolean")
        }
      }
      if nv.path.is_ident("element") {
        if let syn::Lit::Str(lit) = &nv.lit {
          let lit = lit.value();
          element = Some(lit);
        } else {
          panic!("element argument expects a string")
        }
      }
    }
  }
  (visible, element, excluded)
}

/// Adds the standard html attributes to the Properties struct
#[proc_macro_attribute]
pub fn has_html_attributes(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  // Parse the input tokens into a syntax tree
  let args = parse_macro_input!(attr as AttributeArgs);

  let (visible, element, excluded) = parse_has_attributes_args(args);

  let input: DeriveInput = syn::parse(item).unwrap();
  let mut output = input;
  match &mut output.data {
    syn::Data::Struct(strct) => {
      transform_struct(
        strct,
        visible,
        element.as_ref().map(String::as_str),
        &excluded,
      );
    }
    _ => panic!("use_attributes can only be used on structs"),
  }

  // Check that the struct has a Properties & HasHtmlAttributes derive
  let mut has_properties = false;
  let mut has_html_attributes = false;
  for attr in &output.attrs {
    if attr.path.is_ident("derive") {
      if let syn::Meta::List(list) = &attr.parse_meta().unwrap() {
        for nested in &list.nested {
          if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested {
            if path.is_ident("Properties") {
              has_properties = true;
            }
            if path.is_ident("HasHtmlAttributes") {
              has_html_attributes = true;
            }
          }
        }
      }
    }
  }

  if let Some(elem) = &element {
    let meta: syn::Meta = syn::parse_str(&format!("htmlelem = \"{elem}\"")).unwrap();
    if let syn::Meta::NameValue(syn::MetaNameValue {
      path,
      eq_token,
      lit,
    }) = meta
    {
      let mut tokens = eq_token.to_token_stream();
      tokens.extend(lit.to_token_stream());
      output.attrs.push(Attribute {
        pound_token: Pound(Span::call_site()),
        style: syn::AttrStyle::Outer,
        bracket_token: syn::token::Bracket {
          span: Span::call_site(),
        },
        path,
        tokens,
      });
    }
  }

  if !has_properties {
    panic!("has_attributes can only be used on structs with a Properties derive");
  }
  if !has_html_attributes {
    panic!("has_attributes can only be used on structs with a HasHtmlAttributes derive");
  }
  quote!(
    #output
  )
  .into()
}

/// Implements the HasHtmlAttributes trait for the given struct
#[proc_macro_derive(HasHtmlAttributes, attributes(htmlattr, htmlelem))]
pub fn derive_has_html_attributes(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: DeriveInput = syn::parse(item).unwrap();
  let name = &input.ident;

  let mut attr_fields = vec![];

  match input.data {
    syn::Data::Struct(data) => {
      if let syn::Fields::Named(fields) = data.fields {
        for field in fields.named {
          if let Some(attr) = field.attrs.first() {
            if attr.path.is_ident("attr") {
              attr_fields.push(field.ident.unwrap());
            }
          }
        }
      } else {
        panic!("HasHtmlAttributes can only be used on structs with named fields");
      }
    }
    _ => panic!("HasHtmlAttributes can only be used on structs"),
  }

  let set_instructions = generate_set_instructions(&attr_fields);
  let unset_instructions = generate_unset_instructions(&attr_fields);
  quote!(
    impl HasHtmlAttributes for #name {
      fn set_attributes(&self, node: &web_sys::HtmlElement) -> Vec<wasm_bindgen::closure::Closure<dyn Fn(Event)>> {
        let mut listeners: Vec<wasm_bindgen::closure::Closure<dyn Fn(Event)>> = Vec::new();
        #(#set_instructions)*
        listeners
      }
      fn unset_attributes(&self, node: &web_sys::HtmlElement) {
        #(#unset_instructions)*
      }
    }
  ).into()
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

  quote!(
    use_effect_with_deps(|(node_ref, props)|{
      let node = node_ref.cast::<web_sys::HtmlElement>().unwrap();
      let props = props.clone();
      let mut listeners: Vec<wasm_bindgen::closure::Closure::<dyn Fn(Event)>> = props.set_attributes(&node);
      move || {
        let node = node;
        props.unset_attributes(&node);
        drop(listeners);
      }
    }
    , (#node_ref.clone(), #props.clone()))
  )
  .into()
}
