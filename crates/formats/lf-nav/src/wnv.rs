//! Pedestrian navigation mesh tiles (`.wnv`, inflated RSC5 payload).
//!
//! A tile holds a vertex array (3 x u16 quantised over the tile box), an
//! index array (u16 into the vertices), a polygon array (40 bytes each), and
//! a per-corner edge array (about 8 bytes per index) whose neighbour
//! encoding is not established.

use crate::{Error, f32le, u16le, u32le};

/// Minimum header length in bytes.
pub const HEADER_LEN: usize = 0x90;
/// One vertex in bytes.
pub const VERTEX_LEN: usize = 6;
/// One polygon in bytes.
pub const POLYGON_LEN: usize = 40;
/// One edge record in bytes.
pub const EDGE_LEN: usize = 8;
/// Mask applied to RAGE pointers to get buffer offsets.
pub const POINTER_MASK: u32 = 0x0FFF_FFFF;
/// Quantisation steps per axis.
pub const QUANTUM: f32 = 65535.0;
/// Game sector size in metres.
pub const SECTOR_METRES: f32 = 50.0;
/// Navmesh tile size in sectors per side.
pub const SECTORS_PER_TILE: u32 = 2;
/// World origin in metres.
pub const WORLD_ORIGIN: f32 = -3000.0;

/// A parsed tile borrowing its inflated bytes.
#[derive(Debug, Clone, Copy)]
pub struct Tile<'a> {
    data: &'a [u8],
    size_x: f32,
    size_y: f32,
    z_extent: f32,
    vert_base: usize,
    index_base: usize,
    poly_base: usize,
    edge_base: usize,
    tail_base: usize,
    vert_count: u32,
    index_count: u32,
    poly_count: u32,
}

