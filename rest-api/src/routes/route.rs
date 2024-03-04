use warp::Filter;
use super::handlers;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let post = warp::path("post");
    let post_id = warp::path("post" / u64);
    let post_routes = post
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::create_post)
        .or(post
            .and(warp::get())
            .and_then(handlers::get_posts))
        .or(post_id
            .and(warp::get())
            .and_then(handlers::get_post_by_id))
        .or(post_id
            .and(warp::put())
            .and(warp::body::json())
            .and_then(handlers::update_post))
        .or(post_id
            .and(warp::delete())
            .and_then(handlers::delete_post));
    post_routes
}
