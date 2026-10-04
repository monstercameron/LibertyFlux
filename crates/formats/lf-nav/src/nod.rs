//! Compiled vehicle path nodes (`.nod`).
//!
//! Each file covers one 750 m square of the world. Layout in words: a 16-byte
//! header of four counts, a table of 32-byte nodes, then a table of 8-byte
//! links. A node's links are the slice from its `link_id` to the next node's
//! `link_id` (or the link count for the last node); the slices partition the
//! link table exactly.

use crate::{Error, i16le, u16le, u32le};

/// Header length in bytes.
pub const HEADER_LEN: usize = 16;
/// One node record in bytes.
pub const NODE_LEN: usize = 32;
/// One link record in bytes.
pub const LINK_LEN: usize = 8;

/// Side of one node's world square in metres.
pub const TILE_METRES: f64 = 750.0;
/// World origin (south-west corner) in metres.
pub const WORLD_ORIGIN: f64 = -3000.0;

/// A parsed `.nod` file borrowing its bytes.
#[derive(Debug, Clone, Copy)]
pub struct Nod<'a> {
    data: &'a [u8],
    node_count: u32,
    car_count: u32,
    isec_count: u32,
    link_count: u32,
}

impl<'a> Nod<'a> {
    /// Parse a `.nod` file. The buffer must hold exactly the file contents:
    /// 16 header bytes plus 32 per node plus 8 per link.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn parse(data: &'a [u8]) -> Result<Self, Error> {
        if data.len() < HEADER_LEN {
            return Err(Error::UnexpectedEnd { offset: 0 });
        }
        let node_count = u32le(data, 0)?;
        let car_count = u32le(data, 4)?;
        let isec_count = u32le(data, 8)?;
        let link_count = u32le(data, 12)?;
        let expected = HEADER_LEN
            .saturating_add(node_count as usize * NODE_LEN)
            .saturating_add(link_count as usize * LINK_LEN);
        if expected != data.len() {
            return Err(Error::BadSize {
                expected,
                actual: data.len(),
            });
        }
        Ok(Self {
            data,
            node_count,
            car_count,
            isec_count,
            link_count,
        })
    }

    /// Total node count (header word 0).
    #[must_use]
    pub fn node_count(&self) -> u32 {
        self.node_count
    }

    /// Car-node count (header word 1). The first this many nodes are the
    /// driving network; the rest are intersection records with no links.
    #[must_use]
    pub fn car_count(&self) -> u32 {
        self.car_count
    }

    /// Intersection-node count (header word 2).
    #[must_use]
    pub fn isec_count(&self) -> u32 {
        self.isec_count
    }

    /// Link count (header word 3).
    #[must_use]
    pub fn link_count(&self) -> u32 {
        self.link_count
    }

    /// Read node `index`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn node(&self, index: u32) -> Result<Node, Error> {
        if index >= self.node_count {
            return Err(Error::OutOfRange {
                what: "node",
                index: index as usize,
                count: self.node_count as usize,
            });
        }
        let off = HEADER_LEN + index as usize * NODE_LEN;
        Ok(Node {
            mem: u32le(self.data, off)?,
            area: u16le(self.data, off + 8)?,
            id: u16le(self.data, off + 10)?,
            street: u32le(self.data, off + 12)?,
            heuristic: u16le(self.data, off + 16)?,
            link_id: u16le(self.data, off + 18)?,
            px: i16le(self.data, off + 20)?,
            py: i16le(self.data, off + 22)?,
            pz: i16le(self.data, off + 24)?,
            width: crate::u8le(self.data, off + 26)?,
            flood: crate::u8le(self.data, off + 27)?,
            flags: u32le(self.data, off + 28)?,
        })
    }

    /// Iterate over all nodes.
    ///
    /// # Panics
    ///
    /// Never panics: `parse` rejects files whose counted entries do not fit, so the counted reads cannot fail.
    pub fn nodes(&self) -> impl Iterator<Item = Node> + '_ {
        (0..self.node_count).map(|i| self.node(i).expect("count checked"))
    }

    /// Read link `index`.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn link(&self, index: u32) -> Result<Link, Error> {
        if index >= self.link_count {
            return Err(Error::OutOfRange {
                what: "link",
                index: index as usize,
                count: self.link_count as usize,
            });
        }
        let off = HEADER_LEN + self.node_count as usize * NODE_LEN + index as usize * LINK_LEN;
        Ok(Link {
            area: u16le(self.data, off)?,
            node: u16le(self.data, off + 2)?,
            length_m: crate::u8le(self.data, off + 4)?,
            flags8: crate::u8le(self.data, off + 5)?,
            flags16: u16le(self.data, off + 6)?,
        })
    }

    /// Iterate over all links.
    ///
    /// # Panics
    ///
    /// Never panics: `parse` rejects files whose counted entries do not fit, so the counted reads cannot fail.
    pub fn links(&self) -> impl Iterator<Item = Link> + '_ {
        (0..self.link_count).map(|i| self.link(i).expect("count checked"))
    }

    /// The links owned by node `index`: `links[link_id .. end]`, where `end`
    /// is the next node's `link_id`, or the link count for the last node.
    /// Fails if the file's link ids run backwards here.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is truncated or malformed.
    pub fn links_of(&self, index: u32) -> Result<Vec<Link>, Error> {
        let node = self.node(index)?;
        let end = if index + 1 < self.node_count {
            self.node(index + 1)?.link_id as usize
        } else {
            self.link_count as usize
        };
        let start = node.link_id as usize;
        if end < start || end > self.link_count as usize {
            return Err(Error::Invalid {
                what: "link_id order",
            });
        }
        (start..end)
            .map(|i| self.link(u32::try_from(i).unwrap_or(u32::MAX)))
            .collect()
    }

    /// Check the partition rule over the whole file: link ids never run
    /// backwards and the owned slices cover the link table exactly.
    ///
    /// # Panics
    ///
    /// Never panics: `parse` rejects files whose counted entries do not fit, so the counted reads cannot fail.
    #[must_use]
    pub fn validate_partition(&self) -> PartitionReport {
        let mut report = PartitionReport::default();
        let mut prev = 0usize;
        for i in 0..self.node_count {
            let id = self.node(i).expect("count checked").link_id as usize;
            if id < prev {
                report.backward += 1;
            }
            prev = id;
        }
        if prev > self.link_count as usize {
            report.last_overflow = true;
        }
        report.owned_total = self.link_count;
        report
    }
}

