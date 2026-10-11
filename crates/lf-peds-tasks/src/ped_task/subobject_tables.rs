//! The four sub-object tables of a ped task record, and the two routines
//! that walk them: the frame remap and the matrix transform.
//!
//! The record holds four tables of sub-objects. Table `k` (0 to 3) starts
//! [`OBJECT_BASE`]`[k]` bytes into the record and its objects are
//! [`OBJECT_STRIDE`]`[k]` bytes apart. Its signed slot count is a word at
//! `0x400 + 4k`, and each slot has a flag byte at `0x410 + k + slot`. The
//! flag bytes of the tables overlap: table 0's slot 1 and table 1's slot 0
//! are the same byte. The original lays them out this way, and this module
//! keeps the overlap by storing the flags as one run of bytes, indexed by
//! `table + slot`.
//!
//! A flagged slot is notified by its table's callee, a method on the
//! sub-object that receives the frame or matrix block the caller passed.
//! Both routines first call the record's own refresh hook, then walk the
//! four tables in order. The walk answers the number of slots table 0
//! walked, or, when a later table ran last, the position one past that
//! table's last object. Those answers are returned as [`WalkEnd`]; the
//! pointer form is rebuilt from the record's own position by the caller.
//!
//! The two routines differ in two places. The remap measures the record's
//! anchor point from the frame's offset and writes three dot products. The
//! transform blends the record's three output words through the matrix and
//! writes them back. The remap re-reads each table's count on every
//! iteration of its walk, while the transform reads each count once when
//! the table starts; that difference is [`CountReads`]. The sub-objects,
//! their methods and the refresh hook travel as [`SubobjectHooks`].
//!
//! The registry rows for these routines are in [`REGISTRY_ROWS`].

use super::registry::State;
use super::shape_registry::Row;

/// Number of tables in a task record.
pub const TABLE_COUNT: usize = 4;

/// Most slots one table may hold. The walk asserts a count within this
/// bound; the flag run has exactly the room for every table's largest slot.
pub const SLOT_CAPACITY: usize = 8;

/// Flag bytes in the overlapping run: the last table's index plus the
/// largest slot index.
pub const FLAG_BYTES: usize = TABLE_COUNT - 1 + SLOT_CAPACITY;

/// Record offset of table 0's count word. Table `k` counts at `+4k`.
pub const COUNT_OFFSET: usize = 0x400;

/// Record offset of the first flag byte.
pub const FLAG_OFFSET: usize = 0x410;

/// Record offset of the anchor point (three floats).
pub const ANCHOR_OFFSET: usize = 0x420;

/// Record offset of the three output floats.
pub const OUT_OFFSET: usize = 0x430;

/// Record offset of the word after the outputs.
pub const SPARE_OFFSET: usize = 0x43c;

/// Offset of each table's first object from the record.
pub const OBJECT_BASE: [u32; TABLE_COUNT] = [0x90, 0x140, 0x200, 0x2e0];

/// Distance between one table's objects.
pub const OBJECT_STRIDE: [u32; TABLE_COUNT] = [0xb0, 0xc0, 0xe0, 0x120];

/// The word the transform leaves after the outputs. The original stores a
/// word it never wrote in this routine, taken from its own uninitialised
/// stack scratch; the lift stores zero, which is what that scratch holds
/// under the checker's defined stack fill. The narrowing is in
/// [`REGISTRY_ROWS`].
pub const UNINITIALISED_SCRATCH_WORD: u32 = 0;

/// The record's fields these routines read and write, owned.
///
/// The sub-objects themselves are not here: their contents belong to their
/// own methods and are reached only through [`SubobjectHooks`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TaskSubobjects {
    /// The anchor point the remap measures from.
    pub anchor: [f32; 3],
    /// The three output floats: written by the remap and the transform, and
    /// read by the transform as its weights.
    pub out: [f32; 3],
    /// The word after the outputs. Only the transform writes it.
    pub spare: u32,
    /// Signed slot count of each table.
    pub counts: [i32; TABLE_COUNT],
    /// Flag bytes. Table `k`, slot `i` reads `flags[k + i]`.
    pub flags: [u8; FLAG_BYTES],
}

/// A three-by-three transform with a bias, as the frame and the matrix
/// blocks lay it out: row `r` holds three floats 16 bytes after the row
/// before it, and the bias follows the rows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AffineBlock {
    /// Row `r`, column `c`.
    pub rows: [[f32; 3]; 3],
    /// Bias added to each of the three results.
    pub bias: [f32; 3],
}

/// The sub-objects, their notify methods and the record's refresh hook.
///
/// The original calls each one by number with the sub-object or the record
/// as its object and one word of argument. Each call here carries the same
/// information: the table and slot name the object, and the handle is the
/// block the caller passed through unchanged. Methods may change the record
/// (its counts or flags), and the walk reads the record after each call, as
/// the original does.
pub trait SubobjectHooks {
    /// The block the caller passed. The lift only hands it on.
    type Handle: Copy;

    /// The record's refresh hook, called once after the record's own
    /// arithmetic and before the walk.
    fn refresh(&mut self, owner: &mut TaskSubobjects);

    /// Notifies slot `slot` of table `table` with `handle`.
    fn notify(
        &mut self,
        owner: &mut TaskSubobjects,
        table: usize,
        slot: usize,
        handle: Self::Handle,
    );
}

/// What the walk leaves as the routine's answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalkEnd {
    /// No table ran.
    None,
    /// Only table 0 ran, or the others were skipped after it: its answer
    /// is the number of slots it walked.
    Count(u32),
    /// A later table ran last: its answer is the position one past its last
    /// object, that is its base plus `objects` strides from the record.
    Object {
        /// The table that ran last.
        table: usize,
        /// Slots that table walked.
        objects: u32,
    },
}

