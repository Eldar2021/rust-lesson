use crate::assets_data::{ABOUT_US, HTML_FORM, INIT_HTML, NOT_FOUNT_404};
use hyper::{
    body::to_bytes,
    header::{HeaderName, HeaderValue},
    Body, Error, Request, Response, StatusCode,
};
use serde_json::{json, Value};

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    let response = match req.uri().path() {
        "/" => generate_response(INIT_HTML.to_string(), None, None),
        "/about" => generate_response(ABOUT_US.to_string(), None, None),
        "/api" => {
            let api_response = hadle_api();
            let json_str = serde_json::to_string(&api_response).unwrap();
            generate_response(json_str, Some("application/json"), None)
        }
        "/form" => match req.method() {
            &hyper::Method::POST => {
                let fallback_name = ":)".to_string();
                let body = to_bytes(req.into_body()).await?;
                let params = form_urlencoded::parse(&body)
                    .into_owned()
                    .collect::<Vec<(String, String)>>();
                let name = params
                    .iter()
                    .find(|(k, _)| k == "name")
                    .map(|(_, v)| v)
                    .unwrap_or(&fallback_name);

                let html = format!("<html><body><h1>Merhaba, {}!</h1></body></html>", name);

                generate_response(html.to_string(), Some("text/html"), None)
            }
            _ => generate_response(HTML_FORM.to_string(), None, None),
        },
        _ => generate_response(NOT_FOUNT_404.to_string(), None, Some(404)),
    };

    Ok(response)
}

fn generate_response(
    body: String,
    content_type: Option<&str>,
    status_code: Option<u16>,
) -> Response<Body> {
    let content_type = content_type.unwrap_or("text/html; charset=utf-8");
    let status_code = status_code.unwrap_or(200);

    Response::builder()
        .status(StatusCode::from_u16(status_code).unwrap())
        .header(
            HeaderName::from_static("content-type"),
            HeaderValue::from_str(content_type).unwrap(),
        )
        .body(Body::from(body))
        .unwrap()
}

fn hadle_api() -> Value {
    json!({
        "name": "Rust Web Server",
        "version": "1.0",
        "author": "user1"
    })
}
