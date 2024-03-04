use warp::Filter;
use model::post_model::Post;

// In-memory storage for blog posts
lazy_static::lazy_static! {
    static ref POSTS: tokio::sync::Mutex<Vec<Post>> = tokio::sync::Mutex::new(Vec::new());
}

pub async fn create_post(post : post_model::Post) -> Result<impl warp::Reply, warp::Rejection> {
    let mut posts = POSTS.lock().await;
    posts.push(post.clone());
    Ok(warp::reply::json(&post))
}

pub async fn get_posts() -> Result<impl warp::Reply, warp::Rejection> {
    let posts = POSTS.lock().await;
    Ok(warp::reply::json(&*posts))
}      

pub async fn get_post_by_id(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let posts = POSTS.lock().await;
    if let Some(post) = posts.iter().find(|p| p.id == id) {
        Ok(warp::reply::json(&post))
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn update_post(id: u64, updated_post: post_model::Post) -> Result<impl warp::Reply, warp::Rejection> {
    let mut posts = POSTS.lock().await;
    if let Some(post) = posts.get_mut(id) {
        *post = updated_post;
        Ok(warp::reply::json(&post))
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn delete_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let mut posts = POSTS.lock().await;
    if let Some(index) = posts.iter().position(|p| p.id == id) {
        posts.remove(index);
        Ok(warp::reply::json(&posts))
    } else {
        Err(warp::reject::not_found())
    }
}

