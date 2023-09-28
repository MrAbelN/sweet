use fantoccini::ClientBuilder;
use forky_fs::retry_async;
use std::process::Command;
use sweet::*;

// example from fantoccini README https://crates.io/crates/fantoccini

sweet! {
	it skip non_send "works" {
		let mut chromedriver = Command::new("chromedriver")
			.args(["--silent", "--port=9515"])
			.spawn()?;

		let client = retry_async(
			async || {
				let cap = serde_json::from_str(
					r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
				).unwrap();
				ClientBuilder::native().capabilities(cap).connect("http://localhost:9515").await

			},
			std::time::Duration::from_secs(1),
		)
		.await?;

		client.goto("https://example.com").await?;
		let url = client.current_url().await?;
		expect(url.as_ref()).to_contain("example.com")?;

		client.close().await?;
		chromedriver.kill()?;
	}
}
