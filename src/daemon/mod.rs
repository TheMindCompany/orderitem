mod response;
use actix_web::middleware::Logger;
use env_logger::Env;
use response::{OrderItemResponse};
use crate::orders::*;
use crate::command_control;
use crate::command_control::CmdCtl;
use actix_web::{HttpRequest, Responder, HttpResponse, web, App, HttpServer};
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

    pub async fn post_key(req: HttpRequest) -> impl Responder {
        let (sku_id, customer_id, upload_id): (Option<String>, Option<i32>, Option<i32>) = req.match_info().load().unwrap();
        let mut options = CmdCtl::from_args();
        let mut json = OrderItemResponse::new();

        options.method = "POST".to_string();
        options.sku_id = sku_id;
        options.customer_id = customer_id;
        options.upload_id = upload_id;

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

    pub async fn post_from_key(req: HttpRequest) -> impl Responder {
        let order_id: i32 = req.match_info().load().unwrap();
        let mut options = command_control::CmdCtl::from_args();
        let mut json = OrderItemResponse::new();

        options.method = "POST".to_string();
        options.order_id = Some(order_id);

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


    pub async fn put_key(req: HttpRequest) -> impl Responder {
        let order_id: i32 = req.match_info().load().unwrap();
        let mut options = command_control::CmdCtl::from_args();
        let mut json = OrderItemResponse::new();

        options.method = "PUT".to_string();
        options.order_id = Some(order_id);

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

    pub async fn get_key(req: HttpRequest) -> impl Responder {
        let order_id: i32 = req.match_info().load().unwrap();
        let mut options = command_control::CmdCtl::from_args();
        let mut json = OrderItemResponse::new();

        options.method = "GET".to_string();
        options.order_id = Some(order_id);

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


    pub async fn del_key(req: HttpRequest) -> impl Responder {
        let order_id: i32 = req.match_info().load().unwrap();
        let mut options = command_control::CmdCtl::from_args();
        let mut json = OrderItemResponse::new();

        options.method = "DELETE".to_string();
        options.order_id = Some(order_id);

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
                .route("/create/{sku_id}/{customer_id}/{upload_id}", web::post().to( Daemeon::post_key ))
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
