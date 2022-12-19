use crate::util::{RequestHelper, TestApp};
use http::{Method, StatusCode};

#[test]
fn head_method_works() {
    let (_, anon) = TestApp::init().empty();

    let req = anon.request_builder(Method::HEAD, "/api/v1/summary");
    let res = anon.run::<()>(req);
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.into_text(), "");
}

#[test]
fn head_method_works_for_404() {
    let (_, anon) = TestApp::init().empty();

    let req = anon.request_builder(Method::HEAD, "/unknown");
    let res = anon.run::<()>(req);
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    // TODO this should not return a response body
    // assert_eq!(res.into_text(), "");
}
