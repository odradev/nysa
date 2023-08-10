use std::{collections::HashMap, sync::Mutex};

use proc_macro2::Ident;
use solidity_parser::pt;

use crate::utils;

use super::NysaExpression;

pub type NysaFuncArg = pt::Type;

lazy_static::lazy_static! {
    static ref FUNCTIONS: Mutex<HashMap<String, Vec<NysaFunc>>> = Mutex::new(HashMap::new());
}

#[derive(Debug, PartialEq)]
struct NysaFunc {
    real_name: String,
    arg_types: Vec<NysaFuncArg>,
    alias: String,
}

impl From<&pt::FunctionDefinition> for NysaFunc {
    fn from(value: &pt::FunctionDefinition) -> Self {
        let params = value
            .params
            .iter()
            .map(|p| p.1.clone().unwrap().ty.clone())
            .filter_map(|e| match e {
                pt::Expression::Type(_, ty) => Some(ty),
                _ => panic!("Unexpected param"),
            })
            .collect::<Vec<_>>();
        let real_name = value
            .name
            .as_ref()
            .map(|name| name.name.to_owned())
            .expect("function must have a name");

        let funs = FUNCTIONS.lock().unwrap();

        todo!()
    }
}

impl TryFrom<&NysaExpression> for NysaFunc {
    type Error = ();

    fn try_from(value: &NysaExpression) -> Result<Self, Self::Error> {
        match value {
            NysaExpression::Func { name, args } => Err(()),
            NysaExpression::SuperCall { name, args } => Err(()),
            _ => Err(()),
        }
    }
}

pub fn resolve_fn_name(expr: &NysaExpression) -> Ident {
    let func = NysaFunc::try_from(expr)
        .expect("Invalid expr, NysaExpression::Func or NysaExpression::SuperCall expected");
    let ident = utils::to_snake_case_ident(&func.alias);

    ident
    // match expr {
    //     NysaExpression::Func { name, args } => {

    //     }
    //     NysaExpression::SuperCall { name, args } => {

    //     },
    //     _ => panic!("Invalid NysaExpression")
    // }
    // resolve_fn_name(expr)
}
