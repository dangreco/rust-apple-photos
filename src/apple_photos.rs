use crate::album::Album;
use crate::album_version::AlbumVersion;
use crate::master::Master;
use crate::person::Person;
use rusqlite::Result;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Row};

const DEFAULT_LIBRARY: &str = "~/Pictures/Photos Library.photoslibrary/database/photos.db";

pub struct ApplePhotos {
    database_location: String,
    connection: Connection,
}
pub trait Fromable<T> {
    fn from(row: &Row) -> Result<T>;
}

impl ApplePhotos {
    pub fn new_with_path(database_location: &str) -> Result<ApplePhotos> {
        let conn = Connection::open(database_location)?;
        Ok(ApplePhotos {
            database_location: database_location.to_string(),
            connection: conn,
        })
    }

    pub fn new() -> Result<ApplePhotos> {
        return ApplePhotos::new_with_path(DEFAULT_LIBRARY);
    }

    pub fn query<T: Fromable<T>>(&self, query: &str) -> Result<Vec<T>> {
        let mut stmt = self.connection.prepare(query)?;
        let rows = stmt.query_map(NO_PARAMS, |row| {
            let item: T = T::from(row)?;
            Ok(item)
        })?;
        let mut items: Vec<T> = vec![];
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }

    pub fn get_albums(&self) -> Result<Vec<Album>> {
        return self.query("SELECT * FROM RKAlbum");
    }

    pub fn get_album_versions(&self) -> Result<Vec<AlbumVersion>> {
        return self.query("SELECT * FROM RKAlbumVersion");
    }

    pub fn get_masters(&self) -> Result<Vec<Master>> {
        return self.query("SELECT * FROM RKMaster");
    }

    pub fn get_people(&self) -> Result<Vec<Person>> {
        return self.query("SELECT * FROM RKPerson");
    }
}
