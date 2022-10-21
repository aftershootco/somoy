use std::{collections::HashMap, path::Path};

pub fn __exif_raw(path: impl AsRef<Path>) -> Result<HashMap<i32, String>, crate::errors::Error> {
    let mut p: libraw_r::Processor = Default::default();
    p.open(path)?;
    let mut data = HashMap::new();
    unsafe {
        p.set_exifparser_callback(
            Some(libraw_r::exif::exif_parser_callback),
            std::mem::transmute(&mut data),
        )?
    };
    Ok(data)
}
