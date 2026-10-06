// original: 0x00892DB0 audsound_voice_query_a
/// Tail-calls the voice's query slot, or answers 0 when no voice is bound.
///
/// Loads the chain `this+0x94`, its object, and its table; when either link
/// is null returns 0, otherwise tail-jumps to the table slot at +0x20 (a
/// no-argument query returning a dword). The checker intercepts the computed
/// jump by planting its stub in the fabricated table, so both sides land on
/// the same stub.
/// Original: 0x00892DB0 (thiscall, no stack arguments).
export!(thiscall, rw_00892DB0(this: *mut u8) -> u32 {
    unsafe {
        const LINK: usize = 0x94;
        const SLOT: u32 = 0x20;
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
