use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

use crate::{web::AUTH_TOKEN, Error, Result};

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}
