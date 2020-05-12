pub mod raw {
    use std::{collections::HashMap, fs::File, io::prelude::*, path::Path, str::FromStr};

    pub enum Error {
        Json(serde_json::Error),
        Io(std::io::Error),
    }

    #[derive(serde::Deserialize)]
    pub struct Rect {
        pub x: f32,
        pub y: f32,
        pub width: f32,
        pub height: f32,
    }

    #[derive(serde::Deserialize)]
    pub struct Point {
        pub x: f32,
        pub y: f32,
    }

    #[derive(serde::Deserialize)]
    pub struct Size {
        pub w: f32,
        pub h: f32,
    }

    #[derive(serde::Deserialize)]
    pub struct Meta {
        pub image: String,
        pub size: Size,
        pub scale: f32,
    }

    #[derive(serde::Deserialize)]
    pub struct Sprite {
        pub frame: Rect,
        pub rotated: bool,
        pub trimmed: bool,
        pub sprite_source_size: Rect,
        pub source_size: Size,
        pub anchor: Point,
    }

    #[derive(serde::Deserialize)]
    pub struct Sheet {
        pub meta: Meta,
        pub frames: HashMap<String, Sprite>,
        pub animations: HashMap<String, Vec<String>>,
    }

    impl FromStr for Sheet {
        type Err = serde_json::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            serde_json::from_str(s)
        }
    }

    impl Sheet {
        pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
            let rdr = File::open(path).map_err(Error::Io)?;
            Self::from_reader(rdr).map_err(Error::Json)
        }

        pub fn from_reader<R: Read>(rdr: R) -> serde_json::Result<Self> {
            serde_json::from_reader(rdr)
        }

        pub fn from_slice(slice: &[u8]) -> serde_json::Result<Self> {
            serde_json::from_slice(slice)
        }

        pub fn from_value(value: serde_json::Value) -> serde_json::Result<Self> {
            serde_json::from_value(value)
        }
    }
}
