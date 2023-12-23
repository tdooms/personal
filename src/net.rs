use crate::resume::Cv;
use gloo::net::http::Request;

pub async fn get_resume() -> Cv {
    Request::get("/static/resume/data.json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}