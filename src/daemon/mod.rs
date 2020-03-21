mod response;
use actix_web::middleware::Logger;
use env_logger::Env;
use response::{OrderItemResponse};
use crate::orders::*;
use crate::command_control;
use crate::command_control::CmdCtl;
use actix_web::{HttpRequest, Responder, HttpResponse};
use structopt::StructOpt;


pub struct Daemeon { }

impl Daemeon {

    pub fn to_json(order: Order) -> OrderItemResponse {
        let mut json = OrderItemResponse::new();
        json.set_attributes(order);
        json
    }

    pub fn to_json_with_error(error: String) -> OrderItemResponse {
        let mut json = OrderItemResponse::new();
        json.set_error(error);
        json
    }

    pub async fn post_key() -> impl Responder {
        let mut options = CmdCtl::from_args();
        let mut json = OrderItemResponse::new();

        options.method = "POST".to_string();
        match OrderItemRunner::run(&options).await {
            Ok(result) => {
                json = Daemeon::to_json(result);
            },
            Err(err) => {
                json = Daemeon::to_json_with_error(err);
            },
        }

        HttpResponse::Ok().json(json)
    }

    pub async fn post_from_key() -> impl Responder {
        let mut options = command_control::CmdCtl::from_args();
        let mut json = OrderItemResponse::new();

        options.method = "POST".to_string();
        match OrderItemRunner::run(&options).await {
            Ok(result) => {
                json = Daemeon::to_json(result);
            },
            Err(err) => {
                json = Daemeon::to_json_with_error(err);
            },
        }

        HttpResponse::Ok().json(json)
    }


    pub async fn put_key() -> impl Responder {
        let mut options = command_control::CmdCtl::from_args();
        let mut json = OrderItemResponse::new();

        options.method = "PUT".to_string();
        match OrderItemRunner::run(&options).await {
            Ok(result) => {
                json = Daemeon::to_json(result);
            },
            Err(err) => {
                json = Daemeon::to_json_with_error(err);
            },
        }

        HttpResponse::Ok().json(json)
    }

    pub async fn get_key() -> impl Responder {
        let mut options = command_control::CmdCtl::from_args();
        let mut json = OrderItemResponse::new();

        options.method = "GET".to_string();
        match OrderItemRunner::run(&options).await {
            Ok(result) => {
                json = Daemeon::to_json(result);
            },
            Err(err) => {
                json = Daemeon::to_json_with_error(err);
            },
        }

        HttpResponse::Ok().json(json)
    }


    pub async fn del_key() -> impl Responder {
        let mut options = command_control::CmdCtl::from_args();
        let mut json = OrderItemResponse::new();

        options.method = "DELETE".to_string();
        match OrderItemRunner::run(&options).await {
            Ok(result) => {
                json = Daemeon::to_json(result);
            },
            Err(err) => {
                json = Daemeon::to_json_with_error(err);
            },
        }

        HttpResponse::Ok().json(json)
    }

    pub async fn run_as_daemeon() -> std::io::Result<()> {
        use actix_web::{web, App, HttpServer};

        let options = command_control::CmdCtl::from_args();
        let host = format!("{}:{}", options.host, options.port);

        println!("Listening {:#?}", host);

        env_logger::from_env(Env::default().default_filter_or("info")).init();

        HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .wrap(Logger::new("%a %P %{User-Agent}i"))
                .route("/create", web::post().to( Daemeon::post_key ))
                .route("/create/from/{order_id}", web::post().to( Daemeon::post_from_key ))
                .route("/read/{order_id}", web::get().to( Daemeon::get_key ))
                .route("/update/{order_id}", web::put().to( Daemeon::put_key ))
                .route("/delete/{order_id}", web::delete().to( Daemeon::del_key ))
        })
        .bind(host)?
        .run()
        .await
    }

}
