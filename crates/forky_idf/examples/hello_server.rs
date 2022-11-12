use anyhow::Result;
use esp_idf_sys as _;
use forky_idf::*;

fn main() -> Result<()> {
	let wifi = wifi::Connection::new(secret::SSID, secret::PASSWORD)?;
	let _server = wifi.start_server()?;
	println!("howdy!");
	utility::sleep_forever();
}
