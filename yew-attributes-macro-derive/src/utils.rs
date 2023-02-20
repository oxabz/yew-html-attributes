use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static!{
  static ref HTML_ATTRIBUTES: HashMap<String, String> = {
    let mut html_attributes = HashMap::new();

    // Load the fields from the csv html-attributes-general.csv
    let bytes = include_bytes!("html-attributes-general.csv");
    let mut rdr = csv::Reader::from_reader(bytes.as_slice());

    for line in rdr.records() {
      let line = line.expect("yew-attributes panicked : Internal error. Please raise an issue on the Github [code : trlzv]");

      let name = line.get(0).expect("yew-attributes panicked : Internal error. Please raise an issue on the Github [code : kcwlc]");
      let typ = line.get(1).expect("yew-attributes panicked : Internal error. Please raise an issue on the Github [code : yynya]");

      html_attributes.insert(name.to_string(), typ.to_string());
    }

    html_attributes
  };
}

pub(crate) fn get_attributes() -> &'static HashMap<String, String> {
  return &HTML_ATTRIBUTES;
}