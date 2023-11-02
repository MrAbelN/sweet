use super::*;
// use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn impl_node(node: &NodeParser) -> TokenStream {
	let NodeParser {
		ident,
		// child_bounds,
		num_children,
		self_params,
		self_bounds,
		..
	} = node;
	let child_query = child_query(*num_children);
	let child_query_opt = child_query_opt(*num_children);
	let child_query_mut = child_query_mut(*num_children);
	let child_query_opt_mut = child_query_opt_mut(*num_children);
	let node_params = node_params(*num_children);
	let node_params_mut = node_params_mut(*num_children);
	let child_states = build_child_states(quote!(ChildProp), *num_children);

	let child_states_opt =
		build_child_states(quote!(ChildPropOpt), *num_children);
	let state_recast_opt = state_recast_opt(*num_children);

	let child_states_mut =
		build_child_states(quote!(ChildPropMut), *num_children);
	let state_recast_mut = state_recast_mut(*num_children);

	let child_states_opt_mut =
		build_child_states(quote!(ChildPropOptMut), *num_children);
	let state_recast_opt_mut = state_recast_opt_mut(*num_children);


	let child_tree_bundle_types = child_tree_bundle_types(*num_children);
	let child_tree_bundle_values = child_tree_bundle_values(*num_children);
	let get_children = get_children(*num_children);
	let get_children_owned = get_children_owned(*num_children);
	let match_get_children = match_get_children(*num_children);
	let match_get_children_owned = match_get_children_owned(*num_children);


	let child_fields_def = child_fields_def(*num_children);
	let child_fields_args = child_fields_args(*num_children);
	let child_fields = child_fields_assignment(*num_children);
	let child_fields_markers = child_fields_markers(*num_children);


	let children_inferred_types = children_inferred_types(*num_children);
	let children_into_child = children_into_child(*num_children);
	let children_with_path = children_with_path(*num_children);
	let recursive_children = recursive_children(*num_children);

	quote! {
		#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
		pub struct #ident<#self_bounds>{
			phantom: std::marker::PhantomData<Path>,
			#child_fields_def
		}

		impl<#self_bounds> #ident<#self_params> {
			pub fn new<#child_fields_markers>(#child_fields_args) -> Self {
				Self {
					phantom: std::marker::PhantomData,
					#child_fields
				}
			}
		}

		impl<#self_bounds> TreePath for #ident<#self_params> {
			type Parent = Path::Parent;
			const CHILD_INDEX: usize = Path::CHILD_INDEX;
		}

		impl<#self_bounds> AiNode for #ident<#self_params> {

			type WithPath<NewPath: TreePath> = #ident<NewPath,#children_with_path>;

			type ChildQuery<T:IntoProp> = (
				Entity,
				#child_query
			);
			type ChildQueryOpt<T:IntoProp> = (
				Entity,
				#child_query_opt
			);
			type ChildQueryMut<T:IntoProp> = (
				Entity,
				#child_query_mut
			);
			type ChildQueryOptMut<T:IntoProp> = (
				Entity,
				#child_query_opt_mut
			);

			#[allow(unused_parens)]
			type BundleRecursive<T:IntoProp> = (Prop<T,Self>,#child_tree_bundle_types);

			fn tree_bundle<T:IntoProp + Clone>(value: T) -> Self::BundleRecursive<T>{
				(
					Prop::new(value.clone()),
					#child_tree_bundle_values
				)
			}

			fn entity<'a,T:IntoProp>(val: &<Self::ChildQuery<T> as WorldQuery>::Item<'a>) ->Entity{
				val.0
			}

			fn children<'a,T:IntoProp>((entity,#node_params): <Self::ChildQuery<T> as WorldQuery>::Item<'a>)
				-> Vec<Box<dyn IntoChildProp<'a, T> + 'a>> {
					vec![#child_states]
				}

			fn children_opt<'a, T: IntoProp>((entity,#node_params): <Self::ChildQueryOpt<T> as WorldQuery>::Item<'a>,
			) -> Vec<Box<dyn IntoChildPropOpt<'a, T> + 'a>>{
				#state_recast_opt
				vec![#child_states_opt]
			}

			fn children_mut<'a, T: IntoProp>((entity,#node_params_mut): <Self::ChildQueryMut<T> as WorldQuery>::Item<'a>,
			) -> Vec<Box<dyn IntoChildPropMut<'a, T> + 'a>>{
				#state_recast_mut
				vec![#child_states_mut]
			}

			fn children_opt_mut<'a, T: IntoProp>((entity,#node_params): <Self::ChildQueryOptMut<T> as WorldQuery>::Item<'a>,
			) -> Vec<Box<dyn IntoChildPropOptMut<'a, T> + 'a>>{
				#state_recast_opt_mut
				// vec![]
				vec![#child_states_opt_mut]
			}

			fn get_child(&self,index:usize)->&dyn NodeInspector{
				match index{
					#match_get_children
					_=> panic!("invalid child index")
				}
			}
			fn get_child_owned(self,index:usize)->Box<dyn NodeInspector>{
				match index{
					#match_get_children_owned
					_=> panic!("invalid child index")
				}
			}
			fn get_children(&self)->Vec<&dyn NodeInspector>{
				vec![#get_children]
			}
			fn get_children_owned(self)->Vec<Box<dyn NodeInspector>>{
				vec![#get_children_owned]
			}

			fn get_recursive_inner<T: IntoProp>(
				world: &World,
				entity: Entity,
				child_index:usize,
				depth:usize,
			) -> PropTree<T>{
				PropTree{
					child_index,
					depth,
					value:Prop::<T,Self>::get_ref_from_node(world,entity).map(|v|&v.value),
					children:vec![#recursive_children]
				}
			}

			fn into_child<NewPath: TreePath>(self) -> impl AiNode {
				#ident::<NewPath, #children_inferred_types>::new(#children_into_child)
			}
		}
	}
}



fn child_fields_args(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			let ty = child_type_name(index);
			let marker = child_marker_type(index);
			quote!(#field: impl IntoNode<#marker, Out=#ty>,)
		})
		.collect()
}

