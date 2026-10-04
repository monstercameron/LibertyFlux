// original: 0x00698f10 unknown (quantized channel pointer rebinder)
/// Rebinds a quantized channel onto a new owner: stamps the vtable, then
/// for each of the three sub-object offsets (+0x08, +0x14, +0x20) that is
/// non-null, asks the new owner to relocate it and adds the returned delta.
/// Returns this.
lf_k2_rt::export!(thiscall, rw_00698f10(this: *mut u8, target: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = lf_k2_rt::relocated(0x00FE3B34);
        for off in [8usize, 0x14, 0x20] {
            let cur = *((this.add(off)) as *const u32);
            if cur != 0 {
                let delta: u32 = lf_k2_rt::callee_thiscall!(1, u32, target, cur);
                *((this.add(off)) as *mut u32) = cur.wrapping_add(delta);
            }
        }
        this as u32
    }
});
