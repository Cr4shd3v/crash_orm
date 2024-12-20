# v0.7.2
- fix: fixed escaped column names in column struct

# v0.7.1
- fix: removed println! from EntityCreateVec

# v0.7.0
- feat!: primary keys are no longer options
- feat: added delete query for bulk removal
- feat!: removed insert_get_id, insert_set_id, now only insert
- feat: added create structs for each entity for better creation
- feat: auto set uuid per feature flag
- feat: change json db type to jsonb
- feat: Entity::remove no longer needs mutable access
- feat(migration): added down methods for migration manager
- feat(rocket): added rocket integration crate
  - initialize database connection
  - run migrations
  - generate rocket routes for crud operations

# v0.6.3
- fix: fix not escaped update statement

# v0.6.2
- fix: fix wrong update statement

# v0.6.1
- fix: allow reserved keywords as entity names (#90)

# v0.6.0
- feat: added serde support for relation types (#91)

# v0.5.1
- fix: fix other type of foreign key than u32 table creation
- fix: gated geo macro in equal condition behind geo types feature

# v0.5.0
- feat: Allow &str for string comparisons (#88)
- feat: QueryCondition to trait based system (#87)
- feat: Primary key dependent methods into PrimaryKeyEntity (#89)
- feat: ColumnType trait
- feat: Removed unneeded generic from entity
- feat(migration): Removed unneeded generic from migration manager
- feat(schema): edit_column now returns result
- feat: Added ResultMapping to queries
- feat: relation functions only accept correct type
- perf: count by primary key
- feat: Improved order function, order direction cloneable
- feat: Reworked entity_column implementation for count, avg, sum, min, max to virtual columns
- feat: moved Sync constraint to entity
- feat: SingleResult struct for simple result mapping
- feat(migration): get_name now returns &str
- feat: derive result mapping for custom structs

# v0.4.2
- doc: fix doc not building

# v0.4.1
- doc: fix migration doc
- doc: added missing docs on new public modules
- doc: fixed some broken links in doc comments

# v0.4.0
- breaking: no public uses in crash_orm, moved to crash_orm::prelude
  - this means, that all features can be imported by `use crash_orm::prelude::*;`
- feat: repeat on strings now uses `IntoSql<i32>`
- updated tests
- updated dependencies

# v0.3.0
- feat: fetch single result
- feat: add eui48 crate support
- feat: add bit-vec crate support
- feat: add time crate support
- feat: geo-types crate support
- feat: Schemabuilder for migrations

# v0.2.1
- fixed documentation link on crates.io

# v0.2.0
- feat: Uuid, i32, i64 as primary keys (#70)
- feat: owned values now possible in query condition
- feat: Batch execute persist_all / insert_all (#60)
- feat: Move migrations into crash_orm crate behind "migration" feature (#74)
- doc: doc is now in rust (#71)
- test: Automated testing before release (#68)
- fix: entity without any fields besides an id fail to compile (#72)

# v0.1.5
- fix create table sql

# v0.1.4
- added create_table_if_not_exists

# v0.1.3
- fixed wrong crate version

# v0.1.2
- fixed bug with derives not detecting datetime/json in entity

# v0.1.1
- added orm keyword
- added changelog

# v0.1.0
- initial release