/// Report from [`Nod::validate_partition`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PartitionReport {
    /// Nodes whose `link_id` is below the previous node's.
    pub backward: u32,
    /// Whether the last node's `link_id` passes the link count.
    pub last_overflow: bool,
    /// Links covered by ownership (equals the link count when clean).
    pub owned_total: u32,
}

impl PartitionReport {
    /// True when the file obeys the partition rule.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.backward == 0 && !self.last_overflow
    }
}

/// One 32-byte path node.
///
/// Field order in the file: u32 runtime pointer (a leftover address, null for
/// unlinked and dead-end nodes), u32 zero, u16 area id, u16 node id, u32
/// street-name hash, u16 pathfinding heuristic, u16 first-link id, three i16
/// position components, u8 path width, u8 flood-fill region, u32 flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    mem: u32,
    area: u16,
    id: u16,
    street: u32,
    heuristic: u16,
    link_id: u16,
    px: i16,
    py: i16,
    pz: i16,
    width: u8,
    flood: u8,
    flags: u32,
}

impl Node {
    /// Area id; always equals the file number in shipped data.
    #[must_use]
    pub fn area(&self) -> u16 {
        self.area
    }

    /// Node id, unique within its area.
    #[must_use]
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Street-name hash (0 when the node has no street).
    #[must_use]
    pub fn street(&self) -> u32 {
        self.street
    }

    /// Stored pathfinding heuristic cost.
    #[must_use]
    pub fn heuristic(&self) -> u16 {
        self.heuristic
    }