impl<'a> Tile<'a> {
    /// Parse the inflated payload of one `.wnv` resource. Only bounds are
    /// checked here; content validation lives in [`Tile::validate_indices`]
    /// and [`Tile::validate_polygons`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(data: &'a [u8]) -> Result<Self, Error> {
        if data.len() < HEADER_LEN {
            return Err(Error::UnexpectedEnd { offset: 0 });
        }
        let size_x = f32le(data, 0x40)?;
        let size_y = f32le(data, 0x44)?;
        let z_extent = f32le(data, 0x48)?;
        if size_x.is_nan() || size_y.is_nan() || size_x <= 0.0 || size_y <= 0.0 {
            return Err(Error::Invalid { what: "tile size" });
        }
        let vert_base = (u32le(data, 0x58)? & POINTER_MASK) as usize;
        let index_base = (u32le(data, 0x60)? & POINTER_MASK) as usize;
        let edge_base = (u32le(data, 0x64)? & POINTER_MASK) as usize;
        let poly_base = (u32le(data, 0x6c)? & POINTER_MASK) as usize;
        let tail_base = (u32le(data, 0x70)? & POINTER_MASK) as usize;
        let index_count = u32le(data, 0x68)?;
        let vert_count = u32le(data, 0x78)?;
        let poly_count = u32le(data, 0x7c)?;
        let end = |base: usize, count: u32, size: usize| -> Option<usize> {
            base.checked_add(count as usize * size)
        };
        if end(vert_base, vert_count, VERTEX_LEN).is_none_or(|e| e > data.len()) {
            return Err(Error::BadSize {
                expected: vert_base.saturating_add(vert_count as usize * VERTEX_LEN),
                actual: data.len(),
            });
        }
        if end(index_base, index_count, 2).is_none_or(|e| e > data.len()) {
            return Err(Error::BadSize {
                expected: index_base.saturating_add(index_count as usize * 2),
                actual: data.len(),
            });
        }
        if end(poly_base, poly_count, POLYGON_LEN).is_none_or(|e| e > data.len()) {
            return Err(Error::BadSize {
                expected: poly_base.saturating_add(poly_count as usize * POLYGON_LEN),
                actual: data.len(),
            });
        }
        // The edge and tail pointers only have to aim inside the buffer.
        // The edge region runs to the next array or the end, and one shipped
        // tile aims it exactly at the end (no edge region). See `edge_region`.
        if edge_base > data.len() || tail_base > data.len() {
            return Err(Error::BadSize {
                expected: edge_base.max(tail_base),
                actual: data.len(),
            });
        }
        Ok(Self {
            data,
            size_x,
            size_y,
            z_extent,
            vert_base,
            index_base,
            poly_base,
            edge_base,
            tail_base,
            vert_count,
            index_count,
            poly_count,
        })
    }

    /// Tile size in metres along X (always 100 for sector tiles).
    #[must_use]
    pub fn size_x(&self) -> f32 {
        self.size_x
    }

    /// Tile size in metres along Y (always 100 for sector tiles).
    #[must_use]
    pub fn size_y(&self) -> f32 {
        self.size_y
    }

    /// Tile Z extent in metres (0 for open-water corners).
    #[must_use]
    pub fn z_extent(&self) -> f32 {
        self.z_extent
    }

    /// Vertex count.
    #[must_use]
    pub fn vert_count(&self) -> u32 {
        self.vert_count
    }

    /// Index count.
    #[must_use]
    pub fn index_count(&self) -> u32 {
        self.index_count
    }

    /// Polygon count.
    #[must_use]
    pub fn poly_count(&self) -> u32 {
        self.poly_count
    }

    /// Read vertex `index` as raw quantised components.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn vertex(&self, index: u32) -> Result<RawVertex, Error> {
        if index >= self.vert_count {
            return Err(Error::OutOfRange {
                what: "vertex",
                index: index as usize,
                count: self.vert_count as usize,
            });
        }
        let off = self.vert_base + index as usize * VERTEX_LEN;
        Ok(RawVertex {
            qx: u16le(self.data, off)?,
            qy: u16le(self.data, off + 2)?,
            qz: u16le(self.data, off + 4)?,
        })
    }

    /// Iterate over all vertices.
    ///
    /// # Panics
    ///
    /// Never panics: `parse` rejects files whose counted entries do not fit, so the counted reads cannot fail.
    pub fn vertices(&self) -> impl Iterator<Item = RawVertex> + '_ {
        (0..self.vert_count).map(|i| self.vertex(i).expect("count checked"))
    }

    /// Read index `i` (a vertex number).
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn index(&self, i: u32) -> Result<u16, Error> {
        if i >= self.index_count {
            return Err(Error::OutOfRange {
                what: "index",
                index: i as usize,
                count: self.index_count as usize,
            });
        }
        u16le(self.data, self.index_base + i as usize * 2)
    }

    /// Iterate over all indices.
    ///
    /// # Panics
    ///
    /// Never panics: `parse` rejects files whose counted entries do not fit, so the counted reads cannot fail.
    pub fn indices(&self) -> impl Iterator<Item = u16> + '_ {
        (0..self.index_count).map(|i| self.index(i).expect("count checked"))
    }

    /// Read polygon `i`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn polygon(&self, i: u32) -> Result<Polygon<'_>, Error> {
        if i >= self.poly_count {
            return Err(Error::OutOfRange {
                what: "polygon",
                index: i as usize,
                count: self.poly_count as usize,
            });
        }
        let off = self.poly_base + i as usize * POLYGON_LEN;
        let raw = self
            .data
            .get(off..off + POLYGON_LEN)
            .ok_or(Error::UnexpectedEnd { offset: off })?;
        Ok(Polygon { raw })
    }

    /// Iterate over all polygons.
    ///
    /// # Panics
    ///
    /// Never panics: `parse` rejects files whose counted entries do not fit, so the counted reads cannot fail.
    pub fn polygons(&self) -> impl Iterator<Item = Polygon<'_>> + '_ {
        (0..self.poly_count).map(|i| self.polygon(i).expect("count checked"))
    }

    /// Vertex count of polygon `i`: the next polygon's first index minus its
    /// own (or the index count for the last polygon). Shipped tiles give 3
    /// to 15.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn polygon_vertex_count(&self, i: u32) -> Result<u32, Error> {
        let start = u32::from(self.polygon(i)?.first_index());
        let end = if i + 1 < self.poly_count {
            u32::from(self.polygon(i + 1)?.first_index())
        } else {
            self.index_count
        };
        Ok(end.saturating_sub(start))
    }

    /// The vertex numbers used by polygon `i`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn polygon_vertices(&self, i: u32) -> Result<Vec<u16>, Error> {
        let start = u32::from(self.polygon(i)?.first_index());
        let end = if i + 1 < self.poly_count {
            u32::from(self.polygon(i + 1)?.first_index())
        } else {
            self.index_count
        };
        if end < start || end > self.index_count {
            return Err(Error::Invalid {
                what: "polygon index range",
            });
        }
        (start..end).map(|k| self.index(k)).collect()
    }

    /// Raw edge region: bytes from the edge pointer to the next array
    /// start (or the end of the buffer). The region usually sits right after
    /// the polygons, but in about a ninth of shipped tiles the arrays are
    /// ordered differently and it sits later; one shipped tile aims the
    /// pointer exactly at the end (empty region). Most regions hold about one
    /// 8-byte record per index; the records' meaning is unknown.
    #[must_use]
    pub fn edge_region(&self) -> &[u8] {
        let poly_end = self
            .poly_base
            .saturating_add(self.poly_count as usize * POLYGON_LEN);
        let mut region_end = self.data.len();
        for base in [self.vert_base, self.index_base, poly_end, self.tail_base] {
            if base > self.edge_base && base < region_end {
                region_end = base;
            }
        }
        if self.edge_base >= region_end {
            return &[];
        }
        &self.data[self.edge_base..region_end]
    }

    /// How many 8-byte edge records to read: whole records in
    /// [`Tile::edge_region`], capped at the index count.
    #[must_use]
    pub fn edge_count(&self) -> u32 {
        u32::try_from((self.edge_region().len() / EDGE_LEN).min(self.index_count as usize))
            .unwrap_or(u32::MAX)
    }

    /// Read edge record `i`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn edge_record(&self, i: u32) -> Result<EdgeRecord, Error> {
        if i >= self.edge_count() {
            return Err(Error::OutOfRange {
                what: "edge record",
                index: i as usize,
                count: self.edge_count() as usize,
            });
        }
        let off = self.edge_base + i as usize * EDGE_LEN;
        Ok(EdgeRecord {
            a: u16le(self.data, off)?,
            b: u16le(self.data, off + 2)?,
            c: u16le(self.data, off + 4)?,
            d: u16le(self.data, off + 6)?,
        })
    }

    /// Iterate over all edge records.
    ///
    /// # Panics
    ///
    /// Never panics: `parse` rejects files whose counted entries do not fit, so the counted reads cannot fail.
    pub fn edge_records(&self) -> impl Iterator<Item = EdgeRecord> + '_ {
        (0..self.edge_count()).map(|i| self.edge_record(i).expect("count checked"))
    }

    /// Decode a vertex to local tile metres in plan.
    #[must_use]
    pub fn local_xy(&self, v: RawVertex) -> (f32, f32) {
        (
            f32::from(v.qx) / QUANTUM * self.size_x,
            f32::from(v.qy) / QUANTUM * self.size_y,
        )
    }

    /// Decode a vertex height relative to the tile's unknown Z base.
    #[must_use]
    pub fn local_z_rel(&self, v: RawVertex) -> f32 {
        f32::from(v.qz) / QUANTUM * self.z_extent
    }

    /// Check every index is a valid vertex number and how many distinct
    /// vertices are referenced.
    ///
    /// # Panics
    ///
    /// Never panics: `parse` rejects files whose counted entries do not fit, so the counted reads cannot fail.
    #[must_use]
    pub fn validate_indices(&self) -> IndexReport {
        let mut report = IndexReport::default();
        let mut seen = std::collections::HashSet::new();
        for i in 0..self.index_count {
            let v = self.index(i).expect("count checked");
            if u32::from(v) >= self.vert_count {
                report.out_of_range += 1;
            } else {
                seen.insert(v);
            }
            report.max = report.max.max(v);
        }
        report.distinct = u32::try_from(seen.len()).unwrap_or(u32::MAX);
        report
    }

    /// Check polygon first indices run monotonically inside the index array.
    ///
    /// # Panics
    ///
    /// Never panics: `parse` rejects files whose counted entries do not fit, so the counted reads cannot fail.
    #[must_use]
    pub fn validate_polygons(&self) -> PolyReport {
        let mut report = PolyReport {
            min_verts: u32::MAX,
            ..Default::default()
        };
        if self.poly_count == 0 {
            report.min_verts = 0;
            return report;
        }
        let mut prev = 0u32;
        for i in 0..self.poly_count {
            let first = u32::from(self.polygon(i).expect("count checked").first_index());
            if first < prev || first > self.index_count {
                report.non_monotonic += 1;
            }
            prev = prev.max(first);
            let n = self.polygon_vertex_count(i).expect("count checked");
            report.min_verts = report.min_verts.min(n);
            report.max_verts = report.max_verts.max(n);
        }
        report
    }
}

