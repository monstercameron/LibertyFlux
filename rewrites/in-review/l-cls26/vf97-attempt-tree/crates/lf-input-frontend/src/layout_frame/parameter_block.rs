//! The parameter-block refresh of the layout frame.
//!
//! The frame keeps a pointer to a small parameter block in its cache word.
//! When the frame has no block yet, the refresh allocates one, writes its
//! initial words, and then writes the refreshed words with a fresh stamp from
//! the frame. When the frame already has a block, the refresh asks the frame
//! for its current stamp and compares it with the block's stamp word; a match
//! takes a fresh stamp and writes the refreshed words, and a mismatch leaves
//! the block alone. The answer is the stamp used, or the current stamp when
//! the refresh was skipped.
//!
//! The block is memory the frame does not own, so the methods take a
//! [`ParameterBlockHost`] for it, and for the frame's own stamp query and the
//! allocator. The words are written in the order the original writes them.

use super::frame::{UILayoutFrame, field};

/// Size in bytes of the parameter block the allocator is asked for.
pub const BLOCK_BYTES: u32 = 0x14;

/// Index of the words the block holds, in the 32-bit word order of the block.
const WORD_FRACTION: usize = 0;
const WORD_COUNT: usize = 1;
const WORD_STAMP: usize = 2;
const WORD_ZERO_A: usize = 3;
const WORD_ZERO_B: usize = 4;

/// The calls the refresh makes outside the frame's bytes.
pub trait ParameterBlockHost {
    /// The frame's own stamp query. The frame is its receiver.
    fn frame_stamp(&mut self, frame: &mut UILayoutFrame) -> u32;

    /// The allocator, asked for a block of `size` bytes. Answers the block,
    /// or zero when it could not allocate.
    fn allocate_block(&mut self, size: u32) -> u32;

    /// Writes one word of a block.
    fn write_block_word(&mut self, block: u32, index: usize, value: u32);

    /// Reads one word of a block.
    fn read_block_word(&mut self, block: u32, index: usize) -> u32;
}

impl UILayoutFrame {
    /// Refreshes the frame's parameter block. See the module documentation for
    /// the two paths. The answer is the stamp the refresh used, or the current
    /// stamp when a block already exists with a different one.
    pub fn refresh_parameter_block<H: ParameterBlockHost>(
        &mut self,
        host: &mut H,
        fraction_bits: u32,
        count: u32,
    ) -> u32 {
        if self.word_at(field::CACHE_EC) == 0 {
            let block = host.allocate_block(BLOCK_BYTES);
            if block == 0 {
                self.set_word_at(field::CACHE_EC, 0);
            } else {
                host.write_block_word(block, WORD_FRACTION, 0);
                host.write_block_word(block, WORD_COUNT, 1);
                host.write_block_word(block, WORD_STAMP, 0);
                host.write_block_word(block, WORD_ZERO_A, 0);
                host.write_block_word(block, WORD_ZERO_B, 0);
                self.set_word_at(field::CACHE_EC, block);
            }
            let fresh = host.frame_stamp(self);
            let block = self.word_at(field::CACHE_EC);
            write_refreshed(host, block, fresh, count, fraction_bits);
            fresh
        } else {
            let current = host.frame_stamp(self);
            let block = self.word_at(field::CACHE_EC);
            if host.read_block_word(block, WORD_STAMP) == current {
                let fresh = host.frame_stamp(self);
                write_refreshed(host, block, fresh, count, fraction_bits);
                fresh
            } else {
                current
            }
        }
    }
}

/// Writes the refreshed words in the original's order: the two zero words,
/// the stamp, the count, then the fraction bits.
fn write_refreshed<H: ParameterBlockHost>(
    host: &mut H,
    block: u32,
    stamp: u32,
    count: u32,
    fraction_bits: u32,
) {
    host.write_block_word(block, WORD_ZERO_B, 0);
    host.write_block_word(block, WORD_ZERO_A, 0);
    host.write_block_word(block, WORD_STAMP, stamp);
    host.write_block_word(block, WORD_COUNT, count);
    host.write_block_word(block, WORD_FRACTION, fraction_bits);
}
