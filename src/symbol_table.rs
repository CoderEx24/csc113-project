use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Clone)]
enum Type {
    String,
    Object,
    Bool,
    Int,
    IO,
    Custom(Rc<RefCell<Class>>),
}

#[derive(PartialEq, Clone)]
struct Method {
    pub name: String,
    pub class: Rc<RefCell<Class>>,
    pub parameters: HashMap<String, Type>,
}

impl Method {
    pub fn label(&self) -> String {
        format!("{}_{}", self.class.borrow().name, self.name)
    }
}

#[derive(PartialEq, Clone)]
enum ClassFeature {
    Member(String, Type),
    Method(Method),
}

impl ClassFeature {
    pub fn is_member(&self) -> bool {
        match self {
            ClassFeature::Member(_, _) => true,
            _ => false,
        }
    }

    pub fn is_method(&self) -> bool {
        match self {
            ClassFeature::Method(_) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Clone)]
struct Class {
    pub name: String,
    pub parent: Type,
    pub features: HashMap<String, ClassFeature>,
}

impl Class {
    pub fn new(name: &str, parent: Option<Type>) -> Self {
        Self {
            name: name.to_owned(),
            parent: parent.unwrap_or(Type::Object),
            features: HashMap::new(),
        }
    }

    pub fn contains_member_variable(&self, name: &str) -> bool {
        self.features.get(name).map_or_else(
            || self.parent.contains_member_variable(name),
            |u| u.is_member(),
        )
    }

    pub fn contains_method(&self, name: &str) -> bool {
        self.features.get(name).map_or_else(
            || self.parent.contains_method(name),
            |u| u.is_method()
        )
    }

    pub fn add_member_variable(&mut self, name: &str, type_: &Type) -> Result<(), String> {
        if self.contains_member_variable(name) {
            Err(format!(
                "{} already contains member variable {}",
                self.name, name
            ))
        } else {
            let new_member_variable = ClassFeature::Member(name.to_owned(), type_.clone());

            self.features.insert(name.to_owned(), new_member_variable);
            Ok(())
        }
    }

    // NOTE: The method needs to be constructed outside the class
    //       because the Env object is the main owner of all classes
    pub fn add_method(&mut self, method: Method) -> Result<(), String> {
        if self.contains_method(method.name.as_str()) {
            Err(format!(
                "{} already contains method {}",
                self.name, method.name
            ))
        } else {
            self.features
                .insert(method.name.clone(), ClassFeature::Method(method));
            Ok(())
        }
    }

    pub fn redefine_method(&mut self, method: Method) -> Result<(), String> {
        if !self.contains_method(method.name.as_str()) {
            Err(format!(
                "method {} is not defined already", 
                method.name
            ))
        } else {
            self.features.insert(method.name.clone(), ClassFeature::Method(method));
            Ok(())
        }
    }
}

impl Type {
    pub fn contains_member_variable(&self, name: &str) -> bool {
        match self {
            Type::Custom(t) => t.borrow().contains_member_variable(name),
            _ => false,
        }
    }

    pub fn contains_method(&self, name: &str) -> bool {
        match self {
            Type::Object => match name {
                "abort" | "type_name" | "copy" => true,
                _ => false,
            },
            Type::IO => match name {
                "out_string" | "out_int" | "in_string" | "in_int" => true,
                _ => false,
            },
            Type::String => match name {
                "length" | "concat" | "substr" => true,
                _ => false,
            },
            Type::Custom(t) => t.borrow().contains_method(name),
            _ => false,
        }
    }
}
