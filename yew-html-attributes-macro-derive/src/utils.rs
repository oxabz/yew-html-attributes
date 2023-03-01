use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
  static ref HTML_ATTRIBUTES: HashMap<String, HashMap<String, String>> = {
    let mut html_attributes = HashMap::new();

    // Load the fields from the csv html-attributes-general.csv
    let bytes = include_bytes!("html-attributes.csv");
    let mut rdr = csv::Reader::from_reader(bytes.as_slice());

    for line in rdr.records() {
      let line = line.expect("yew-attributes panicked : Internal error. Please raise an issue on the Github [code : trlzv]");

      let name = line.get(0).expect("yew-attributes panicked : Internal error. Please raise an issue on the Github [code : kcwlc]");
      let usages = line.get(1).expect("yew-attributes panicked : Internal error. Please raise an issue on the Github [code : kcwld]");
      let typ = line.get(2).expect("yew-attributes panicked : Internal error. Please raise an issue on the Github [code : yynya]");

      let usages = usages.split(','); 

      for usage in usages {
        let usage = usage.to_string();
        html_attributes.entry(usage).or_insert(HashMap::new()).insert(name.to_string(), typ.to_string());
      }
    }

    html_attributes
  };
}

 


pub(crate) fn get_attributes(visible: bool, element:Option<&str>, excluded:&[String]) -> HashMap<String, String> {
  let mut attributes = HTML_ATTRIBUTES["*"].clone();
  
  if visible{
    for (name, typ) in HTML_ATTRIBUTES["visible"].iter(){
      let (name, typ) = (name.clone(), typ.clone());
      attributes.insert(name, typ);
    }
  }

  if let Some(element) = element {
    for (name, typ) in HTML_ATTRIBUTES[element].iter(){
      let (name, typ) = (name.clone(), typ.clone());
      attributes.insert(name, typ);
    }
  }

  for ex in excluded{
    attributes.remove(ex);
  }

  attributes
}

pub(crate) fn get_all_attributes() -> HashMap<String, String> {
  HTML_ATTRIBUTES.iter().fold(HashMap::new(), |acc, (_,x)|{
    let mut acc = acc;
    for (name, typ) in x.iter(){
      let (name, typ) = (name.clone(), typ.clone());
      acc.insert(name, typ);
    }
    acc
  })
}

