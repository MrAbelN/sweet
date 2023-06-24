use wasm_bindgen::JsValue;
use web_sys::{window, Url, UrlSearchParams};


pub struct SearchParams;

impl SearchParams {
	pub fn get(key: &str) -> Option<String> {
		let search = window().unwrap().location().search().unwrap();
		let params = UrlSearchParams::new_with_str(search.as_str()).unwrap();
		params.get(key)
	}
	pub fn set(key: &str, value: &str) {
		let loc = window().unwrap().location();
		let params = loc.search().unwrap();
		let params = UrlSearchParams::new_with_str(params.as_str()).unwrap();
		params.set(key, value);
		let params = params.to_string().as_string().unwrap();
		loc.set_search(params.as_str()).unwrap();
	}
	pub fn get_flag(key: &str) -> bool { Self::get(key).is_some() }
	pub fn set_flag(key: &str, val: bool) {
		if val {
			Self::set(key, "1");
		} else {
			Self::remove(key);
		}
	}

	pub fn remove(key: &str) {
		let search = window().unwrap().location().search().unwrap();
		let params = UrlSearchParams::new_with_str(search.as_str()).unwrap();
		params.delete(key);
	}
}

pub fn path_name() -> String {
	window().unwrap().location().pathname().unwrap()
}


pub fn navigate(path: &str) {
	window().unwrap().location().set_href(path).unwrap();
}
pub fn navigate_replace(path: &str) {
	window().unwrap().location().replace(path).unwrap();
}


pub struct History;

impl History {
	pub fn push(path: &str) {
		window()
			.unwrap()
			.history()
			.unwrap()
			.push_state_with_url(&JsValue::UNDEFINED, "", Some(path))
			.unwrap();
	}
	pub fn push_preserve_params(path: &str) {
		let location = window().unwrap().location();
		let href = location.href().unwrap();
		let url = Url::new(&href).unwrap();
		url.set_pathname(path);
		Self::push(url.href().as_str());
	}
	pub fn replace(path: &str) {
		window()
			.unwrap()
			.history()
			.unwrap()
			.replace_state_with_url(&JsValue::UNDEFINED, path, Some(path))
			.unwrap();
	}
}
