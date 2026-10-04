// original: 0x009e74e0 ped_frame_or_slot
/// Resolves the live source (virtual slot `+0xA0`, refined through
/// slot `+0xE0`, else the fallback at `+0x100`): with none, publishes
/// the identity-plus-position frame to the shared matrix global and
/// returns its address; otherwise returns entry `index` (stride 64)
/// past the source's table at `+0x14`. (thiscall, 1 arg.)
lf_checker_rt::export!(thiscall, rw_009e74e0(this_ptr: u32, index: u32) -> u32 {
    unsafe {
        const SLOT_A: u32 = 0xA0;
        const SLOT_B: u32 = 0xE0;
        const FALLBACK_OFF: u32 = 0x100;
        const POS_LINK_OFF: u32 = 0x20;
        const TABLE_OFF: u32 = 0x14;
        const MATRIX: u32 = 0x1632C20;
        const ONE_BITS: u32 = 0x3F800000;
        let vt = (this_ptr as *const u32).read_unaligned();
        let slot_a = (vt.wrapping_add(SLOT_A) as *const u32).read_unaligned();
        let fa: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot_a as usize) };
        let src = if fa(this_ptr) == 0 {
            (this_ptr.wrapping_add(FALLBACK_OFF) as *const u32).read_unaligned()
        } else {
            let a = fa(this_ptr);
            let vt2 = (a as *const u32).read_unaligned();
            let slot_b = (vt2.wrapping_add(SLOT_B) as *const u32).read_unaligned();
            let fb: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(slot_b as usize) };
            fb(a)
        };
        if src == 0 {
            let m = lf_checker_rt::relocated(MATRIX);
            (m as *mut u32).write_unaligned(ONE_BITS);
            (m.wrapping_add(4) as *mut u32).write_unaligned(0);
            (m.wrapping_add(8) as *mut u32).write_unaligned(0);
            (m.wrapping_add(0x10) as *mut u32).write_unaligned(0);
            (m.wrapping_add(0x14) as *mut u32).write_unaligned(ONE_BITS);
            (m.wrapping_add(0x18) as *mut u32).write_unaligned(0);
            (m.wrapping_add(0x20) as *mut u32).write_unaligned(0);
            (m.wrapping_add(0x24) as *mut u32).write_unaligned(0);
            (m.wrapping_add(0x28) as *mut u32).write_unaligned(ONE_BITS);
            let base = (this_ptr.wrapping_add(POS_LINK_OFF) as *const u32).read_unaligned();
            let psrc = if base == 0 {
                this_ptr.wrapping_add(0x10)
            } else {
                base.wrapping_add(0x30)
            };
            for i in 0..3u32 {
                let w = (psrc.wrapping_add(i * 4) as *const u32).read_unaligned();
                (m.wrapping_add(0x30 + i * 4) as *mut u32).write_unaligned(w);
            }
            m
        } else {
            let table = (src.wrapping_add(TABLE_OFF) as *const u32).read_unaligned();
            table.wrapping_add(index.wrapping_shl(6))
        }
    }
});
