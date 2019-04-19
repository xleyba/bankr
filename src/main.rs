#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

use actix_web::{web, App, HttpServer};
use actix_web::client::Client;

use colored::*;
use log::{debug};
use log::Level;
use serde::Deserialize;

mod handlers;
use crate::handlers::index;
use crate::handlers::echo_handler;
use crate::handlers::factorial_iter_handler;
use crate::handlers::factorial_recur_handler;
use crate::handlers::Parameters;

// Defines the default port
const DEFAULT_PORT: u16          = 9296;

// Defines the workers used by server
const DEFAULT_WORKERS: usize     = 2;

// Defines the url for the called service (calculator)
const DEFAULT_CLIENT_URL: &str   = "http://127.0.0.1:9596"; 

#[derive(Deserialize, Debug)]
struct ConfigPort {
    port: u16,
}

#[derive(Deserialize, Debug)]
struct ConfigWorkers {
    workers: usize,
}

#[derive(Deserialize, Debug)]
struct ConfigCalledService {
    client_url: String,
}


// Displays intro banner
fn intro() {
    println!("{}", "===========================================================".yellow().bold());
    println!("{}", "                    Bank v 0.2.0".yellow().bold());
    println!("{}", "===========================================================".yellow().bold());
    println!("{}", "   Please use env variables for configuration:".yellow().bold());
    println!("{}", "       BANK_PORT=port number".yellow().bold());
    println!("{}", "       BANK_WORKERS=workers for server".yellow().bold());
    println!("{}", "       BANK_CLIENT_URL=url of called service".yellow().bold());
    println!("{}", "-----------------------------------------------------------");
    println!("Starting configuration......\n");
}

fn config_port() -> u16 {
    match envy::prefixed("BANK_").from_env::<ConfigPort>() {
      Ok(config) => {
          info!("Port set to: {}", config.port);
          config.port
      },
      Err(error) => {
          error!("Error with env var PORT {}", error);
          info!("Port set to {} - default value", DEFAULT_PORT);
          DEFAULT_PORT
      }
   }
}

fn config_workers() -> usize {
    match envy::prefixed("BANK_").from_env::<ConfigWorkers>() {
      Ok(config) => {
          info!("Workers set to: {}", config.workers);
          config.workers
      },
      Err(error) => {
          error!("Error with env var WORKERS {}", error);
          info!("Workers set to {} - default value", DEFAULT_WORKERS);
          DEFAULT_WORKERS
      }
   }
}

fn config_called_service() -> String {
    match envy::prefixed("BANK_").from_env::<ConfigCalledService>() {
      Ok(config) => {
          info!("Client URL set to: {}", config.client_url.to_string());
          config.client_url
      },
      Err(error) => {
          error!("Error with env var CLIENT_URL {}", error);
          info!("Client URL set to {} - default value", DEFAULT_CLIENT_URL.to_string());
          DEFAULT_CLIENT_URL.to_string()
      }
   }
}




fn main() -> std::io::Result<()>  {
    env_logger::init();
    /*Builder::new()
        .parse(&env::var("BANK_LOG").unwrap_or_default())
        .init();*/

    intro();

    let port = config_port();
    let workers = config_workers();
    let endpoint = config_called_service();    

    println!("{}", "-----------------------------------------------------------");
    println!("Starting server.... Press Ctrl-C to stop it.");

    if log_enabled!(Level::Info) {
        debug!("Starting server");
    }

    HttpServer::new(
        move || App::new()
            //.data({Client::default(), endpoint})
            .data(Parameters{
                client: Client::default(), 
                //c_endpoint: "http://127.0.0.1:9596".to_string(),
                c_endpoint: endpoint.clone(),
            })
            .service(
                web::resource("/")
                    .route(web::get().to(index))
            ) // end service
            .service(
                web::resource("/echo/{message}")
                .route(web::get().to_async(echo_handler))
            ) // end hello service
            .service(
                web::resource("factorialIterative/{number}")
                .route(web::get().to_async(factorial_iter_handler))
            ) // end iter service
            .service(
                web::resource("factorialRecursive/{number}")
                .route(web::get().to_async(factorial_recur_handler))
            ) // end recur service
            
    ) // end http server
    .workers(workers)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
}
