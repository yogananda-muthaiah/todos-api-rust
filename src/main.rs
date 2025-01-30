use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Todo {
    #[serde(rename = "userId")] // Map JSON's `userId` to Rust's `user_id`
    user_id: u32,
    id: u32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // URL of the JSONPlaceholder API endpoint for todos
    let url = "https://jsonplaceholder.typicode.com/todos/1";

    // Send a GET request to the API
    let response = reqwest::get(url).await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the JSON response into a vector of Todo structs
        let todos: Vec<Todo> = response.json().await?;

        // Print each todo item
        for todo in todos {
            println!("User ID: {}", todo.user_id);
            println!("ID: {}", todo.id);
            println!("Title: {}", todo.title);
            println!("Completed: {}", todo.completed);
            println!("-----------------------------");
        }
    } else {
        eprintln!("Failed to fetch data: {}", response.status());
    }

    Ok(())
}
