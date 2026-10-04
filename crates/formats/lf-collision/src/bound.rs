//! Bound types: the collision shapes stored in the files.
//!
//! Every bound starts with a 128-byte [`BoundHeader`]; the kind byte selects
//! the tail layout. See the crate documentation for the full format tables.

use crate::error::Error;
use crate::rsc::{Cursor, resolve};

/// Largest vertex or polygon count accepted for one mesh bound. The biggest
/// shipped mesh is two orders of magnitude smaller; anything above this is
/// treated as corrupt input rather than allocated.
const MAX_MESH_COUNT: i64 = 2_000_000;

/// Largest child count accepted for one composite bound.
const MAX_CHILDREN: usize = 4096;

/// Maximum composite nesting depth.
const MAX_DEPTH: u8 = 8;

/// Size of the bound base header in bytes.
const BASE_SIZE: usize = 128;

/// A 3-component single-precision vector: positions, extents, normals.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

impl Vec3 {
    /// True when every component of `self` lies within the matching closed
    /// interval of `min`..=`max`, widened by `epsilon` on each side.
    #[must_use]
    pub fn within(&self, min: &Vec3, max: &Vec3, epsilon: f32) -> bool {
        self.x >= min.x - epsilon
            && self.x <= max.x + epsilon
            && self.y >= min.y - epsilon
            && self.y <= max.y + epsilon
            && self.z >= min.z - epsilon
            && self.z <= max.z + epsilon
    }

    /// Euclidean length.
    #[must_use]
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

/// Bound kind byte (base offset 4).
///
/// Kinds 2, 8 and 9 never appear in shipped files and have no confirmed
/// meaning; kinds 5, 6, 7 and 11 have engine classes but no shipped samples,
/// so their tails are unknown. All of those surface as [`Bound::Unparsed`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundType {
    /// Kind 0: sphere primitive.
    Sphere,
    /// Kind 1: capsule primitive.
    Capsule,
    /// Kind 3: axis-aligned cuboid stored as an 8-vertex mesh.
    Box,
    /// Kind 4: triangle/quad mesh with a second vertex array.
    Geometry,
    /// Kind 5: curved-surface mesh (no shipped samples).
    CurvedGeometry,
    /// Kind 6: uniform-grid partitioned bound (no shipped samples).
    Grid,
    /// Kind 7: ribbon/strip bound (no shipped samples).
    Ribbon,
    /// Kind 10: mesh with the tree flag set.
    Bvh,
    /// Kind 11: surface-like bound (no shipped samples).
    Surface,
    /// Kind 12: group of child bounds with transforms.
    Composite,
    /// Any other kind byte; the tail layout is unknown.
    Unknown(u8),
}

impl BoundType {
    /// Decode a kind byte.
    #[must_use]
    pub fn from_byte(b: u8) -> Self {
        match b {
            0 => BoundType::Sphere,
            1 => BoundType::Capsule,
            3 => BoundType::Box,
            4 => BoundType::Geometry,
            5 => BoundType::CurvedGeometry,
            6 => BoundType::Grid,
            7 => BoundType::Ribbon,
            10 => BoundType::Bvh,
            11 => BoundType::Surface,
            12 => BoundType::Composite,
            other => BoundType::Unknown(other),
        }
    }

    /// The kind byte for this variant.
    #[must_use]
    pub fn byte(self) -> u8 {
        match self {
            BoundType::Sphere => 0,
            BoundType::Capsule => 1,
            BoundType::Box => 3,
            BoundType::Geometry => 4,
            BoundType::CurvedGeometry => 5,
            BoundType::Grid => 6,
            BoundType::Ribbon => 7,
            BoundType::Bvh => 10,
            BoundType::Surface => 11,
            BoundType::Composite => 12,
            BoundType::Unknown(b) => b,
        }
    }

    /// Short human-readable name.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            BoundType::Sphere => "sphere",
            BoundType::Capsule => "capsule",
            BoundType::Box => "box",
            BoundType::Geometry => "geometry",
            BoundType::CurvedGeometry => "curved-geometry",
            BoundType::Grid => "grid",
            BoundType::Ribbon => "ribbon",
            BoundType::Bvh => "bvh",
            BoundType::Surface => "surface",
            BoundType::Composite => "composite",
            BoundType::Unknown(_) => "unknown",
        }
    }
}

