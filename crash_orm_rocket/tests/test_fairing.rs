use crash_orm::prelude::CrashOrmDatabaseConnection;
use crash_orm_rocket::CrashOrmDatabaseFairing;
use crash_orm_test::TEST_DB_URL;

#[tokio::test]
async fn test_rocket() {
    let rocket = rocket::build()
        .attach(CrashOrmDatabaseFairing::from_url(TEST_DB_URL)).ignite().await.unwrap();
    let conn = rocket.state::<CrashOrmDatabaseConnection>();
    assert!(conn.is_some());
    assert_eq!(conn.unwrap().is_closed(), false);
}

#[tokio::test]
async fn test_rocket_env() {
    unsafe { std::env::set_var("DATABASE_URL", TEST_DB_URL); }

    let rocket = rocket::build()
        .attach(CrashOrmDatabaseFairing::default()).ignite().await.unwrap();
    let conn = rocket.state::<CrashOrmDatabaseConnection>();
    assert!(conn.is_some());
    assert_eq!(conn.unwrap().is_closed(), false);
}


