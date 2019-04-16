#[macro_use] extern crate log;
//#[macro_use] extern crate env_logger;
#[macro_use] extern crate serde_derive;

use actix_web::{web, App, HttpServer};
use actix_web::client::Client;

use colored::*;
use std::env;
use log::{info, debug};
use log::Level;

mod handlers;
use crate::handlers::index;
use crate::handlers::echo;
use crate::handlers::Parameters;

// Defines the default port
const DEFAULT_PORT: u16          = 9296;

// Defines the workers used by server
const DEFAULT_WORKERS: usize     = 2;

// Defines the url for the called service (calculator)
const DEFAULT_CLIENT_URL: &str = "http://127.0.0.1:9596"; 


// Displays intro banner
fn intro() {
    println!("{}", "===========================================================".yellow().bold());
    println!("{}", "                    Bank v 0.1.0".yellow().bold());
    println!("{}", "===========================================================".yellow().bold());
    println!("{}", "   Please use env variables for configuration:".yellow().bold());
    println!("{}", "       BANK_PORT=port number".yellow().bold());
    println!("{}", "       BANK_WORKERS=workers for server".yellow().bold());
    println!("{}", "       BANK_CLIENT_URL=url of called service".yellow().bold());
    println!("{}", "-----------------------------------------------------------");
    println!("Starting configuration......\n");
}

// Read env variable to configure port
fn config_port() -> u16 {
    let key = "BANK_PORT";
    let mut port: u16 = DEFAULT_PORT;
    
    println!("- Port:");

    match env::var(key) {
        Ok(val) => {
            println!("... Config variable?:  {}", "exists!".green());
            match val.as_str().parse::<u16>() {
                Ok(n) => {
                    println!("... Valid?:            {}", "Yes".to_string().green());
                    println!("... Port set to:       {}", n.to_string().green());
                    port = n;
                    return port;
                },
                Err(e) => {
                    println!("... Valid?:            {} - {}", "No".red(), e.to_string().red());
                    println!("... Port set to:       {} - (by default)", 
                        DEFAULT_PORT.to_string().green());
                    return DEFAULT_PORT;
                },
            }
        }
        Err(_e) => {
            println!("... Config variable?:  {}", "No".red());
            println!("... Port set to:       {} - (by default)", DEFAULT_PORT.to_string().green());
            return DEFAULT_PORT;
        }
    } 
}

// Read env variable to configure workers.
fn config_workers() -> usize {
    let key = "BANK_WORKERS";
    let mut workers: usize = DEFAULT_WORKERS;

    println!("\n- Workers:");
    match env::var(key) {
        Ok(val) => {
            println!("... Config variable?:  {}", "exists!".green());
            match val.as_str().parse::<usize>() {
                Ok(n) => {
                    println!("... Valid?:            {}", "Yes".to_string().green());
                    println!("... Workers set to:    {}", n.to_string().green());
                    workers = n;
                    return workers;
                },
                Err(e) => {
                    println!("... Valid?:            {} - {}", "No".red(), e.to_string().red());
                    println!("... Workers set to:    {} - (by default)", format!("{}", DEFAULT_WORKERS)
                        .green());
                    return DEFAULT_WORKERS;
                },
            }
        }
        Err(_e) => {
            println!("... Config variable?:  {}", "No".red());
            println!("... Workers set to:    {} - (by default)", 
                format!("{}", DEFAULT_WORKERS).green());
            return DEFAULT_WORKERS;
        },
    } 
}

// Read env variable to configure workers.
fn config_called_service() -> String {
    let key = "BANK_CLIENT_URL";
    let mut endpoint: String = DEFAULT_CLIENT_URL.to_string();

    println!("\n- Client URL:");
    match env::var(key) {
        Ok(val) => {
            println!("... Config variable?:  {}", "exists!".green());

            match val.as_str().parse::<String>() {
                Ok(n) => {
                    println!("... Valid?:            {}", "Yes".to_string().green());
                    println!("... Client URL set to: {}", n.green());
                    endpoint = n;
                    return endpoint;
                },
                Err(e) => {
                    println!("... Valid?:            {} - {}", "No".red(), e.to_string().red());
                    println!("... Workers set to:    {} - (by default)", format!("{}", DEFAULT_CLIENT_URL
                        .to_string().green()));
                    return DEFAULT_CLIENT_URL.to_string();
                },
            }
        }
        Err(_e) => {
            println!("... Config variable?:  {}", "No".red());
            println!("... Workers set to:    {} - (by default)", 
                format!("{}", DEFAULT_CLIENT_URL.to_string().green()));
            return DEFAULT_CLIENT_URL.to_string();
        },
    } 
}

fn main() -> std::io::Result<()>  {
    env_logger::init();

    intro();

    let port = config_port();
    let workers = config_workers();
    let endpoint = config_called_service();

    println!("{}", "-----------------------------------------------------------");
    println!("Starting server.... Press Ctrl-C to stop it.");

    if log_enabled!(Level::Debug) {
        debug!("Startting server");
    }

    HttpServer::new(
        || App::new()
            //.data({Client::default(), endpoint})
            .data(Parameters{
                client: Client::default(), 
                c_endpoint: "http://127.0.0.1:9596/".to_string(),
            })
            .service(
                web::resource("/")
                    .route(web::get().to(index))
            ) // end service
            .service(
                web::resource("/echo/{message}")
                .route(web::get().to_async(echo))
            ) // end hello service
            
    ) // end http server
    .workers(workers)
    .bind(format!("127.0.0.1:{}", port))?
    .run()
}
