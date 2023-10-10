use super::*;
use proc_macro2::TokenStream;
use quote::quote;
// use quote::ToTokens;

pub fn impl_self(node: &NodeParser) -> TokenStream {
	// let states_typed = get_states_typed(node);
	// let params_nested = child_params_nested(node);
	let NodeParser {
		ident,
		self_bounds,
		self_params,
		// child_params,
		num_children: num_edges,
		..
	} = node;

	let child_fields_def = child_fields_def(*num_edges);
	let child_fields_args = child_fields_args(*num_edges);
	let child_fields_into_child = child_fields_into_child(*num_edges);

	quote! {
		// #[derive(Clone)]
		pub struct #ident<#self_bounds>{
			attributes: Attr,
			#child_fields_def
		}

		impl<#self_bounds> #ident<#self_params> {
			pub fn new(attributes: Attr, #child_fields_args) -> Self {
				Self {
					attributes,
					#child_fields_into_child
				}
			}

		}
	}
}

fn child_fields_def(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			let ty = child_type_name(index);
			quote!(#field: #ty,)
		})
		.collect()
}

fn child_fields_into_child(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			// let ty = child_type_name(index);
			quote!(#field: #field.into_child_node(),)
		})
		.collect()
}
fn child_fields_args(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			let ty = child_type_name(index);
			let node_id_params = node_id_params_child(index);
			// quote! {#field: impl IntoChildNode<0,{GRAPH_DEPTH + 1},0,0,0,Out=#ty>,
			quote! {#field: impl IntoChildNode<#node_id_params,Out = #ty>,}
		})
		.collect()
}
