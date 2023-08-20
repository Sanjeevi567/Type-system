use actix_web::{web,App,HttpServer,get, Responder, HttpRequest,HttpResponse};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
           //It's because we are calling the method which returns different type
            .route("/", web::to(HttpResponse::Ok()))
      //.route("/", web::to(HttpResponse::Ok))
            //Closure must be async and the return type is anying that implements Responder.
            .route("/hello",web::to(|| {format!("This must be async");}))
      //.route("/hello",web::to(||async {format!("This must be async")}))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

//in cargo.toml under dependencies 
//actix-web = "4.3.1"
