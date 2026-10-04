//! Load lists: `gta.dat`, `default.dat` and `images.txt` style files.
//!
//! `gta.dat` is the root manifest: it names `IDE` and `IPL` files in load
//! order and points at `images.txt` (the archive list) with `IMGLIST`.
//! `default.dat` binds tuning files with one keyword per file. All three share
//! one line shape: a keyword, whitespace-separated arguments, `#` comments.

use crate::Error;

/// A parsed load list: directives in file order.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoadList {
    /// Directives in file order; comments and blank lines are removed.
    pub directives: Vec<Directive>,
}

/// One `KEYWORD arg...` line.
#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    /// The keyword exactly as written (usually upper case).
    pub keyword: String,
    /// Whitespace-separated arguments.
    pub args: Vec<String>,
    /// 1-based line number.
    pub line: usize,
}

impl LoadList {
    /// Parse a load list from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(bytes: &[u8]) -> Result<LoadList, Error> {
        let text = std::str::from_utf8(bytes).map_err(|e| Error::Utf8 {
            offset: Some(e.valid_up_to()),
        })?;
        let mut directives = Vec::new();
        for (idx, raw) in text.lines().enumerate() {
            let line = match raw.find('#') {
                Some(i) => &raw[..i],
                None => raw,
            };
            let mut parts = line.split_whitespace();
            let Some(keyword) = parts.next() else {
                continue;
            };
            directives.push(Directive {
                keyword: keyword.to_string(),
                args: parts.map(str::to_string).collect(),
                line: idx + 1,
            });
        }
        Ok(LoadList { directives })
    }

    /// Arguments of every directive whose keyword matches (case-insensitive).
    #[must_use]
    pub fn args_of(&self, keyword: &str) -> Vec<&Directive> {
        self.directives
            .iter()
            .filter(|d| d.keyword.eq_ignore_ascii_case(keyword))
            .collect()
    }

    /// Paths named by `IDE` directives, in load order.
    #[must_use]
    pub fn ide_files(&self) -> Vec<&str> {
        self.args_of("IDE")
            .into_iter()
            .filter_map(|d| d.args.first().map(String::as_str))
            .collect()
    }

    /// Paths named by `IPL` directives, in load order.
    #[must_use]
    pub fn ipl_files(&self) -> Vec<&str> {
        self.args_of("IPL")
            .into_iter()
            .filter_map(|d| d.args.first().map(String::as_str))
            .collect()
    }
}

/// A parsed `images.txt`: archive paths with an optional flag word.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ImagesList {
    /// Entries in file order.
    pub entries: Vec<ImagesEntry>,
}

/// One `images.txt` line.
#[derive(Debug, Clone, PartialEq)]
pub struct ImagesEntry {
    /// Archive path.
    pub path: String,
    /// Optional trailing flag (usually `1` when present).
    pub flag: Option<String>,
    /// 1-based line number.
    pub line: usize,
}

impl ImagesList {
    /// Parse an `images.txt` file from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(bytes: &[u8]) -> Result<ImagesList, Error> {
        let text = std::str::from_utf8(bytes).map_err(|e| Error::Utf8 {
            offset: Some(e.valid_up_to()),
        })?;
        let mut entries = Vec::new();
        for (idx, raw) in text.lines().enumerate() {
            let line = match raw.find('#') {
                Some(i) => &raw[..i],
                None => raw,
            };
            let mut parts = line.split_whitespace();
            let Some(path) = parts.next() else {
                continue;
            };
            entries.push(ImagesEntry {
                path: path.to_string(),
                flag: parts.next().map(str::to_string),
                line: idx + 1,
            });
        }
        Ok(ImagesList { entries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_gta_dat_shape() {
        let l = LoadList::parse(
            b"# comment\nIMGLIST common:/data/images.txt\nIDE common:/DATA/gtxd.IDE\nIPL platform:/DATA/MAPS/x.wpl\n",
        )
        .unwrap();
        assert_eq!(l.directives.len(), 3);
        assert_eq!(l.ide_files(), vec!["common:/DATA/gtxd.IDE"]);
        assert_eq!(l.ipl_files(), vec!["platform:/DATA/MAPS/x.wpl"]);
    }

    #[test]
    fn parses_images_txt_shape() {
        let l =
            ImagesList::parse(b"# note\nplatformimg:/anim/cuts\t\t1\ncommonimg:/data/x\n").unwrap();
        assert_eq!(l.entries.len(), 2);
        assert_eq!(l.entries[0].flag.as_deref(), Some("1"));
        assert_eq!(l.entries[1].flag, None);
    }

    #[test]
    fn invalid_utf8_errors() {
        assert!(LoadList::parse(b"IDE \xff\n").is_err());
    }
}
