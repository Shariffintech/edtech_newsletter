extern crate actix_web;
use edtech_newsletter::run;

#[actix_web::main]
fn main() -> std::io::Result<()> {
    match run(){
        Ok(_server) => {
            // do something with the server
            Ok(())
        },
        Err(error) => {
            // handle the error
            Err(error)
        }
    }
    
}


    