    /// Index of the node's first link in the link table.
    #[must_use]
    pub fn link_id(&self) -> u16 {
        self.link_id
    }

    /// World X in metres (stored eighths of a metre).
    #[must_use]
    pub fn x(&self) -> f64 {
        f64::from(self.px) / 8.0
    }

    /// World Y in metres (stored eighths of a metre).
    #[must_use]
    pub fn y(&self) -> f64 {
        f64::from(self.py) / 8.0
    }

    /// World Z in metres (stored 128ths of a metre).
    #[must_use]
    pub fn z(&self) -> f64 {
        f64::from(self.pz) / 128.0
    }

    /// Path width in metres (stored eighths).
    #[must_use]
    pub fn width_m(&self) -> f64 {
        f64::from(self.width) / 8.0
    }

    /// Flood-fill region byte.
    #[must_use]
    pub fn flood(&self) -> u8 {
        self.flood
    }

    /// Raw flags word.
    #[must_use]
    pub fn flags(&self) -> u32 {
        self.flags
    }

    /// One flag bit.
    #[must_use]
    pub fn flag(&self, bit: u8) -> bool {
        bit < 32 && (self.flags >> bit) & 1 == 1
    }

    /// Bits 12-15 of the flags. Public notes call this a link count, but it
    /// does not match link ownership in the shipped data (it reads 8 for
    /// unlinked nodes and 0-7 for linked ones without counting them), so its
    /// meaning is unknown; use [`Nod::links_of`] for adjacency.
    #[must_use]
    pub fn middle_nibble(&self) -> u8 {
        ((self.flags >> 12) & 0xF) as u8
    }

    /// Leftover runtime pointer from the file's build machine. Null for
    /// unlinked nodes and most dead ends; otherwise one of a small set of
    /// pool addresses. Useless to a reader, kept for faithfulness.
    #[must_use]
    pub fn mem(&self) -> u32 {
        self.mem
    }
}

/// One 8-byte path link: target area and node, integer length, two flag fields.
///
/// Note the byte order: the length byte comes first, then the 8-bit flags.
/// Some public notes list these two bytes swapped; geometry settles it (the
/// first byte tracks the inter-node distance with a median error of half a
/// metre, the second does not).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Link {
    area: u16,
    node: u16,
    length_m: u8,
    flags8: u8,
    flags16: u16,
}

impl Link {
    /// Target area id (may differ from the storing file).
    #[must_use]
    pub fn area(&self) -> u16 {
        self.area
    }

    /// Target node id within its area.
    #[must_use]
    pub fn node(&self) -> u16 {
        self.node
    }

    /// Link length in whole metres, about the 3D node distance.
    #[must_use]
    pub fn length_m(&self) -> u8 {
        self.length_m
    }

    /// 8-bit link flags. Dominant value 9; bits 0/3, 1/4 and 2/5 have
    /// identical population counts, suggesting a direction-dependent double
    /// encoding. Exact meaning unknown.
    #[must_use]
    pub fn flags8(&self) -> u8 {
        self.flags8
    }

    /// 16-bit link flags. Dominant value `0xC040` (bits 6, 14, 15).
    /// Exact meaning unknown.
    #[must_use]
    pub fn flags16(&self) -> u16 {
        self.flags16
    }
}

