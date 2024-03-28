pub use sea_orm_migration::prelude::*;

mod utils;

mod m20220101_000001_init_;
mod m20240216_201704_seed_init_;
mod m20240224_175322_status_index_;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    // #[cfg(debug_assertions)]
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_init_::Migration),
            Box::new(m20240216_201704_seed_init_::Migration),
            Box::new(m20240224_175322_status_index_::Migration),
        ]
    }
    //
    // #[cfg(not(debug_assertions))]
    // fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    //     vec![
    //         Box::new(m20220101_000001_init_::Migration),
    //         Box::new(m20240224_175322_status_index_::Migration),
    //     ]
    // }
}
