# Query
Crash ORM has a unique query API to provide only suitable functions in query.

For example, a nullable column needs the condition is_null and is_not_null. 
But those function won't appear on not nullable columns.

In short, you can't produce type errors in your queries, unless there is a bug in the ORM.

## Base Query
Every Query on every entity has some basic functions. 
In general, all queries are based on the same functions.

### Creating a Query
Queries can only be created at an entity.

Example:

```rust
let mut query = TestEntity::query();
```

A query has the entity as generic type.
That means, that all the following parameters must be from this entity.

### Set condition
Once you created a query, you might want to add a condition. 
This will be translated to the WHERE clause in Postgres.

Example:

```rust
// WHERE id = 1
let mut query = TestEntity::query()
    .condition(TestEntityColumn::ID.equals(&1));
```

A detailed documentation of all available QueryConditions can be found [here](../../crash_orm/src/query_condition.rs).

### Set Order
You can also set multiple orders for the query.
This will be translated to ORDER BYs.

Example:

```rust
let mut query = TestEntity::query()
    .order(&TestEntityColumn::ID, OrderDirection::DESC)
    .add_order(&TestEntityColumn::TEST, OrderDirection::ASC);
```

### Execute Query
When you are done building the query, you can finally execute it.

```rust
let results: Vec<TestEntity> = TestEntity::query()
    .condition(TestEntityColumn::ID.equals(&1))
    .execute(&conn).await?;
```


