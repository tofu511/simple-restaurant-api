use std::cmp::Ordering;

use chrono::{Duration, NaiveDateTime};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Item {
    pub id: Option<u32>,
    pub name: String,
    pub quantity: u32,
    pub table_number: Option<u32>,
    pub start_cooking_at: NaiveDateTime,
    pub finish_cooking_at: NaiveDateTime,
}

impl Item {
    pub fn new(name: String, quantity: u32, now: NaiveDateTime) -> Result<Self, String> {
        if name.is_empty() {
            return Err(String::from("name should not be empty"));
        }
        if quantity < 1 {
            return Err(String::from("quantity should not be negative number"));
        }
        let start_cooking_at = now;
        let finish_cooking_at = match quantity {
            n if n < 10 => now + Duration::minutes(5),
            n if n < 20 => now + Duration::minutes(10),
            _ => now + Duration::minutes(15),
        };
        Ok(Item {
            id: None,
            name,
            quantity,
            table_number: None,
            start_cooking_at,
            finish_cooking_at,
        })
    }

    pub fn of(
        id: u32,
        name: String,
        quantity: u32,
        table_number: u32,
        start_cooking_at: NaiveDateTime,
        finish_cooking_at: NaiveDateTime,
    ) -> Self {
        Item {
            id: Some(id),
            name,
            quantity,
            table_number: Some(table_number),
            start_cooking_at,
            finish_cooking_at,
        }
    }

    // TODO use Mock instead of now argument
    pub fn time_to_finish(self, now: NaiveDateTime) -> Duration {
        match self.finish_cooking_at.cmp(&now) {
            Ordering::Equal | Ordering::Less => Duration::minutes(0),
            Ordering::Greater => self.finish_cooking_at - now,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Local;
    use pretty_assertions::assert_eq;

    #[test]
    fn new_item_with_0_quantity() {
        assert_eq!(
            Item::new(String::from("sandwich"), 0, Local::now().naive_local()).is_err(),
            true
        );
    }
    #[test]
    fn new_item_with_9_quantity() {
        let res = Item::new(String::from("Cheese Burger"), 9, Local::now().naive_local());
        assert_eq!(res.is_ok(), true);
        let item = res.unwrap();
        assert_eq!(
            item.finish_cooking_at,
            item.start_cooking_at + Duration::minutes(5)
        );
    }
    #[test]
    fn new_item_with_10_quantity() {
        let res = Item::new(
            String::from("Cheese Burger"),
            10,
            Local::now().naive_local(),
        );
        assert_eq!(res.is_ok(), true);
        let item = res.unwrap();
        assert_eq!(
            item.finish_cooking_at,
            item.start_cooking_at + Duration::minutes(10)
        );
    }
    #[test]
    fn new_item_with_20_quantity() {
        let res = Item::new(
            String::from("Cheese Burger"),
            20,
            Local::now().naive_local(),
        );
        assert_eq!(res.is_ok(), true);
        let item = res.unwrap();
        assert_eq!(
            item.finish_cooking_at,
            item.start_cooking_at + Duration::minutes(15)
        );
    }

    #[test]
    fn time_to_finish() {
        let now = Local::now().naive_local();
        let quantity_1 = Item::new(String::from("sandwich"), 1, now).unwrap();
        assert_eq!(quantity_1.time_to_finish(now), Duration::minutes(5));

        let quantity_10 = Item::new(String::from("sandwich"), 10, now).unwrap();
        assert_eq!(quantity_10.time_to_finish(now), Duration::minutes(10));

        let quantity_20 = Item::new(String::from("sandwich"), 20, now).unwrap();
        assert_eq!(quantity_20.time_to_finish(now), Duration::minutes(15));
    }
}
