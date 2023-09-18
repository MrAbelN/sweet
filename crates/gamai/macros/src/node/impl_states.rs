use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn edge_type(node: &NodeParser, index: usize) -> TokenStream {
	let phantom = edge_phantom(node, index);
	quote!(ChildEdgeState<#phantom>)
}
// pub fn child_node_type(node: &AiNode, index: usize) -> TokenStream {
// 	let phantom = edge_phantom(node, index);
// 	quote!(ChildNodeState<#phantom>)
// }
pub fn default_child_node_state(
	node: &NodeParser,
	index: usize,
) -> TokenStream {
	let phantom = edge_phantom(node, index);
	quote!(ChildNodeState::<#phantom>::default())
}

pub fn edge_phantom(node: &NodeParser, index: usize) -> TokenStream {
	let ident = &node.ident;
	quote!(EdgePhantom<#ident,#index>)
}