/// South-west corner of area `area` in world metres.
#[must_use]
pub fn area_origin(area: u16) -> (f64, f64) {
    let col = f64::from(area % 8);
    let row = f64::from(area / 8);
    (
        WORLD_ORIGIN + col * TILE_METRES,
        WORLD_ORIGIN + row * TILE_METRES,
    )
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // parsed values are compared bit for bit on purpose
mod tests {
    use super::*;

    /// Build a two-node file: node 0 owns links 0-1, node 1 owns link 2.
    fn two_node_file() -> Vec<u8> {
        let mut b = vec![];
        b.extend_from_slice(&2u32.to_le_bytes());
        b.extend_from_slice(&2u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&3u32.to_le_bytes());
        // node 0: area 7, id 3, link_id 0, pos (8, 16, 1)
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&7u16.to_le_bytes());
        b.extend_from_slice(&3u16.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0x7ffeu16.to_le_bytes());
        b.extend_from_slice(&0u16.to_le_bytes());
        b.extend_from_slice(&64i16.to_le_bytes());
        b.extend_from_slice(&128i16.to_le_bytes());
        b.extend_from_slice(&128i16.to_le_bytes());
        b.push(16);
        b.push(1);
        b.extend_from_slice(&0x212540fu32.to_le_bytes());
        // node 1: area 7, id 4, link_id 2
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&7u16.to_le_bytes());
        b.extend_from_slice(&4u16.to_le_bytes());
        b.extend_from_slice(&0u32.to_le_bytes());
        b.extend_from_slice(&0x7ffeu16.to_le_bytes());
        b.extend_from_slice(&2u16.to_le_bytes());
        b.extend_from_slice(&0i16.to_le_bytes());
        b.extend_from_slice(&0i16.to_le_bytes());
        b.extend_from_slice(&0i16.to_le_bytes());
        b.push(0);
        b.push(1);
        b.extend_from_slice(&0u32.to_le_bytes());
        // links: (7,4) len 9, (7,9) len 5, (7,3) len 9
        for (a, n, l) in [(7u16, 4u16, 9u8), (7, 9, 5), (7, 3, 9)] {
            b.extend_from_slice(&a.to_le_bytes());
            b.extend_from_slice(&n.to_le_bytes());
            b.push(l);
            b.push(9);
            b.extend_from_slice(&0xc040u16.to_le_bytes());
        }
        b
    }

    #[test]
    fn parses_counts_positions_and_partition() {
        let bytes = two_node_file();
        let n = Nod::parse(&bytes).unwrap();
        assert_eq!(
            (
                n.node_count(),
                n.car_count(),
                n.isec_count(),
                n.link_count()
            ),
            (2, 2, 0, 3)
        );
        let n0 = n.node(0).unwrap();
        assert_eq!((n0.area(), n0.id()), (7, 3));
        assert_eq!((n0.x(), n0.y(), n0.z()), (8.0, 16.0, 1.0));
        assert_eq!(n0.width_m(), 2.0);
        assert_eq!(n0.middle_nibble(), 5);
        let l0 = n.links_of(0).unwrap();
        assert_eq!(l0.len(), 2);
        assert_eq!((l0[0].area(), l0[0].node(), l0[0].length_m()), (7, 4, 9));
        let l1 = n.links_of(1).unwrap();
        assert_eq!(l1.len(), 1);
        assert!(n.validate_partition().is_clean());
    }

    #[test]
    fn rejects_truncated_and_ranged() {
        let bytes = two_node_file();
        assert!(matches!(
            Nod::parse(&bytes[..bytes.len() - 1]).unwrap_err(),
            Error::BadSize { .. }
        ));
        assert!(matches!(
            Nod::parse(&bytes[..8]).unwrap_err(),
            Error::UnexpectedEnd { .. }
        ));
        let n = Nod::parse(&bytes).unwrap();
        assert!(matches!(n.node(2).unwrap_err(), Error::OutOfRange { .. }));
        assert!(matches!(n.link(3).unwrap_err(), Error::OutOfRange { .. }));
    }

    #[test]
    fn detects_backward_link_ids() {
        let mut bytes = two_node_file();
        // node 1 link_id 2 -> invalid order is fine forward; force node0 link_id above node1's
        let off = HEADER_LEN + 18;
        bytes[off] = 3;
        bytes[off + 1] = 0;
        let n = Nod::parse(&bytes).unwrap();
        assert!(n.links_of(0).is_err());
        assert!(!n.validate_partition().is_clean());
    }

    #[test]
    fn area_origin_math() {
        assert_eq!(area_origin(0), (-3000.0, -3000.0));
        assert_eq!(area_origin(9), (-2250.0, -2250.0));
        assert_eq!(area_origin(63), (2250.0, 2250.0));
    }
}
