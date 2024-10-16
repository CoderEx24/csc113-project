use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Display};
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
    pub return_type: Type,
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
        self.features
            .get(name)
            .map_or_else(|| self.parent.contains_method(name), |u| u.is_method())
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
            Err(format!("method {} is not defined already", method.name))
        } else {
            self.features
                .insert(method.name.clone(), ClassFeature::Method(method));
            Ok(())
        }
    }

    pub fn get_member_type(&self, name: &str) -> Option<&Type> {
        self.features
            .get(name)
            .map(|f| match f {
                ClassFeature::Member(_, t) => Some(t),
                _ => None,
            })
            .flatten()
    }

    pub fn get_method(&self, name: &str) -> Option<&Method> {
        self.features
            .get(name)
            .map(|f| match f {
                ClassFeature::Method(m) => Some(m),
                _ => None,
            })
            .flatten()
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Object => write!(f, "Object"),
            Type::String => write!(f, "String"),
            Type::Bool => write!(f, "Bool"),
            Type::Int => write!(f, "Int"),
            Type::IO => write!(f, "IO"),
            Type::Custom(t) => write!(f, "{}", t.borrow().name),
        }
    }
}

impl Type {
    pub fn is_builtin(name: &str) -> bool {
        match name {
            "Object" | "String" | "Bool" | "Int" | "IO" => true,
            _ => false,
        }
    }

    pub fn is_inheritable(name: &str) -> bool {
        match name {
            "String" | "Bool" | "Int" => false,
            _ => true,
        }
    }

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

    pub fn add_member_variable(&self, name: &str, type_: &Type) -> Result<(), String> {
        match self {
            Type::Custom(t) => t.borrow_mut().add_member_variable(name, type_),
            _ => Err(format!(
                "Cannot add members to builtin type {}.\nSomething is very wrong",
                self
            )),
        }
    }

    pub fn add_method(
        &self,
        name: &str,
        parameters: HashMap<String, Type>,
        return_type: &Type,
    ) -> Result<(), String> {
        match self {
            Type::Custom(t) => {
                let method = Method {
                    name: name.to_owned(),
                    class: t.clone(),
                    parameters,
                    return_type: return_type.clone(),
                };
                t.borrow_mut().add_method(method)
            }
            _ => Err(format!(
                "Cannot add methods to builtin type {}.\nSomething is very wrong",
                self
            )),
        }
    }

    pub fn redefine_method(
        &self,
        name: &str,
        parameters: HashMap<String, Type>,
        return_type: &Type,
    ) -> Result<(), String> {
        match self {
            Type::Custom(t) => {
                let method = Method {
                    name: name.to_owned(),
                    class: t.clone(),
                    parameters,
                    return_type: return_type.clone(),
                };
                t.borrow_mut().redefine_method(method)
            }
            _ => Err(format!("Cannot redefined methods of builtin {}", self)),
        }
    }

    pub fn unwrap_class(&self) -> Option<Rc<RefCell<Class>>> {
        match self {
            Type::Custom(t) => Some(t.clone()),
            _ => None
        }
    }
}

struct SymbolTable {
    symbols: HashMap<String, Type>,
    i: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            i: 0,
        }
    }

    pub fn contains_symbol(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    pub fn add_new_variable(&mut self, name: &str, t: Type) -> Result<(), String> {
        if self.contains_symbol(name) {
            Err(format!("{} is already defined", name))
        } else {
            self.symbols.insert(name.to_owned(), t);
            Ok(())
        }
    }

    pub fn add_new_temporary(&mut self, t: Type) {
        let name = format!("t{}", self.i);
        self.i += 1;
        self.symbols.insert(name, t);
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        self.symbols.get(name)
    }
}

pub struct Env {
    classes: HashMap<String, Rc<RefCell<Class>>>,
    symbol_tables: Vec<SymbolTable>,
}

impl Env {
    fn into_type(&self, name: &str) -> Result<Type, String> {
        Ok(match name {
            "Object" => Type::Object,
            "String" => Type::String,
            "Bool" => Type::Bool,
            "Int" => Type::Int,
            "IO" => Type::IO,
            _ => {
                let c = self
                    .classes
                    .get(name)
                    .cloned()
                    .ok_or(format!("Type {} is not defined", name))?;
                Type::Custom(c)
            }
        })
    }

    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            symbol_tables: vec![],
        }
    }

    pub fn type_declared(&self, name: &str) -> bool {
        Type::is_builtin(name) || self.classes.contains_key(name)
    }

    pub fn declare_class(&mut self, name: &str, parent: Option<&str>) -> Result<(), String> {
        if self.type_declared(name) {
            Err(format!("class {} is already defined", name))
        } else {
            let parent_is_inheritable = parent.map_or(true, |c| Type::is_inheritable(c));

            if !parent_is_inheritable {
                return Err(format!("No class can inherit from Int, Bool or String"));
            }

            let parent = parent.map(|n| self.into_type(n).unwrap());

            let new_class = Class::new(name, parent);
            let new_class = Rc::new(RefCell::new(new_class));
            self.classes.insert(name.to_owned(), new_class);

            Ok(())
        }
    }

    pub fn define_member_variable(
        &mut self,
        classname: &str,
        name: &str,
        type_: &str,
    ) -> Result<(), String> {
        if Type::is_builtin(classname) {
            Err(format!(
                "Cannot define member variables for {}.\nSomething is very wrong!!!",
                classname
            ))
        } else {
            let class = self.into_type(classname)?;
            let t = self.into_type(type_)?;

            class.add_member_variable(name, &t)
        }
    }

    pub fn define_method(
        &mut self,
        classname: &str,
        name: &str,
        parameters: HashMap<String, String>,
        return_type: &str,
    ) -> Result<(), String> {
        if Type::is_builtin(classname) {
            Err(format!(
                "Cannot define member variables for {}.\nSomething is very wrong!!!",
                classname
            ))
        } else if !parameters.values().all(|t| self.type_declared(t.as_str())) {
            let param = parameters
                .iter()
                .find(|(_, t)| !self.type_declared(t.as_str()))
                .unwrap();
            Err(format!(
                "type {} of parameter {} is not declared",
                param.1, param.0
            ))
        } else {
            let parameters: HashMap<String, Type> = parameters
                .iter()
                .map(|(n, t)| (n.clone(), self.into_type(t).unwrap()))
                .collect();

            let class = self.into_type(classname)?;
            let return_type = self.into_type(return_type)?;

            class.add_method(name, parameters, &return_type)
        }
    }

    pub fn start_scope(&mut self) {
        self.symbol_tables.push(SymbolTable::new());
    }

    pub fn end_scope(&mut self) {
        self.symbol_tables.pop();
    }

    pub fn add_variable(&mut self, name: &str, type_: &str) -> Result<(), String> {
        let t = self.into_type(type_)?;

        self.symbol_tables
            .iter_mut()
            .last()
            .unwrap()
            .add_new_variable(name, t)
    }

    pub fn add_temporary(&mut self, type_: &str) -> Result<(), String> {
        let t = self.into_type(type_)?;

        self.symbol_tables
            .iter_mut()
            .last()
            .unwrap()
            .add_new_temporary(t);

        Ok(())
    }
}
