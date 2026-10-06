// original: 0x00892DD0 audsound_voice_query_b
/// Tail-calls the voice's alternate query slot, or 0 when unbound.
///
/// Identical to the sibling query except for the table slot (+0x24): loads
/// `this+0x94`, its object, and its table; null at either link returns 0,
/// otherwise tail-jumps to slot +0x24. The stub is planted the same way.
/// Original: 0x00892DD0 (thiscall, no stack arguments).
export!(thiscall, rw_00892DD0(this: *mut u8) -> u32 {
    unsafe {
        const LINK: usize = 0x94;
        const SLOT: u32 = 0x24;
        let a = *(this.add(LINK) as *const u32);
        if a == 0 {
            return 0;
        }
        let inner = (a as *const u32).read_unaligned();
        if inner == 0 {
            return 0;
        }
        let vt = (inner as *const u32).read_unaligned();
        let target = ((vt.wrapping_add(SLOT)) as *const u32).read_unaligned();
        let f: extern "cdecl" fn() -> u32 = core::mem::transmute(target as usize);
        f()
    }
});
