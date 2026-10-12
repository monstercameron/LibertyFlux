// original: 0x00c257c0 CPortalInst::get_bounds
/// Copies the portal's two bounds vectors and their packed fields to the caller's 32-byte output buffer. The floating-point words are copied as raw bits, and EAX returns the final packed field.
lf_checker_rt::export!(thiscall, rw_00c257c0(this: u32, out: u32) -> u32 {
    unsafe {
            let source = this as usize as *const u8;
            let destination = out as usize as *mut u8;
            const SOURCE_0: usize = 0x90;
            const OUTPUT_0: usize = 0x0;
            let word_0 = core::ptr::read_unaligned(source.add(0x90) as *const u32);
            core::ptr::write_unaligned(destination.add(OUTPUT_0) as *mut u32, word_0);
            const SOURCE_1: usize = 0x94;
            const OUTPUT_1: usize = 0x4;
            let word_1 = core::ptr::read_unaligned(source.add(0x94) as *const u32);
            core::ptr::write_unaligned(destination.add(OUTPUT_1) as *mut u32, word_1);
            const SOURCE_2: usize = 0x98;
            const OUTPUT_2: usize = 0x8;
            let word_2 = core::ptr::read_unaligned(source.add(0x98) as *const u32);
            core::ptr::write_unaligned(destination.add(OUTPUT_2) as *mut u32, word_2);
            const SOURCE_3: usize = 0x9c;
            const OUTPUT_3: usize = 0xc;
            let word_3 = core::ptr::read_unaligned(source.add(0x9c) as *const u32);
            core::ptr::write_unaligned(destination.add(OUTPUT_3) as *mut u32, word_3);
            const SOURCE_4: usize = 0xa0;
            const OUTPUT_4: usize = 0x10;
            let word_4 = core::ptr::read_unaligned(source.add(0xa0) as *const u32);
            core::ptr::write_unaligned(destination.add(OUTPUT_4) as *mut u32, word_4);
            const SOURCE_5: usize = 0xa4;
            const OUTPUT_5: usize = 0x14;
            let word_5 = core::ptr::read_unaligned(source.add(0xa4) as *const u32);
            core::ptr::write_unaligned(destination.add(OUTPUT_5) as *mut u32, word_5);
            const SOURCE_6: usize = 0xa8;
            const OUTPUT_6: usize = 0x18;
            let word_6 = core::ptr::read_unaligned(source.add(0xa8) as *const u32);
            core::ptr::write_unaligned(destination.add(OUTPUT_6) as *mut u32, word_6);
            const SOURCE_7: usize = 0xac;
            const OUTPUT_7: usize = 0x1c;
            let word_7 = core::ptr::read_unaligned(source.add(0xac) as *const u32);
            core::ptr::write_unaligned(destination.add(OUTPUT_7) as *mut u32, word_7);
        }
        core::ptr::read_unaligned(source.add(0xac) as *const u32)
});
