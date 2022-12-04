use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

#[derive(Default)]
pub struct ContentSecurityPolicy {}

/// Applies the HTTP Content-Security-Policy (CSP) response header to help
/// guard against cross-site scripting attacks (XSS).
impl Fairing for ContentSecurityPolicy {
    fn info(&self) -> Info {
        Info {
            name: "Content Security Policy",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _: &Request, response: &mut Response) {
        let csp = Header::new(
            "Content-Security-Policy",
            "\
            default-src 'none';\
            base-uri 'none';\
            manifest-src 'self';\
            script-src 'self';\
            style-src 'self';\
            img-src 'self' data: blob:;\
            font-src 'self' data:;\
            connect-src 'self';\
            media-src 'self';\
            frame-ancestors 'self';\
            worker-src 'self' blob:",
        );
        response.set_header(csp);
    }
}
