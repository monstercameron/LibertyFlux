//! File roots: single-bound files and bound dictionaries, plus validation.

use crate::bound::{Bound, BoundType};
use crate::error::Error;
use crate::rsc::{self, Cursor};

/// Resource type id for bounds files.
const KIND_BOUNDS: u32 = 32;

/// Single-bound file: one root bound plus two root words.
#[derive(Debug, Clone, PartialEq)]
pub struct WbnFile {
    /// Opaque root vtable word (file offset 0).
    pub root_vtable: u32,
    /// Auxiliary root word (file offset 4): always a system pointer on files
    /// seen, pointing at a zero word followed by padding; purpose unknown.
    /// Kept raw so future work can interpret it.
    pub aux: u32,
    /// Segment offset the root pointer resolved to.
    pub root_offset: usize,
    /// The root bound.
    pub root: Bound,
}

/// One dictionary entry: a model-name hash paired with its bound.
#[derive(Debug, Clone, PartialEq)]
pub struct WbdEntry {
    /// Unsigned 32-bit model-name hash (algorithm unknown).
    pub hash: u32,
    /// The bound for this entry.
    pub bound: Bound,
}

/// Bound dictionary file: hashes paired with one bound each.
#[derive(Debug, Clone, PartialEq)]
pub struct WbdFile {
    /// Opaque root vtable word (file offset 0).
    pub root_vtable: u32,
    /// Block-map pointer target as a segment offset, if the pointer is set.
    /// The target starts with a zero word followed by padding on files seen;
    /// purpose unknown.
    pub block_map: Option<usize>,
    /// Unknown root word at offset 8 (always 0 on files seen).
    pub unknown08: u32,
    /// Unknown root word at offset 12 (always 1 on files seen).
    pub unknown0c: u32,
    /// Entries in file order.
    pub entries: Vec<WbdEntry>,
}

/// One parsed collision file of either kind.
#[derive(Debug, Clone, PartialEq)]
pub enum CollisionFile {
    /// Single-bound file.
    Wbn(WbnFile),
    /// Bound dictionary file.
    Wbd(WbdFile),
}

impl CollisionFile {
    /// True for single-bound files.
    #[must_use]
    pub fn is_wbn(&self) -> bool {
        matches!(self, CollisionFile::Wbn(_))
    }

    /// Root bounds: exactly one for single-bound files, one per dictionary
    /// entry otherwise.
    #[must_use]
    pub fn roots(&self) -> Box<dyn Iterator<Item = &Bound> + '_> {
        match self {
            CollisionFile::Wbn(f) => Box::new(core::iter::once(&f.root)),
            CollisionFile::Wbd(f) => Box::new(f.entries.iter().map(|e| &e.bound)),
        }
    }

    /// Every bound in the file, descending into composite children.
    #[must_use]
    pub fn all_bounds(&self) -> Vec<&Bound> {
        fn walk<'a>(bound: &'a Bound, out: &mut Vec<&'a Bound>) {
            out.push(bound);
            if let Bound::Composite(c) = bound {
                for child in &c.children {
                    walk(child, out);
                }
            }
        }
        let mut out = Vec::new();
        for root in self.roots() {
            walk(root, &mut out);
        }
        out
    }

    /// Count of bounds per kind, sorted by kind byte. Composite children are
    /// included alongside roots.
    #[must_use]
    pub fn bound_type_counts(&self) -> Vec<(BoundType, usize)> {
        let mut counts: Vec<(BoundType, usize)> = Vec::new();
        for bound in self.all_bounds() {
            let kind = bound.bound_type();
            match counts.iter_mut().find(|(k, _)| *k == kind) {
                Some(slot) => slot.1 += 1,
                None => counts.push((kind, 1)),
            }
        }
        counts.sort_by_key(|(k, _)| k.byte());
        counts
    }

    /// Validate every mesh in the file and aggregate the findings.
    #[must_use]
    pub fn validate(&self) -> ValidationReport {
        let mut report = ValidationReport::default();
        for bound in self.all_bounds() {
            match bound {
                Bound::Mesh(m) => {
                    report.meshes += 1;
                    report.vertices += m.vertices.len() as u64;
                    report.polygons += m.polygons.len() as u64;
                    report.triangles += m.triangle_count() as u64;
                    report.quads += m.polygons.iter().filter(|p| p.is_quad()).count() as u64;
                    report.outside_bbox += m.outside_bbox_count() as u64;
                    report.index_oob += m.index_oob_count() as u64;
                    report.bad_normals += m.non_unit_normal_count(0.02) as u64;
                    report.neighbour_oob += m.neighbour_oob_count() as u64;
                    if m.marker != 0xFFFF_FFFF {
                        report.bad_markers += 1;
                    }
                    for p in &m.polygons {
                        report.max_material = report.max_material.max(p.material_index());
                    }
                }
                Bound::Unparsed(_) => report.unparsed += 1,
                _ => {}
            }
        }
        report
    }
}

