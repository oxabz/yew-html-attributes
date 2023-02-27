use syn::DataStruct;

use crate::utils::get_attributes;

pub(crate) fn transform_struct(input: &mut DataStruct, visible:bool, element:Option<&str>, exclude: &[String]) {
  match &mut input.fields {
    syn::Fields::Named(fields) => {
      let new_fields = generate_fields(visible, element, exclude);
      for field in new_fields {
        fields.named.push(field);
      }
    }
    _ => panic!("use_attributes can only be used on structs with named fields"),
  }
}

fn generate_fields(visible:bool, element:Option<&str>, exclude:&[String]) -> Vec<syn::Field> {
  let mut fields = Vec::new();

  for (name, typ) in get_attributes(visible, element, exclude).iter() {
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
