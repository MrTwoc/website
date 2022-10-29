use salvo::prelude::*;

#[handler]
async fn set_user(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    depot.insert("current_user", "Elon Musk");
}
#[handler]
async fn hello(depot: &mut Depot) -> String {
    // Notic: Don't use String here, because you inserted a &str.
    let user = depot.get::<&str>("current_user").copied().unwrap();
    format!("Hey {}, I love your money and girls!", user)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().hoop(set_user).handle(hello);

    let acceptor = TcpListener::new("127.0.0.1:7878").bind().await;
    Server::new(acceptor).serve(router).await;
}