/// Aggregated validation findings for one file (or many merged files).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ValidationReport {
    /// Mesh bounds examined (box, geometry, BVH).
    pub meshes: u64,
    /// Total decoded vertices.
    pub vertices: u64,
    /// Total polygon records.
    pub polygons: u64,
    /// Total triangles counting each quad as two.
    pub triangles: u64,
    /// Polygon records that are quads.
    pub quads: u64,
    /// Vertices outside their bound's stated bounding box.
    pub outside_bbox: u64,
    /// Polygon vertex indices past the end of the vertex array.
    pub index_oob: u64,
    /// Face normals whose length differs from 1 by more than 0.02.
    pub bad_normals: u64,
    /// Neighbour indices that are neither `0xFFFF` nor valid.
    pub neighbour_oob: u64,
    /// Mesh bounds whose marker word is not `0xFFFFFFFF`.
    pub bad_markers: u64,
    /// Bounds of kinds without a known tail layout.
    pub unparsed: u64,
    /// Highest material index observed.
    pub max_material: u8,
}

impl ValidationReport {
    /// True when every failure counter is zero. Unparsed kinds and the
    /// material ceiling do not count as failures on their own.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.outside_bbox == 0
            && self.index_oob == 0
            && self.bad_normals == 0
            && self.neighbour_oob == 0
            && self.bad_markers == 0
    }

    /// Add another report's counters into this one (for whole-corpus runs).
    pub fn merge(&mut self, other: &ValidationReport) {
        self.meshes += other.meshes;
        self.vertices += other.vertices;
        self.polygons += other.polygons;
        self.triangles += other.triangles;
        self.quads += other.quads;
        self.outside_bbox += other.outside_bbox;
        self.index_oob += other.index_oob;
        self.bad_normals += other.bad_normals;
        self.neighbour_oob += other.neighbour_oob;
        self.bad_markers += other.bad_markers;
        self.unparsed += other.unparsed;
        self.max_material = self.max_material.max(other.max_material);
    }
}

/// Kind bytes with a known tail layout (or a deliberately unparsed one).
/// Used to recognise a plausible root bound during file-kind detection.
fn plausible_kind(byte: u8) -> bool {
    matches!(byte, 0 | 1 | 3 | 4 | 5 | 6 | 7 | 10 | 11 | 12)
}

/// Parse one collision file from its raw bytes.
pub(crate) fn parse(data: &[u8]) -> Result<CollisionFile, Error> {
    let resource = rsc::open(data)?;
    if resource.kind != KIND_BOUNDS {
        return Err(Error::BadRoot);
    }
    let system = &resource.system;
    if looks_like_dictionary(system) {
        parse_wbd(system)
    } else if looks_like_single(system) {
        parse_wbn(system)
    } else {
        Err(Error::BadRoot)
    }
}

