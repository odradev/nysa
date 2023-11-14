use c3_lang_linearization::Class;
use solidity_parser::pt::{self, VariableAttribute};

use crate::model::expr::to_expr;

use super::expr::Expression;

/// Stores a basic contract metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractMetadata {
    pub(super) name: String,
    base_impl: Vec<BaseCall>,
    is_abstract: bool,
    is_library: bool,
}

impl ContractMetadata {
    pub fn new(
        name: String,
        base_impl: Vec<BaseCall>,
        is_abstract: bool,
        is_library: bool,
    ) -> Self {
        Self {
            name,
            base_impl,
            is_abstract,
            is_library,
        }
    }

    pub fn base_impl(&self) -> &[BaseCall] {
        &self.base_impl
    }

    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }

    pub fn is_library(&self) -> bool {
        self.is_library
    }
}

impl From<&pt::ContractDefinition> for ContractMetadata {
    fn from(value: &pt::ContractDefinition) -> Self {
        let base_impl = value
            .base
            .iter()
            .map(|base| BaseCall {
                class_name: base.name.name.to_owned(),
                args: base.args.clone().map(to_expr).unwrap_or_default(),
            })
            .collect::<Vec<_>>();

        Self {
            name: value.name.name.to_owned(),
            base_impl,
            is_abstract: matches!(value.ty, pt::ContractTy::Abstract(_)),
            is_library: matches!(value.ty, pt::ContractTy::Library(_)),
        }
    }
}

/// Value type representation.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Type {
    Address,
    Bool,
    String,
    Int(u16),
    Uint(u16),
    Bytes(u8),
    Mapping(Box<Expression>, Box<Expression>),
    Custom(String),
    Array(Box<Type>),
    Unknown,
}

impl Type {
    pub fn as_unit(&self) -> Option<u16> {
        if let Type::Uint(size) = self {
            return Some(*size);
        }
        None
    }
}

impl From<&pt::Type> for Type {
    fn from(value: &pt::Type) -> Self {
        match value {
            pt::Type::Address => Self::Address,
            pt::Type::AddressPayable => Self::Address,
            pt::Type::Payable => Self::Address,
            pt::Type::Bool => Self::Bool,
            pt::Type::String => Self::String,
            pt::Type::Int(i) => Self::Int(*i),
            pt::Type::Uint(i) => Self::Uint(*i),
            pt::Type::Bytes(i) => Self::Bytes(*i),
            pt::Type::Mapping(_, k, v) => Self::Mapping(
                Box::new(Expression::from(&**k)),
                Box::new(Expression::from(&**v)),
            ),
            _ => panic!("Unsupported type {:?}", value),
        }
    }
}

impl From<&pt::Identifier> for Type {
    fn from(value: &pt::Identifier) -> Self {
        let name = value.name.clone();
        Self::Custom(name)
    }
}

impl TryFrom<&Expression> for Type {
    type Error = ();

    fn try_from(value: &Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::Type(ty) => Ok(ty.clone()),
            Expression::Variable(ty) => Ok(Type::Custom(ty.to_owned())),
            _ => Err(()),
        }
    }
}

/// State variable.
///
/// A variable apart from the name and type can have a initializer.
/// The initializer is called in a [Constructor](super::func::Constructor).
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Var {
    pub name: String,
    pub ty: Type,
    pub initializer: Option<Expression>,
    pub is_immutable: bool,
}

impl From<&&pt::VariableDefinition> for Var {
    fn from(value: &&pt::VariableDefinition) -> Self {
        Self {
            name: value.name.name.to_owned(),
            ty: match &value.ty {
                pt::Expression::Type(_, ty) => Type::from(ty),
                pt::Expression::Variable(id) => Type::from(id),
                pt::Expression::ArraySubscript(_, ty, _) => {
                    let ty = Expression::from(&**ty);
                    let ty = Type::try_from(&ty).expect("Should be a valid array type");
                    Type::Array(Box::new(ty))
                }
                t => panic!("Not a type. {:?}", t),
            },
            initializer: value.initializer.as_ref().map(Expression::from),
            is_immutable: value
                .attrs
                .iter()
                .any(|attr| matches!(attr, VariableAttribute::Constant(_))),
        }
    }
}

impl Into<Class> for &Var {
    fn into(self) -> Class {
        self.name.as_str().into()
    }
}

/// Stores data required to create an event.
pub struct Event {
    pub name: String,
    pub fields: Vec<(String, Expression)>,
}

impl From<&&pt::EventDefinition> for Event {
    fn from(value: &&pt::EventDefinition) -> Self {
        let name = value.name.name.to_owned();

        let to_field_name = |(idx, f): (usize, &pt::EventParameter)| {
            (
                f.name
                    .as_ref()
                    .map(|id| id.name.to_owned())
                    .unwrap_or(format!("field_{}", idx)),
                Expression::from(&f.ty),
            )
        };

        let fields = value.fields.iter().enumerate().map(to_field_name).collect();
        Self { name, fields }
    }
}

/// Stores data required to build an error.
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub name: String,
}

impl From<&&pt::ErrorDefinition> for Error {
    fn from(value: &&pt::ErrorDefinition) -> Self {
        let name = value.name.name.to_owned();
        Self { name }
    }
}

/// Represents base implementation call (a modifier call, or super constructor call).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BaseCall {
    pub class_name: String,
    pub args: Vec<Expression>,
}

/// Stores data required to create a custom enum.
#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}

impl From<&&pt::EnumDefinition> for Enum {
    fn from(value: &&pt::EnumDefinition) -> Self {
        let name = value.name.name.to_owned();
        let variants = value.values.iter().map(|v| v.name.to_string()).collect();
        Self { name, variants }
    }
}

/// Stores data required to create a custom struct.
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub namespace: Option<String>,
    pub name: String,
    pub fields: Vec<(String, Expression)>,
}

impl From<(Option<String>, &pt::StructDefinition)> for Struct {
    fn from(value: (Option<String>, &pt::StructDefinition)) -> Self {
        let (namespace, def) = value;
        let name = def.name.name.to_owned();
        let fields = def
            .fields
            .iter()
            .map(|v| (v.name.name.to_owned(), Expression::from(&v.ty)))
            .collect();
        Struct {
            name,
            fields,
            namespace: namespace.clone(),
        }
    }
}

/// Stores data representing solidity using `lib_name` for `type` expression.
#[derive(Debug, Clone, PartialEq)]
pub struct LibUsing {
    pub name: String,
    pub ty: Expression,
}

impl From<&&pt::Using> for LibUsing {
    fn from(value: &&pt::Using) -> Self {
        let name = value.library.name.to_owned();
        let ty = value.ty.as_ref().map(Expression::from).unwrap();
        Self { name, ty }
    }
}
