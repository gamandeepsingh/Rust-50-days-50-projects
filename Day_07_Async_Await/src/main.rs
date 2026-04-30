use reqwest;

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url).await?;
    // println!("{}", res.status());
    // println!("{:?}", res.headers());
    let body = res.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://jsonplaceholder.typicode.com/todos/1",
        "https://jsonplaceholder.typicode.com/todos/2",
        "https://jsonplaceholder.typicode.com/todos/3",
    ];

    // create async tasks
    let futures = urls.iter().map(|url| fetch_url(url));

    // run all at once (concurrently)
    let results = futures::future::join_all(futures).await;

    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(body) => println!("{}: {}", i, body),
            Err(e) => println!("{}: Error: {}", i, e),
        }
    }
}
