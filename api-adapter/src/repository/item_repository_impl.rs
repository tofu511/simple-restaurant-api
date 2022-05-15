use crate::model::item::ItemRow;
use crate::persistence::mysql::Db;
use api_core::{domain::item::Item, error::Error, repository::item_repository::ItemRepository};
use async_trait::async_trait;
use derive_new::new;

#[derive(new, Clone)]
pub struct ItemRepositoryImpl {
    db: Db,
}

#[async_trait]
impl ItemRepository for ItemRepositoryImpl {
    async fn find_all(&self, table_number: u32) -> Result<Vec<Item>, Error> {
        let result = sqlx::query_as!(
          ItemRow,
          r#"SELECT id, name, quantity, table_number, start_cooking_at, finish_cooking_at, created_at, updated_at
          FROM items
          WHERE table_number = ?
          "#,
          table_number
        ).fetch_all(&self.db.pool)
        .await;

        if result.is_err() {
            Err(Error::ItemNotFoundError)
        } else {
            Ok(ItemRow::from_rows(result.unwrap()))
        }
    }
    async fn find_one(&self, table_number: u32, item_id: u32) -> Result<Item, Error> {
        let result = sqlx::query_as!(
        ItemRow,
        r#"SELECT id, name, quantity, table_number, start_cooking_at, finish_cooking_at, created_at, updated_at
          FROM items
          WHERE id = ? AND table_number = ?"#,
        item_id,
        table_number
      ).fetch_one(&self.db.pool)
      .await;

        if result.is_err() {
            Err(Error::ItemNotFoundError)
        } else {
            Ok(ItemRow::from_row(result.unwrap()))
        }
    }
    async fn create(&self, table_number: u32, item: Item) -> Result<u64, Error> {
        let res = sqlx::query!(
            r#"
    INSERT INTO items (name, quantity, table_number, start_cooking_at, finish_cooking_at)
    VALUES ( ?, ?, ?, ?, ? )
            "#,
            item.name,
            item.quantity,
            table_number,
            item.start_cooking_at,
            item.finish_cooking_at
        )
        .execute(&self.db.pool)
        .await;

        if res.is_err() {
            Err(Error::ItemInsertionError)
        } else {
            Ok(res.unwrap().last_insert_id())
        }
    }

    async fn delete(&self, table_number: u32, item_id: u32) -> Result<(), Error> {
        let result = sqlx::query!(
            r#"DELETE FROM items
          WHERE table_number = ? AND id = ?"#,
            table_number,
            item_id
        )
        .execute(&self.db.pool)
        .await;

        if result.is_err() {
            Err(Error::ItemDeletionError)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    impl ItemRepositoryImpl {
        async fn setup(&self) -> () {
            sqlx::query!("INSERT INTO tables VALUES (), (), (), (), ()")
                .execute(&self.db.pool)
                .await
                .unwrap();
        }

        async fn tear_down(&self) -> () {
            sqlx::query!("DELETE FROM tables")
                .execute(&self.db.pool)
                .await
                .unwrap();
            sqlx::query!("DELETE FROM items")
                .execute(&self.db.pool)
                .await
                .unwrap();
        }
    }

    use super::*;
    use crate::persistence::mysql::Env;
    use chrono::Local;
    use pretty_assertions::assert_eq;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_insert_and_query_item() {
        let db = Db::new(Env::Test).await;
        let repo = ItemRepositoryImpl::new(db);
        repo.setup().await;
        repo.tear_down().await;

        let item = Item::new(String::from("Yakisoba"), 10, Local::now().naive_local()).unwrap();
        let table_number = 1;
        let db_item_id = repo.create(table_number, item.clone()).await.unwrap();
        let item_id = u32::try_from(db_item_id).unwrap();

        let acutual_data = repo
            .find_one(table_number, u32::try_from(item_id).unwrap())
            .await
            .unwrap();

        assert_eq!(acutual_data.id.unwrap(), item_id);
        assert_eq!(acutual_data.name, item.name);
        assert_eq!(acutual_data.quantity, item.quantity);
        assert_eq!(acutual_data.table_number.unwrap(), table_number);

        repo.tear_down().await;
    }

    #[tokio::test]
    #[serial]
    async fn test_insert_and_query_multiple_items() {
        let db = Db::new(Env::Test).await;
        let repo = ItemRepositoryImpl::new(db);
        repo.setup().await;
        repo.tear_down().await;

        let items = vec![
            Item::new(String::from("Yakisoba"), 10, Local::now().naive_local()).unwrap(),
            Item::new(String::from("Takoyaki"), 20, Local::now().naive_local()).unwrap(),
        ];
        let table_number = 1;

        for item in items {
            repo.create(table_number, item)
                .await
                .expect("db error occured");
        }

        let expected_items = vec![("Yakisoba".to_string(), 10), ("Takoyaki".to_string(), 20)];
        let acutual_data = repo.find_all(table_number).await.unwrap();

        assert_eq!(acutual_data.len(), 2);

        for (e, a) in expected_items.iter().zip(acutual_data.iter()) {
            assert_eq!(e.0, a.name);
            assert_eq!(e.1, a.quantity);
        }

        repo.tear_down().await;
    }

    #[tokio::test]
    #[serial]
    async fn test_insert_and_delete() {
        let db = Db::new(Env::Test).await;
        let repo = ItemRepositoryImpl::new(db);
        repo.setup().await;
        repo.tear_down().await;

        let items = vec![
            Item::new(String::from("Yakisoba"), 10, Local::now().naive_local()).unwrap(),
            Item::new(String::from("Takoyaki"), 20, Local::now().naive_local()).unwrap(),
        ];
        let table_number = 1;

        let mut item_id = 0;
        for item in items {
            item_id = repo
                .create(table_number, item)
                .await
                .expect("db error occured");
        }

        let result = repo
            .delete(table_number, u32::try_from(item_id).unwrap())
            .await;
        assert_eq!(result.is_ok(), true);

        let query_res = repo.find_all(table_number).await.unwrap();
        assert_eq!(query_res.len(), 1);

        repo.tear_down().await;
    }
}
