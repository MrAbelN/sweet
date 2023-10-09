use super::*;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use rstml::Parser;
use rstml::ParserConfig;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use syn::Result;

static GRAPH_ID: AtomicUsize = AtomicUsize::new(0);

pub fn parse_tree(tokens: proc_macro::TokenStream) -> Result<TokenStream> {
	let config = ParserConfig::new()
		.recover_block(true)
		.number_of_top_level_nodes(1);

	let parser = Parser::new(config);
	let (nodes_rsx, errors) = parser.parse_recoverable(tokens).split_vec();

	if let Some(node_rsx) = nodes_rsx.first() {
		let errors = errors.into_iter().map(|e| e.emit_as_expr_tokens());
		let graph_id = GRAPH_ID.fetch_add(1, Ordering::SeqCst);
		let root = TreeParser::root(node_rsx, graph_id)?;
		let out = root.to_instance();

		Ok(quote! {
			#(#errors;)*
			#out
		})
	} else {
		Err(syn::Error::new(Span::call_site(), "Expected a root node"))
	}
}