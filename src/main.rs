use rocket::{serde::{json::Json}, http::Status, response::{Responder, self}, Request};
use rocket_db_pools::{Database, Connection};

use rust_rest::posts::Post;
use rust_rest::posts::PostUpdate;
use rust_rest::posts::NewPost;
use rust_rest::posts::PostId;

use rust_rest::posts::Reply;
use rust_rest::posts::ReplyUpdate;
use rust_rest::posts::NewReply;
use rust_rest::posts::ReplyId;

#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("posts")]
struct PostsDatabase(sqlx::PgPool);

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

// POST CRUD
#[post("/newpost", data = "<new_post>")]
async fn create_post(new_post: Json<NewPost<'_>>, mut db: Connection<PostsDatabase>) -> Result<Json<Post>, DatabaseError> {
    let new_posts = sqlx::query_as::<_, Post>("INSERT INTO posts (content) VALUES ($1) RETURNING *")
        .bind(new_post.content)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(new_posts))
}

#[get("/getposts")]
async fn get_all_posts(mut db: Connection<PostsDatabase>) -> Result<Json<Vec<Post>>, DatabaseError> {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&mut *db)
        .await?;

    Ok(Json(posts))
}

#[put("/editpost", data = "<update_post>")]
async fn edit_post(update_post: Json<PostUpdate>, mut db: Connection<PostsDatabase>) -> Result<Json<Post>, DatabaseError> {
    let updated_posts = sqlx::query_as::<_, Post>("UPDATE posts SET content = $1 WHERE id = $2 RETURNING *")
        .bind(&update_post.content)
        .bind(update_post.id)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(updated_posts))
}

#[delete("/deletepost", data = "<post_id>")]
async fn delete_post(post_id: Json<PostId>, mut db: Connection<PostsDatabase>) -> Result<Json<Post>, DatabaseError> {
    let deleted_posts = sqlx::query_as::<_, Post>("DELETE FROM posts WHERE id = $1 RETURNING *")
        .bind(post_id.id)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(deleted_posts))
}

// REPLY CRUD
#[post("/<post_id>/newreply", data = "<new_reply>")]
async fn create_reply(new_reply: Json<NewReply<'_>>, post_id: i64 , mut db: Connection<PostsDatabase>) -> Result<Json<Reply>, DatabaseError> {
    let new_reply = sqlx::query_as::<_, Reply>("INSERT INTO replies (postid, content) VALUES ($1, $2) RETURNING *") //  WHERE postid = $1
        .bind(post_id)
        .bind(new_reply.content)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(new_reply))
}

#[get("/<post_id>/getreplies")]
async fn get_replies(post_id: i64 , mut db: Connection<PostsDatabase>) -> Result<Json<Vec<Reply>>, DatabaseError> {
    let replies = sqlx::query_as::<_, Reply>("SELECT * FROM replies WHERE postid = $1")
        .bind(post_id)
        .fetch_all(&mut *db)
        .await?;

    Ok(Json(replies))
}

#[put("/<post_id>/editreply", data = "<reply>")]
async fn edit_reply(reply: Json<ReplyUpdate>, post_id: i64 , mut db: Connection<PostsDatabase>) -> Result<Json<Reply>, DatabaseError> {
    let updated_replies = sqlx::query_as::<_, Reply>("UPDATE replies SET content = $1 WHERE postid = $2 AND id = $3 RETURNING *") //  WHERE postid = $2 AND id = $3
        .bind(&reply.content)
        .bind(post_id)
        .bind(reply.id)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(updated_replies))
}

#[delete("/<post_id>/deletereply", data = "<reply_id>")]
async fn delete_reply(reply_id: Json<ReplyId>, post_id: i64 , mut db: Connection<PostsDatabase>) -> Result<Json<Reply>, DatabaseError> {
    let deleted_replies = sqlx::query_as::<_, Reply>("DELETE FROM replies WHERE postid = $1 AND id = $2 RETURNING *")
        .bind(post_id)
        .bind(reply_id.id)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(deleted_replies))
}

// LIKES
#[patch("/<post_id>/likepost")]
async fn like_post(post_id: i64, mut db: Connection<PostsDatabase>) -> Result<Json<Post>, DatabaseError> {
    let liked_post = sqlx::query_as::<_, Post>("UPDATE posts SET likes = likes + 1 WHERE id = $1 RETURNING *")
        .bind(post_id)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(liked_post))
}

#[patch("/<post_id>/dislikepost")]
async fn dislike_post(post_id: i64, mut db: Connection<PostsDatabase>) -> Result<Json<Post>, DatabaseError> {
    let liked_post = sqlx::query_as::<_, Post>("UPDATE posts SET likes = likes - 1 WHERE id = $1 RETURNING *")
        .bind(post_id)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(liked_post))
}

#[patch("/<post_id>/likereply", data = "<reply_id>")]
async fn like_reply(reply_id: Json<ReplyId>, post_id: i64, mut db: Connection<PostsDatabase>) -> Result<Json<Reply>, DatabaseError> {
    let liked_reply = sqlx::query_as::<_, Reply>("UPDATE replies SET likes = likes + 1 WHERE postid = $1 AND id = $2 RETURNING *")
        .bind(post_id)
        .bind(reply_id.id)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(liked_reply))
}

#[patch("/<post_id>/dislikereply", data = "<reply_id>")]
async fn dislike_reply(reply_id: Json<ReplyId>, post_id: i64, mut db: Connection<PostsDatabase>) -> Result<Json<Reply>, DatabaseError> {
    let liked_reply = sqlx::query_as::<_, Reply>("UPDATE replies SET likes = likes - 1 WHERE postid = $1 AND id = $2 RETURNING *")
        .bind(post_id)
        .bind(reply_id.id)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(liked_reply))
}

#[get("/latest")]
async fn get_latest(mut db: Connection<PostsDatabase>) -> Result<Json<Vec<Post>>, DatabaseError> {
    let latest_posts = sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY post_date DESC LIMIT 50")
    .fetch_all(&mut *db)
    .await?;

    Ok(Json(latest_posts))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(PostsDatabase::init())
    .mount("/", routes![index, create_post, get_all_posts, edit_post, delete_post])
    .mount("/", routes![create_reply, get_replies, edit_reply, delete_reply])
    .mount("/", routes![like_post, dislike_post, like_reply, dislike_reply])
    .mount("/", routes![get_latest])
}
