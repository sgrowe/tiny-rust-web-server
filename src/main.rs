use tokio::time::{delay_for, Duration};
use warp::Filter;

#[tokio::main]
async fn main() {
    tokio::join!(web_server(), bg_work_from_queue());
}

async fn web_server() {
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3000)).await;
}

async fn bg_work_from_queue() {
    loop {
        let next_task = poll_queue::<()>().await;

        if let Some(task) = next_task {
            println!("Working on task: {:?}", task);
        } else {
            println!("No work available, sleeping...");
            delay_for(Duration::from_secs(3)).await;
        }
    }
}

async fn poll_queue<Task>() -> Option<Task> {
    // Pull a task from some sort of queue
    None
}
