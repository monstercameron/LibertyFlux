// original: 0x009A3630 audRadioAudioEntity::vf0

/// Deleting destructor of the radio audio entity.
///
/// Retargets the object to the base layouts (two relocated table addresses
/// at +0 and +8), runs the base teardown (callee 1, thiscall/0) and, when
/// the low bit of the flags word is set, releases the object itself through
/// the heap free (callee 2, cdecl/1). Returns the object pointer in all
/// cases. Thiscall with one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_009A3630(this: u32, flags: u32) -> u32 {
    unsafe {
        const INNER_TABLE: u32 = 0x00E8E23C;
        const BASE_TABLE: u32 = 0x00E83134;
        const BASE_TEARDOWN: u32 = 1;
        const FREE_CALLEE: u32 = 2;
        const DELETE_FLAG: u32 = 1;
        ((this.wrapping_add(8)) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(INNER_TABLE));
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(BASE_TABLE));
        lf_checker_rt::callee_thiscall!(BASE_TEARDOWN, u32, this);
        if (flags & DELETE_FLAG) != 0 {
            lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, this);
        }
        this
    }
});
