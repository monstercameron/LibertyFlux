//! Parser for `radiohud.dat`: the radio-station strip configuration.
//!
//! Two variants exist. The common variant holds a texture-container list
//! between two `+` marker lines — each row a container path plus a flag
//! word — followed by one row per station: radio id, hash name, monochrome
//! and colour texture names, visible width, and the high-definition and
//! standard-definition vertical-alignment modifiers. The platform variant has
//! no markers; each row is a station name, monochrome and colour texture
//! names, and a container name.

use crate::dat::{DataError, data_file_lines};

/// Either shape of `radiohud.dat`, detected by the `+` markers.
#[derive(Debug, Clone)]
pub enum RadioHudFile {
    /// The `+`-delimited common variant.
    Full(RadioHud),
    /// The marker-less platform variant: one row per station.
    Simple(Vec<SimpleStation>),
}

/// One row of the platform `radiohud.dat` variant.
#[derive(Debug, Clone)]
pub struct SimpleStation {
    /// Station name, e.g. `beat`.
    pub name: String,
    /// Monochrome logo texture name.
    pub mono_texture: String,
    /// Colour logo texture name.
    pub colour_texture: String,
    /// Texture container name.
    pub container: String,
}

impl RadioHudFile {
    /// Parse either `radiohud.dat` variant from its bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(data: &[u8]) -> Result<Self, DataError> {
        let lines = data_file_lines(data)?;
        if lines.iter().any(|l| l.text == "+") {
            return Ok(Self::Full(RadioHud::parse(data)?));
        }
        let mut stations = Vec::new();
        for item in &lines {
            let mut parts = item.text.split_whitespace();
            let mut get = || parts.next().ok_or(DataError::BadLine { line: item.line });
            let station = SimpleStation {
                name: get()?.to_owned(),
                mono_texture: get()?.to_owned(),
                colour_texture: get()?.to_owned(),
                container: get()?.to_owned(),
            };
            if parts.next().is_some() {
                return Err(DataError::BadLine { line: item.line });
            }
            stations.push(station);
        }
        Ok(Self::Simple(stations))
    }
}

/// A parsed `radiohud.dat` file.
#[derive(Debug, Clone)]
pub struct RadioHud {
    /// Texture containers in file order.
    pub containers: Vec<RadioContainer>,
    /// Stations in file order.
    pub stations: Vec<RadioStation>,
}

/// One texture-container row.
#[derive(Debug, Clone)]
pub struct RadioContainer {
    /// Container path, e.g. `platform:/textures/radio_hud_colored`.
    pub path: String,
    /// Flag word carried after the path.
    pub flag: u32,
}

/// One radio-station row.
#[derive(Debug, Clone)]
pub struct RadioStation {
    /// Radio id, e.g. `RADIO_0`.
    pub id: String,
    /// Name hashed to identify the station, e.g. `BEAT_95`.
    pub hash_name: String,
    /// Monochrome logo texture name.
    pub mono_texture: String,
    /// Colour logo texture name.
    pub colour_texture: String,
    /// Visible width of the texture.
    pub visible_width: u32,
    /// Vertical-alignment modifier for high-definition display.
    pub y_hd: f32,
    /// Vertical-alignment modifier for standard display.
    pub y_sd: f32,
}

impl RadioHud {
    /// Parse a `radiohud.dat` file from its bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(data: &[u8]) -> Result<Self, DataError> {
        let lines = data_file_lines(data)?;
        let mut positions = lines.iter().filter(|l| l.text == "+");
        let first = positions.next().ok_or(DataError::BadLine {
            line: lines.first().map_or(1, |l| l.line),
        })?;
        let second = positions
            .next()
            .ok_or(DataError::BadLine { line: first.line })?;
        let mut containers = Vec::new();
        let mut stations = Vec::new();
        let mut in_containers = false;
        for item in &lines {
            if item.line == first.line {
                in_containers = true;
                continue;
            }
            if item.line == second.line {
                in_containers = false;
                continue;
            }
            if in_containers {
                let mut parts = item.text.split_whitespace();
                let path = parts.next().ok_or(DataError::BadLine { line: item.line })?;
                let flag = parts.next().ok_or(DataError::BadLine { line: item.line })?;
                if parts.next().is_some() {
                    return Err(DataError::BadLine { line: item.line });
                }
                containers.push(RadioContainer {
                    path: path.to_owned(),
                    flag: flag
                        .parse::<u32>()
                        .map_err(|_| DataError::BadLine { line: item.line })?,
                });
            } else if item.line > second.line {
                let mut parts = item.text.split_whitespace();
                let line = item.line;
                let mut get = || parts.next().ok_or(DataError::BadLine { line });
                let station = RadioStation {
                    id: get()?.to_owned(),
                    hash_name: get()?.to_owned(),
                    mono_texture: get()?.to_owned(),
                    colour_texture: get()?.to_owned(),
                    visible_width: get()?
                        .parse::<u32>()
                        .map_err(|_| DataError::BadLine { line })?,
                    y_hd: get()?
                        .parse::<f32>()
                        .map_err(|_| DataError::BadLine { line })?,
                    y_sd: get()?
                        .parse::<f32>()
                        .map_err(|_| DataError::BadLine { line })?,
                };
                if parts.next().is_some() {
                    return Err(DataError::BadLine { line: item.line });
                }
                stations.push(station);
            }
        }
        Ok(Self {
            containers,
            stations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_radiohud() {
        let input = b"# c\n+\nplatform:/textures/a 1\n+\nRADIO_0 BEAT_95 beat_bw beat_col 74 -0.004 -0.028\n";
        let file = RadioHud::parse(input).unwrap();
        assert_eq!(file.containers.len(), 1);
        assert_eq!(file.containers[0].path, "platform:/textures/a");
        assert_eq!(file.containers[0].flag, 1);
        assert_eq!(file.stations.len(), 1);
        let station = &file.stations[0];
        assert_eq!(station.id, "RADIO_0");
        assert_eq!(station.visible_width, 74);
    }

    #[test]
    fn rejects_missing_markers() {
        let err = RadioHud::parse(b"RADIO_0 A B C 1 2 3\n").unwrap_err();
        assert!(matches!(err, DataError::BadLine { .. }));
    }
}
