use super::expr::Expr;
use crate::Span;
use std::ops::Range;

/// Represents a type in the `Pkl` language.
///
/// # Overview
///
/// The `Type` struct encapsulates a type definition within the `Pkl` language.
/// A type typically has a name, optional parameters, and optional constraints
/// or restraints associated with it. This struct is designed to hold all
/// relevant information needed to represent a type in the `Pkl` abstract
/// syntax tree (AST).
///
/// # Fields
///
/// * `name: (String, Span)` - This tuple contains the name of the type as a `String`
/// along with its `Span`, which represents the location of the type name in the source code.
///
/// * `params: Option<Vec<(Type, Span)>>` - An optional vector of type parameters,
/// each represented by a tuple containing a `Type` and its associated `Span`.
/// If the type has no parameters, this will be `None`.
///
/// * `restraints: Option<Box<Expr>>` - An optional expression that imposes
/// constraints or restraints on the type. This could be used to enforce certain
/// conditions or requirements on the type. If there are no restraints, this will be `None`.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primary {
        name: (String, Span),
        params: Option<Vec<(Type, Span)>>,
        restraints: Option<Box<Expr>>,
        is_nullable: bool,
        is_default: bool,
    },

    String {
        value: (String, Span),
        is_nullable: bool,
        is_default: bool,
    },
    Function {
        is_nullable: bool,
        is_default: bool,
    },

    Union(Box<Type>, Box<Type>),
    Parenthesized {
        value: Box<Type>,
        is_nullable: bool,
        is_default: bool,
    },
}

impl Type {
    pub fn is_default(&self) -> bool {
        match self {
            Type::Primary { is_default, .. } => *is_default,
            Type::Union(_, t) => t.is_default(),
            Type::Parenthesized { is_default, .. } => *is_default,
            Type::String { is_default, .. } => *is_default,
            Type::Function { is_default, .. } => *is_default,
        }
    }
    pub fn is_nullable(&self) -> bool {
        match self {
            Type::Primary { is_nullable, .. } => *is_nullable,
            Type::Union(_, t) => t.is_nullable(),
            Type::Parenthesized { is_nullable, .. } => *is_nullable,
            Type::String { is_nullable, .. } => *is_nullable,
            Type::Function { is_nullable, .. } => *is_nullable,
        }
    }
    pub fn set_name(&mut self, name: (String, Span)) {
        match self {
            Type::Primary { name: n, .. } => *n = name,
            Type::Union(_, t) => t.set_name(name),
            _ => unreachable!(),
        }
    }
    pub fn set_default(&mut self) {
        match self {
            Type::Primary { is_default, .. } => *is_default = true,
            Type::Union(_, t) => t.set_default(),
            Type::Parenthesized { is_default, .. } => *is_default = true,
            Type::String { is_default, .. } => *is_default = true,
            Type::Function { is_default, .. } => *is_default = true,
        }
    }
    pub fn set_nullable(&mut self) {
        match self {
            Type::Primary { is_nullable, .. } => *is_nullable = true,
            Type::Union(_, t) => t.set_nullable(),
            Type::Parenthesized { is_nullable, .. } => *is_nullable = true,
            Type::String { is_nullable, .. } => *is_nullable = true,
            Type::Function { is_nullable, .. } => *is_nullable = true,
        }
    }
    pub fn set_restraints(&mut self, restraints: Expr) {
        match self {
            Type::Primary {
                name,
                params,
                restraints: r,
                is_nullable,
                is_default,
            } => *r = Some(Box::new(restraints)),
            Type::Union(_, t) => t.set_restraints(restraints),
            _ => unreachable!(),
        }
    }
    pub fn set_params(&mut self, params: Vec<(Type, Span)>) {
        match self {
            Type::Primary {
                name,
                params: p,
                restraints,
                is_nullable,
                is_default,
            } => *p = Some(params),
            Type::Union(_, t) => t.set_params(params),
            _ => unreachable!(),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Primary {
            name: (String::default(), Range::default()),
            params: None,
            restraints: None,
            is_nullable: false,
            is_default: false,
        }
    }
}
