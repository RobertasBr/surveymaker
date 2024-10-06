#[macro_use] extern crate rocket;

use prisma_client_rust::PrismaClient;
use rocket::serde::{json::Json, Deserialize};
use rocket::State;
use std::sync::Arc;

// JSON structure for creating requests
#[derive(Debug, Deserialize)]
struct NewQuestion {
    question_text: String,
}

// JSON structure for updated answer field for the question
#[derive(Debug, Deserialize)]
struct Answer {
    answer: String,
}