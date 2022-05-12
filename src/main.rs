use warp::Filter;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let port = 3030;
    println!("Starting on port {:?}", port);

    let hello = warp::path!("hello").then(hello_handler);
    let async_sleep = warp::path!("async_sleep").then(async_sleep_handler);
    let sync_sleep = warp::path!("sync_sleep").then(sync_sleep_handler);
    let sync_sleep_nb = warp::path!("sync_sleep_nb").then(sync_sleep_nb_handler);
    let routes = warp::get().and(hello.or(async_sleep).or(sync_sleep).or(sync_sleep_nb));

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

async fn hello_handler() -> String {
    "Hello, World!".to_string()
}

const SLEEP_TIME_IN_MS: u64 = 100;

async fn sync_sleep_handler() -> String {
    std::thread::sleep(std::time::Duration::from_millis(SLEEP_TIME_IN_MS));
    "OK".to_string()
}

async fn sync_sleep_nb_handler() -> String {
    let _ = tokio::task::spawn_blocking(|| {
        std::thread::sleep(std::time::Duration::from_millis(SLEEP_TIME_IN_MS))
    })
    .await;
    "OK".to_string()
}

async fn async_sleep_handler() -> String {
    tokio::time::sleep(std::time::Duration::from_millis(SLEEP_TIME_IN_MS)).await;
    "OK".to_string()
}
