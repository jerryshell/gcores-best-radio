pub mod model;

use anyhow::Result;

pub fn write_to_csv(data_list: &[crate::model::response_root::Datum]) -> Result<()> {
    let mut csv_writer = csv::Writer::from_path("data.csv")?;

    csv_writer.write_record([
        "id",
        "type",
        "title",
        "desc",
        "excerpt",
        "is-published",
        "thumb",
        "app-cover",
        "cover",
        "comments-count",
        "likes-count",
        "bookmarks-count",
        "is-verified",
        "published-at",
        "option-is-official",
        "option-is-focus-showcase",
        "duration",
        "is-free",
        "url",
    ])?;

    for data_item in data_list.iter() {
        csv_writer.write_record(&[
            data_item.id.to_string(),
            data_item.datum_type.to_string(),
            data_item.attributes.title.to_string(),
            data_item.attributes.desc.to_string(),
            data_item.attributes.excerpt.to_string(),
            data_item.attributes.is_published.to_string(),
            data_item.attributes.thumb.to_string(),
            data_item.attributes.app_cover.to_string(),
            data_item.attributes.cover.to_string(),
            data_item.attributes.comments_count.to_string(),
            data_item.attributes.likes_count.to_string(),
            data_item.attributes.bookmarks_count.to_string(),
            data_item.attributes.is_verified.to_string(),
            data_item.attributes.published_at.to_string(),
            data_item.attributes.option_is_official.to_string(),
            data_item.attributes.option_is_focus_showcase.to_string(),
            data_item.attributes.duration.to_string(),
            data_item.attributes.is_free.to_string(),
            format!("https://www.gcores.com/radios/{}", data_item.id),
        ])?;
    }

    csv_writer.flush()?;

    Ok(())
}
