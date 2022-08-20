use std::ffi::OsStr;
use std::path::Path;

pub(crate) trait MatchesExtension {
    fn matches_extension<IT: AsRef<str>, I: IntoIterator<Item = IT>>(&self, iter: I) -> bool;
}

impl<T> MatchesExtension for T
where
    T: AsRef<Path>,
{
    fn matches_extension<IT: AsRef<str>, I: IntoIterator<Item = IT>>(&self, extensions: I) -> bool {
        let extension = self
            .as_ref()
            .extension()
            .and_then(OsStr::to_str)
            .map(str::to_lowercase);
        let extension = extension.as_deref();
        extensions
            .into_iter()
            .any(|ext| Some(ext.as_ref()) == extension)
    }
}