/// The 128-byte base header every bound starts with.
#[derive(Debug, Clone, PartialEq)]
pub struct BoundHeader {
    /// Opaque vtable word (a small id, constant per bound kind).
    pub vtable: u32,
    /// Bound kind.
    pub bound_type: BoundType,
    /// Flags byte (usually 1; meaning unknown).
    pub flags: u8,
    /// Part index (usually 1; meaning unknown).
    pub part_index: u16,
    /// Bounding-sphere radius around the centroid.
    pub radius: f32,
    /// Unknown float at base offset 12.
    pub unknown0c: f32,
    /// Bounding-box maximum corner.
    pub bbox_max: Vec3,
    /// Bounding-box minimum corner.
    pub bbox_min: Vec3,
    /// Centroid offset.
    pub centroid_offset: Vec3,
    /// Centre-of-gravity offset.
    pub cg_offset: Vec3,
    /// Volume distribution.
    pub volume_distribution: Vec3,
    /// Trailing 32 bytes (base offset 96..128): not yet interpreted.
    pub reserved: [u8; 32],
}

/// Sphere bound (kind 0): base plus the radius replicated as a vector.
#[derive(Debug, Clone, PartialEq)]
pub struct SphereBound {
    /// Base header.
    pub header: BoundHeader,
    /// Radius vector: the bounding radius three times. Matches
    /// [`BoundHeader::radius`] on files seen.
    pub radius_vec: Vec3,
}

/// Capsule bound (kind 1): base plus radius, length and two zero vectors.
#[derive(Debug, Clone, PartialEq)]
pub struct CapsuleBound {
    /// Base header.
    pub header: BoundHeader,
    /// Cylinder radius, replicated three times.
    pub radius_vec: Vec3,
    /// Cylinder length, replicated three times. The base bounding radius
    /// always equals radius + length / 2 on files seen.
    pub length_vec: Vec3,
    /// Third tail vector: all zeros on every shipped capsule; purpose unknown.
    pub extra_a: Vec3,
    /// Fourth tail vector: all zeros on every shipped capsule; purpose unknown.
    pub extra_b: Vec3,
}

/// Which mesh flavour a [`MeshBound`] carries. All three share one layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeshKind {
    /// Kind 3: always exactly 8 vertices and 6 quadrilateral polygons.
    Box,
    /// Kind 4: general mesh, always with a second vertex array.
    Geometry,
    /// Kind 10: general mesh with the tree flag set, never with a second
    /// vertex array.
    Bvh,
}

/// Mesh bound (kinds 3, 4, 10): quantised vertices plus polygon records.
#[derive(Debug, Clone, PartialEq)]
pub struct MeshBound {
    /// Base header.
    pub header: BoundHeader,
    /// Which mesh flavour this is.
    pub mesh_kind: MeshKind,
    /// Per-component unquantise factors.
    pub unquantize: Vec3,
    /// Quantisation centre: decoded position is `centre + raw * unquantise`.
    pub center: Vec3,
    /// Decoded vertex positions.
    pub vertices: Vec<Vec3>,
    /// Polygon records.
    pub polygons: Vec<Polygon>,
    /// Second vertex array in the same quantisation, present exactly for
    /// [`MeshKind::Geometry`] on files seen. Purpose unknown.
    pub shrunk_vertices: Option<Vec<Vec3>>,
    /// Tree flag byte (mesh offset 184): 1 for BVH, 0 otherwise, on files seen.
    pub tree_flag: u8,
    /// Marker word (mesh offset 188): always `0xFFFFFFFF` on files seen.
    pub marker: u32,
}

impl MeshBound {
    /// Number of vertices outside the header bounding box (with a small
    /// tolerance for quantisation rounding). Zero on every shipped file.
    #[must_use]
    pub fn outside_bbox_count(&self) -> usize {
        self.vertices
            .iter()
            .filter(|v| !v.within(&self.header.bbox_min, &self.header.bbox_max, 1e-3))
            .count()
    }

    /// Number of polygon vertex indices at or above the vertex count.
    /// Zero on every shipped file.
    #[must_use]
    pub fn index_oob_count(&self) -> usize {
        let n = self.vertices.len();
        self.polygons
            .iter()
            .flat_map(|p| p.vertices.iter().enumerate())
            .filter(|(slot, idx)| {
                let v = **idx as usize;
                // Slot 3 of a triangle is the zero "unused" marker.
                !(*slot == 3 && **idx == 0) && v >= n
            })
            .count()
    }

    /// Number of face normals whose length differs from 1 by more than `tol`.
    /// Zero on every shipped file for any tolerance above 0.02.
    #[must_use]
    pub fn non_unit_normal_count(&self, tol: f32) -> usize {
        self.polygons
            .iter()
            .filter(|p| (p.normal.length() - 1.0).abs() > tol)
            .count()
    }