/// Dictionary detection: two collections with matching positive counts, valid
/// pointers, and a plausible first bound. Strict on purpose: a single-bound
/// file holds bounding-box floats at these offsets, which can only match by
/// an astronomically unlikely coincidence.
fn looks_like_dictionary(system: &[u8]) -> bool {
    if system.len() < 32 {
        return false;
    }
    let word =
        |o: usize| u32::from_le_bytes([system[o], system[o + 1], system[o + 2], system[o + 3]]);
    let Ok(Some(hashes_at)) = rsc::resolve(word(0x10), 0x10) else {
        return false;
    };
    let Ok(Some(bounds_at)) = rsc::resolve(word(0x18), 0x18) else {
        return false;
    };
    let h_count = u16::from_le_bytes([system[0x14], system[0x15]]) as usize;
    let h_size = u16::from_le_bytes([system[0x16], system[0x17]]) as usize;
    let b_count = u16::from_le_bytes([system[0x1C], system[0x1D]]) as usize;
    let b_dup = u16::from_le_bytes([system[0x1E], system[0x1F]]) as usize;
    if h_count == 0 || h_count != h_size || h_count != b_count || b_count != b_dup {
        return false;
    }
    if h_count > 100_000 {
        return false;
    }
    if hashes_at
        .checked_add(h_count * 4)
        .is_none_or(|e| e > system.len())
    {
        return false;
    }
    if bounds_at
        .checked_add(h_count * 4)
        .is_none_or(|e| e > system.len())
    {
        return false;
    }
    // At least the first entry must point at a plausible bound.
    let first = u32::from_le_bytes([
        system[bounds_at],
        system[bounds_at + 1],
        system[bounds_at + 2],
        system[bounds_at + 3],
    ]);
    matches!(
        rsc::resolve(first, bounds_at),
        Ok(Some(o)) if o + 5 <= system.len() && plausible_kind(system[o + 4])
    )
}

fn looks_like_single(system: &[u8]) -> bool {
    if system.len() < 12 {
        return false;
    }
    let raw = u32::from_le_bytes([system[8], system[9], system[10], system[11]]);
    matches!(
        rsc::resolve(raw, 8),
        Ok(Some(o)) if o + 5 <= system.len() && plausible_kind(system[o + 4])
    )
}

fn parse_wbn(system: &[u8]) -> Result<CollisionFile, Error> {
    let mut cur = Cursor::new(system);
    let root_vtable = cur.u32()?;
    let aux = cur.u32()?;
    let root_raw = cur.u32()?;
    let root_offset = rsc::resolve(root_raw, 8)?.ok_or(Error::BadRoot)?;
    let root = crate::bound::parse_bound(system, root_offset, 0)?;
    Ok(CollisionFile::Wbn(WbnFile {
        root_vtable,
        aux,
        root_offset,
        root,
    }))
}

