pub mod model;

use anyhow::Result;

pub fn write_to_csv(data_list: &[crate::model::Datum]) -> Result<()> {
    let mut csv_writer = csv::Writer::from_path("data.csv")?;

    csv_writer.write_record([
        "id",
        "type",
        "title",
        "desc",
        "excerpt",
        "thumb",
        "cover",
        "comments-count",
        "likes-count",
        "bookmarks-count",
        "published-at",
        "duration",
        "url",
    ])?;

    for data_item in data_list.iter() {
        csv_writer.write_record(&[
            data_item.id.to_string(),
            data_item.datum_type.to_string(),
            data_item.attributes.title.to_string(),
            data_item.attributes.desc.to_string(),
            data_item.attributes.excerpt.to_string(),
            data_item.attributes.thumb.to_string(),
            data_item.attributes.cover.to_string(),
            data_item.attributes.comments_count.to_string(),
            data_item.attributes.likes_count.to_string(),
            data_item.attributes.bookmarks_count.to_string(),
            data_item.attributes.published_at.to_string(),
            data_item.attributes.duration.to_string(),
            format!("https://www.gcores.com/radios/{}", data_item.id),
        ])?;
    }

    csv_writer.flush()?;

    Ok(())
}
