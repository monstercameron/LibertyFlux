// original: 0x009e8610 model_pair_to_outs
/// Writes the sign-extended bytes at table entry `+0xB0`/`+0xB1`
/// (entry from the shared model table by the tag at `+0x2E`) to the
/// two out-pointers. Returns the second out-pointer, as the original
/// leaves it in eax. (thiscall, 2 args.)
lf_checker_rt::export!(thiscall, rw_009e8610(this_ptr: u32, out_a: u32, out_b: u32) -> u32 {
    unsafe {
        const TAG_OFF: u32 = 0x2E;
        const MODEL_TABLE: u32 = 0x1295CD8;
        const A_OFF: u32 = 0xB0;
        const B_OFF: u32 = 0xB1;
        let tag = (this_ptr.wrapping_add(TAG_OFF) as *const i16).read_unaligned() as i32;
        let entry = lf_checker_rt::global::<u32>(MODEL_TABLE)
            .wrapping_offset(tag as isize)
            .read_unaligned();
        let a = (entry.wrapping_add(A_OFF) as *const i8).read() as i32 as u32;
        let b = (entry.wrapping_add(B_OFF) as *const i8).read() as i32 as u32;
        (out_a as *mut u32).write_unaligned(a);
        (out_b as *mut u32).write_unaligned(b);
        out_b
    }
});
