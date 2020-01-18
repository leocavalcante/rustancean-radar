use warp::Filter;

use backend::application::SearchQuery;

#[tokio::main]
async fn main() {
    let list_devs = warp::path!("devs")
        .and(warp::get())
        .and_then(backend::controllers::dev::index)
        .clone();

    let create_dev = warp::path!("devs")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(backend::controllers::dev::store)
        .clone();

    let search_dev = warp::path!("search")
        .and(warp::get())
        .and(warp::query::<SearchQuery>())
        .and_then(backend::controllers::search::search)
        .clone();

    let routes = list_devs.or(create_dev).or(search_dev);
    warp::serve(routes).run(([127, 0, 0, 1], 3333)).await;
}