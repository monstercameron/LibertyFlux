// original: 0x005d4ec0 CPhysical::is_physical_flag3_clear
/// Return one when the physical object's word at offset 0x118 has flag bit 3
/// clear, and zero when the bit is set. The method takes `this` in ECX and
/// returns the predicate in EAX.
lf_checker_rt::export!(thiscall, rw_005d4ec0(this: u32) -> u32 {
    const FLAGS_WORD_OFFSET: u32 = 0x118;
    const PHYSICAL_FLAG_3: u32 = 1 << 3;

    unsafe {
        let flags = (this.wrapping_add(FLAGS_WORD_OFFSET) as *const u32)
            .read_unaligned();
        u32::from(flags & PHYSICAL_FLAG_3 == 0)
    }
});
