// original: 0x006681B0 rage::ptxEventEmitter::vf3

/// Calls this object's virtual methods at slots +0x10, +0x14, and +0x18 in order. Each call receives the two incoming words and the third incoming word as raw single-precision bits; the bits are forwarded unchanged and no float operation is done. The first two callee results are ignored, while the third becomes this function's result. Each callee is thiscall and cleans its three stack arguments.
lf_checker_rt::export!(thiscall, rw_006681B0(this: u32, first_word: u32, second_word: u32, raw_float_bits: u32) -> u32 {
    const VTABLE: u32 = 0;
    const FIRST_SLOT: u32 = 0x10;
    const SECOND_SLOT: u32 = 0x14;
    const THIRD_SLOT: u32 = 0x18;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    let vtable = unsafe { read_u32(this.wrapping_add(VTABLE)) };
    let first_target = unsafe { read_u32(vtable.wrapping_add(FIRST_SLOT)) };
    let first: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(first_target as usize) };
    let _first_result = first(this, first_word, second_word, raw_float_bits);
    let second_target = unsafe { read_u32(vtable.wrapping_add(SECOND_SLOT)) };
    let second: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(second_target as usize) };
    let _second_result = second(this, first_word, second_word, raw_float_bits);
    let third_target = unsafe { read_u32(vtable.wrapping_add(THIRD_SLOT)) };
    let third: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(third_target as usize) };
    third(this, first_word, second_word, raw_float_bits)
});
