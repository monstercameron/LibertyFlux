// original: 0x00CAB490 event_vec_construct (proposed)

/// Construct a vector-carrying event record in place and return its address.
///
/// Runs the base constructor on `this`, installs the record's virtual table,
/// stores the integer argument at `+0x14` and the two float arguments (as bit
/// patterns) at `+0x30` and `+0x34`, copies three words from the vector
/// argument into `+0x20`..`+0x28`, and stores the last two integer arguments
/// at `+0x38` and `+0x3C`. All float movement is bitwise.
///
/// Original: 0x00CAB490 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00cab490(this: u32, tag: u32, vec: u32, f0_bits: u32, f1_bits: u32, aux0: u32, aux1: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const VTABLE: u32 = 0x00ED886C;
        const TAG: u32 = 0x14;
        const VEC_DST: u32 = 0x20;
        const F0: u32 = 0x30;
        const F1: u32 = 0x34;
        const AUX0: u32 = 0x38;
        const AUX1: u32 = 0x3C;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this.wrapping_add(TAG) as *mut u32).write_unaligned(tag);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let mut i = 0u32;
        while i < 3 {
            let w = (vec.wrapping_add(i * 4) as *const u32).read_unaligned();
            (this.wrapping_add(VEC_DST + i * 4) as *mut u32).write_unaligned(w);
            i += 1;
        }
        (this.wrapping_add(AUX0) as *mut u32).write_unaligned(aux0);
        (this.wrapping_add(F0) as *mut u32).write_unaligned(f0_bits);
        (this.wrapping_add(F1) as *mut u32).write_unaligned(f1_bits);
        (this.wrapping_add(AUX1) as *mut u32).write_unaligned(aux1);
        this
    }
});