    /// Number of neighbour indices that are neither `0xFFFF` (none) nor a
    /// valid polygon index.
    #[must_use]
    pub fn neighbour_oob_count(&self) -> usize {
        let n = self.polygons.len();
        self.polygons
            .iter()
            .flat_map(|p| p.neighbours.iter())
            .filter(|nb| **nb != 0xFFFF && (**nb as usize) >= n)
            .count()
    }

    /// Total triangles if every quad counts as two.
    #[must_use]
    pub fn triangle_count(&self) -> usize {
        self.polygons
            .iter()
            .map(|p| p.triangle_count() as usize)
            .sum()
    }
}

/// One 32-byte polygon record.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Polygon {
    /// Face normal (unit length on files seen).
    pub normal: Vec3,
    /// Face area (never negative on files seen). The low bits of the float
    /// representation carry the material index.
    pub area: f32,
    /// Vertex indices; slot 3 is zero for triangles.
    pub vertices: [u16; 4],
    /// Neighbour-polygon indices; `0xFFFF` means no neighbour on that edge.
    pub neighbours: [u16; 4],
}

impl Polygon {
    /// Material index: the low byte of the area float's bit representation.
    ///
    /// Values 0..=117 appear in shipped files. The packing is inferred from
    /// the open-source readers and from the observed value range, not from
    /// any in-file declaration, so treat it as informed convention.
    #[must_use]
    pub fn material_index(&self) -> u8 {
        (self.area.to_bits() & 0xFF) as u8
    }

    /// True for quadrilaterals (fourth vertex slot non-zero).
    #[must_use]
    pub fn is_quad(&self) -> bool {
        self.vertices[3] != 0
    }

    /// Triangles this polygon triangulates to: 2 for quads, 1 otherwise.
    #[must_use]
    pub fn triangle_count(&self) -> u32 {
        if self.is_quad() { 2 } else { 1 }
    }
}

/// One 64-byte composite child matrix: four rows of three floats plus a NaN
/// pad word each. Rows 0..3 are the X basis, Y basis, Z basis and translation.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Matrix4 {
    /// Matrix rows as stored, including the pad words.
    pub rows: [[f32; 4]; 4],
}

impl Matrix4 {
    /// Translation part (fourth row, first three components).
    #[must_use]
    pub fn translation(&self) -> Vec3 {
        Vec3 {
            x: self.rows[3][0],
            y: self.rows[3][1],
            z: self.rows[3][2],
        }
    }
}

/// Composite bound (kind 12): child bounds with per-child transforms.
#[derive(Debug, Clone, PartialEq)]
pub struct CompositeBound {
    /// Base header.
    pub header: BoundHeader,
    /// Child bounds in array order. Always leaf bounds on files seen.
    pub children: Vec<Bound>,
    /// Current transform per child, in the same order.
    pub current_matrices: Vec<Matrix4>,
    /// Last transform per child (identical target to current on files seen).
    pub last_matrices: Vec<Matrix4>,
    /// Local bounding box (min, max) per child, in the same order.
    pub local_boxes: Vec<(Vec3, Vec3)>,
    /// Maximum bound count (mesh offset 144 of the tail).
    pub max_bounds: u16,
    /// Current bound count; always equals `max_bounds` on files seen.
    pub num_bounds: u16,
}

/// A bound whose kind byte has no known tail layout: kinds 2, 5, 6, 7, 8, 9
/// and 11, and anything else unrecognised. Only the base header is read; the
/// tail bytes are left in place.
#[derive(Debug, Clone, PartialEq)]
pub struct UnparsedBound {
    /// Base header.
    pub header: BoundHeader,
    /// The raw kind byte.
    pub kind_byte: u8,
}

/// One parsed bound.
#[derive(Debug, Clone, PartialEq)]
pub enum Bound {
    /// Kind 0.
    Sphere(SphereBound),
    /// Kind 1.
    Capsule(CapsuleBound),
    /// Kinds 3, 4 and 10.
    Mesh(MeshBound),
    /// Kind 12.
    Composite(CompositeBound),
    /// Any kind without a known tail layout.
    Unparsed(UnparsedBound),
}

impl Bound {
    /// This bound's base header.
    #[must_use]
    pub fn header(&self) -> &BoundHeader {
        match self {
            Bound::Sphere(b) => &b.header,
            Bound::Capsule(b) => &b.header,
            Bound::Mesh(b) => &b.header,
            Bound::Composite(b) => &b.header,
            Bound::Unparsed(b) => &b.header,
        }
    }

