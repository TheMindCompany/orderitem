mod response;
use actix_web::middleware::Logger;
use env_logger::Env;
use response::{OrderItemResponse, OrderItemError};
use crate::orders::*;
use crate::command_control;
use crate::command_control::CmdCtl;
use actix_web::{HttpRequest, HttpResponse, web, App, HttpServer};
use actix_web::http::StatusCode;
use structopt::StructOpt;


pub struct Daemeon { }

impl Daemeon {

    pub fn to_json(order: Order) -> OrderItemResponse {
        let mut json = OrderItemResponse::new();
        json.set_attributes(order);
        json
    }

    pub async fn post_key(req: HttpRequest) -> Result<web::HttpResponse, OrderItemError> {
        let (sku_id, customer_id, upload_id): (Option<String>, Option<i32>, Option<i32>) = req.match_info().load().unwrap();
        let mut options = CmdCtl::from_args();

        options.method = "POST".to_string();
        options.sku_id = sku_id;
        options.customer_id = customer_id;
        options.upload_id = upload_id;

        match OrderItemRunner::run(&options).await {
            Ok(result) => {
                let json = Daemeon::to_json(result);
                Ok(HttpResponse::build(StatusCode::CREATED).json(json))
            },
            Err(err) => {
                Err(OrderItemError {
                    msg: err,
                    status: 400,
                })
            },
        }

    }

    pub async fn post_from_key(req: HttpRequest) -> Result<web::HttpResponse, OrderItemError> {
        let order_id: i32 = req.match_info().load().unwrap();
        let mut options = command_control::CmdCtl::from_args();

        options.method = "POST".to_string();
        options.order_id = Some(order_id);

        match OrderItemRunner::run(&options).await {
            Ok(result) => {
                let json = Daemeon::to_json(result);
                Ok(HttpResponse::build(StatusCode::CREATED).json(json))
            },
            Err(err) => {
                Err(OrderItemError {
                    msg: err,
                    status: 400,
                })
            },
        }
    }


    pub async fn put_key(info: web::Json<OrderItemResponse>, req: HttpRequest) -> Result<web::HttpResponse, OrderItemError> {
        let order_id: i32 = req.match_info().load().unwrap();
        let mut options = command_control::CmdCtl::from_args();

        options.method = "PUT".to_string();
        options.order_id = Some(order_id);
        options.upload_id = info.data.attributes.upload_id;
        options.status = info.data.attributes.status.clone();
        options.sku_id = info.data.attributes.sku_id.clone();
        options.quantity = info.data.attributes.quantity;
        options.discount = info.data.attributes.discount.clone();
        options.payment_id = info.data.attributes.payment_id;
        options.shipping_id = info.data.attributes.shipping_id;
        options.ready_to_ship = info.data.attributes.ready_to_ship;
        options.notes = info.data.attributes.notes.clone();

        match OrderItemRunner::run(&options).await {
            Ok(result) => {
                let json = Daemeon::to_json(result);
                Ok(HttpResponse::build(StatusCode::ACCEPTED).json(json))
            },
            Err(err) => {
                Err(OrderItemError {
                    msg: err,
                    status: 400,
                })
            },
        }
    }

    pub async fn get_key(req: HttpRequest) -> Result<web::HttpResponse, OrderItemError> {
        let order_id: i32 = req.match_info().load().unwrap();
        let mut options = command_control::CmdCtl::from_args();

        options.method = "GET".to_string();
        options.order_id = Some(order_id);

        match OrderItemRunner::run(&options).await {
            Ok(result) => {
                let json = Daemeon::to_json(result);
                Ok(HttpResponse::Ok().json(json))
            },
            Err(err) => {
                Err(OrderItemError {
                    msg: err,
                    status: 204,
                })
            },
        }
    }


    pub async fn del_key(req: HttpRequest) -> Result<web::HttpResponse, OrderItemError> {
        let order_id: i32 = req.match_info().load().unwrap();
        let mut options = command_control::CmdCtl::from_args();

        options.method = "DELETE".to_string();
        options.order_id = Some(order_id);

        match OrderItemRunner::run(&options).await {
            Ok(result) => {
                let json = Daemeon::to_json(result);
                Ok(HttpResponse::build(StatusCode::ACCEPTED).json(json))
            },
            Err(err) => {
                Err(OrderItemError {
                    msg: err,
                    status: 400,
                })
            },
        }
    }

    pub async fn run_as_daemeon() -> std::io::Result<()> {
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
        .keep_alive(None) // <- Disable keep-alive
        // https://docs.rs/actix-web/3.0.0-alpha.1/actix_web/struct.HttpServer.html#method.bind_rustls
        // https://docs.rs/rustls/0.17.0/rustls/struct.ServerConfig.html
        // .bind_rustls(tls_config)
        .bind(host)?
        .run()
        .await
    }

}
