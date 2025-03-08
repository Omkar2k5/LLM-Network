pub fn set_cookie_and_redirect(response: HttpResponse, username: String, hash: String, location: &str) -> HttpResponse {
    response.cookie(cookie::Cookie::build("username", username)
        .finish())
        .cookie(cookie::Cookie::build("hash", hash)
        .finish())
        .append_header((header::LOCATION, location))
        .finish()
}