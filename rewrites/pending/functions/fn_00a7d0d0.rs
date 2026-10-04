// original: 0x00a7d0d0 taskinfo_init_from_desc
/// Initialises the record with defaults, then, when a descriptor is given,
/// copies its fields over, converts its scale to single precision and clears
/// the fresh flag unless every echoed-back word still matches.
/// (The original's stack-cookie check is CRT boilerplate: the rewrite makes
/// the same call through the intercepted callee but keeps no cookie, since a
/// Rust frame cannot be overrun the way the check guards against.)
export!(thiscall, rw_00a7d0d0(obj: *mut u8, desc: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, obj as u32);
        *((obj as *mut u8).add(0x29) as *mut u8) |= 1;
        *((obj as *mut u8) as *mut u32) = relocated(0xEA1994);
        *((obj as *mut u8).add(0x14) as *mut u32) = 0x8f;
        *((obj as *mut u8).add(0x18) as *mut u32) = 0x288;
        *((obj as *mut u8).add(0x1c) as *mut u32) = 0;
        *((obj as *mut u8).add(0x20) as *mut u32) = 0;
        *((obj as *mut u8).add(0x24) as *mut u32) = 0;
        *((obj as *mut u8).add(0x28) as *mut u8) = 0;
        callee_thiscall!(2, u32, obj as u32);
        if desc != 0 {
            let v14 = *((desc + 0x10) as *const u32);
            *((obj as *mut u8).add(0x14) as *mut u32) = v14;
            let v18 = *((desc + 0xc) as *const u32);
            *((obj as *mut u8).add(0x18) as *mut u32) = v18;
            let scale: f64 = callee_thiscall!(3, f64, desc);
            *((obj as *mut u8).add(0x20) as *mut f32) = scale as f32;
            *((obj as *mut u8).add(0x28) as *mut u8) = *((desc + 8) as *const u8);
            let v24 = *((desc + 0x54) as *const u32);
            *((obj as *mut u8).add(0x24) as *mut u32) = v24;
            let v1c = *((desc + 4) as *const u32);
            *((obj as *mut u8).add(0x29) as *mut u8) |= 1;
            *((obj as *mut u8).add(0x1c) as *mut u32) = v1c;
            let table = *(global::<u32>(0x16DD63C));
            let mut echo = [0u32; 3];
            let r = callee_thiscall!(4, u32, table, v14, v18, echo.as_mut_ptr() as u32);
            if r & 0xFF != 0 {
                let key = *((desc + 8) as *const u32);
                let mut stale = key != echo[2];
                if !stale {
                    let fx = *((desc + 0x54) as *const f32);
                    let want = *(global::<f32>(0xFE88E8));
                    let id = *((desc + 4) as *const u32);
                    stale = fx != want || id != echo[1];
                }
                if stale {
                    *((obj as *mut u8).add(0x29) as *mut u8) &= !1;
                }
            }
        }
        // The original returns whatever the cookie check left in EAX (its
        // scripted answer under the checker), not the object pointer.
        callee_cdecl!(5, u32,)
    }
});
