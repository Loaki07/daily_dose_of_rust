use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("json_parsing_rust");

    let url = "https://jsonplaceholder.typicode.com/todos";

    let todos: Vec<Todo> = reqwest::Client::new()
        .get(format!("{}?userId=1", url))
        .send()
        .await?
        .json()
        .await?;

    // println!("{:#?}", todos);

    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: "json parsing in rust".to_owned(),
        completed: false,
    };

    // to use serde_json to submit a body without creating a struct
    // let new_todo = serde_json::json!({
    //     "userId": 1,
    //     "id": "",
    //     "title": "json parsing in rust".to_owned(),
    //     "completed": false,
    // });

    let new_todo: Todo = reqwest::Client::new()
        .post(url)
        .json(&new_todo)
        .send()
        .await?
        .json()
        .await?;


    println!("{:#?}", new_todo);

    Ok(())
}