    /// This bound's kind.
    #[must_use]
    pub fn bound_type(&self) -> BoundType {
        self.header().bound_type
    }
}

fn read_vec3_padded(cur: &mut Cursor<'_>) -> Result<Vec3, Error> {
    let x = cur.f32()?;
    let y = cur.f32()?;
    let z = cur.f32()?;
    cur.skip(4)?; // NaN pad word
    Ok(Vec3 { x, y, z })
}

fn parse_header(system: &[u8], offset: usize) -> Result<BoundHeader, Error> {
    let mut cur = Cursor::at(system, offset)?;
    let vtable = cur.u32()?;
    let kind_byte = cur.u8()?;
    let flags = cur.u8()?;
    let part_index = cur.u16()?;
    let radius = cur.f32()?;
    let unknown0c = cur.f32()?;
    let bbox_max = read_vec3_padded(&mut cur)?;
    let bbox_min = read_vec3_padded(&mut cur)?;
    let centroid_offset = read_vec3_padded(&mut cur)?;
    let cg_offset = read_vec3_padded(&mut cur)?;
    let volume_distribution = read_vec3_padded(&mut cur)?;
    let reserved = cur.bytes(32)?;
    let mut raw = [0u8; 32];
    raw.copy_from_slice(reserved);
    Ok(BoundHeader {
        vtable,
        bound_type: BoundType::from_byte(kind_byte),
        flags,
        part_index,
        radius,
        unknown0c,
        bbox_max,
        bbox_min,
        centroid_offset,
        cg_offset,
        volume_distribution,
        reserved: raw,
    })
}

fn word_at(system: &[u8], offset: usize) -> Result<u32, Error> {
    if offset.checked_add(4).is_none_or(|end| end > system.len()) {
        return Err(Error::Truncated {
            offset,
            needed: 4,
            available: system.len(),
        });
    }
    Ok(u32::from_le_bytes([
        system[offset],
        system[offset + 1],
        system[offset + 2],
        system[offset + 3],
    ]))
}

fn check_count(value: i32, offset: usize) -> Result<usize, Error> {
    let v = i64::from(value);
    if !(0..=MAX_MESH_COUNT).contains(&v) {
        return Err(Error::BadCount { offset, value: v });
    }
    usize::try_from(v).map_err(|_| Error::BadCount { offset, value: v })
}

fn decode_vertices(
    system: &[u8],
    array: usize,
    ptr_pos: usize,
    count: usize,
    unquantize: &Vec3,
    center: &Vec3,
) -> Result<Vec<Vec3>, Error> {
    let bytes = count.checked_mul(6).ok_or(Error::BadCount {
        offset: ptr_pos,
        value: i64::try_from(count).unwrap_or(i64::MAX),
    })?;
    if array
        .checked_add(bytes)
        .is_none_or(|end| end > system.len())
    {
        return Err(Error::Truncated {
            offset: array,
            needed: bytes,
            available: system.len(),
        });
    }
    let mut out = Vec::with_capacity(count);
    let mut cur = Cursor::at(system, array)?;
    for _ in 0..count {
        let qx = f32::from(cur.i16()?);
        let qy = f32::from(cur.i16()?);
        let qz = f32::from(cur.i16()?);
        out.push(Vec3 {
            x: center.x + qx * unquantize.x,
            y: center.y + qy * unquantize.y,
            z: center.z + qz * unquantize.z,
        });
    }
    Ok(out)
}

fn parse_polygons(
    system: &[u8],
    array: usize,
    ptr_pos: usize,
    count: usize,
) -> Result<Vec<Polygon>, Error> {
    let bytes = count.checked_mul(32).ok_or(Error::BadCount {
        offset: ptr_pos,
        value: i64::try_from(count).unwrap_or(i64::MAX),
    })?;
    if array
        .checked_add(bytes)
        .is_none_or(|end| end > system.len())
    {
        return Err(Error::Truncated {
            offset: array,
            needed: bytes,
            available: system.len(),
        });
    }
    let mut cur = Cursor::at(system, array)?;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let normal = Vec3 {
            x: cur.f32()?,
            y: cur.f32()?,
            z: cur.f32()?,
        };
        let area = cur.f32()?;
        let vertices = [cur.u16()?, cur.u16()?, cur.u16()?, cur.u16()?];
        let neighbours = [cur.u16()?, cur.u16()?, cur.u16()?, cur.u16()?];
        out.push(Polygon {
            normal,
            area,
            vertices,
            neighbours,
        });
    }
    Ok(out)
}

