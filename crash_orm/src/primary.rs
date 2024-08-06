//! Contains the [PrimaryKeyType] trait.

use tokio_postgres::types::ToSql;

/// Trait marking a type as a primary key.
///
/// This trait **requires** for all implementations [Sync], [Send], [ToSql] and [FromSql](tokio_postgres::types::FromSql).
///
/// The trait is already implemented for [u32], [i32], [i64] and [Uuid](uuid::Uuid) (if the with-uuid feature is active)
pub trait PrimaryKeyType: Sync + Send + ToSql + 'static {}

impl PrimaryKeyType for u32 {}
impl PrimaryKeyType for i32 {}
impl PrimaryKeyType for i64 {}

#[cfg(feature = "uuid")]
impl PrimaryKeyType for uuid::Uuid {}

/// Trait marking a type as a primary key.
///
/// This trait is implemented for tuples of [PrimaryKeyType].
pub trait PrimaryKey: Sync + Send + 'static {}

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: PrimaryKeyType),+> PrimaryKey for ($($name,)+) {}
    };
}

impl<P: PrimaryKeyType> PrimaryKey for P {}

// Up to 16 columns can be composited to a primary key
tuple_impls!(P1);
tuple_impls!(P1 P2);
tuple_impls!(P1 P2 P3);
tuple_impls!(P1 P2 P3 P4);
tuple_impls!(P1 P2 P3 P4 P5);
tuple_impls!(P1 P2 P3 P4 P5 P6);
tuple_impls!(P1 P2 P3 P4 P5 P6 P7);
tuple_impls!(P1 P2 P3 P4 P5 P6 P7 P8);
tuple_impls!(P1 P2 P3 P4 P5 P6 P7 P8 P9);
tuple_impls!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10);
tuple_impls!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11);
tuple_impls!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12);
tuple_impls!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13);
tuple_impls!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14);
tuple_impls!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 P15);
tuple_impls!(P1 P2 P3 P4 P5 P6 P7 P8 P9 P10 P11 P12 P13 P14 P15 P16);