use actix_web::{web, HttpResponse};
use actix_web::client::Client;
use actix_web::web::Path;
use futures::{Future};
use std::str;

#[derive(Deserialize)]
pub struct Number {
    number: i32,
}

#[derive(Deserialize)]
pub struct Echo {
    message: String,
}

pub struct Parameters {
    pub client: Client,
    pub c_endpoint: String,
}

// Handle index route
pub fn index() -> &'static str {
    "Hello world!\r\n"
}  

// echo handler
pub fn echo_handler(parameters: web::Data<Parameters>, msg: Path<Echo>) -> 
    impl Future<Item = HttpResponse, Error = ()> {

    let mut endpoint = parameters.c_endpoint.to_string();
    endpoint.push_str(&"echo/".to_string());
    endpoint.push_str(&msg.message);
    debug!("Calling endpoint: {}", endpoint);

    parameters.client.get(endpoint)   // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.finish().unwrap()
            .send()                               // <- Send http request
            .map_err(|_| ())
            //.map_err(Error::from)
            .and_then(|mut response| {
                    response.body().and_then( |body| {
                        debug!("Received from endpoint: {}", str::from_utf8(&body).unwrap());
                        Ok(HttpResponse::Ok().body(body))
                    }).map_err(|_| ())
                }).map_err(|_| ())
} 

/// Calls to iter factorial service
pub fn factorial_iter_handler(parameters: web::Data<Parameters>, number: Path<Number>) -> 
    impl Future<Item = HttpResponse, Error = ()> {
    
    let mut endpoint = parameters.c_endpoint.to_string();
    endpoint.push_str(&"factorialIterative/".to_string());
    endpoint.push_str(&number.number.to_string());
    debug!("Calling endpoint: {}", endpoint);

    parameters.client.get(endpoint)   // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.finish().unwrap()
            .send()                               // <- Send http request
            .map_err(|_| ())
            //.map_err(Error::from)
            .and_then(|mut response| {
                    response.body().and_then( |body| {
                        debug!("Received from endpoint: {}", str::from_utf8(&body).unwrap());
                        Ok(HttpResponse::Ok().body(body))
                    }).map_err(|_| ())
                }).map_err(|_| ())
}

/// Calls to recursi factorial service
pub fn factorial_recur_handler(parameters: web::Data<Parameters>, number: Path<Number>) -> 
    impl Future<Item = HttpResponse, Error = ()> {
    
    let mut endpoint = parameters.c_endpoint.to_string();
    endpoint.push_str(&"factorialRecursive/".to_string());
    endpoint.push_str(&number.number.to_string());
    debug!("Calling endpoint: {}", endpoint);

    parameters.client.get(endpoint)   // <- Create request builder
            .header("User-Agent", "Actix-web")
            //.finish().unwrap()
            .send()                               // <- Send http request
            .map_err(|_| ())
            //.map_err(Error::from)
            .and_then(|mut response| {
                    response.body().and_then( |body| {
                        debug!("Received from endpoint: {}", str::from_utf8(&body).unwrap());
                        Ok(HttpResponse::Ok().body(body))
                    }).map_err(|_| ())
                }).map_err(|_| ())
}