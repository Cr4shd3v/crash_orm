# Types
The following types are valid as properties for entities.

| Rust type             | Postgres type            |
|-----------------------|--------------------------|
| bool                  | BOOL                     |
| i8                    | CHAR                     |
| i16                   | INT2                     |
| i32                   | INT4                     |
| i64                   | INT8                     |
| u32                   | OID                      |
| f32                   | FLOAT4                   |
| f64                   | FLOAT8                   |
| String                | TEXT                     |
| rust_decimal::Decimal | NUMERIC                  |
| chrono::DateTime      | TIMESTAMP WITH TIME ZONE |
| chrono::NaiveDateTime | TIMESTAMP                |
| chrono::NaiveDate     | DATE                     |
| chrono::NaiveTime     | TIME                     |
| uuid::Uuid            | UUID                     |
| serde_json::Value     | JSON                     |
| crash_orm::OneToOne   | OID (Foreign Key)        |
| crash_orm::ManyToOne  | OID (Foreign Key)        |

Those are not all valid types.
A valid type must implement ToSql and FromSql from tokio-postgres.

To make a column nullable, just put the type in an Option.