use bevy::prelude::*;
use extend::ext;
use forky_core::*;
#[derive(Deref, DerefMut)]
pub struct AppRes(pub RcCell<App>);

impl AppRes {
	pub fn new() -> RcCell<App> { Self::init(App::new()) }

	pub fn init(app: App) -> RcCell<App> {
		let app = rccell(app);
		let app2 = app.clone();
		app.borrow_mut().insert_non_send_resource(AppRes(app2));
		app
	}
}

#[ext]
pub impl RcCell<App> {
	#[cfg(target_arch = "wasm32")]
	fn run_on_animation_frame(self) -> forky_web::AnimationFrame {
		forky_web::AnimationFrame::new(move || {
			self.borrow_mut().update();
		})
	}
}