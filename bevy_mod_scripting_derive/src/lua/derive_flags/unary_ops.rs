use indexmap::{IndexMap, IndexSet};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, ToTokens, quote_spanned};
use syn::{punctuated::Punctuated, Token, LitInt, Type, spanned::Spanned, parse_quote_spanned, Attribute, parse_quote, Ident};

use crate::{lua::{lua_method::LuaMethod, LuaImplementor}, common::{derive_flag::DeriveFlag, newtype::Newtype, ops::{OpName, OpExpr, Side}, type_base_string}, EmptyToken};

pub(crate) fn make_unary_ops<'a>(flag: &DeriveFlag,new_type : &'a Newtype, out : &mut Vec<LuaMethod>) -> Result<(), syn::Error> {

    let newtype = &new_type.args.wrapper_type;


    let (ident, ops) = match flag {
        DeriveFlag::UnaryOps { ident, ops, .. } => (ident, ops),
        _ => panic!("Expected UnaryOps flag")
    };

  
    ops.iter().for_each(|op| {

        let meta = op.op.to_rlua_metamethod_path();
        let mut body = op.map_side(Side::Right,|s| {
            if s.type_().unwrap_err().is_any_ref(){
                quote_spanned!{op.span()=>(&ud.inner()?)}
            } else {
                quote_spanned!{op.span()=>ud.inner()?}
            }
        }).expect("Expected unary expression");
        op.map_return_type_with_default(parse_quote!{Wrapped(#newtype)},|v| {
            if v.is_wrapped() {
                let resolved_type = v.type_().cloned()
                    .unwrap_or_else(|self_| self_.resolve_as(parse_quote!(#newtype)));
                body = quote_spanned!{op.span()=>#resolved_type::new(#body)}
            }

        });

        out.push(parse_quote_spanned! {ident.span()=>
            (mlua::MetaMethod::#meta) => |_,ud,()|{
                return Ok(#body)
            }
        });
    });

    Ok(())

}