/// How a table's count is read during its walk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountReads {
    /// Read again on every iteration (the frame remap).
    PerIteration,
    /// Read once when the table starts (the matrix transform).
    OncePerTable,
}

/// The position of slot `slot` of table `table` inside the record.
#[must_use]
pub fn object_offset(table: usize, slot: usize) -> u32 {
    OBJECT_BASE[table].wrapping_add((slot as u32).wrapping_mul(OBJECT_STRIDE[table]))
}

impl TaskSubobjects {
    /// Remaps the record's three output floats into the frame: the anchor
    /// minus the frame's offset, dotted with the frame's three rows in
    /// order. Then refreshes the record, walks the tables and returns the
    /// walk's answer.
    pub fn remap_vectors<H: SubobjectHooks>(
        &mut self,
        hooks: &mut H,
        frame: &AffineBlock,
        frame_handle: H::Handle,
    ) -> WalkEnd {
        let m = &frame.rows;
        let dx = sub(self.anchor[0], frame.bias[0]);
        let dy = sub(self.anchor[1], frame.bias[1]);
        let dz = sub(self.anchor[2], frame.bias[2]);
        self.out[0] = add(add(mul(m[0][1], dy), mul(m[0][0], dx)), mul(m[0][2], dz));
        self.out[1] = add(add(mul(m[1][1], dy), mul(dx, m[1][0])), mul(m[1][2], dz));
        self.out[2] = add(add(mul(m[2][1], dy), mul(dx, m[2][0])), mul(m[2][2], dz));
        hooks.refresh(self);
        self.walk(hooks, frame_handle, CountReads::PerIteration)
    }

    /// Blends the record's three output floats through the matrix and adds
    /// the bias, storing the three results back in the outputs and zero in
    /// the spare word. Then refreshes the record, walks the tables and
    /// returns the walk's answer.
    pub fn apply_matrix<H: SubobjectHooks>(
        &mut self,
        hooks: &mut H,
        matrix: &AffineBlock,
        matrix_handle: H::Handle,
    ) -> WalkEnd {
        let w = self.out;
        let m = &matrix.rows;
        let b = &matrix.bias;
        // Each result is the first column's weight times the middle output,
        // plus the first output's term, plus the last output's term, plus
        // the bias, in that order.
        let row = |r: usize| {
            add(
                add(
                    add(mul(m[1][r], w[1]), mul(m[0][r], w[0])),
                    mul(m[2][r], w[2]),
                ),
                b[r],
            )
        };
        self.out = [row(0), row(1), row(2)];
        self.spare = UNINITIALISED_SCRATCH_WORD;
        hooks.refresh(self);
        self.walk(hooks, matrix_handle, CountReads::OncePerTable)
    }

    /// The four-table walk shared by both routines. Table `k` runs only
    /// when its count is positive; each flagged slot is notified in order.
    fn walk<H: SubobjectHooks>(
        &mut self,
        hooks: &mut H,
        handle: H::Handle,
        reads: CountReads,
    ) -> WalkEnd {
        let mut end = WalkEnd::None;
        for table in 0..TABLE_COUNT {
            let first = self.counts[table];
            if first <= 0 {
                continue;
            }
            let mut walked: u32 = 0;
            loop {
                let slot = walked as usize;
                assert!(slot < SLOT_CAPACITY, "slot count above the table capacity");
                if self.flags[table + slot] != 0 {
                    hooks.notify(self, table, slot, handle);
                }
                walked += 1;
                let count = match reads {
                    CountReads::PerIteration => self.counts[table],
                    CountReads::OncePerTable => first,
                };
                assert!(
                    count <= SLOT_CAPACITY as i32,
                    "slot count above the table capacity"
                );
                if i64::from(walked) >= i64::from(count) {
                    break;
                }
            }
            end = if table == 0 {
                WalkEnd::Count(walked)
            } else {
                WalkEnd::Object {
                    table,
                    objects: walked,
                }
            };
        }
        end
    }
}

/// Multiplication with both operands pinned, so the operand order matches
/// the original's and a not-a-number keeps the same sign and payload.
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// Addition with both operands pinned, in the order given.
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

/// Subtraction with both operands pinned, in the order given.
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

/// Registry rows for the two routines of this module: what the proof
/// compares and what each one narrows.
pub const REGISTRY_ROWS: &[Row] = &[
    Row {
        class: "TaskSubobjects",
        method: "remap_vectors",
        state: State::Proven,
        narrows: &[
            "the record's sub-objects are not modelled: each notify and the refresh are compared as calls, with the object position rebuilt from the table and slot, and their own effects on the record are scripted as count or flag writes",
            "the scripted writes reach counts only in the range the table capacity allows; a count above the capacity is an assertion in the lift and is outside the proof",
            "the returned word is compared as the rewrite's answer rebuilt from the walk's answer: a count, a record position, or zero",
            "the frame block is a generated twelve-word block; its address is passed through as the handle and not read by the lift",
        ],
    },
    Row {
        class: "TaskSubobjects",
        method: "apply_matrix",
        state: State::Proven,
        narrows: &[
            "the stored spare word is zero: the original stores a word from its own uninitialised stack scratch, which the checker's defined stack fill makes zero; in the game that word is whatever the stack held, so the lift's zero is a narrowing",
            "the sub-objects, scripted count and flag writes and the returned word are compared as in the remap",
            "the transform's three results are compared bit for bit, with operand order pinned in the lift",
        ],
    },
];
