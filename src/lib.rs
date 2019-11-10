pub mod settings;
use futures::prelude::*;
use futures::stream;
//use tokio::sync::mpsc;

use tokio::timer::delay;

use std::time::Duration;


pub async fn run(s: settings::Settings) -> Result<(), Box<dyn std::error::Error>> {
    //    let (tx, mut rx) = mpsc::channel(1_024);
    let urls = vec![
        format!("http://example.com/1"),
        format!("http://example.com/2"),
        format!("http://example.com/3"),
        format!("http://example.com/4"),
    ];
    let fs = urls.iter().map(|url| do_ping(url));

    let buffered = stream::iter(fs).buffer_unordered(s.misc.concurrency as usize);

    let results: Vec<_> = buffered.collect().await;

    println!("{:?}",results);

    /*   rx.close();

    while let Some(i) = rx.recv().await {
        println!("got = {}", i);
    }*/

    Ok(())
}

async fn do_ping(url: &str) -> Result<u16, Box<dyn std::error::Error>> {
    println!("pinging {}", url);
    let resp = reqwest::get(url).await?;

    let when = tokio::clock::now() + Duration::from_millis(1000);
    delay(when).await;

    let status_code = resp.status().as_u16();
    println!("status_code {}", status_code);
    Ok(status_code)
}