/// One vertex as stored: three u16 components quantised over the tile box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawVertex {
    /// X quantum (0-65535 across the tile width).
    pub qx: u16,
    /// Y quantum (0-65535 across the tile depth).
    pub qy: u16,
    /// Z quantum (0-65535 across the tile Z extent, base unknown).
    pub qz: u16,
}

/// One 40-byte polygon.
///
/// Only the first-index field (u16 at +4) is understood: the polygon owns
/// indices `first_index .. next_first_index`. The rest is exposed as observed
/// words. What probing showed: +0 a small id (21 distinct values), +2 a
/// flags-like word (`0xCC00` range, steps of 32), +6 a small id (29 distinct
/// values), +8 to +15 always zero, +16 to +23 mixed small and near-`0xFFFF`
/// words, +24 to +29 small words that are NOT polygon adjacency (asymmetric
/// in every tile), +30 to +33 not a float, +34 the constant 205, +36 to +39
/// always zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Polygon<'a> {
    raw: &'a [u8],
}

impl Polygon<'_> {
    fn w(&self, at: usize) -> u16 {
        u16::from_le_bytes([self.raw[at], self.raw[at + 1]])
    }

    /// Small id at +0 (meaning unknown).
    #[must_use]
    pub fn kind(&self) -> u16 {
        self.w(0)
    }

    /// Flags-like word at +2 (meaning unknown).
    #[must_use]
    pub fn flags_a(&self) -> u16 {
        self.w(2)
    }

    /// First index into the index array.
    #[must_use]
    pub fn first_index(&self) -> u16 {
        self.w(4)
    }

    /// Small id at +6 (meaning unknown).
    #[must_use]
    pub fn aux(&self) -> u16 {
        self.w(6)
    }

    /// Words at +16, +18, +20, +22 (meaning unknown).
    #[must_use]
    pub fn edge_hint(&self) -> [u16; 4] {
        [self.w(16), self.w(18), self.w(20), self.w(22)]
    }

    /// Words at +24, +26, +28. Small like polygon ids but proven NOT to be
    /// adjacency; meaning unknown.
    #[must_use]
    pub fn refs(&self) -> [u16; 3] {
        [self.w(24), self.w(26), self.w(28)]
    }

    /// Words at +30, +32 (meaning unknown).
    #[must_use]
    pub fn data(&self) -> [u16; 2] {
        [self.w(30), self.w(32)]
    }

    /// Constant marker at +34 (always 205 in shipped tiles).
    #[must_use]
    pub fn marker(&self) -> u16 {
        self.w(34)
    }

    /// The full 40 raw bytes.
    #[must_use]
    pub fn raw(&self) -> &[u8] {
        self.raw
    }
}

