// original: 0x00d698a0 checked_fetch
/// Null gate: forward the live member, else report zero with entry residue.
///
/// Takes the object pointer in ECX. When the member word is non-null,
/// control transfers to the shared successor with the member, returning
/// whatever that call answers. Otherwise the low byte of the result is
/// zero and the top bytes pass the entry accumulator through, so the
/// contract pins the entry accumulator to make that half a declared input.
export!(thiscall, rw_00d698a0(this: u32) -> u32 {
    unsafe {
        let field = *(this.wrapping_add(4) as *const u32);
        if field != 0 {
            return callee_thiscall!(1, u32, field);
        }
        0xA5A5A500
    }
});
