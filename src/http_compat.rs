use http as http_new;
use warp::http as http_old;

pub fn status_to_warp(status: http_new::StatusCode) -> http_old::StatusCode {
    let code = status.as_u16();
    http_old::StatusCode::from_u16(code)
        .expect("Status code should be valid")
}

pub fn header_to_warp(name: http_new::header::HeaderName) -> http_old::header::HeaderName {
    // Convert to static str and then to warp's HeaderName
    let name_str = name.as_str();
    http_old::header::HeaderName::from_bytes(name_str.as_bytes())
        .expect("Header name should be valid")
}
