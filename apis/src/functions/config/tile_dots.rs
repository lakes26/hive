use crate::common::config_options::TileDots;
use leptos::*;

#[server]
pub async fn change_tile_dots(tile_dots: TileDots) -> Result<TileDots, ServerFnError> {
    use actix_web::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
    use chrono::Duration;
    use leptos_actix::{ResponseOptions, ResponseParts};

    let response = expect_context::<ResponseOptions>();
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    let max_age = Duration::days(365)
        .to_std()
        .expect("Not a negative duration")
        .as_secs();

    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!(
            "tile_dots={tile_dots}; Max-Age={}; Path=/",
            max_age
        ))
        .expect("to create header value"),
    );
    response_parts.headers = headers;

    response.overwrite(response_parts);
    Ok(tile_dots)
}
