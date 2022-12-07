#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ExifError(#[from] exif::Error),
    #[error("{0}")]
    XmpError(#[from] xmp::errors::XmpError),
    #[error("{0}")]
    IOError(#[from] std::io::Error),
    #[error("{0}")]
    DateTimeParseError(#[from] chrono::ParseError),
    #[error("{0}")]
    LibrawError(#[from] libraw_r::LibrawError),
    #[error("{0}")]
    ImageError(#[from] img_parts::Error),
}
