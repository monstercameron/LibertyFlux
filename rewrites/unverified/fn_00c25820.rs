// original: 0x00c25820 CPortalInst::get_normal
/// Copies the portal normal's three raw component words to the caller's 12-byte output buffer and returns that buffer address in EAX.
lf_checker_rt::export!(thiscall, rw_00c25820(this: u32, out: u32) -> u32 {
    unsafe {
            let source = this as usize as *const u8;
            let destination = out as usize as *mut u8;
            const SOURCE_0: usize = 0x80;
            const OUTPUT_0: usize = 0x0;
            let word_0 = core::ptr::read_unaligned(source.add(0x80) as *const u32);
            core::ptr::write_unaligned(destination.add(OUTPUT_0) as *mut u32, word_0);
            const SOURCE_1: usize = 0x84;
            const OUTPUT_1: usize = 0x4;
            let word_1 = core::ptr::read_unaligned(source.add(0x84) as *const u32);
            core::ptr::write_unaligned(destination.add(OUTPUT_1) as *mut u32, word_1);
            const SOURCE_2: usize = 0x88;
            const OUTPUT_2: usize = 0x8;
            let word_2 = core::ptr::read_unaligned(source.add(0x88) as *const u32);
            core::ptr::write_unaligned(destination.add(OUTPUT_2) as *mut u32, word_2);
        }
        out
});
