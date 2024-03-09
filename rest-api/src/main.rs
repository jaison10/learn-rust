use warp::Filter;
use tokio;
mod route;

#[tokio::main]
async fn main(){
    let routes = route::routes();

    println!("Server started at http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}