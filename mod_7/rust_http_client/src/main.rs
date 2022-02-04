use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://swapi.dev/api/planets/1/";
    // let res = reqwest::get(url).await?;
    let res = reqwest::Client::new().get(url).send().await?;

    // eprintln!("Response: {:?} {}", res.version(), res.status());
    // // eprintln!("Headers: {:#?}\n", res.headers());

    let body = res.text().await?;

    println!("{:#?}", body);


     let req = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&serde_json::json!({
            "title": "Reqwest.rs",
            "body": "https://docs.rs/reqwest",
            "userId": 1
        }))
        .send()
        .await?;

    println!("{:#?}", req.status());

    let res: serde_json::Value = req
        .json()
        .await?;

    println!("{:#?}", res);

    Ok(())
}
