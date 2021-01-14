use serde::Deserialize;

use crate::schema::applications;

#[derive(Queryable, Deserialize, Insertable, Debug)]
#[serde(rename_all = "camelCase")]
#[table_name = "applications"]
pub struct Application {
    #[serde(skip_deserializing)]
    pub row_id: i32,
    pub minecraft_username: String,
    pub age: i32,
    pub linking_id: i64,
    pub add_one_thing: String,
    pub projects_on_biome: String,
    pub biggest_project: String,
    pub showcase: String,
    #[serde(skip_deserializing)]
    pub status: i32,
}
