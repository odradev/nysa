use c3_lang_parser::c3_ast::{FnDef, PlainFnDef, ClassFnImpl};
use syn::parse_quote;
use c3_lang_linearization::Class;

use crate::{parser::context::{TypeInfo, StorageInfo, EventsRegister, ExternalCallsRegister, ContractInfo, FnContext}, model::{ir::{Package, Function, Func}, Named}, ParserError, utils};

use super::common;

pub(crate) fn libraries_def<T>(
    package: &Package,
    ctx: &mut T,
) -> Result<Vec<syn::Item>, ParserError>
where T: TypeInfo + StorageInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext {
    package.libraries().iter().map(|lib| {
        let ident = utils::to_snake_case_ident(&lib.name());
        let fns = lib.fns()
            .iter()
            .filter_map(|f| match f {
                Function::Function(f) => Some(f),
                _ => None
            })
            .map(|f| parse_fn(f, ctx))
            .collect::<Result<Vec<_>, _>>();
        
        match fns {
            Ok(fns) => Ok(parse_quote!(pub struct #ident {
                #(#fns)*
            })),
            Err(err) => Err(err),
        }
    }).collect()
}

fn parse_fn<T>(func: &Func, ctx: &mut T) -> Result<FnDef, ParserError> 
where T: TypeInfo + StorageInfo + EventsRegister + ExternalCallsRegister + ContractInfo + FnContext  {
    let attrs = vec![];
    let args = common::parse_parameters(&func.params, ctx)?;
    let mut stmts: Vec<syn::Stmt> = vec![];
    
    stmts.extend(common::parse_statements(&func.stmts, ctx));
    let name: Class = func.name.as_str().into();

    Ok(FnDef::Plain(PlainFnDef {
        attrs,
        name: name.clone(),
        args,
        ret: common::parse_ret_type(&func.ret, ctx)?,
        implementation: ClassFnImpl {
            class: None,
            fun: name,
            implementation: parse_quote!({ #(#stmts)* }),
            visibility: parse_quote!(pub),
        },
    }))
}