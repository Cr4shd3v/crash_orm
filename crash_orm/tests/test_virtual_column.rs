use crash_orm::prelude::{Entity, EntityCreateVec, EqualQueryColumn, RoundVirtualColumn, Schema, SqrtVirtualColumn, StringVirtualColumn};
use crash_orm_test::setup_test_connection;

#[derive(Entity, Debug, Schema)]
pub struct TestItem15 {
    pub id: u32,
    pub name1: Option<String>,
    pub active: bool,
    pub number: Option<i32>,
    pub decimal: f32,
    pub sqrt: f64,
}

impl TestItem15Create {
    fn test() -> Self {
        Self {
            name1: Some(String::from("Test1234")),
            active: false,
            number: Some(441),
            decimal: 1.5,
            sqrt: 16.0,
        }
    }

    fn test2() -> Self {
        Self {
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

    vec![TestItem15Create::test(), TestItem15Create::test2()]
        .insert_all(&conn)
        .await
        .unwrap();

    let results = TestItem15::query()
        .condition(TestItem15Column::NAME1.length().equals(7))
        .fetch(&conn)
        .await;
    println!("{:?}", results);
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(
            TestItem15Column::NAME1
                .lowercase()
                .equals("test1234"),
        )
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(
            TestItem15Column::NAME1
                .uppercase()
                .equals("TEST123"),
        )
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(
            TestItem15Column::NAME1
                .reverse()
                .equals("321tset"),
        )
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(
            TestItem15Column::NAME1
                .repeat(2)
                .equals("test123test123"),
        )
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(
            TestItem15Column::NAME1
                .concat(vec![&TestItem15Column::ID])
                .equals("test1232"),
        )
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(
            TestItem15Column::NAME1
                .md5()
                .equals("cc03e747a6afbbcbf8be7668acfebee5"),
        )
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    let results = TestItem15::query()
        .condition(TestItem15Column::DECIMAL.ceil().equals(1.0))
        .fetch(&conn)
        .await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].active);

    let results = TestItem15::query()
        .condition(TestItem15Column::DECIMAL.floor().equals(0.0))
        .fetch(&conn)
        .await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].active);

    let results = TestItem15::query()
        .condition(TestItem15Column::DECIMAL.round().equals(2.0))
        .fetch(&conn)
        .await;
    println!("{:?}", results);
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 1);
    assert!(!results[0].active);

    let results = TestItem15::query()
        .condition(TestItem15Column::SQRT.sqrt().equals(4.0))
        .fetch(&conn)
        .await;
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 1);

    assert!(TestItem15::drop_table(&conn).await.is_ok());
}
