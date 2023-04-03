use cfg_if::cfg_if;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    // _ = Test::register();
}

// #[server(Test, "/api")]
// pub async fn test() -> Result<i32, ServerFnError> {
//     Ok(123)
// }

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix::{Actor, StreamHandler};
        use actix_web::{web, Error, HttpRequest, HttpResponse};
        use actix_web_actors::ws;
        use std::sync::atomic::AtomicI32;

        static COUNT: AtomicI32 = AtomicI32::new(0);

        /// Define HTTP actor
        struct MyWs;

        impl Actor for MyWs {
            type Context = ws::WebsocketContext<Self>;
        }

        /// Handler for ws::Message message
        impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
            fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
                match msg {
                    Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
                    Ok(ws::Message::Text(_)) => { 
                        let count = COUNT.load(std::sync::atomic::Ordering::Relaxed);
                        COUNT.store(count + 1, std::sync::atomic::Ordering::Relaxed);
                        ctx.text(count.to_string())
                    },
                    Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
                    _ => (),
                }
            }
        }

        pub async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
            ws::start(MyWs {}, &req, stream)
        }
    }
}
