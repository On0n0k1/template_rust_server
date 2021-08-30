use warp::Filter;
// use warp::{Filter, Rejection, Reply};


// Tokio main function. Start it's async runtime.
#[tokio::main]
async fn main() {

    // routes for each request:    
    // map is for sync closures
    let route_home = warp::path::end().map(|| "Home!");
    // and_then is for async closures
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let route_hello = warp::path!("hello" / String)
        .and_then(|name| async move{
            hello(name).await
        });

    let routes = warp::get().and(
        route_home
            .or(route_hello)
    );
    let routes = routes.with(warp::cors().allow_any_origin());

    println!("Started server at localhost:8000");

    // Starts the server with given inputs
    // [0, 0, 0, 0] is 0.0.0.0, it will request an available endpoint from docker.
    // port 8000
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

// We can make more async functions like these.
// Return type must be Result<impl warp::Reply, warp::Rejection>
// So that it's compatible with warp.
async fn hello(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = format!("Hello, {}!", name);
    // println!("{}", &reply);
    Ok(warp::reply::html(reply))
}
