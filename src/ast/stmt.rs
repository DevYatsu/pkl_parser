use amends::Amends;
use class::ClassDeclaration;
use extends::Extends;
use import::Import;
use module::Module;
use property::Property;
use typealias::TypeAlias;

mod amends;
mod class;
mod extends;
mod function;
mod import;
mod module;
mod property;
mod typealias;

/// Represent any valid Pkl Statement.
#[derive(Debug, PartialEq, Clone)]
pub enum PklStatement<'a> {
    /// A pkl property
    Property(Property<'a>),

    /// A pkl import
    Import(Import<'a>),

    /// A class declaration
    Class(ClassDeclaration<'a>),

    /// A typealias
    TypeAlias(TypeAlias<'a>),

    /// A module clause, used to declare a module name
    ModuleClause(Module<'a>),

    /// An amends clause, it's like extending
    /// but then you can't create any variable
    /// that is not declared in the amended
    /// module
    AmendsClause(Amends<'a>),

    /// An extends clause, literally it's like importing
    /// but directly in the main context,
    /// not in a variable creating in the context
    /// containing the import values.
    ExtendsClause(Extends<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModifiersList {
    list: Vec<i8>,
}

impl ModifiersList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn push(&mut self, modifier: &str) {
        self.list.push(Self::modifier_as_id(modifier))
    }
    // pub fn is_for_class(&self) -> bool {
    //     for mod_id in &self.list {
    //         if ![1, 4].contains(&mod_id) {
    //             return false;
    //         }
    //     }
    //     true
    // }
    // pub fn is_for_property(&self) -> bool {
    //     for mod_id in &self.list {
    //         if ![0, 5, 6].contains(&mod_id) {
    //             return false;
    //         }
    //     }
    //     true
    // }
    // pub fn is_for_typealias(&self) -> bool {
    //     for mod_id in &self.list {
    //         if ![1, 4, 5, 6].contains(&mod_id) {
    //             return false;
    //         }
    //     }
    //     true
    // }

    pub fn modifier_as_id(modifier: &str) -> i8 {
        match modifier {
            "hidden" => 0,
            "local" => 1,
            "fixed" => 2,
            "const" => 3,
            "external" => 4,
            "open" => 5,
            "abstract" => 6,
            _ => panic!("{}", format!("unsupported modifier `{modifier}`")),
        }
    }
    pub fn modifier_as_str(modifier_id: i8) -> &'static str {
        match modifier_id {
            0 => "hidden",
            1 => "local",
            2 => "fixed",
            3 => "const",
            4 => "external",
            5 => "open",
            6 => "abstract",
            _ => panic!("{}", format!("unsupported modifier id `{modifier_id}`")),
        }
    }
}
