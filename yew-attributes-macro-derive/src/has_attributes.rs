use syn::DataStruct;

use crate::utils::get_attributes;

pub(crate) fn transform_struct(input: &mut DataStruct, visible:bool, exclude: &[String]) {
  match &mut input.fields {
    syn::Fields::Named(fields) => {
      let new_fields = generate_fields(visible, exclude);
      for field in new_fields {
        fields.named.push(field);
      }
    }
    _ => panic!("use_attributes can only be used on structs with named fields"),
  }
}

fn generate_fields(visible:bool, exclude:&[String]) -> Vec<syn::Field> {
  let mut fields = Vec::new();

  for (name, typ) in get_attributes(visible, None, exclude).iter() {
    let name = if name == "type" {
      "typ"
    } else {
      name
    };
    let field: syn::FieldsNamed = syn::parse_str(&format!("{{#[htmlattr]pub {}: Option<{}>}}", name, typ))
      .expect("yew-attributes panicked [ code : vxmnq ]");
    let field: syn::Field = field
      .named
      .first()
      .expect("yew-attributes panicked [ code : uDDWI ]")
      .clone();
    fields.push(field);
  }

  fields
}
