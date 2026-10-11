//! Text path graph (`paths*.ipl`): `vnod` nodes and `link` edges.
//!
//! Each file is parsed in its own index space; concatenation offsets are the
//! caller's job. Lines that do not parse are skipped, since the files carry
//! map comments and unrelated sections.

/// One vehicle node: position plus street hash.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vnod {
    /// World position in metres.
    pub x: f32,
    /// World position in metres.
    pub y: f32,
    /// World position in metres.
    pub z: f32,
    /// Street-name hash (column 9, 0 when absent).
    pub street: i32,
}

/// One undirected edge between two node indices of the same file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IplLink {
    /// First endpoint (per-file index).
    pub a: u32,
    /// Second endpoint (per-file index).
    pub b: u32,
    /// Lane count (column 3).
    pub lanes: u32,
    /// Columns 2, 4, 5 (meanings not established).
    pub extra: [i32; 3],
}

/// Parsed `vnod` and `link` sections of one file.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IplPaths {
    /// Nodes in file order.
    pub nodes: Vec<Vnod>,
    /// Links in file order.
    pub links: Vec<IplLink>,
    /// Data lines skipped as malformed.
    pub skipped: u32,
}

impl IplPaths {
    /// Parse the text of one `paths*.ipl` file.
    pub fn parse(text: &str) -> Self {
        let mut out = Self::default();
        let mut section = "";
        for raw in text.lines() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line == "end" {
                section = "";
                continue;
            }
            let fields: Vec<&str> = line.split(',').map(str::trim).collect();
            if fields.len() < 2 {
                section = line;
                continue;
            }
            match section {
                "vnod" => {
                    if fields.len() < 3 {
                        out.skipped += 1;
                        continue;
                    }
                    let nums: Vec<f32> = fields[..3]
                        .iter()
                        .map(|f| f.parse().unwrap_or(f32::NAN))
                        .collect();
                    if nums.iter().any(|v| !v.is_finite()) {
                        out.skipped += 1;
                        continue;
                    }
                    let street = fields.get(9).and_then(|f| f.parse().ok()).unwrap_or(0);
                    out.nodes.push(Vnod {
                        x: nums[0],
                        y: nums[1],
                        z: nums[2],
                        street,
                    });
                }
                "link" => {
                    let a: Result<u32, _> = fields[0].parse();
                    let b: Result<u32, _> = fields[1].parse();
                    match (a, b) {
                        (Ok(a), Ok(b)) => {
                            let lanes = fields.get(3).and_then(|f| f.parse().ok()).unwrap_or(1);
                            let extra = [
                                fields.get(2).and_then(|f| f.parse().ok()).unwrap_or(0),
                                fields.get(4).and_then(|f| f.parse().ok()).unwrap_or(0),
                                fields.get(5).and_then(|f| f.parse().ok()).unwrap_or(0),
                            ];
                            out.links.push(IplLink { a, b, lanes, extra });
                        }
                        _ => out.skipped += 1,
                    }
                }
                _ => {}
            }
        }
        out
    }

    /// Links whose endpoints both exist (and differ).
    pub fn valid_links(&self) -> impl Iterator<Item = &IplLink> {
        let n = u32::try_from(self.nodes.len()).unwrap_or(u32::MAX);
        self.links
            .iter()
            .filter(move |l| l.a != l.b && l.a < n && l.b < n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "# comment\ninst\nend\nvnod\n1.5, 2.5, 3.5, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0\n\
         4.0, 5.0, 6.0\nbad, line, here\nlink\n0, 1, 0, 2, 0, 0\n1, 9, 0, 1, 0, 0\nend\n";

    #[test]
    fn parses_sample() {
        let p = IplPaths::parse(SAMPLE);
        assert_eq!(p.nodes.len(), 2);
        assert_eq!(p.nodes[0].street, 42);
        assert_eq!(p.links.len(), 2);
        assert_eq!(p.valid_links().count(), 1);
        assert_eq!(p.skipped, 1);
    }
}
