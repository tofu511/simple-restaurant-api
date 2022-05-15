use api_core::domain::item::Item;
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct ItemRow {
    pub id: u32,
    pub name: String,
    pub quantity: u32,
    pub table_number: u32,
    pub start_cooking_at: NaiveDateTime,
    pub finish_cooking_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ItemRow {
    pub fn from_row(row: ItemRow) -> Item {
        Item::of(
            row.id,
            row.name.to_string(),
            row.quantity,
            row.table_number,
            row.start_cooking_at,
            row.finish_cooking_at,
        )
    }

    pub fn from_rows(rows: Vec<ItemRow>) -> Vec<Item> {
        rows.iter()
            .map(|row| {
                Item::of(
                    row.id,
                    row.name.to_string(),
                    row.quantity,
                    row.table_number,
                    row.start_cooking_at,
                    row.finish_cooking_at,
                )
            })
            .collect()
    }
}
