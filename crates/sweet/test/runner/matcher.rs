use forky_core::log;
use sweet::*;

sweet! {
	test "to_be_bool" {
		//lmao
		let result = expect(true).to_be_true();
		expect(result.is_ok()).to_be(true)?;

		let result = expect(true).to_be_false();
		let result = expect(true).to_be(false);
		expect(result.is_ok()).to_be(false)?;
		expect(result.unwrap_err().message.as_str()).to_contain("this line")?;

	}

	test "to_be" {
		//lmao
		let result = expect(true).to_be(true);
		expect(result.is_ok()).to_be(true)?;

		let result = expect(true).to_be(false);
		expect(result.unwrap_err().message.as_str()).to_contain("this line")?;

	}

	test "to contain" {
		let result = expect("foo").to_contain("foo");
		expect(result.is_ok()).to_be_true()?;
		let result = expect("foo").to_contain("bar");
		expect(result.unwrap_err().message.as_str()).to_contain("this line")?;
	}
}