fn parse_wbd(system: &[u8]) -> Result<CollisionFile, Error> {
    let mut cur = Cursor::new(system);
    let root_vtable = cur.u32()?;
    let block_raw = cur.u32()?;
    let reserved = [cur.u32()?, cur.u32()?];
    let hashes_raw = cur.u32()?;
    let h_count = cur.u16()? as usize;
    let _h_size = cur.u16()?;
    let bounds_raw = cur.u32()?;
    let b_count = cur.u16()? as usize;
    let _b_dup = cur.u16()?;
    if h_count != b_count {
        return Err(Error::CountMismatch {
            hashes: h_count,
            bounds: b_count,
        });
    }
    let block_map = rsc::resolve(block_raw, 4)?;
    let hashes_at = rsc::resolve(hashes_raw, 0x10)?.ok_or(Error::BadRoot)?;
    let bounds_at = rsc::resolve(bounds_raw, 0x18)?.ok_or(Error::BadRoot)?;
    if hashes_at
        .checked_add(h_count * 4)
        .is_none_or(|e| e > system.len())
    {
        return Err(Error::Truncated {
            offset: hashes_at,
            needed: h_count * 4,
            available: system.len(),
        });
    }
    if bounds_at
        .checked_add(b_count * 4)
        .is_none_or(|e| e > system.len())
    {
        return Err(Error::Truncated {
            offset: bounds_at,
            needed: b_count * 4,
            available: system.len(),
        });
    }
    let mut entries = Vec::with_capacity(h_count);
    for i in 0..h_count {
        let ho = hashes_at + i * 4;
        let hash = u32::from_le_bytes([system[ho], system[ho + 1], system[ho + 2], system[ho + 3]]);
        let bo = bounds_at + i * 4;
        let praw = u32::from_le_bytes([system[bo], system[bo + 1], system[bo + 2], system[bo + 3]]);
        let target = rsc::resolve(praw, bo)?.ok_or(Error::BadRoot)?;
        let bound = crate::bound::parse_bound(system, target, 0)?;
        entries.push(WbdEntry { hash, bound });
    }
    Ok(CollisionFile::Wbd(WbdFile {
        root_vtable,
        block_map,
        unknown08: reserved[0],
        unknown0c: reserved[1],
        entries,
    }))
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // parsed values are compared bit for bit on purpose
mod tests {
    use super::*;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;
    use std::io::Write as _;

    const NAN_PAD: u32 = 0x7F80_0001;

    fn w(v: u32, out: &mut Vec<u8>) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    fn f(v: f32, out: &mut Vec<u8>) {
        out.extend_from_slice(&v.to_le_bytes());
    }
    fn v3(x: f32, y: f32, z: f32, out: &mut Vec<u8>) {
        f(x, out);
        f(y, out);
        f(z, out);
        w(NAN_PAD, out);
    }
    fn base(kind: u8, out: &mut Vec<u8>) {
        w(0x77, out);
        out.push(kind);
        out.push(1);
        out.extend_from_slice(&1u16.to_le_bytes());
        f(1.0, out);
        f(0.0, out);
        v3(1.0, 1.0, 1.0, out);
        v3(-1.0, -1.0, -1.0, out);
        v3(0.0, 0.0, 0.0, out);
        v3(0.0, 0.0, 0.0, out);
        v3(1.0, 1.0, 1.0, out);
        out.extend([0u8; 32]);
    }
    fn ptr(target: u32) -> u32 {
        0x5000_0000 | target
    }

    /// Wrap a hand-built system segment in a resource file. Sizes are padded
    /// up to a multiple of 256 so the flags packing is exact.
    fn resource(system: &mut Vec<u8>, kind: u32) -> Vec<u8> {
        while !system.len().is_multiple_of(256) {
            system.push(0);
        }
        let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
        enc.write_all(system).unwrap();
        let payload = enc.finish().unwrap();
        assert_eq!(&payload[..2], &[0x78, 0xDA]);
        let mut out = Vec::new();
        out.extend_from_slice(&0x0543_5352u32.to_le_bytes());
        out.extend_from_slice(&kind.to_le_bytes());
        out.extend_from_slice(&((system.len() / 256) as u32).to_le_bytes());
        out.extend_from_slice(&payload);
        out
    }

    #[test]
    fn wbn_sphere_round_trip() {
        let mut sys = Vec::new();
        w(0x11, &mut sys); // root vtable
        w(ptr(0x80), &mut sys); // aux (points at padding; never followed)
        let root_pos = sys.len();
        w(0, &mut sys); // root ptr (patched)
        let bound_at = sys.len() as u32;
        base(0, &mut sys);
        v3(1.0, 1.0, 1.0, &mut sys); // radius tail
        let p = ptr(bound_at);
        sys[root_pos..root_pos + 4].copy_from_slice(&p.to_le_bytes());
        let file = parse(&resource(&mut sys, 32)).unwrap();
        assert!(file.is_wbn());
        assert_eq!(file.all_bounds().len(), 1);
        assert_eq!(file.bound_type_counts(), [(BoundType::Sphere, 1)]);
        assert!(file.validate().is_clean());
    }

    #[test]
    fn wbd_two_entries() {
        let mut sys = Vec::new();
        w(0x22, &mut sys); // root vtable
        w(0, &mut sys); // block map: null here
        w(0, &mut sys);
        w(1, &mut sys);
        let h_pos = sys.len();
        w(0, &mut sys);
        sys.extend_from_slice(&2u16.to_le_bytes());
        sys.extend_from_slice(&2u16.to_le_bytes());
        let b_pos = sys.len();
        w(0, &mut sys);
        sys.extend_from_slice(&2u16.to_le_bytes());
        sys.extend_from_slice(&2u16.to_le_bytes());
        let hashes_at = sys.len() as u32;
        w(0xAAAA_AAAA, &mut sys);
        w(0xBBBB_BBBB, &mut sys);
        let ptrs_at = sys.len() as u32;
        let s0 = sys.len() as u32 + 8;
        w(0, &mut sys);
        w(0, &mut sys);
        base(0, &mut sys);
        v3(1.0, 1.0, 1.0, &mut sys);
        let s1 = sys.len() as u32;
        base(1, &mut sys); // capsule
        v3(0.5, 0.5, 0.5, &mut sys);
        v3(1.0, 1.0, 1.0, &mut sys);
        v3(0.0, 0.0, 0.0, &mut sys);
        v3(0.0, 0.0, 0.0, &mut sys);
        for (at, target) in [
            (h_pos, hashes_at),
            (b_pos, ptrs_at),
            (ptrs_at as usize, s0),
            (ptrs_at as usize + 4, s1),
        ] {
            let p = ptr(target);
            sys[at..at + 4].copy_from_slice(&p.to_le_bytes());
        }
        let file = parse(&resource(&mut sys, 32)).unwrap();
        assert!(!file.is_wbn());
        match &file {
            CollisionFile::Wbd(d) => {
                assert_eq!(d.entries.len(), 2);
                assert_eq!(d.entries[0].hash, 0xAAAA_AAAA);
                assert_eq!(d.entries[1].bound.bound_type(), BoundType::Capsule);
            }
            _ => panic!("want wbd"),
        }
        assert_eq!(
            file.bound_type_counts(),
            [(BoundType::Sphere, 1), (BoundType::Capsule, 1)]
        );
    }

    #[test]
    fn wbn_composite_two_children() {
        let mut sys = Vec::new();
        w(0x11, &mut sys);
        w(0, &mut sys);
        let root_pos = sys.len();
        w(0, &mut sys);
        let comp_at = sys.len() as u32;
        base(12, &mut sys);
        let tails = sys.len();
        sys.extend([0u8; 24]); // 4 ptrs + 2 counts + pad
        // Children live after the tail.
        let c0 = sys.len() as u32;
        base(0, &mut sys);
        v3(1.0, 1.0, 1.0, &mut sys);
        let c1 = sys.len() as u32;
        base(0, &mut sys);
        v3(2.0, 2.0, 2.0, &mut sys);
        let arr_at = sys.len() as u32;
        w(ptr(c0), &mut sys);
        w(ptr(c1), &mut sys);
        let mat_at = sys.len() as u32;
        for _ in 0..2 {
            // Identity rows with NaN pads.
            for r in 0..4 {
                for c in 0..4 {
                    f(if r == c { 1.0 } else { 0.0 }, &mut sys);
                }
            }
        }
        let box_at = sys.len() as u32;
        for _ in 0..2 {
            v3(-1.0, -1.0, -1.0, &mut sys);
            v3(1.0, 1.0, 1.0, &mut sys);
        }
        for (at, target) in [
            (root_pos, comp_at),
            (tails, arr_at),
            (tails + 4, mat_at),
            (tails + 8, mat_at),
            (tails + 12, box_at),
        ] {
            let p = ptr(target);
            sys[at..at + 4].copy_from_slice(&p.to_le_bytes());
        }
        sys[tails + 16..tails + 18].copy_from_slice(&2u16.to_le_bytes());
        sys[tails + 18..tails + 20].copy_from_slice(&2u16.to_le_bytes());
        let file = parse(&resource(&mut sys, 32)).unwrap();
        assert_eq!(file.all_bounds().len(), 3); // root + 2 children
        match &file {
            CollisionFile::Wbn(f) => match &f.root {
                Bound::Composite(c) => {
                    assert_eq!(c.children.len(), 2);
                    assert_eq!(c.current_matrices.len(), 2);
                    assert_eq!(c.local_boxes.len(), 2);
                    assert_eq!(c.current_matrices[0].translation().x, 0.0);
                }
                other => panic!("want composite, got {other:?}"),
            },
            _ => panic!("want wbn"),
        }
    }

    #[test]
    fn rejects_wrong_kind_and_bad_roots() {
        let mut sys = vec![0u8; 256];
        let bytes = resource(&mut sys, 8); // texture kind, not bounds
        assert!(matches!(parse(&bytes), Err(Error::BadRoot)));
        let mut sys = vec![0xFFu8; 256];
        let bytes = resource(&mut sys, 32);
        assert!(matches!(parse(&bytes), Err(Error::BadRoot)));
        assert!(matches!(parse(&[0u8; 8]), Err(Error::Truncated { .. })));
    }
}
