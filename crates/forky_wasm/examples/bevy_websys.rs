// #![cfg(web_sys_unstable_apis)]
use bevy::{prelude::*, winit::*};
use forky_play::{app::SimplePlugin, *};
use forky_wasm::*;
use std::{thread, time};
use wasm_bindgen::prelude::*;
use web_sys::*;


fn main() {
	core::set_panic_hook();
	let mut app = App::new();
	app.add_plugins(DefaultPlugins)
		// .add_plugins(DefaultPlugins.build().disable::<WinitPlugin>())
		.add_plugin(SimplePlugin)
		.run();
	// .update();

	let update = Closure::<dyn FnMut()>::new(move || {
		// log!("update!");
		app.update();
	});

	web_sys::window()
		.unwrap()
		.set_interval_with_callback_and_timeout_and_arguments_0(
			update.as_ref().unchecked_ref(),
			1,
		);
	update.forget(); //terrible, memory leak
}
