//use granite_postgres::PgType;

#[derive(Debug)]
pub struct User {
    pub user_id: i32,
    pub first_name: String,
    pub last_name: String,
}

pub async fn list(db: &impl granite_postgres::DB) -> granite::Result<Vec<User>> {
    let client = db;

    let rows = granite::pg_row_vec!(
        db = client;
        row = {
            user_id: i32,
            first_name: String,
            last_name: String,
        };
        SELECT
            user_id,
            first_name,
            last_name
        FROM
            user
        ORDER BY
            user_id
    )
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| User {
            user_id: row.user_id,
            first_name: row.first_name,
            last_name: row.last_name,
        })
        .collect())
}
