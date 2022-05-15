use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use libycresources::common::types::errors::Error;
use libycresources::common::types::models::Identifier;
use libycresources::common::types::models::prototype::Kind;
use libycresources::formats::map::parse::PrototypeProvider;
use libycresources::formats::pro;
use libycresources::formats::pro::Prototype;

pub struct Provider<'a> {
    pub directory: &'a Path,
}

impl PrototypeProvider for Provider<'_> {
    fn provide(&self, identifier: &Identifier<Kind>) -> Result<Prototype, Error> {
        let kind = match identifier.kind {
            Kind::Item => "ITEMS",
            Kind::Critter => "CRITTERS",
            Kind::Scenery => "SCENERY",
            Kind::Wall => "WALLS",
            Kind::Tile => "TILES",
            Kind::Misc => "MISC",
        };

        let directory = &self.directory.join(kind);
        let path = directory.join((|| -> Result<String, Error> {
            let lst = &directory.join(kind.to_owned() + ".LST");

            return BufReader::with_capacity(1 * 1024 * 1024, File::open(lst)?)
                .lines()
                .nth(identifier.value as usize - 1)
                .ok_or(Error::Format)?
                .map_err(|e| Error::Read(e));
        })()?);

        let file = File::open(&path)?;
        let mut reader = BufReader::with_capacity(1 * 1024 * 1024, file);

        Ok(pro::parse::prototype(&mut reader)?)
    }
}