use crate::field_ui_option;
use crate::utils::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DataStruct;
use syn::Field;
use syn::Result;




pub fn parse_struct(input: DataStruct) -> Result<TokenStream> {
	let fields = input
		.fields
		.iter()
		.map(parse_struct_field)
		// .collect::<Vec<_>>()
		.collect::<Result<Vec<_>>>()?
		.collect_comma_punct();

	Ok(quote! {
	GroupField::new(reflect.display_name.clone(), vec![
		#fields
		]).into()
	})
}

fn parse_struct_field(field: &Field) -> Result<TokenStream> {
	let ident = field.ident.as_ref().expect("field must have an ident");
	let ident_str = ident.to_string();

	let reflect = quote! {
		FieldReflect::new(
			#ident_str.to_string(),
			{
				let get_cb = reflect.clone_get_cb();
				move || get_cb().#ident.clone()
			},
			{
				let get_cb = reflect.clone_get_cb();
				let set_cb = reflect.clone_set_cb();
				move |val| {
					let mut parent = get_cb();
					parent.#ident = val;
					set_cb(parent);
				}
			},
		)
	};

	Ok(field_ui_option(field, &reflect)?)
}
