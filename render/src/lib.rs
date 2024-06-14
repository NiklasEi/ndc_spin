use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use templating::render_template;
use templating_files::wasi::WasiTemplateFiles;
use typst::foundations::Smart;

/// A simple Spin HTTP component.
#[http_component]
fn handle_render(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let input = "{
  \"test\": \"from sample data\",
  \"items\": [
    {
      \"name\": \"Frank\",
      \"one\": \"first\",
      \"two\": \"second\",
      \"three\": \"third\"
    },
    {
      \"name\": \"John\",
      \"one\": \"first_john\",
      \"two\": \"second_john\",
      \"three\": \"third_john\"
    }
  ]
}";
    let document = render_template(
        "test".to_string(),
        input.to_string(),
        WasiTemplateFiles::new("templates", "test"),
    )
    .expect("Failed to render");
    let pdf_buffer = typst_pdf::pdf(&document, Smart::Auto, None);
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/pdf")
        .body(pdf_buffer)
        .build())
}
