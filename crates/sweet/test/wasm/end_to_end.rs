// use forky_core::*;
use sweet::*;

sweet! {
	test e2e "same origin" {
		let page = visit("http://localhost:7777/__ping__").await?;
		expect(page)
			.poll(|p|p.to_contain_text("__ping__")).await?;
	}
	test e2e "docs origin" {
		let page = visit("http://localhost:3000").await?;
		expect(page)
			.poll(|p|p.to_contain_text("Very early stage warning")).await?;
	}

	test skip e2e "example origin" {
		let page = visit("http://example.com").await?;
		expect(page)
			.poll(|p|p.to_contain_text("Example")).await?;
	}
}