use super::*;
use crate::DocumentExt;
use leptos::*;
use web_sys::Document;

pub fn mount<F, N>(f: F)
where
	F: FnOnce(Scope) -> N + 'static,
	N: IntoView,
{
	set_panic_hook();
	leptos::mount_to_body(f)
}


pub fn mount_with_unmount<F, N>(f: F) -> Mount
where
	F: FnOnce(Scope) -> N + 'static,
	N: IntoView,
{
	mount(f);
	Mount
}

pub struct Mount;

impl Drop for Mount {
	fn drop(&mut self) { Document::x_clear() }
}