// original: 0x00CAB720 event_aim2_construct (proposed)

/// Construct a reference-carrying aiming event in place, return its address.
///
/// Runs the base constructor on `this`, installs the record's virtual table,
/// copies three words from the vector argument into `+0x20`..`+0x28`,
/// stores the integer argument at `+0x30`, the float argument (bitwise) at
/// `+0x34`, the reference argument at `+0x3C` and the last integer argument
/// at `+0x40`, zeroes `+0x38`, `+0x44`, `+0x48` and `+0x4C`, and clears the
/// low two flag bits at `+0x50`. When the reference is non-null it is passed
/// (pointer and value) to the reference keeper. All float movement is
/// bitwise.
///
/// Original: 0x00CAB720 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00cab720(this: u32, tag: u32, vec: u32, aref: u32, f_bits: u32, aux: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const KEEPER: u32 = 2;
        const VTABLE: u32 = 0x00ED88C4;
        const VEC_DST: u32 = 0x20;
        const TAG: u32 = 0x30;
        const F_SLOT: u32 = 0x34;
        const AREF: u32 = 0x3C;
        const AUX: u32 = 0x40;
        const FLAGS: u32 = 0x50;
        const FLAGS_KEEP: u32 = 0xFFFF_FFFC;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let mut i = 0u32;
        while i < 3 {
            let w = (vec.wrapping_add(i * 4) as *const u32).read_unaligned();
            (this.wrapping_add(VEC_DST + i * 4) as *mut u32).write_unaligned(w);
            i += 1;
        }
        (this.wrapping_add(TAG) as *mut u32).write_unaligned(tag);
        (this.wrapping_add(F_SLOT) as *mut u32).write_unaligned(f_bits);
        (this.wrapping_add(AREF) as *mut u32).write_unaligned(aref);
        (this.wrapping_add(0x38) as *mut u32).write_unaligned(0);
        (this.wrapping_add(AUX) as *mut u32).write_unaligned(aux);
        (this.wrapping_add(0x44) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x48) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x4C) as *mut u16).write_unaligned(0);
        let flags = (this.wrapping_add(FLAGS) as *const u32).read_unaligned();
        (this.wrapping_add(FLAGS) as *mut u32).write_unaligned(flags & FLAGS_KEEP);
        let kept = (this.wrapping_add(AREF) as *const u32).read_unaligned();
        if kept != 0 {
            lf_checker_rt::callee_thiscall!(KEEPER, u32, kept, this.wrapping_add(AREF));
        }
        this
    }
});
