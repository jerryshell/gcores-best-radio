use std::{
    fs::File,
    io::{BufWriter, Write},
};

use gcores_best_radio::{model::ResponseRoot, write_to_csv};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = reqwest::Client::builder().build().unwrap();

    let mut result = vec![];

    let page_limit = 100;
    let mut page_offset = 0;

    let snapshot_countdown_init = 10;
    let mut snapshot_countdown = snapshot_countdown_init;

    loop {
        let url = format!(
            "https://www.gcores.com/gapi/v1/radios?page[limit]={page_limit}&page[offset]={page_offset}&sort=-published-at&fields[radios]=title%2Cdesc%2Cexcerpt%2Cthumb%2Ccover%2Ccomments-count%2Clikes-count%2Cbookmarks-count%2Cpublished-at%2Cduration"
        );
        tracing::info!("{}", url);

        let response = client.get(url).send().await.unwrap();

        let response_json = response.json::<ResponseRoot>().await.unwrap();
        match response_json.data {
            Some(data) => {
                if data.is_empty() {
                    break;
                }
                result.extend(data);
            }
            None => {
                tracing::warn!("response_json.data is None {:#?}", response_json);
                break;
            }
        };

        page_offset += page_limit;

        snapshot_countdown -= 1;
        if snapshot_countdown <= 0 {
            write_to_csv(&result).unwrap();
            snapshot_countdown = snapshot_countdown_init;
        }

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    tracing::info!("sort_by...");
    result.sort_by(|a, b| {
        b.attributes
            .bookmarks_count
            .as_i64()
            .unwrap_or(0)
            .cmp(&a.attributes.bookmarks_count.as_i64().unwrap_or(0))
    });

    tracing::info!("write to csv file...");
    write_to_csv(&result).unwrap();

    // write to json
    tracing::info!("write to json file...");
    let json_file = File::create("data.json").unwrap();
    let mut json_writer = BufWriter::new(json_file);
    serde_json::to_writer(&mut json_writer, &result).unwrap();
    json_writer.flush().unwrap();

    tracing::info!("finished");
}
