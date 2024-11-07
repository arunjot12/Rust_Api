use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Blocks {
    pub id: i32,
    pub block_number: u128,
    pub timestamp:u128,
}

// #[derive(Insertable)]
// #[diesel(table_name = crate::schema::blocks)]
// pub struct NewBlocks{
//     pub block_number:i32,
//     pub timestamp: i32,
// }