use syn::DataStruct;

use crate::utils::get_attributes;

pub(crate) fn transform_struct(input: &mut DataStruct) {
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

fn generate_fields() -> Vec<syn::Field> {
  let mut fields = Vec::new();

  for (name, typ) in get_attributes().iter() {
    let field: syn::FieldsNamed =
      syn::parse_str(&format!("{{pub {}: Option<{}>}}", name, typ)).expect("yew-attributes panicked [ code : vxmnq ]");
    let field: syn::Field = field.named.first().expect("yew-attributes panicked [ code : uDDWI]").clone();
    fields.push(field);
  }

  fields
}
