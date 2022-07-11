use std::path::Path;
pub mod errors;

use std::ops::{Deref, DerefMut};
#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    /// The time in utc
    pub time: i64,
    /// The time offset that must be added to the time to make it the local time of the place it was
    /// captured
    pub offset: Option<i64>,
    /// Mili seconds (if it exists)
    pub ms: Option<i64>,
}

impl DateTime {
    pub fn to_utc(&self) -> i64 {
        self.time
    }
    pub fn to_original(&self) -> i64 {
        self.time + self.offset.unwrap_or(0)
    }
    // fn to_local(&self) -> i64 {
    //     todo!()
    // }
    pub fn as_string(&self) -> String {
        chrono::NaiveDateTime::from_timestamp(self.to_original(), 0).to_string()
    }

    pub fn from_string(date: impl AsRef<str>) -> Result<Self, errors::Error> {
        let date_offset = xmp::time::timestamp_offset(date.as_ref()).ok_or_else(NOT_FOUND)?;
        Ok(DateTime {
            time: date_offset.0,
            offset: date_offset.1,
            ms: None,
        })
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
impl std::fmt::Display for DateTimeOriginal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
impl std::fmt::Display for CreateDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

const DTO_NOT_FOUND: fn() -> std::io::Error =
    || std::io::Error::new(std::io::ErrorKind::NotFound, "DateTimeOriginal");
const CD_NOT_FOUND: fn() -> std::io::Error =
    || std::io::Error::new(std::io::ErrorKind::NotFound, "CreateDate");
const NOT_FOUND: fn() -> std::io::Error =
    || std::io::Error::new(std::io::ErrorKind::NotFound, "DateTime");

#[derive(Debug, Clone, Copy)]
pub struct DateTimeOriginal(pub DateTime);
#[derive(Debug, Clone, Copy)]
pub struct CreateDate(pub DateTime);

macro_rules! impl_deref_datetime {
    ($date:ty) => {
        impl Deref for $date {
            type Target = DateTime;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl DerefMut for $date {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

impl_deref_datetime!(DateTimeOriginal);
impl_deref_datetime!(CreateDate);

pub trait FromRaw: Sized {
    type Error;
    fn from_raw(path: impl AsRef<Path>) -> Result<Self, Self::Error>;
}
pub trait FromExif: Sized {
    type Error;
    fn from_exif(path: impl AsRef<Path>) -> Result<Self, Self::Error>;
}
pub trait FromXmp: Sized {
    type Error;
    fn from_xmp(path: impl AsRef<Path>) -> Result<Self, Self::Error>;
}

impl FromXmp for DateTimeOriginal {
    type Error = errors::Error;
    fn from_xmp(path: impl AsRef<Path>) -> Result<Self, Self::Error> {
        let x = xmp::try_load_element(std::io::BufReader::new(std::fs::File::open(path)?))?;
        let d = xmp::try_get_description(&x)?;

        let date_str = xmp::try_get_item(d, xmp::EXIF_DATETIMEORIGINAL)?;
        let date_offset = xmp::time::timestamp_offset(date_str).ok_or_else(DTO_NOT_FOUND)?;
        Ok(Self(DateTime {
            time: date_offset.0,
            offset: date_offset.1,
            ms: None,
        }))
    }
}

impl FromXmp for CreateDate {
    type Error = errors::Error;
    fn from_xmp(path: impl AsRef<Path>) -> Result<Self, Self::Error> {
        let x = xmp::try_load_element(std::io::BufReader::new(std::fs::File::open(path)?))?;
        let d = xmp::try_get_description(&x)?;

        let date_str = xmp::try_get_item(d, xmp::XMP_CREATEDATE)?;
        let date_offset = xmp::time::timestamp_offset(date_str).ok_or_else(CD_NOT_FOUND)?;
        Ok(Self(DateTime {
            time: date_offset.0,
            offset: date_offset.1,
            ms: None,
        }))
    }
}

impl FromExif for CreateDate {
    type Error = errors::Error;
    fn from_exif(path: impl AsRef<Path>) -> Result<Self, Self::Error> {
        let file = std::fs::File::open(path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;
        let date_str = exif
            .fields()
            .find(|f| f.tag.1 == 0x0132)
            .ok_or_else(CD_NOT_FOUND)?
            .display_value()
            .to_string();

        let date = chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")?;

        Ok(Self(DateTime {
            time: date.timestamp(),
            offset: None,
            ms: None,
        }))
    }
}

impl FromExif for DateTimeOriginal {
    type Error = errors::Error;
    fn from_exif(path: impl AsRef<Path>) -> Result<Self, Self::Error> {
        let file = std::fs::File::open(path)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;
        let date_str = exif
            .fields()
            .find(|f| f.tag.1 == 0x9003)
            .ok_or_else(DTO_NOT_FOUND)?
            .display_value()
            .to_string();

        // let date_offset = xmp::time::timestamp_offset(&date_str).ok_or_else(NOT_FOUND)?;
        let date = chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")?;
        Ok(Self(DateTime {
            time: date.timestamp(),
            offset: None,
            ms: None,
        }))
    }
}

impl FromRaw for DateTimeOriginal {
    type Error = errors::Error;
    fn from_raw(path: impl AsRef<Path>) -> Result<Self, Self::Error> {
        let mut p = libraw_r::Processor::default();
        p.open(path)?;
        let xml = p.xmpdata()?;
        let x = xmp::try_load_element(std::io::Cursor::new(xml))?;
        let d = xmp::try_get_description(&x)?;
        let date_str = xmp::try_get_item(d, xmp::EXIF_DATETIMEORIGINAL)?;
        let date_offset = xmp::time::timestamp_offset(date_str).ok_or_else(DTO_NOT_FOUND)?;
        Ok(Self(DateTime {
            time: date_offset.0,
            offset: date_offset.1,
            ms: None,
        }))
    }
}

impl FromRaw for CreateDate {
    type Error = errors::Error;
    fn from_raw(path: impl AsRef<Path>) -> Result<Self, Self::Error> {
        use libraw_r::traits::LRString;
        let mut p = libraw_r::Processor::default();
        p.open(&path)?;
        let xml = p.xmpdata()?;
        let x = xmp::try_load_element(std::io::Cursor::new(xml))?;
        let d = xmp::try_get_description(&x)?;
        let date_str = xmp::try_get_item(d, xmp::XMP_CREATEDATE).unwrap_or_default();
        let sony_time = if is_arw(&path) {
            sony_time(p.makernotes().sony.SonyDateTime.as_ascii()).unwrap_or_default()
        } else {
            0
        };
        let date_offset = xmp::time::timestamp_offset(date_str)
            .ok_or_else(CD_NOT_FOUND)
            .unwrap_or((
                if sony_time != 0 {
                    sony_time
                } else {
                    p.imgother().timestamp - chrono::Local::now().offset().utc_minus_local() as i64
                },
                None,
            ));
        Ok(Self(DateTime {
            time: date_offset.0,
            offset: date_offset.1,
            ms: None,
        }))
    }
}

impl DateTimeOriginal {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, errors::Error> {
        if let Ok(s) = Self::from_xmp(path.as_ref().with_extension("xmp")) {
            Ok(s)
        } else if let Ok(s) = Self::from_exif(path.as_ref()) {
            Ok(s)
        } else if let Ok(s) = Self::from_raw(path) {
            Ok(s)
        } else {
            Err(DTO_NOT_FOUND())?
        }
    }
}

impl CreateDate {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, errors::Error> {
        if let Ok(s) = Self::from_xmp(path.as_ref().with_extension("xmp")) {
            Ok(s)
        } else if let Ok(s) = Self::from_exif(path.as_ref()) {
            Ok(s)
        } else if let Ok(s) = Self::from_raw(path) {
            Ok(s)
        } else {
            Err(CD_NOT_FOUND())?
        }
    }
}

fn is_arw<P: AsRef<Path>>(path: P) -> bool {
    let extension = path
        .as_ref()
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(str::to_lowercase);
    let extension = extension.as_deref();
    Some("arw") == extension
}

pub fn sony_time(date: impl AsRef<str>) -> Option<i64> {
    Some(
        chrono::NaiveDateTime::parse_from_str(date.as_ref(), "%Y:%m:%d %H:%M:%S")
            .ok()?
            .timestamp(),
    )
}

pub fn get_timestamp(path: impl AsRef<Path>) -> Result<i64, errors::Error> {
    Ok(if let Ok(t) = DateTimeOriginal::from_file(&path) {
        t.to_original()
    } else {
        CreateDate::from_file(&path)?.to_original()
    })
}
