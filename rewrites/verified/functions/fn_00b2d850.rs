// original: 0x00b2d850 garage_record_apply
// 0xB2D850 garage_record_apply (thiscall/1).
//
// Looks up the handle for this record (falling back to the default source
// when the lookup misses) and applies it together with the given level.
export!(thiscall, rw_00b2d850(this: u32, level: f32) -> () {
    unsafe {
        let lookup: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let fallback: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let apply: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        let handle = if lookup(0) != 0 { lookup(0) } else { fallback(0) };
        apply(this, handle, level.to_bits());
    }
});
