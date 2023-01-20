use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct NewNote<'r> {
    pub content: &'r str,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Note{
    pub id: i64,
    pub content: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct NoteId {
    pub id: i64,
}