fn child_fields_assignment(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			quote!(#field:#field.into_node(),)
		})
		.collect()
}


pub fn child_marker_type(index: usize) -> TokenStream {
	field_ident("ChildMarker", index).to_token_stream()
}

fn child_fields_markers(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_marker_type(index);
			quote!(#field,)
		})
		.collect()
}


fn child_query(num_children: usize) -> TokenStream {
	(0..num_children)
		.fold(TokenStream::new(), |prev, index| {
			let child = child_type_name(index);
			quote!((&'static Prop<T,#child>, #prev))
		})
		.into_token_stream()
}
fn child_query_opt(num_children: usize) -> TokenStream {
	(0..num_children)
		.fold(TokenStream::new(), |prev, index| {
			let child = child_type_name(index);
			quote!((Option<&'static Prop<T,#child>>, #prev))
		})
		.into_token_stream()
}
fn child_query_mut(num_children: usize) -> TokenStream {
	(0..num_children)
		.fold(TokenStream::new(), |prev, index| {
			let child = child_type_name(index);
			quote!((&'static mut Prop<T,#child>, #prev))
		})
		.into_token_stream()
}
fn child_query_opt_mut(num_children: usize) -> TokenStream {
	(0..num_children)
		.fold(TokenStream::new(), |prev, index| {
			let child = child_type_name(index);
			quote!((Option<&'static mut Prop<T,#child>>, #prev))
		})
		.into_token_stream()
}
fn node_params(num_children: usize) -> TokenStream {
	(0..num_children)
		.fold(TokenStream::new(), |prev, index| {
			let value = field_ident("value", index);
			quote!((#value, #prev))
		})
		.into_token_stream()
}
fn node_params_mut(num_children: usize) -> TokenStream {
	(0..num_children)
		.fold(TokenStream::new(), |prev, index| {
			let value = field_ident("value", index);
			quote!((mut #value, #prev))
		})
		.into_token_stream()
}

fn build_child_states(ident: TokenStream, num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident = child_type_name(index);
			let value = field_ident("value", index);
			quote! {
				Box::new(#ident{
					entity: entity.clone(),
					index: #index,
					value: #value,
					marker: std::marker::PhantomData::<#child_ident>
				}),
			}
		})
		.collect()
}
fn child_tree_bundle_types(num_children: usize) -> TokenStream {
	(0..num_children)
		.fold(TokenStream::new(), |prev, index| {
			let ident = child_type_name(index);
			quote!((#ident::BundleRecursive<T>, #prev))
		})
		.into_token_stream()
}
fn child_tree_bundle_values(num_children: usize) -> TokenStream {
	(0..num_children)
		.fold(TokenStream::new(), |prev, index| {
			let ident = child_type_name(index);
			quote!((#ident::tree_bundle::<T>(value.clone()), #prev))
		})
		.into_token_stream()
}

// there is probably a better way to do this
// fn state_recast(num_children: usize) -> TokenStream {
// 	(0..num_children)
// 		.map(|index| {
// 			let value = field_ident("value", index);
// 			quote! {
// 				let #value = &#value.value;
// 				// let #value = &#value.into_inner();
// 				// let #node = if let Some(val) = #node{
// 				// 	Some(val.into_inner() as DerefNode<'_>)
// 				// }else{
// 				// 	None
// 				// };
// 			}
// 		})
// 		.collect()
// }
fn state_recast_opt(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let value = field_ident("value", index);
			quote! {
				let #value = if let Some(val) = #value{
					Some(val as &dyn std::ops::Deref<Target = T>)
				}else{
					None
				};
			}
		})
		.collect()
}

fn state_recast_mut(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let value = field_ident("value", index);
			quote! (let #value = #value.map_unchanged(|v|&mut v.value);)
		})
		.collect()
}
fn state_recast_opt_mut(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let value = field_ident("value", index);
			quote! {
				let #value = if let Some(val) = #value{
					Some(val.map_unchanged(|v|&mut v.value))
				}else{
					None
				};
			}
		})
		.collect()
}

fn match_get_children(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident = child_field_name(index);
			quote!(#index => &self.#child_ident,)
		})
		.collect()
}
fn get_children(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident = child_field_name(index);
			quote!(&self.#child_ident,)
		})
		.collect()
}
fn recursive_children(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident = child_type_name(index);
			quote!(#child_ident::get_recursive_inner::<T>(world,entity,#index,depth + 1),)
		})
		.collect()
}
fn get_children_owned(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident = child_field_name(index);
			quote!(Box::new(self.#child_ident),)
		})
		.collect()
}

fn match_get_children_owned(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident = child_field_name(index);
			quote!(#index => Box::new(self.#child_ident),)
		})
		.collect()
}
fn children_inferred_types(num_children: usize) -> TokenStream {
	(0..num_children).map(|_index| quote!(_,)).collect()
}
fn children_into_child(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_field = child_field_name(index);
			quote!(self.#child_field.into_child::<TreePathSegment<#index, NewPath>>(),)
		})
		.collect()
}
fn children_with_path(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let ty = child_type_name(index);
			quote!(#ty::WithPath::<TreePathSegment<#index, NewPath>>,)
		})
		.collect()
}
