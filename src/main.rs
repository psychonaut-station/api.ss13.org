use poem::{Route, Server as PoemServer, listener::TcpListener};
use poem_openapi::{OpenApi, OpenApiService, param::Query, payload::PlainText};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api = OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api.stoplight_elements();
    let app = Route::new().nest("/api", api).nest("/", ui);

    PoemServer::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
