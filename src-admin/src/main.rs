use actix_web::{get, guard, web, App, HttpResponse, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "127.0.0.1:8080"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .service(
                web::scope("/index")
                    .guard(guard::fn_guard(|ctx| {
                        ctx.head().method.to_string() == "POST"
                    }))
                    .route(
                        "",
                        web::to(|| async { HttpResponse::Ok().body("ggggggggggggggggg") }),
                    )
                    .route(
                        "test",
                        web::to(|| async { HttpResponse::Ok().body("tttttttttttttttttttttttttt") }),
                    ),
            )
            .route("/", web::to(HttpResponse::Ok))
            .route(
                "/index",
                web::to(|| async { HttpResponse::Ok().body("hhhhhhhhhhhhhhhhhhhhh") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
