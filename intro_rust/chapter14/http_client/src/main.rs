use reqwest::StatusCode;
use std::{thread, time};

async fn sample14_1_3() -> Result<(), Box<dyn std::error::Error>> {
    thread::sleep(time::Duration::from_millis(1000));
    let n = 7;
    let url = format!("http://openccpm.com/blog/?p={}", n);
    println!("call {}", url);
    let body = reqwest::get(&url).await?.text().await?;
    println!("response is \n{}", body);
    Ok(())
}

async fn sample14_1_4() -> Result<(), Box<dyn std::error::Error>> {
    thread::sleep(time::Duration::from_millis(1000));
    let url = "http://openccpm.com/unknown.txt";
    println!("call {}", url);
    let res = reqwest::get(url).await?;
    match res.status() {
        StatusCode::OK => {
            let body = res.text().await?;
            println!("response is \n{}", body);
        }
        StatusCode::NOT_FOUND => {
            println!("error: not found!!");
        }
        _ => {
            println!("error: other error occured");
        }
    }

    Ok(())
}

async fn sample14_1_5() -> Result<(), Box<dyn std::error::Error>> {
    thread::sleep(time::Duration::from_millis(1000));
    let url = "http://unknown.openccpm.com/blog";
    println!("call {}", url);
    if let Ok(res) = reqwest::get(url).await {
        match res.status() {
            StatusCode::OK => {
                let body = res.text().await?;
                println!("response is \n{}", body);
            }
            StatusCode::NOT_FOUND => {
                println!("error: not found!!");
            }
            _ => {
                println!("error: other error occured");
            }
        }
    } else {
        println!("error:: servver access error");
    }

    Ok(())
}

async fn sample14_3_2() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://openccpm.com/redmine/projects.json";
    println!("call {}", url);

    let res = reqwest::get(url).await?;

    let body = res.text().await?;
    /* json parse */

    let json: serde_json::Value = serde_json::from_str(&body)?;
    let projects = json["projects"].as_array().unwrap();
    for item in projects {
        let identifier = &item["identifier"].as_str().unwrap();
        let name = &item["name"].as_str().unwrap();
        println!("tag : {}:{}", identifier, name);
    }

    println!("response is /n{}", body);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let url = "http://openccpm.com/blog/";
    println!(" call {} ", url);

    let res = reqwest::get(url).await?;

    let body = res.text().await?;

    println!("response is \n {}", body);

    /* */
    sample14_1_3().await.unwrap();

    sample14_1_4().await.unwrap();

    sample14_1_5().await.unwrap();

    sample14_3_2().await.unwrap();

    Ok(())
}
