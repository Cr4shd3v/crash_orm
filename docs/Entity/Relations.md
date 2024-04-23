# Entity Relations

Crash ORM provides a convenient API for OneToOne and OneToMany/ManyToOne relations.

Right now you have to manually construct cross-reference tables (ManyToMany relations).

## OneToOne

Declaring a OneToOne is quite simple:

```rust
#[derive(Entity, Debug, Schema)]
pub struct TestItem1 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub other: Option<OneToOne<TestItem2, u32>>, // nullable
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem2 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
}
```

This describes the owning site of the relation, this is where the id of TestItem2 is stored.

It will generate a function with the following signature for TestItem1:

```rust
async fn get_other(&self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<Option<TestItem2>>;
```

This function allows you to retrieve the linked entity.

**NOTE**: You can still create your own impl block for TestItem1, the function above will be implemented via trait.

You might want to be able to retrieve TestItem1 from an instance of TestItem2.
If you need this function, you can add the following field to TestItem2:

```rust
#[mapped_by("other")]
pub test_item_1: OneToOneRef<TestItem1>,
```

This will generate a similar function like above with the name get_test_item_1.

**NOTE**: mapped_by must contain the field name of the field which it corresponds to. 
In this case, we called the field of TestItem1 "other", so we can pass it here. 
This is **MANDATORY**.

### Full OneToOne Example
```rust
#[derive(Entity, Debug, Schema)]
pub struct TestItem1 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub other: Option<OneToOne<TestItem2, u32>>,
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem2 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    #[mapped_by("other")]
    pub test_item_1: OneToOneRef<TestItem1>,
}
```

## OneToMany/ManyToOne

For this type of relation, you must declare, once again, the owning site.
In this kind of relation the ManyToOne is the owning site.
So let's start declaring a simple ManyToOne relation:

```rust
#[derive(Entity, Debug, Schema)]
pub struct TestItem1 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub other: Option<ManyToOne<TestItem2, u32>>,
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem2 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
}
```

Once again, it will generate a function with the following signature for TestItem1:

```rust
async fn get_other(&self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<Option<TestItem2>>;
```

If you need to retrieve all TestItem1 from a TestItem2, you need to add the following field:

```rust
#[mapped_by("other")]
pub test_items_1: OneToMany<TestItem1>,
```

This will generate a slightly different function:

```rust
async fn get_test_items_1(&self, connection: &impl crash_orm::DatabaseConnection) -> crash_orm::Result<Vec<TestItem1>>;
```

**NOTE**: mapped_by must contain the field name of the field which it corresponds to.
In this case, we called the field of TestItem1 "other", so we can pass it here.
This is **MANDATORY**.

### Full OneToMany/ManyToOne Example
```rust
#[derive(Entity, Debug, Schema)]
pub struct TestItem1 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    pub other: Option<ManyToOne<TestItem2, u32>>,
}

#[derive(Entity, Debug, Schema)]
pub struct TestItem2 {
    pub id: Option<u32>,
    pub name1: Option<String>,
    pub active: bool,
    #[mapped_by("other")]
    pub test_items_1: OneToMany<TestItem1>,
}
```


