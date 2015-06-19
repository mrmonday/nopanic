// Copyright (c) 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Linting and other functionality which requires type information

use rustc::lint::{Context, LintPass, LintArray};

use syntax::ast;
use syntax::visit;

declare_lint! {
    NOPANIC_LINT,
    Deny,
    "prevent a function or method from panicking"
}

declare_lint! {
    NOPANIC_USAGE,
    Forbid,
    "incorrect usage of #[nopanic] attribute"
}

fn should_check(attrs: &[ast::Attribute]) -> bool {
    for attr in attrs {
        if let ast::MetaWord(ref s) = attr.node.value.node {
            if &s[..] == "_nopanic_lint" {
                return true;
            }
        }
    }

    false
}

pub struct NoPanicVisitor<'a, 'b : 'a> {
    cx: &'a Context<'a, 'b>,
}

impl<'a, 'b, 'v> visit::Visitor<'v> for NoPanicVisitor<'a, 'b> {
    fn visit_expr(&mut self, expr: &'v ast::Expr) {
        match expr.node {
            ast::ExprCall(ref expr_path, _) => {
                //println!("SOME CALL EXPR");
                if let ast::ExprPath(_, ref path) = expr_path.node {
                    if path.segments.len() == 3 &&
                       path.segments[0].identifier.as_str() == "std" &&
                       path.segments[1].identifier.as_str() == "rt" &&
                       (path.segments[2].identifier.as_str() == "begin_unwind" ||
                        path.segments[2].identifier.as_str() == "begin_unwind_fmt" ||
                        path.segments[2].identifier.as_str() == "rust_begin_unwind"
                        ){
                        self.cx.span_lint(NOPANIC_LINT, expr.span, "this expression could cause a panic!");
                    }
                }
            },
            _ => {}
        }
        visit::walk_expr(self, expr);
    }
}

pub struct NoPanicPass;

impl LintPass for NoPanicPass {
    fn get_lints(&self) -> LintArray {
        lint_array!(NOPANIC_LINT, NOPANIC_USAGE)
    }

    fn check_trait_item(&mut self, cx: &Context, item: &ast::TraitItem) {
        if !should_check(&item.attrs) {
            return;
        }

        match item.node {
            ast::MethodTraitItem(ref _sig, ref block) => {
                if let &Some(ref block) = block {
                    check_block(cx, block);
                } else {
                    panic!("should be caught in lib.rs");
                }
            },
            _ => {
                panic!("should be caught in lib.rs");
            }
        }
    }

    fn check_impl_item(&mut self, cx: &Context, item: &ast::ImplItem) {
        if !should_check(&item.attrs) {
            return;
        }

        match item.node {
            ast::MethodImplItem(ref _sig, ref block) => {
                check_block(cx, &block);
            },
            _ => {
                panic!("should be caught in lib.rs");
            }
        }
    }

    fn check_item(&mut self, cx: &Context, item: &ast::Item) {
        if !should_check(&item.attrs) {
            return;
        }
        match item.node {
            ast::ItemFn(ref _decl, ref _unsafety, ref _constness,
                        ref _abi, ref _generics, ref block) => {
                check_block(cx, block);
            },
            _ => {
                panic!("should be caught in lib.rs");
            }
        }
    }
}

fn check_block(cx: &Context, block: &ast::Block) {
    println!("block: {:?}", block);

    visit::walk_block(&mut NoPanicVisitor { cx: cx }, &block);
}
