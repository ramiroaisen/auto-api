use std::collections::HashMap;

use ts_rs::{TypeVisitor, TS};

pub fn inline<T: TS + 'static + ?Sized>() -> String {
  let mut generics = GenericsVisitor { decl: Default::default() };
  let mut dependencies = GenericsVisitor { decl: Default::default() };
  T::visit_generics(&mut generics);
  T::visit_dependencies(&mut dependencies);

  let mut target= T::inline();

  macro_rules! replace {
    ($types:expr) => {{
      for (name, inline) in $types.decl.iter() {
        // TODO: improve this
        if(name == "Array") {
          continue;
        }
        
        // println!("replacing {} with {}", name, inline);

        let re = regex::Regex::new(&format!("([<,:]?)(\\s*){name}(\\s*)([,><])", name=regex::escape(&name))).unwrap();
        target = re.replace_all(&target, |caps: &regex::Captures<'_>| { 
          format!(
            "{}{}{}{}{}",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            inline,
            caps.get(3).unwrap().as_str(),
            caps.get(4).unwrap().as_str(),
          )
        }).to_string();
      }
    }}
  }

  replace!(generics);
  replace!(dependencies);

  target
}

pub struct GenericsVisitor {
  pub decl: HashMap<String, String>,
}

impl TypeVisitor for GenericsVisitor {
  fn visit<T: TS + 'static + ?Sized>(&mut self) {
    let name = T::name().split("<").next().unwrap().to_string();
    self.decl.insert(name, inline::<T>());
  }
}
