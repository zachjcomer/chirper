use std::{fs::{OpenOptions, File}, io::{Write, BufReader, BufRead}};
use rocket::{serde::{json::Json}, http::Status, response::{Responder, self}, Request};
use rocket_db_pools::{Database, Connection};

use rust_rest::note::Note;
use rust_rest::note::NewNote;
use rust_rest::note::NoteId;

#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("notes")]
struct NotesDatabase(sqlx::PgPool);

struct DatabaseError(rocket_db_pools::sqlx::Error);

impl<'r> Responder<'r, 'r> for DatabaseError {
    fn respond_to(self, _request: &Request) -> response::Result<'r> {
        Err(Status::InternalServerError)
    }
}

impl From<rocket_db_pools::sqlx::Error> for DatabaseError {
    fn from (error: rocket_db_pools::sqlx::Error) -> Self {
        DatabaseError(error)
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/* fn open_note(path: &str) -> File {
    OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path)
        .expect("unable to access file")
}

fn open_temp_note(path: &str) -> File {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .expect("unable to access new_notes.txt")
} */

#[post("/createnote", data = "<note>")]
async fn create_note(note: Json<NewNote<'_>>, mut db: Connection<NotesDatabase>) -> Result<Json<Note>, DatabaseError> {
    let new_note = sqlx::query_as::<_, Note>("INSERT INTO notes (content) VALUES ($1) RETURNING *")
        .bind(note.content)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(new_note))
}

#[get("/readnotes")]
async fn read_notes(mut db: Connection<NotesDatabase>) -> Result<Json<Vec<Note>>, DatabaseError> {
    let notes = sqlx::query_as::<_, Note>("SELECT * FROM notes")
        .fetch_all(&mut *db)
        .await?;

    Ok(Json(notes))
}

#[put("/updatenote", data = "<note_update>")]
async fn update_note(note_update: Json<Note>, mut db: Connection<NotesDatabase>) -> Result<Json<Note>, DatabaseError> {
    let updated_note = sqlx::query_as::<_, Note>("UPDATE notes SET content = $1 WHERE id = $2 RETURNING *")
        .bind(&note_update.content)
        .bind(note_update.id)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(updated_note))
}

#[delete("/deletenote", data = "<note_id>")]
async fn delete_note(note_id: Json<NoteId>, mut db: Connection<NotesDatabase>) -> Result<Json<Note>, DatabaseError> {
    let deleted_note = sqlx::query_as::<_, Note>("DELETE FROM notes WHERE id = $1 RETURNING *")
        .bind(note_id.id)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(deleted_note))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(NotesDatabase::init())
    .mount("/", routes![index, create_note, read_notes, update_note, delete_note])
}
