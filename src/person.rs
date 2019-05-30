use crate::apple_photos::Fromable;
use rusqlite::{Result, Row};

#[derive(Debug)]
pub struct Person {
    pub model_id: i32,
    pub uuid: String,
    pub name: String,
    pub cloud_library_state: i32,
    pub has_been_synced: i32,
    pub cloud_identifier: String,
    pub is_cloud_quarantined: i32,
    pub representative_face_id: i32,
    pub manual_order: i64,
    pub person_type: i32,
    pub person_uri: String,
    pub is_in_person_naming_model: i32,
    pub is_verified: i32,
    pub associated_cluster_id: i32,
    pub face_count: i32,
    pub display_name: String,
    pub verified_type: i32,
    pub merge_target_person_id: i32,
    pub pending_merge_target_person: String,
}

impl Person {
    pub fn new(
        model_id: i32,
        uuid: String,
        name: String,
        cloud_library_state: i32,
        has_been_synced: i32,
        cloud_identifier: String,
        is_cloud_quarantined: i32,
        representative_face_id: i32,
        manual_order: i64,
        person_type: i32,
        person_uri: String,
        is_in_person_naming_model: i32,
        is_verified: i32,
        associated_cluster_id: i32,
        face_count: i32,
        display_name: String,
        verified_type: i32,
        merge_target_person_id: i32,
        pending_merge_target_person: String,
    ) -> Person {
        Person {
            model_id,
            uuid,
            name,
            cloud_library_state,
            has_been_synced,
            cloud_identifier,
            is_cloud_quarantined,
            representative_face_id,
            manual_order,
            person_type,
            person_uri,
            is_in_person_naming_model,
            is_verified,
            associated_cluster_id,
            face_count,
            display_name,
            verified_type,
            merge_target_person_id,
            pending_merge_target_person,
        }
    }

    pub fn from(row: &Row) -> Result<Person> {
        Ok(Person::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2).unwrap_or("".to_string()),
            row.get(3)?,
            row.get(4)?,
            row.get(5).unwrap_or("".to_string()),
            row.get(6)?,
            row.get(7).unwrap_or(-1),
            row.get(8)?,
            row.get(9)?,
            row.get(10).unwrap_or("".to_string()),
            row.get(11).unwrap_or(-1),
            row.get(12)?,
            row.get(13).unwrap_or(-1),
            row.get(14)?,
            row.get(15).unwrap_or("".to_string()),
            row.get(16)?,
            row.get(17).unwrap_or(-1),
            row.get(18).unwrap_or("".to_string()),
        ))
    }
}

impl Fromable<Person> for Person {
    fn from(row: &Row) -> Result<Person> {
        return Person::from(row);
    }
}
