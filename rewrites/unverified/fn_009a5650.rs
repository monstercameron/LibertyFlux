// original: 0x009A5650 audio_register_voice_or_slot (proposed)

/// Register a voice handle, parking it in the free list on failure.
///
/// The provider (callee 1, thiscall/0) answers a record whose word at +4 is
/// the voice handle. The register call (callee 2, thiscall/1) takes it: a
/// non-negative (SIGNED) answer is final and returned as is. On a negative
/// answer the handle parks in the first zero word of the twenty-entry table
/// at `this`+0x3A10 (entries are eight bytes, only the first word is the
/// marker; a full table parks nowhere), and when the argument byte is
/// nonzero and the selected index at `this`+0x3ABC is still -1, the parked
/// position (or 20 when full) is selected. That position is returned.
/// Thiscall with one stack word (only its low byte is read), callee pops 4.
lf_checker_rt::export!(thiscall, rw_009A5650(this: u32, arg: u32) -> u32 {
    unsafe {
        const HANDLE_OFF: u32 = 4;
        const TABLE: u32 = 0x3A10;
        const ENTRY_STRIDE: u32 = 8;
        const TABLE_LEN: u32 = 20;
        const SELECTED: u32 = 0x3ABC;
        const NONE: u32 = 0xFFFF_FFFF;
        const PROVIDER_CALLEE: u32 = 1;
        const REGISTER_CALLEE: u32 = 2;
        let rec = lf_checker_rt::callee_thiscall!(PROVIDER_CALLEE, u32, this);
        let handle = ((rec.wrapping_add(HANDLE_OFF)) as *const u32).read_unaligned();
        let ans = lf_checker_rt::callee_thiscall!(REGISTER_CALLEE, u32, this, handle);
        // Signed non-negative test, as the original's `jns`.
        if (ans as i32) >= 0 {
            return ans;
        }
        let mut pos: u32 = 0;
        while pos < TABLE_LEN {
            let mark = (this.wrapping_add(TABLE).wrapping_add(pos * ENTRY_STRIDE)
                        as *const u32).read_unaligned();
            if mark == 0 {
                break;
            }
            pos += 1;
        }
        if pos < TABLE_LEN {
            (this.wrapping_add(TABLE).wrapping_add(pos * ENTRY_STRIDE) as *mut u32)
                .write_unaligned(handle);
        }
        if (arg as u8) != 0 {
            let selp = this.wrapping_add(SELECTED) as *mut u32;
            if selp.read_unaligned() == NONE {
                selp.write_unaligned(pos);
            }
        }
        pos
    }
});
