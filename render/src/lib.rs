use anyhow::Context;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use templating::render_template;
use templating_files::wasi::WasiTemplateFiles;
use typst::foundations::Smart;

/// A simple Spin HTTP component.
#[http_component]
fn handle_render(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    let input = serde_json::from_reader::<_, serde_json::Value>(req.body())
        .context("Failed to parse input as json")?;
    let template = req
        .header("spin-path-match-id")
        .context("Failed to get template name from request path")?
        .as_str()
        .context("Failed to convert template id to str")?;

    let document = render_template(
        template.to_string(),
        input.to_string(),
        WasiTemplateFiles::new("templates", template),
    )
    .context("Failed to render template")?;
    let pdf_buffer = typst_pdf::pdf(&document, Smart::Auto, None);
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/pdf")
        .body(pdf_buffer)
        .build())
}
