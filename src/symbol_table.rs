use std::collections::HashMap;

#[derive(PartialEq, Clone)]
enum Type<'a> {
    String,
    Object,
    Bool,
    Int,
    IO,
    Custom(&'a Class<'a>),
}

#[derive(PartialEq, Clone)]
struct Method<'a> {
    pub name: String,
    pub classname: String,
    pub parameters: HashMap<String, Type<'a>>,
}

impl<'a> Method<'a> {
    pub fn label(&self) -> String {
        format!("{}_{}", self.classname, self.name)
    }
}

#[derive(PartialEq, Clone)]
enum ClassFeature<'a> {
    Member(String, Type<'a>),
    Method(Method<'a>),
}

#[derive(PartialEq, Clone)]
struct Class<'a> {
    pub name: String,
    pub parent: Option<Type<'a>>,
    pub features: HashMap<String, ClassFeature<'a>>,
}

impl<'a> Class<'a> {
    pub fn contains_feature(&self, feature_name: &str) -> bool {
        self.features.contains_key(feature_name)
    }

    pub fn add_member_variable(&mut self, name: &str, type_: &'a Type) -> Result<(), String> {
        if self.contains_feature(name) {
            Err(format!("{} already contains member variable {}", self.name, name))
        } else {
            let new_member_variable = 
                ClassFeature::Member(name.to_owned(), type_.clone());

            self.features.insert(name.to_owned(), new_member_variable);
            Ok(())
        }
    }

    pub fn add_method(&'a mut self, name: &str, parameters: HashMap<String, Type<'a>>) -> Result<(), String> {
        if self.contains_feature(name) {
            Err(format!("{} already contains method {}", self.name, name))
        } else {
            let n = name.to_string();
            let new_method = Method {
                name: n,
                classname: self.name.clone(),
                parameters,
            };

            let new_method = ClassFeature::Method(new_method);

            self.features.insert(name.to_owned(), new_method);
            Ok(())
        }
    }
}

impl<'a> Type<'a> {
    pub fn contains_feature(&self, feature_name: &str) -> bool {
        match self {
            Type::Object => match feature_name {
                "abort" | "type_name" | "copy" => true,
                _ => false,
            },
            Type::IO => match feature_name {
                "out_string" | "out_int" |
                    "in_string" | "in_int" => true,
                _ => false,
            },
            Type::String => match feature_name {
                "length" | "concat" | "substr" => true,
                _ => false,
            },
            Type::Custom(t) => t.contains_feature(feature_name),
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone)]
struct Expr<'a> {
    pub type_: Type<'a>,
    pub name: String,
}
