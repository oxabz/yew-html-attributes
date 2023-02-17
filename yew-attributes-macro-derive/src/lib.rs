extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DataStruct, DeriveInput};

fn generate_fields() -> Vec<syn::Field> {
  // Load the fields from the csv html-attributes-general.csv
  let bytes = include_bytes!("html-attributes-general.csv");
  let mut rdr = csv::Reader::from_reader(bytes.as_slice());

  // Create a vector of fields
  let mut fields = Vec::new();
  for result in rdr.records() {
    let record = result.unwrap();
    let name = record[0].to_string();
    let ty = record[1].to_string();
    let field: syn::FieldsNamed =
      syn::parse_str(&format!("{{pub {}: Option<{}>}}", name, ty)).unwrap();
    let field: syn::Field = field.named.first().unwrap().clone();
    fields.push(field);
  }
  fields
}

fn transform_struct(input: &mut DataStruct) {
  match &mut input.fields {
    syn::Fields::Named(fields) => {
      let new_fields = generate_fields();
      for field in new_fields {
        fields.named.push(field);
      }
    }
    _ => panic!("use_attributes can only be used on structs with named fields"),
  }
}

#[proc_macro_attribute]
pub fn has_attributes(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  // Parse the input tokens into a syntax tree
  let args = parse_macro_input!(attr as AttributeArgs);
  if args.len() > 0 {
    panic!("use_attributes does not take any arguments");
  }
  let input: DeriveInput = syn::parse(item).unwrap();
  let mut output = input.clone();
  match &mut output.data {
    syn::Data::Struct(strct) => {
      transform_struct(strct);
    }
    _ => panic!("use_attributes can only be used on structs"),
  }
  quote!(#output).into()
}

fn generate_instructions() -> Vec<TokenStream> {
  // Load the fields from the csv html-attributes-general.csv
  let bytes = include_bytes!("html-attributes-general.csv");
  let mut rdr = csv::Reader::from_reader(bytes.as_slice());

  // Create a vector of fields
  let mut instructions = Vec::new();
  for result in rdr.records() {
    let record = result.unwrap();
    let name = record[0].to_string();
    let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
    let ty = record[1].to_string();
    if ty == "String" {
      let instruction = quote!(
          if let Some(#ident) = &props.#ident {
              node.set_attribute(#name, #ident).expect("set_attribute failed");
          }
      )
      .into();
      instructions.push(instruction);
    } else if ty == "Callback<Event>" {
      let fnid = syn::Ident::new(&format!("set_{}", name), proc_macro2::Span::call_site());
      let instruction = quote!(
        if let Some(#ident) = &props.#ident {
          let listener = wasm_bindgen::closure::Closure::<dyn Fn(Event)>::wrap(Box::new({
            let #ident = #ident.clone();
            move |e: Event| {
              #ident.emit(e)
            }
          }));
          node.#fnid(Some(listener.as_ref().unchecked_ref()));
          listeners.push(listener);
        }else {
          node.#fnid(None);
        }
      )
      .into();
      instructions.push(instruction);
    }
  }
  instructions
}

fn generate_unset_instructions()-> Vec<TokenStream> {
  // Load the fields from the csv html-attributes-general.csv
  let bytes = include_bytes!("html-attributes-general.csv");
  let mut rdr = csv::Reader::from_reader(bytes.as_slice());

  // Create a vector of fields
  let mut instructions = Vec::new();
  for result in rdr.records() {
    let record = result.unwrap();
    let name = record[0].to_string();
    let ty = record[1].to_string();
    if ty == "Callback<Event>" {
      let fnid = syn::Ident::new(&format!("set_{}", name), proc_macro2::Span::call_site());
      let instruction = quote!(
        node.#fnid(None);
      )
      .into();
      instructions.push(instruction);
    }
  }
  instructions
}

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

  let instructions = generate_instructions().into_iter();
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
