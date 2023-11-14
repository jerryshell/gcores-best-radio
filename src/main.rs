use gcores_best_radio::{model::response_root::ResponseRoot, write_to_csv};

#[tokio::main]
async fn main() {
    // init tracing
    tracing_subscriber::fmt::init();

    // init client
    let client = reqwest::Client::builder().build().unwrap();

    let mut result = vec![];

    let page_limit = 12;
    let mut page_offset = 0;

    let snapshot_count_init = 10;
    let mut snapshot_count = snapshot_count_init;

    loop {
        let url = format!("https://www.gcores.com/gapi/v1/radios?page[limit]={}&page[offset]={}&sort=-published-at&filter[list-all]=0&fields[radios]=title%2Cdesc%2Cexcerpt%2Cis-published%2Cthumb%2Capp-cover%2Ccover%2Ccomments-count%2Clikes-count%2Cbookmarks-count%2Cis-verified%2Cpublished-at%2Coption-is-official%2Coption-is-focus-showcase%2Cduration%2Cdraft%2Caudit-draft%2Cuser%2Ccomments%2Ccategory%2Ctags%2Centries%2Centities%2Csimilarities%2Clatest-collection%2Ccollections%2Coperational-events%2Cportfolios%2Ccatalog-tags%2Cmedia%2Cdjs%2Clatest-album%2Calbums%2Cis-free&meta[categories]=%2C&meta[users]=%2C", page_limit, page_offset);
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
                tracing::info!("{:#?}", response_json);
                break;
            }
        };

        page_offset += page_limit;

        snapshot_count -= 1;
        if snapshot_count <= 0 {
            tracing::info!("{:#?}", result);
            write_to_csv(&result).unwrap();
            snapshot_count = snapshot_count_init;
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    write_to_csv(&result).unwrap();
}
