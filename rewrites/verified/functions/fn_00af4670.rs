// original: 0x00AF4670 drawable_destroy_all (proposed)

/// Destroy every entry of the drawable's slot set, then chain to base.
///
/// Stamps the drawable vtable pointer at `this+0`. When the 16-bit entry
/// count at `this+0x86` is nonzero, each entry (0x6C bytes apart from the
/// base at `this+0x80`) is destroyed through its own function table slot
/// 0 with argument 0, and the entry array is released; then the base
/// destructor is tail-called with this object. Returns the tail result.
///
/// Original: 0x00AF4670 (thiscall, no stack words, one table-indirect, one
/// direct and one tail callee).
lf_checker_rt::export!(thiscall, rw_00af4670(this: u32) -> u32 {
    unsafe {
        const FREE_CALLEE: u32 = 2;
        const BASE_CALLEE: u32 = 3;
        const VTABLE: u32 = 0x00EA839C;
        const SET_OFF: u32 = 0x80;
        const COUNT_OFF: u32 = 0x86;
        const ENTRY_STRIDE: u32 = 0x6C;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let count = ((this.wrapping_add(COUNT_OFF)) as *const u16).read_unaligned() as u32;
        if count != 0 {
            let arr = ((this.wrapping_add(SET_OFF)) as *const u32).read_unaligned();
            let mut i: u32 = 0;
            while i < count {
                let ent = arr.wrapping_add(i.wrapping_mul(ENTRY_STRIDE));
                let vt = ((ent) as *const u32).read_unaligned();
                let tgt = ((vt) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                f(ent, 0);
                i = i.wrapping_add(1);
            }
            lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, arr);
        }
        lf_checker_rt::callee_thiscall!(BASE_CALLEE, u32, this)
    }
});
