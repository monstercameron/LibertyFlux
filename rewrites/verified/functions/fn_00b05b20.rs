// original: 0x00b05b20 dispatch_indexed_jump
/// Flagged dispatch: forward zero or a scaled index, chosen by a tag bit.
///
/// Takes the object pointer in ECX and a table pointer on the stack. Reads
/// the tag byte selected by the object's index word: when its top bit is
/// set, control transfers to the shared dispatcher with a zero word.
/// Otherwise the result word is the object's scale times the table pointer
/// plus the object's base, and that word is forwarded instead. The incoming
/// slot is overwritten in place on both paths, so the stack comparison is
/// off and the forwarded word is compared through the call log instead.
export!(thiscall, rw_00b05b20(this: u32, table: u32) -> u32 {
    unsafe {
        let idx = *(this.wrapping_add(4) as *const u32);
        let tag = *(table.wrapping_add(idx) as *const u8);
        if tag & 0x80 != 0 {
            return callee_thiscall!(1, u32, this, 0);
        }
        let scale = *(this.wrapping_add(0xC) as *const u32);
        let base = *(this as *const u32);
        let out = scale.wrapping_mul(table).wrapping_add(base);
        callee_thiscall!(1, u32, this, out)
    }
});
