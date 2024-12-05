use bit_vec::BitVec;

use crash_orm::prelude::*;
use crash_orm_test::{default_create_table, setup_test_connection};

#[derive(Entity, Debug, Schema)]
pub struct TestItemBitVec {
    id: u32,
    bit_vec: BitVec,
}

#[tokio::test]
async fn test_bit_vec() {
    let conn = setup_test_connection().await;
    default_create_table!(TestItemBitVec, conn);

    let bit_vec = BitVec::from_bytes(&[1, 2, 3]);

    TestItemBitVecCreate {
        bit_vec: bit_vec.clone(),
    }.insert(&conn).await.unwrap();

    let result = TestItemBitVec::query()
        .condition(TestItemBitVecColumn::BIT_VEC.equals(bit_vec))
        .fetch_single(&conn).await;
    assert!(result.is_ok());

    TestItemBitVec::drop_table(&conn).await.unwrap();
}