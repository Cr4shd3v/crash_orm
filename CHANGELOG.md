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




