// Copyright (c) 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate provides a #[nopanic] attribute, which will prevent compilation for any function or
//! method which has the attribute but could panic! somewhere
#![warn(missing_docs)]

#![feature(plugin_registrar, quote, rustc_private)]

extern crate syntax;
#[macro_use] extern crate rustc;

use rustc::lint::{LintPassObject};
use rustc::plugin::Registry;

use syntax::ast;
use syntax::codemap::{Span};
use syntax::parse::token;
use syntax::ext::base::{Annotatable, ExtCtxt, MultiModifier};
use syntax::ptr::P;

mod lint;

/// Basic error checking for #[nopanic], and insert some extra attributes
fn nopanic_modifier(ecx: &mut ExtCtxt,
                    span: Span,
                    meta_item: &ast::MetaItem,
                    item: Annotatable) -> Annotatable {
    match &meta_item.node {
        &ast::MetaWord(_) => {},
        _ => {
            ecx.span_err(span, "#[nopanic] does not have arguments or values");
            return item;
        }
    }

    match item {
        Annotatable::Item(item) => {
            match item.node {
                ast::ItemFn(..) => {
                    let mut new_item = (*item).clone();

                    new_item.attrs.push(quote_attr!(ecx, #[_nopanic_lint]));
                    new_item.attrs.push(quote_attr!(ecx, #[allow(unused_attributes)]));

                    Annotatable::Item(P(new_item))
                },
                _ => {
                    ecx.span_err(item.span,
                                 "#[nopanic] attribute may only be used for functions and methods");
                    return Annotatable::Item(item);
                }
            }
        },
        Annotatable::TraitItem(item) => {
            let mut new_item = (*item).clone();
            match item.node {
                ast::MethodTraitItem(ref _sig, ref block) => {
                    if let &None = block {
                        ecx.span_err(item.span,
                                     "#[nopanic] on trait methods requires a block");
                        return Annotatable::TraitItem(P(new_item));
                    }

                    new_item.attrs.push(quote_attr!(ecx, #[_nopanic_lint]));
                    new_item.attrs.push(quote_attr!(ecx, #[allow(unused_attributes)]));

                    Annotatable::TraitItem(P(new_item))
                },
                _ => {
                    ecx.span_err(item.span,
                                 "#[nopanic] attribute may only be used for functions and methods");
                    return Annotatable::TraitItem(P(new_item));
                }
            }
        },
        Annotatable::ImplItem(item) => {
           match item.node {
                ast::MethodImplItem(..) => {
                    let mut new_item = (*item).clone();

                    new_item.attrs.push(quote_attr!(ecx, #[_nopanic_lint]));
                    new_item.attrs.push(quote_attr!(ecx, #[allow(unused_attributes)]));

                    Annotatable::ImplItem(P(new_item))
                },
                _ => {
                    ecx.span_err(item.span,
                                 "#[nopanic] attribute may only be used for functions and methods");
                    return Annotatable::ImplItem(item);
                }
            }
        },
    }
}

/// The entry point for the plugin/lint
#[plugin_registrar]
pub fn plugin_registrar(registry: &mut Registry) {
    registry.register_syntax_extension(token::intern("nopanic"),
                                       MultiModifier(Box::new(nopanic_modifier)));

    registry.register_lint_pass(Box::new(lint::NoPanicPass) as LintPassObject);
}
