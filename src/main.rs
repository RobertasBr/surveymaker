#[macro_use]
extern crate rocket;

use mongodb::{bson::doc, Client, Collection};
use rocket::form::Form;
use rocket::serde::{Deserialize, Serialize};
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::State;
use std::env;
use dotenv::dotenv;

// Define a struct to hold the form data
#[derive(FromForm, Serialize, Deserialize, Debug)]
struct QuestionForm {
    question_text: String,
    answer: Option<String>,
}

// Creates the HTML form page
#[get("/")]
fn form_page() -> RawHtml<&'static str> {
    RawHtml(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Survey Form</title>
    </head>
    <body>
        <h1>Survey Form</h1>
        <form action="/submit" method="post">
            <label for="question_text">Question:</label><br>
            <input type="text" id="question_text" name="question_text" required><br><br>
            <label for="answer">Answer:</label><br>
            <input type="text" id="answer" name="answer"><br><br>
            <input type="submit" value="Submit">
        </form>
    </body>
    </html>
    "#)
}

// Handles submit logic
#[post("/submit", data = "<form>")]
async fn submit_question(
    form: Form<QuestionForm>,
    db: &State<Collection<QuestionForm>>,
) -> Redirect {
    // Adds to database
    let new_question = QuestionForm {
        question_text: form.question_text.clone(),
        answer: form.answer.clone(),
    };

    db.insert_one(new_question, None)
        .await
        .expect("Failed to insert question into MongoDB");

    Redirect::to(uri!(form_page)) // Reloads site
}

// Starts the Rocket server
#[launch]
async fn rocket() -> _ {
    dotenv().ok(); // Loads .env

    // Get connection string from .env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Starts MongoDB Client
    let client = Client::with_uri_str(&database_url)
        .await
        .expect("Failed to connect to MongoDB");
    let db = client.database("survey_db");
    let collection = db.collection::<QuestionForm>("questions");

    // Launch the Rocket server with MongoDB collection
    rocket::build()
        .manage(collection)
        .mount("/", routes![form_page, submit_question])
}
