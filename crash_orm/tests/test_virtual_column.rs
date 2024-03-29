use crash_orm::{
    CrashOrmDatabaseConnection, Entity, EntityVec, EqualQueryColumn, RoundVirtualColumn, Schema,
    SqrtVirtualColumn, StringVirtualColumn,
};
use crash_orm_derive::{Entity, Schema};
use tokio_postgres::NoTls;

pub async fn setup_test_connection() -> CrashOrmDatabaseConnection {
    CrashOrmDatabaseConnection::new(
        "postgresql://crash_orm:postgres@localhost/crash_orm_test",
        NoTls,
    )
    .await
    .unwrap()
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem15 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub number: Option<i32>,
    pub decimal: f32,
    pub sqrt: f64,
}

impl TestItem15 {
    fn test() -> Self {
        Self {
            id: None,
            name1: Some(String::from("Test1234")),
            active: false,
            number: Some(441),
            decimal: 1.5,
            sqrt: 16.0,
        }
    }

    fn test2() -> Self {
        Self {
            id: None,
            name1: Some(String::from("test123")),
            active: true,
            number: Some(440),
            decimal: 0.4,
            sqrt: 0.0,
        }
    }
}

#[tokio::test]
async fn test_virtual_column() {
    let conn = setup_test_connection().await;

    if !TestItem15::table_exists(&conn).await.unwrap() {
        assert!(TestItem15::create_table(&conn).await.is_ok());
    } else {
        assert!(TestItem15::truncate_table(&conn).await.is_ok());
    }

    vec![TestItem15::test(), TestItem15::test2()]
        .persist_all(&conn)
        .await
        .unwrap();

    let results = TestItem15::query()
        .condition(TestItem15Column::NAME1.length().equals(&7))
        .execute(&conn)
        .await;
    println!("{:?}", results);
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(
            TestItem15Column::NAME1
                .lowercase()
                .equals(&String::from("test1234")),
        )
        .execute(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(
            TestItem15Column::NAME1
                .uppercase()
                .equals(&String::from("TEST123")),
        )
        .execute(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(
            TestItem15Column::NAME1
                .reverse()
                .equals(&String::from("321tset")),
        )
        .execute(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(
            TestItem15Column::NAME1
                .repeat(&2)
                .equals(&String::from("test123test123")),
        )
        .execute(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(
            TestItem15Column::NAME1
                .concat(vec![&TestItem15Column::ID])
                .equals(&String::from("test1232")),
        )
        .execute(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(
            TestItem15Column::NAME1
                .md5()
                .equals(&String::from("cc03e747a6afbbcbf8be7668acfebee5")),
        )
        .execute(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(TestItem15Column::DECIMAL.ceil().equals(&1.0))
        .execute(&conn)
        .await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].active);

    let results = TestItem15::query()
        .condition(TestItem15Column::DECIMAL.floor().equals(&0.0))
        .execute(&conn)
        .await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].active);

    let results = TestItem15::query()
        .condition(TestItem15Column::DECIMAL.round().equals(&2.0))
        .execute(&conn)
        .await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 1);
    assert!(!results[0].active);

    let results = TestItem15::query()
        .condition(TestItem15Column::SQRT.sqrt().equals(&4.0))
        .execute(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    assert!(TestItem15::drop_table(&conn).await.is_ok());
}
