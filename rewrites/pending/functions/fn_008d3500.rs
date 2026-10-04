// original: 0x008d3500 flag_set_bit4_from_arg
/// Sets bit 4 of the flag word (at +0x3d0 of the object's +0x228 record plus
/// 0x70) from bit 0 of the argument byte. Returns the xor mask applied.
/// (With a null +0x228 link the original faults on the flag read; the
/// rewrite performs the same read.)
export!(thiscall, rw_008d3500(this_: u32, value: u32) -> u32 {
    unsafe {
        let rec = *((this_ + 0x228) as *const u32);
        let base = if rec == 0 { 0 } else { rec + 0x70 };
        let slot = (base + 0x3d0) as *mut u32;
        let want = (value & 0xFF) << 4;
        let before = *slot;
        let flip = (want ^ before) & 0x10;
        *slot = before ^ flip;
        flip
    }
});
