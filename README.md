<p align="center">
  <img width="100%" src="https://raw.githubusercontent.com/dangreco/rust-apple-photos/master/logo.png">
</p>

[![Latest version](https://img.shields.io/crates/v/rust-apple-photos.svg)](https://crates.io/crates/rust-apple-photos)
[![License](https://img.shields.io/crates/l/rust-apple-photos.svg)](https://github.com/dangreco/rust-apple-photos/blob/master/LICENSE)

A simple yet powerful library to interact with the SQLite database used by the Apple Photos app on macOS.
## Installation 
Add the following to your Cargo.toml:
```toml
[Dependencies]
rust-apple-photos = "0.1.0"
```

## Basic Usage
```rust
extern crate rust_apple_photos;
extern crate rusqlite;

use rust_apple_photos::ApplePhotos;
use rust_apple_photos::types::{ Album };

fn main() -> Result<()>
{
    /* Specify other .db path using "new_with_path" method! */
    let photos: ApplePhotos = ApplePhotos::new()?;
    let albums: Vec<Album> = photos.get_albums()?;
    
    println!("You have {} albums!", albums.len());
}
```
## Custom Queries
```rust
...

fn get_large_images() -> Result<()>
{
    ...
    let big_boys: Vec<Master> = photos.query("SELECT * FROM RKMaster WHERE width >= 2000 AND height >= 2000")?;
    println!("You have {} images that are big!", big_boys.len());
}

...
```

## Roadmap
There are a lot of models to implement to complete just the base layer of models. There are additional sub-models that will need to be implemented if the goal is to fully replace Apple's own implementation.
### Models
- [ ] RKAdminData
- [X] RKAlbum
- [X] RKAlbumVersion
- [ ] RKAttachment
- [ ] RKBookmark
- [ ] RKCloudResource
- [ ] RKCustomSortOrder
- [X] RKFace
- [ ] RKFaceCrop
- [ ] RKFaceGroup
- [ ] RKFacePrint
- [ ] RKFolder
- [ ] RKImageMask
- [ ] RKImageProxyState
- [ ] RKImportGroup
- [ ] RKImportMoment
- [ ] RKKeyword
- [X] RKMaster
- [ ] RKMemory
- [ ] RKMemoryCuratedVersion
- [ ] RKMemoryExtendedCuratedVersion
- [ ] RKMemoryMovieVersion
- [ ] RKMemoryRepresentativeVersion
- [ ] RKModelResource
- [ ] RKMoment
- [ ] RKMomentCollection
- [ ] RKMomentYear
- [X] RKPerson
- [ ] RKPersonInvalidMergeCandidatePerson
- [ ] RKPersonMergeCandidatePerson
- [ ] RKPersonRejectedFace
- [ ] RKPersonRejectedFaceNeedingFaceCrop
- [ ] RKPlace
- [ ] RKPlaceForVersion
- [ ] RKSceneInVersion
- [ ] RKVersion
- [ ] RKVersionAnalysisState
- [ ] RKVolume



## Contributions
If you want to improve anything, go ahead and open up a pull request.

## License
rust-apple-photos is available under the MIT license. See the LICENSE file for more info.
