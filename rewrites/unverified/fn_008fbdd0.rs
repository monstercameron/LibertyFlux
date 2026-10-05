// original: 0x008FBDD0 MenuOrHud::SetFloatField9B8
/// Store a float argument into the field at `+0x9b8`.
///
/// Bit-exact float move with no arithmetic. Thiscall, one stack
/// argument; the original returns whatever was in eax on entry, so
/// the return value is not compared (see the contract).
export!(thiscall, rw_008fbdd0(this: u32, v: u32) -> u32 {
    unsafe {
        const FIELD: u32 = 0x9b8;
        ((this + FIELD) as *mut u32).write_unaligned(v);
        0
    }
});