fn parse_mesh(
    system: &[u8],
    offset: usize,
    header: BoundHeader,
    mesh_kind: MeshKind,
) -> Result<Bound, Error> {
    let base = offset + BASE_SIZE;
    let shrunk_pos = base + 4;
    let poly_pos = base + 12;
    let vert_pos = base + 48;
    let shrunk = resolve(word_at(system, shrunk_pos)?, shrunk_pos)?;
    let poly_ptr = resolve(word_at(system, poly_pos)?, poly_pos)?;
    let mut cur = Cursor::at(system, base + 16)?;
    let unquantize = read_vec3_padded(&mut cur)?;
    let center = read_vec3_padded(&mut cur)?;
    let vert_ptr = resolve(word_at(system, vert_pos)?, vert_pos)?;
    let mut cur = Cursor::at(system, base + 56)?;
    let tree_flag = cur.u8()?;
    cur.skip(3)?;
    let marker = cur.u32()?;
    cur.skip(8)?;
    let num_verts = check_count(cur.i32()?, base + 72)?;
    let num_polys = check_count(cur.i32()?, base + 76)?;

    let no_array = |ptr_pos: usize, count: usize, elem: usize| Error::Truncated {
        offset: ptr_pos,
        needed: count * elem,
        available: 0,
    };
    let vertices = match vert_ptr {
        Some(array) => {
            if array > system.len() {
                return Err(Error::PointerOutOfRange {
                    offset: vert_pos,
                    target: array,
                });
            }
            decode_vertices(system, array, vert_pos, num_verts, &unquantize, &center)?
        }
        None if num_verts == 0 => Vec::new(),
        None => return Err(no_array(vert_pos, num_verts, 6)),
    };
    let polygons = match poly_ptr {
        Some(array) => {
            if array > system.len() {
                return Err(Error::PointerOutOfRange {
                    offset: poly_pos,
                    target: array,
                });
            }
            parse_polygons(system, array, poly_pos, num_polys)?
        }
        None if num_polys == 0 => Vec::new(),
        None => return Err(no_array(poly_pos, num_polys, 32)),
    };
    let shrunk_vertices = match shrunk {
        Some(array) => {
            if array > system.len() {
                return Err(Error::PointerOutOfRange {
                    offset: shrunk_pos,
                    target: array,
                });
            }
            Some(decode_vertices(
                system,
                array,
                shrunk_pos,
                num_verts,
                &unquantize,
                &center,
            )?)
        }
        None => None,
    };
    Ok(Bound::Mesh(MeshBound {
        header,
        mesh_kind,
        unquantize,
        center,
        vertices,
        polygons,
        shrunk_vertices,
        tree_flag,
        marker,
    }))
}

fn parse_matrix(system: &[u8], offset: usize) -> Result<Matrix4, Error> {
    let mut cur = Cursor::at(system, offset)?;
    let mut rows = [[0f32; 4]; 4];
    for row in &mut rows {
        row[0] = cur.f32()?;
        row[1] = cur.f32()?;
        row[2] = cur.f32()?;
        row[3] = cur.f32()?;
    }
    Ok(Matrix4 { rows })
}

