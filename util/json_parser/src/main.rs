async fn get_data(url: String) -> Result<String, Box<dyn std::error::Error>> {
    println!("get from url {}", url);
    let resp = reqwest::get(&url).await?.text().await?;
    Ok(resp)
}

struct Info {
    attr: String,
    value: u64,
}

impl Info {
    fn print_info(&self) {
        println!("attr={},value={}", self.attr, self.value);
    }
}

async fn get_main_summary(data: String) -> Vec<Info> {
    let json: serde_json::Value = serde_json::from_str(&data).unwrap();
    let result = &json["main_summary"];
    let arr = result["children"].as_array();
    let mut list = Vec::new();
    for item in arr.unwrap() {
        for it in item["children"].as_array().unwrap() {
            let _info = Info {
                attr: it["attr"].as_str().unwrap().to_string(),
                value: it["value"].as_u64().unwrap(),
            };
            list.push(_info);
        }
    }
    list
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let url =
        "https://raw.githubusercontent.com/tokyo-metropolitan-gov/covid19/master/data/data.json";
    println!("url:{}\n", url);
    let resp = get_data(url.to_string()).await.unwrap();
    let results = get_main_summary(resp).await;
    for it in results {
        it.print_info();
    }

    Ok(())
}
