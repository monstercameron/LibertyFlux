// original: 0x00c62990 anim_player_setup
/// Player setup: stores eight arguments, resolving two handles on demand.
///
/// Stores the identity, animation, flags and count arguments, then resolves
/// the first handle from an explicit value, the name registry, or the fallback
/// registry, and likewise the second handle, announcing the first to its sink.
/// Finishes by re-initing the player with the address argument and the flag
/// bit, returning that call's answer.
export!(thiscall, rw_00c62990(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
) -> u32 {
    unsafe {
        *((this + 0xC) as *mut u32) = a2;
        *((this + 0x10) as *mut u32) = a1;
        *((this + 4) as *mut u32) = a3;
        let registry = *global::<u32>(0x16DD63C);
        if a6 != 0xFFFF_FFFF {
            *((this + 0x14) as *mut u32) = a6;
        } else if a1 != 0xFFFF_FFFF {
            let resolved: u32 = callee_thiscall!(0, u32, registry, a1);
            *((this + 0x14) as *mut u32) = resolved;
        } else if a4 != 0 {
            let resolved: u32 = callee_cdecl!(1, u32, a4);
            *((this + 0x14) as *mut u32) = resolved;
        }
        let handle = *((this + 0x14) as *const u32);
        let _: u32 = callee_cdecl!(2, u32, handle);
        if a7 != 0xFFFF_FFFF {
            *((this + 0x18) as *mut u32) = a7;
        } else if a1 != 0xFFFF_FFFF && a2 != 0xFFFF_FFFF {
            let resolved: u32 = callee_thiscall!(3, u32, registry, a1, a2);
            *((this + 0x18) as *mut u32) = resolved;
        } else if a5 != 0 {
            let resolved: u32 = callee_cdecl!(4, u32, a5, 0);
            *((this + 0x18) as *mut u32) = resolved;
        }
        callee_thiscall!(5, u32, this, a0, (a3 >> 5) & 1)
    }
});
