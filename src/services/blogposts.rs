use gloo_net::{Error, http::*};

pub async fn get_blogposts() -> Result<String, Error> {
    let response = Request::get("127.0.0.1:3000/post").send().await.unwrap();

    response.text().await
}
