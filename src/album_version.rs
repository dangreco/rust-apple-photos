use crate::apple_photos::Fromable;
use rusqlite::Result;
use rusqlite::Row;

#[derive(Debug)]
pub struct AlbumVersion {
    pub model_id: i32,
    pub version_id: i32,
    pub album_id: i32,
}

impl AlbumVersion {
    pub fn new(model_id: i32, version_id: i32, album_id: i32) -> AlbumVersion {
        AlbumVersion {
            model_id,
            version_id,
            album_id,
        }
    }

    pub fn from(row: &Row) -> Result<AlbumVersion> {
        Ok(AlbumVersion::new(row.get(0)?, row.get(1)?, row.get(2)?))
    }
}

impl Fromable<AlbumVersion> for AlbumVersion {
    fn from(row: &Row) -> Result<AlbumVersion> {
        return AlbumVersion::from(row);
    }
}
