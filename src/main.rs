extern crate dotenv;

type DbPool = diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>;
use actix_multipart::Multipart;
use actix_svelte_template::{create_todo, delete_todo, update_todo};
use actix_web::{
    get, post,
    web::{self},
    App, Error, HttpResponse, HttpServer, Responder,
};
use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};
use futures_util::TryStreamExt as _;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TodoFields {
    text: Option<String>,
    id: Option<String>,
    done: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct TodoQuery {
    _method: Option<String>,
}

async fn parse_multipart_form(mut form: Multipart) -> Result<TodoFields, Error> {
    let mut fields = TodoFields {
        text: None,
        id: None,
        done: None,
    };

    while let Some(mut field) = form.try_next().await? {
        let mut buffer = Vec::new();
        while let Some(chunk) = field.try_next().await? {
            buffer.extend_from_slice(&chunk);
        }
        let name = field.name();
        let string_value = String::from_utf8(buffer).unwrap();
        match name {
            "id" => fields.id = Some(string_value),
            "text" => fields.text = Some(string_value),
            "done" => fields.done = Some(string_value == "true"),
            _ => (),
        }
    }

    Ok(fields)
}

#[post("/api/todos")]
async fn create_update_or_delete_todo(
    form: Multipart,
    query: web::Query<TodoQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let fields = parse_multipart_form(form).await?;
    match &query._method {
        Some(method) => match method.as_str() {
            "PATCH" => {
                update_todo(
                    &mut conn,
                    &fields.id.expect("ID required"),
                    fields.text,
                    fields.done,
                );
            }
            "DELETE" => {
                delete_todo(&mut conn, &fields.id.expect("ID required"));
            }
            _ => panic!("Unsupported method {}", method),
        },
        None => {
            let mut conn = pool.get().expect("couldn't get db connection from pool");
            let todo = create_todo(
                &mut conn,
                &fields.text.expect("text is required"),
                fields.done.unwrap_or(false),
            );
            return Ok(HttpResponse::Ok().json(todo));
        }
    }
    Ok(HttpResponse::Ok().finish())
}

#[get("/api/todos/{id}")]
async fn get_todo(path: web::Path<String>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let id = path.into_inner();
    web::Json(actix_svelte_template::get_todo(&mut conn, &id))
}

#[get("/api/todos")]
async fn get_todos(query: web::Query<TodoFields>, pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let res = query.0;
    web::Json(actix_svelte_template::get_todos(
        &mut conn, res.id, res.text, res.done,
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("PORT must be a 16 bit int");
    let path = std::env::var("STATIC_FILE_PATH").expect("STATIC_FILE_PATH must be set");
    let static_files = String::from(path.strip_suffix("/").unwrap_or(&path));
    HttpServer::new(move || {
        let app = App::new()
            .service(create_update_or_delete_todo)
            .service(get_todo)
            .service(get_todos)
            .app_data(web::Data::new(pool.clone()));

        if cfg!(not(debug_assertions)) {
            return app.service(
                actix_files::Files::new("/", static_files.clone())
                    .index_file("index.html")
                    .default_handler(
                        actix_files::NamedFile::open(
                            vec![static_files.clone(), "index.html".to_string()].join("/"),
                        )
                        .expect("index file should exist"),
                    ),
            );
        }
        app
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
