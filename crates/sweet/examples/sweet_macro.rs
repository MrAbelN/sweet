#![feature(imported_main)]
pub use sweet::*;

async fn foo() {}

sweet! {
	it{}
	it{foo().await;}
	it nonSend{foo().await;}
}
