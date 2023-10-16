#![feature(associated_type_bounds)]
use proc_macro::TokenStream;
mod tree;
use tree::*;
mod node_def;
use node_def::*;
mod action;
use action::*;

#[proc_macro]
pub fn define_node(attr: TokenStream) -> TokenStream { parse_node(attr) }

#[proc_macro_attribute]
pub fn action(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_action(attr, item)
}

#[proc_macro]
pub fn tree(item: TokenStream) -> TokenStream {
	parse_tree(item)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}
#[proc_macro_attribute]
pub fn tree_builder(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_tree_builder(attr, item)
}

#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream { html_inner(tokens, false) }