/// One 8-byte per-corner edge record: four u16 words of unknown meaning.
///
/// The first word often repeats a header value, the second is usually below
/// the polygon count, the last sits in the `0xC000` range, and `0xFFFF`
/// appears as a none-marker. None of the words behaves as symmetric polygon
/// adjacency on its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeRecord {
    /// First word.
    pub a: u16,
    /// Second word (usually below the polygon count).
    pub b: u16,
    /// Third word.
    pub c: u16,
    /// Fourth word (usually in the `0xC000` range).
    pub d: u16,
}

/// Report from [`Tile::validate_indices`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IndexReport {
    /// Indices at or past the vertex count.
    pub out_of_range: u32,
    /// Highest index seen.
    pub max: u16,
    /// Distinct vertices referenced.
    pub distinct: u32,
}

/// Report from [`Tile::validate_polygons`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PolyReport {
    /// First indices running backwards or past the index count.
    pub non_monotonic: u32,
    /// Fewest vertices on any polygon.
    pub min_verts: u32,
    /// Most vertices on any polygon.
    pub max_verts: u32,
}

/// World corner (metres) of the tile with sector numbers `sx`, `sy`.
#[must_use]
// Sector ids are tiny in practice; a huge id would lose precision but stay
// finite, and only shifts a debug readout.
#[allow(clippy::cast_precision_loss)]
pub fn tile_origin(sx: u32, sy: u32) -> (f32, f32) {
    (
        sx as f32 * SECTOR_METRES + WORLD_ORIGIN,
        sy as f32 * SECTOR_METRES + WORLD_ORIGIN,
    )
}

