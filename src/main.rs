use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::time::{Duration, Instant};
use reqwest::Client;
use tokio::task;
use colored::Colorize;
use clap::Parser;

// Asynchronous tool for HTTP stress testing
#[derive(Parser, Debug)]
#[command(author = "V3cTr4X", version = "1.0", about = "Asynchronous tool for HTTP stress testing")]
struct Args {
    /// URL for testing
    #[arg(short, long)]
    url: Option<String>,

    /// Assincronous task
    #[arg(short, long)]
    tasks: Option<usize>,

    /// Attack duration
    #[arg(short, long)]
    duration: Option<u64>,
}

#[tokio::main]
async fn main() {
    print_ascii_art();

    let args = Args::parse();

    // If there are not arguments, show tip
    if args.url.is_none() || args.tasks.is_none() || args.duration.is_none() {
        println!("{}", "Missing arguments. Use --help for more information.".yellow());
        return;
    }

    let url = args.url.unwrap();
    let tasks = args.tasks.unwrap();
    let duration = args.duration.unwrap();

    let client = Arc::new(
        Client::builder()
            .pool_idle_timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(100)
            .build()
            .unwrap(),
    );

    let counter = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();

    println!("Starting the attack for {} seconds with {} task to {}...", duration, tasks, url);

    let mut handles = Vec::new();

    for _ in 0..tasks {
        let client = Arc::clone(&client);
        let counter = Arc::clone(&counter);
        let url = url.clone();
        let start = start.clone();

        let handle = task::spawn(async move {
            let mut local_count = 0;
            while start.elapsed() < Duration::from_secs(duration) {
                let response = client
                    .get(&url)
                    .header("Accept-Encoding", "gzip, deflate")
                    .header("User-Agent", "Mozilla/5.0 (compatible; FloodSim/1.0)")
                    .send()
                    .await;

                if let Ok(resp) = response {
                    if resp.status().is_success() {
                        local_count += 1;
                    }
                }
            }
            counter.fetch_add(local_count, Ordering::Relaxed);
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let total = counter.load(Ordering::Relaxed);
    println!("Good petitions: {}", total);
    println!("Task per seconds: {:.2}", total as f64 / duration as f64);
}

fn print_ascii_art() {
    let art = r#"
          @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          

         @@                                 @@          
         @    @@@@@@@@@@@@@@@@@@@@@@@@@@@   @@          
         @   .@@@@ @@ @@ @@ @@ @  @@ @@@@   @@           ______   __         ______     ______     _____     ______     __     __    __   
         @   :@@@@ @@ @@ @@ @@ @  @@ @@@@   @@          /\  ___\ /\ \       /\  __ \   /\  __ \   /\  __-.  /\  ___\   /\ \   /\ "-./  \  
         @   =@@@@ @@ @@ @@ @@ @  @@ @@@@   @@          \ \  __\ \ \ \____  \ \ \/\ \  \ \ \/\ \  \ \ \/\ \ \ \___  \  \ \ \  \ \ \-./\ \ 
         @   #@@@@@@@@@@@@@@@@@@@@@@@@@@@   @@           \ \_\    \ \_____\  \ \_____\  \ \_____\  \ \____-  \/\_____\  \ \_\  \ \_\ \ \_\
         @   #@@@@@@@@@@@@@@@@@@@@@@@@@@@                \/_/     \/_____/   \/_____/   \/_____/   \/____/   \/_____/   \/_/   \/_/  \_/
         @   #@@@@@@@@@@@@@@ ##  @@@@@@@@    ##                                    v1.0 --->Coded by V3cTr4X<---
         @   #@@@@@@@@@@@@ ######  @@@@@   ######       
         @   #@@@@@@@@@@@@ ########  @@  ########       
         @        @@@@@@@@@  ########  ########         
         @        @@@@@@@@@@@  ##############=          
         @           @@@@@@@@@@  ###########            
         @           @@@@@@@@@@   ########              
         @                      ############            
         @@@@@@@@@@@@@@@@@@@  ################          
                            ########    ########        
                           #######        ########      
                            ####            ####        
"#;

    println!("{}", art.red());
}
