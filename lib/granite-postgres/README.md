```rust
## This example is being inline struct

    let rows = approck::pg_row!(
        db = &dbcx; 
        SELECT
            CONCAT(user_id, 'ff') AS user_id: i32,
            first_name: String,
            last_name: String,
        FROM
            t1
        WHERE
            user_id = $user_id
        ORDER BY
            user_id
    )
    .await?;


## This example is being explicit about the resulting struct

    let rows = approck::pg_row_vec!(
        row = {
            user_id: i32,
            first_name: String,
            last_name: String,
        };
        db = &dbcx;
        sql = {
            SELECT
                CONCAT(user_id, "ff") AS user_id,
                first_name,
                last_name,
            FROM
                t1
            WHERE
                user_id = $user_id
            ORDER BY
                user_id
        }
    )
    .await?;


## this example is using inline struct defintiion

    let rows = approck::pg_row_vec!(
        db = &dbcx;
        sql = {
            INSERT INTO 
                user
                (user_id, first_name, last_name)
            VALUES
                ($user_id:i32, $first_name:String, $last_name:String)
            RETURNING
                user_id:i32,
                first_name: String,
                last_name: String
        }
    )
    .await?;




## Preparing a query


dbrock::prepare!{
    ident = Query1;
    impl = row | row_list;
    sql = {
        SELECT 
            id : i32,
            name: String,
            email: String,
            message: String,
        FROM
            contact
        WHERE
            id = $id:i32
    }
};

fn foo() {
    let rows:Vec<Query1> = Query1::row_list(&dbcx, 1)?;
}


pub struct Query1 {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub message: String,
}

impl Query1 {
    pub async fn row_list(dbcx: ..., id: i32) -> Result<Vec<Self>, dbrock::Error> {
        let rows = dbrock::row_list!{
            CONN 
                &dbcx
            SELECT
                id : i32,
                name: String,
                email: String,
                message: String,
            FROM
                contact
            WHERE
                id = $id
        }.await?;
        Ok(rows)
    }

}





dbrock::function!{
    name = MySuperDuperFunction, 
    params = {
        id: i32
    }
    returns = {
        id: i32,
        name: String,
        email: String,
        message: String,
    }
    plpgsql = {
        FOO() IF BAR() else ()
    }
};



dbrock::transacton!{
    // bans any .await calls
};




```

