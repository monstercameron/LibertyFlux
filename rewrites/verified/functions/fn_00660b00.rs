// original: 0x00660b00 gamer_slot_append
/// Append a gamer entry to the session roster and hand out its slot.
///
/// Copies the 16-byte record at `gamer` into row `count` of the roster,
/// stores the two id words from `ids` beside it, clears the row's flag
/// byte, records `tag` in the tag table, publishes the slot address
/// (`this+0x420+count*8`) through `slot_out`, bumps the count at
/// `+0x540`, and returns 1.
export!(thiscall, rw_00660b00(this: u32, gamer: u32, ids: u32, tag: u32, slot_out: u32) -> u32 {
    unsafe {
        let count = ((this + 0x540) as *const u32).read();
        let row = (count + 0xa) * 2;
        core::ptr::copy_nonoverlapping(
            gamer as *const u8,
            (this + row * 8) as *mut u8,
            16,
        );
        ((this + count * 8 + 0x2a0) as *mut u32).write((ids as *const u32).read());
        ((this + count * 8 + 0x2a4) as *mut u32)
            .write(((ids + 4) as *const u32).read());
        ((this + count + 0x520) as *mut u8).write(0);
        ((this + count * 4 + 0x3a0) as *mut u32).write(tag);
        (slot_out as *mut u32).write(this + 0x420 + count * 8);
        ((this + 0x540) as *mut u32).write(count.wrapping_add(1));
        (count.wrapping_add(1) & 0xffff_ff00) | 1
    }
});