/// Parse `sectors2x2_<sx>_<sy>.wnv` into its sector numbers.
#[must_use]
pub fn parse_sector_name(name: &str) -> Option<(u32, u32)> {
    let rest = name.strip_prefix("sectors2x2_")?.strip_suffix(".wnv")?;
    let (x, y) = rest.split_once('_')?;
    Some((x.parse().ok()?, y.parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Hand-built minimal tile: 4 verts, 4 indices, 1 quad, 4 edge records.
    fn mini_tile() -> Vec<u8> {
        let mut b = vec![0u8; 0x200];
        b[0x40..0x44].copy_from_slice(&100f32.to_le_bytes());
        b[0x44..0x48].copy_from_slice(&100f32.to_le_bytes());
        b[0x48..0x4c].copy_from_slice(&10f32.to_le_bytes());
        let poly_base = 0xa0usize;
        let edge_base = poly_base + POLYGON_LEN;
        let vert_base = edge_base + EDGE_LEN + 4 * EDGE_LEN;
        let index_base = vert_base + 4 * VERTEX_LEN;
        for (at, base) in [
            (0x58, vert_base),
            (0x60, index_base),
            (0x64, edge_base),
            (0x6c, poly_base),
        ] {
            b[at..at + 4].copy_from_slice(&((0x5000_0000u32) | base as u32).to_le_bytes());
        }
        b[0x68..0x6c].copy_from_slice(&4u32.to_le_bytes());
        b[0x78..0x7c].copy_from_slice(&4u32.to_le_bytes());
        b[0x7c..0x80].copy_from_slice(&1u32.to_le_bytes());
        // one polygon with first_index 0
        b[poly_base + 4..poly_base + 6].copy_from_slice(&0u16.to_le_bytes());
        b[poly_base + 34..poly_base + 36].copy_from_slice(&205u16.to_le_bytes());
        // vertices: corners of the tile at half height
        for (i, (x, y)) in [(0u16, 0u16), (65535, 0), (65535, 65535), (0, 65535)]
            .iter()
            .enumerate()
        {
            let o = vert_base + i * VERTEX_LEN;
            b[o..o + 2].copy_from_slice(&x.to_le_bytes());
            b[o + 2..o + 4].copy_from_slice(&y.to_le_bytes());
            b[o + 4..o + 6].copy_from_slice(&32768u16.to_le_bytes());
        }
        // indices 0,1,2,3
        for i in 0..4u16 {
            b[index_base + i as usize * 2..index_base + i as usize * 2 + 2]
                .copy_from_slice(&i.to_le_bytes());
        }
        b
    }

    #[test]
    fn parses_hand_built_tile() {
        let bytes = mini_tile();
        let t = Tile::parse(&bytes).unwrap();
        assert_eq!((t.vert_count(), t.index_count(), t.poly_count()), (4, 4, 1));
        assert_eq!(t.polygon_vertex_count(0).unwrap(), 4);
        assert_eq!(t.polygon_vertices(0).unwrap(), vec![0, 1, 2, 3]);
        let v = t.vertex(2).unwrap();
        assert_eq!(t.local_xy(v), (100.0, 100.0));
        assert!((t.local_z_rel(v) - 5.0).abs() < 0.01);
        let p = t.polygon(0).unwrap();
        assert_eq!(p.first_index(), 0);
        assert_eq!(p.marker(), 205);
        assert_eq!(t.edge_records().count(), 4);
        let ir = t.validate_indices();
        assert_eq!((ir.out_of_range, ir.distinct), (0, 4));
        let pr = t.validate_polygons();
        assert_eq!((pr.non_monotonic, pr.min_verts, pr.max_verts), (0, 4, 4));
    }

    #[test]
    fn rejects_short_and_bad_sizes() {
        assert!(Tile::parse(&[0u8; 16]).is_err());
        let mut bytes = mini_tile();
        // point the vertex array off the end
        bytes[0x58..0x5c].copy_from_slice(&0x500f_ffffu32.to_le_bytes());
        assert!(matches!(
            Tile::parse(&bytes).unwrap_err(),
            Error::BadSize { .. }
        ));
        let bytes = mini_tile();
        let t = Tile::parse(&bytes).unwrap();
        assert!(matches!(t.vertex(4).unwrap_err(), Error::OutOfRange { .. }));
        assert!(matches!(
            t.polygon(1).unwrap_err(),
            Error::OutOfRange { .. }
        ));
    }

    #[test]
    fn sector_names() {
        assert_eq!(parse_sector_name("sectors2x2_60_92.wnv"), Some((60, 92)));
        assert_eq!(parse_sector_name("dinghy.wnv"), None);
        assert_eq!(tile_origin(0, 0), (-3000.0, -3000.0));
        assert_eq!(tile_origin(60, 60), (0.0, 0.0));
    }
}