// One composite-bound layout; the length is sequential field reads.
#[allow(clippy::too_many_lines)]
fn parse_composite(
    system: &[u8],
    offset: usize,
    header: BoundHeader,
    depth: u8,
) -> Result<Bound, Error> {
    if depth >= MAX_DEPTH {
        return Err(Error::TooDeeplyNested);
    }
    let base = offset + BASE_SIZE;
    let bounds_pos = base;
    let current_pos = base + 4;
    let last_pos = base + 8;
    let boxes_pos = base + 12;
    let bounds_ptr = resolve(word_at(system, bounds_pos)?, bounds_pos)?;
    let current_ptr = resolve(word_at(system, current_pos)?, current_pos)?;
    let last_ptr = resolve(word_at(system, last_pos)?, last_pos)?;
    let boxes_ptr = resolve(word_at(system, boxes_pos)?, boxes_pos)?;
    let mut cur = Cursor::at(system, base + 16)?;
    let max_bounds = cur.u16()?;
    let num_bounds = cur.u16()?;
    let count = (if max_bounds == 0 {
        num_bounds
    } else {
        max_bounds
    }) as usize;
    if count > MAX_CHILDREN {
        return Err(Error::BadCount {
            offset: base + 16,
            value: i64::try_from(count).unwrap_or(i64::MAX),
        });
    }
    let missing = |ptr_pos: usize| Error::Truncated {
        offset: ptr_pos,
        needed: 1,
        available: 0,
    };
    let array = bounds_ptr.ok_or_else(|| missing(bounds_pos))?;
    let current = current_ptr.ok_or_else(|| missing(current_pos))?;
    let last = last_ptr.ok_or_else(|| missing(last_pos))?;
    let boxes = boxes_ptr.ok_or_else(|| missing(boxes_pos))?;
    for (pos, target) in [
        (bounds_pos, array),
        (current_pos, current),
        (last_pos, last),
        (boxes_pos, boxes),
    ] {
        if target > system.len() {
            return Err(Error::PointerOutOfRange {
                offset: pos,
                target,
            });
        }
    }
    if array
        .checked_add(count * 4)
        .is_none_or(|end| end > system.len())
    {
        return Err(Error::Truncated {
            offset: array,
            needed: count * 4,
            available: system.len(),
        });
    }
    let mut children = Vec::new();
    let mut current_matrices = Vec::with_capacity(count);
    let mut last_matrices = Vec::with_capacity(count);
    let mut local_boxes = Vec::with_capacity(count);
    for i in 0..count {
        let child = resolve(word_at(system, array + i * 4)?, array + i * 4)?;
        if let Some(target) = child {
            if target
                .checked_add(BASE_SIZE)
                .is_none_or(|end| end > system.len())
            {
                return Err(Error::PointerOutOfRange {
                    offset: array + i * 4,
                    target,
                });
            }
            children.push(parse_bound(system, target, depth + 1)?);
        }
        current_matrices.push(parse_matrix(system, current + i * 64).map_err(|e| match e {
            Error::Truncated {
                needed, available, ..
            } => Error::Truncated {
                offset: current + i * 64,
                needed,
                available,
            },
            other => other,
        })?);
        last_matrices.push(parse_matrix(system, last + i * 64).map_err(|e| match e {
            Error::Truncated {
                needed, available, ..
            } => Error::Truncated {
                offset: last + i * 64,
                needed,
                available,
            },
            other => other,
        })?);
        let mut bcur = Cursor::at(system, boxes + i * 32)?;
        let bmin = read_vec3_padded(&mut bcur)?;
        let bmax = read_vec3_padded(&mut bcur)?;
        local_boxes.push((bmin, bmax));
    }
    Ok(Bound::Composite(CompositeBound {
        header,
        children,
        current_matrices,
        last_matrices,
        local_boxes,
        max_bounds,
        num_bounds,
    }))
}

