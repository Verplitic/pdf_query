use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::prelude::MetadataExt;
use std::path::Path;

#[derive(Serialize, Deserialize)]
#[allow(non_camel_case_types)]
struct _profile {
    pub(crate) aliases: HashMap<String, String>,
    pub(crate) bookmarks: HashMap<String, HashMap<String, i32>>,
}

pub(crate) struct Profile {
    profile: _profile,
    file: File,
}

impl Profile {
    pub(crate) fn from<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut data = String::new();
        let mut file = File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(path)?;

        if file.metadata()?.size() == 0 {
            let mut bm = HashMap::new();
            bm.insert(String::from("__reserved__"), -1);
            let mut bookmarks = HashMap::new();
            bookmarks.insert(String::from("__reserved__"), bm);

            let mut aliases = HashMap::new();
            aliases.insert(String::from("__reserved__"), String::from("__reserved__"));

            let profile = _profile {
                aliases,
                bookmarks
            };

            return Ok(Self { profile, file });
        }

        _ = file.read_to_string(&mut data)?;
        let profile: _profile = serde_yaml::from_str(&data)?;
        Ok(Self { profile, file })
    }

    fn write_file(&mut self) -> Result<(), Box<dyn Error>> {
        let data = serde_yaml::to_string(&self.profile)?;
        self.file.set_len(0)?;
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub(crate) fn modify_alias(
        &mut self,
        key: &String,
        value: &String,
    ) -> Result<(), Box<dyn Error>> {
        self.profile.aliases.insert(key.clone(), value.clone());
        self.profile.aliases.remove("__reserved__");

        self.write_file()?;
        Ok(())
    }

    pub(crate) fn modify_bookmark(
        &mut self,
        alias: &str,
        key: &String,
        value: i32,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(bookmarks) = self.profile.bookmarks.get_mut(alias) {
            bookmarks.insert(key.clone(), value);
        } else {
            if self.find_alias(alias).is_some() {
                self.profile.bookmarks
                    .insert(String::from(alias), HashMap::new());
                self.modify_bookmark(alias, key, value)?;
            } else {
                Err(format!(
                    "Alias `{alias}` does not exist, so create it first."
                ))?;
            }
        }
        self.profile.bookmarks.remove("__reserved__");
        self.write_file()?;
        Ok(())
    }

    pub(crate) fn find_alias(&self, key: &str) -> Option<&String> {
        self.profile.aliases.get(key)
    }

    pub(crate) fn find_bookmark(&self, alias: &str, key: &str) -> Option<&i32> {
        let bookmarks = self.profile.bookmarks.get(alias);
        return if let Some(bookmark) = bookmarks {
            bookmark.get(key)
        } else {
            None
        };
    }
}
