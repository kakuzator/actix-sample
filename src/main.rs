use actix_web::{get , post , web , App , HttpResponse , HttpServer};
use rusqlite::params;

#[get("/")]
async fn index() -> Result<HttpResponse , actix_web::Error>
{
    let response_body = "HellorWorld";
    Ok(HttpResponse::Ok().body(response_body))
}

#[post("/add")]
async fn add( ){
    /* let con = db.get()?;
    conn.execute("INSETR INTO todo (text) VALUES (?)" , $[&params.text])?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION , "/")
        .finish()) */
    let data = params.text;
}

#[actix_rt::main]
async fn main() -> Result< (), actix_web::Error> 
{
    HttpServer::new( move || App::new().service(index) )
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
        Ok(())
}