/// Parse one bound at `offset` in the system segment.
pub(crate) fn parse_bound(system: &[u8], offset: usize, depth: u8) -> Result<Bound, Error> {
    if depth > MAX_DEPTH {
        return Err(Error::TooDeeplyNested);
    }
    if offset
        .checked_add(BASE_SIZE)
        .is_none_or(|end| end > system.len())
    {
        return Err(Error::Truncated {
            offset,
            needed: BASE_SIZE,
            available: system.len(),
        });
    }
    let header = parse_header(system, offset)?;
    match header.bound_type {
        BoundType::Sphere => {
            let mut cur = Cursor::at(system, offset + BASE_SIZE)?;
            let radius_vec = read_vec3_padded(&mut cur)?;
            Ok(Bound::Sphere(SphereBound { header, radius_vec }))
        }
        BoundType::Capsule => {
            let mut cur = Cursor::at(system, offset + BASE_SIZE)?;
            let radius_vec = read_vec3_padded(&mut cur)?;
            let length_vec = read_vec3_padded(&mut cur)?;
            let extra_a = read_vec3_padded(&mut cur)?;
            let extra_b = read_vec3_padded(&mut cur)?;
            Ok(Bound::Capsule(CapsuleBound {
                header,
                radius_vec,
                length_vec,
                extra_a,
                extra_b,
            }))
        }
        BoundType::Box => parse_mesh(system, offset, header, MeshKind::Box),
        BoundType::Geometry => parse_mesh(system, offset, header, MeshKind::Geometry),
        BoundType::Bvh => parse_mesh(system, offset, header, MeshKind::Bvh),
        BoundType::Composite => parse_composite(system, offset, header, depth),
        other => Ok(Bound::Unparsed(UnparsedBound {
            header,
            kind_byte: other.byte(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAN_PAD: f32 = f32::from_bits(0x7F80_0001);

    fn push_vec3(out: &mut Vec<u8>, v: (f32, f32, f32)) {
        for x in [v.0, v.1, v.2, NAN_PAD] {
            out.extend_from_slice(&x.to_le_bytes());
        }
    }

    /// Hand-built 128-byte base header; nothing here comes from game files.
    fn base(kind: u8) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&0x1234u32.to_le_bytes()); // vtable
        out.push(kind);
        out.push(1); // flags
        out.extend_from_slice(&1u16.to_le_bytes()); // part
        out.extend_from_slice(&2.5f32.to_le_bytes()); // radius
        out.extend_from_slice(&0.5f32.to_le_bytes()); // unknown0c
        push_vec3(&mut out, (1.0, 2.0, 3.0)); // max
        push_vec3(&mut out, (-1.0, -2.0, -3.0)); // min
        push_vec3(&mut out, (0.0, 0.0, 0.0)); // centroid
        push_vec3(&mut out, (0.1, 0.2, 0.3)); // cg
        push_vec3(&mut out, (1.0, 1.0, 1.0)); // volume
        out.extend([0xCDu8; 32]); // reserved
        assert_eq!(out.len(), BASE_SIZE);
        out
    }

    #[test]
    fn header_fields() {
        let h = parse_header(&base(10), 0).unwrap();
        assert_eq!(h.bound_type, BoundType::Bvh);
        assert_eq!(h.flags, 1);
        assert_eq!(h.part_index, 1);
        assert_eq!(h.radius, 2.5);
        assert_eq!(
            h.bbox_max,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
        assert_eq!(
            h.bbox_min,
            Vec3 {
                x: -1.0,
                y: -2.0,
                z: -3.0
            }
        );
        assert_eq!(h.reserved, [0xCDu8; 32]);
    }

    #[test]
    fn sphere_tail() {
        let mut sys = base(0);
        push_vec3(&mut sys, (2.5, 2.5, 2.5));
        match parse_bound(&sys, 0, 0).unwrap() {
            Bound::Sphere(s) => assert_eq!(s.radius_vec.x, 2.5),
            other => panic!("want sphere, got {other:?}"),
        }
    }

    #[test]
    fn capsule_tail_and_radius_rule() {
        let mut sys = base(1);
        push_vec3(&mut sys, (0.3, 0.3, 0.3)); // radius
        push_vec3(&mut sys, (0.4, 0.4, 0.4)); // length
        push_vec3(&mut sys, (0.0, 0.0, 0.0));
        push_vec3(&mut sys, (0.0, 0.0, 0.0));
        match parse_bound(&sys, 0, 0).unwrap() {
            Bound::Capsule(c) => {
                // Shipped files obey radius + length / 2; mirror the check here.
                let expect = c.radius_vec.x + c.length_vec.x / 2.0;
                assert!((expect - 0.5).abs() < 1e-6);
                assert_eq!(
                    c.extra_a,
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0
                    }
                );
            }
            other => panic!("want capsule, got {other:?}"),
        }
    }

    /// Hand-built mesh bound: 4 vertices forming a unit square, one quad.
    fn mesh_system() -> Vec<u8> {
        let mut sys = base(4);
        let tail_at = sys.len();
        sys.extend_from_slice(&0u32.to_le_bytes()); // +0 reserved
        let shrunk_pos = sys.len();
        sys.extend_from_slice(&0u32.to_le_bytes()); // +4 shrunk (patched)
        sys.extend_from_slice(&0u32.to_le_bytes()); // +8 reserved
        let poly_pos = sys.len();
        sys.extend_from_slice(&0u32.to_le_bytes()); // +12 polys (patched)
        push_vec3(&mut sys, (0.001, 0.001, 0.001)); // unquantise
        push_vec3(&mut sys, (0.0, 0.0, 0.0)); // centre
        let vert_pos = sys.len();
        sys.extend_from_slice(&0u32.to_le_bytes()); // verts (patched)
        sys.extend_from_slice(&0u32.to_le_bytes()); // reserved
        sys.push(0); // tree flag
        sys.extend([0xCDu8; 3]);
        sys.extend_from_slice(&0xFFFF_FFFFu32.to_le_bytes()); // marker
        sys.extend_from_slice(&0u32.to_le_bytes());
        sys.extend_from_slice(&0u32.to_le_bytes());
        sys.extend_from_slice(&4i32.to_le_bytes()); // nverts
        sys.extend_from_slice(&1i32.to_le_bytes()); // npolys
        assert_eq!(sys.len() - tail_at, 80);
        // Vertices: (0,0,0) (1000,0,0) (1000,1000,0) (0,1000,0).
        let verts_at = sys.len() as u32;
        for (x, y) in [(0i16, 0i16), (1000, 0), (1000, 1000), (0, 1000)] {
            sys.extend_from_slice(&x.to_le_bytes());
            sys.extend_from_slice(&y.to_le_bytes());
            sys.extend_from_slice(&0i16.to_le_bytes());
        }
        // One quad: normal +Z, area bits with material 7, verts 0..3.
        let polys_at = sys.len() as u32;
        for x in [0f32, 0.0, 1.0] {
            sys.extend_from_slice(&x.to_le_bytes());
        }
        sys.extend_from_slice(&(7u32).to_le_bytes()); // area bits: material 7
        for v in [0u16, 1, 2, 3] {
            sys.extend_from_slice(&v.to_le_bytes());
        }
        for _ in 0..4 {
            sys.extend_from_slice(&0xFFFFu16.to_le_bytes());
        }
        let patch = |sys: &mut Vec<u8>, at: usize, target: u32| {
            let w = 0x5000_0000 | target;
            sys[at..at + 4].copy_from_slice(&w.to_le_bytes());
        };
        patch(&mut sys, vert_pos, verts_at);
        patch(&mut sys, poly_pos, polys_at);
        patch(&mut sys, shrunk_pos, verts_at); // reuse verts as the 2nd array
        sys
    }

    #[test]
    fn mesh_decode_and_material() {
        let sys = mesh_system();
        match parse_bound(&sys, 0, 0).unwrap() {
            Bound::Mesh(m) => {
                assert_eq!(m.mesh_kind, MeshKind::Geometry);
                assert_eq!(m.vertices.len(), 4);
                assert_eq!(
                    m.vertices[2],
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 0.0
                    }
                );
                assert_eq!(m.polygons.len(), 1);
                let p = &m.polygons[0];
                assert!(p.is_quad());
                assert_eq!(p.triangle_count(), 2);
                assert_eq!(p.material_index(), 7);
                assert!(m.shrunk_vertices.is_some());
                assert_eq!(m.outside_bbox_count(), 0);
                assert_eq!(m.index_oob_count(), 0);
                assert_eq!(m.non_unit_normal_count(1e-6), 0);
                assert_eq!(m.neighbour_oob_count(), 0);
            }
            other => panic!("want mesh, got {other:?}"),
        }
    }

    #[test]
    fn mesh_validation_catches_bad_geometry() {
        let mut sys = mesh_system();
        // Move vertex 2 far outside the header box (x = 30000 * 0.001 = 30).
        let verts_at = BASE_SIZE + 80;
        sys[verts_at + 12..verts_at + 14].copy_from_slice(&30000i16.to_le_bytes());
        // Point polygon slot 0 past the end.
        let polys_at = verts_at + 24 + 16;
        sys[polys_at..polys_at + 2].copy_from_slice(&9u16.to_le_bytes());
        match parse_bound(&sys, 0, 0).unwrap() {
            Bound::Mesh(m) => {
                assert_eq!(m.outside_bbox_count(), 1);
                assert_eq!(m.index_oob_count(), 1);
            }
            other => panic!("want mesh, got {other:?}"),
        }
    }

    #[test]
    fn mesh_rejects_garbage_counts_and_tags() {
        let mut sys = mesh_system();
        let tail_at = BASE_SIZE;
        // Negative vertex count.
        sys[tail_at + 72..tail_at + 76].copy_from_slice(&(-1i32).to_le_bytes());
        assert!(matches!(
            parse_bound(&sys, 0, 0),
            Err(Error::BadCount { value: -1, .. })
        ));
        // Graphics-segment tag on the vertex pointer.
        let mut sys = mesh_system();
        sys[tail_at + 48..tail_at + 52].copy_from_slice(&0x6000_0000u32.to_le_bytes());
        assert!(matches!(
            parse_bound(&sys, 0, 0),
            Err(Error::BadPointerTag { .. })
        ));
    }

    #[test]
    fn unparsed_kind_keeps_header() {
        let sys = base(6); // grid: no shipped samples, tail unknown
        match parse_bound(&sys, 0, 0).unwrap() {
            Bound::Unparsed(u) => {
                assert_eq!(u.kind_byte, 6);
                assert_eq!(u.header.bound_type, BoundType::Grid);
            }
            other => panic!("want unparsed, got {other:?}"),
        }
        let sys = base(42);
        assert!(matches!(
            parse_bound(&sys, 0, 0).unwrap(),
            Bound::Unparsed(_)
        ));
    }

    #[test]
    fn truncated_base_is_an_error_not_a_panic() {
        let sys = base(0);
        let err = parse_bound(&sys[..64], 0, 0).unwrap_err();
        assert!(matches!(err, Error::Truncated { .. }));
    }
}
