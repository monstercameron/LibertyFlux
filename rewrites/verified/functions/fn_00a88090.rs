// original: 0x00a88090 mode_shadow_swap
/// Swap the mode globals with their shadows in both directions.
///
/// When the liveness helper reports active and the inhibit flag is clear,
/// shadows six mode bytes into their backup slots; when the object flag is
/// set (or the helper reports active while inhibited), restores them.
/// Returns the second helper answer, with its low byte replaced by the last
/// restored value on the restore path, exactly as the original leaves EAX.
export!(thiscall, rw_00a88090(this_obj: u32) -> u32 {
    unsafe {
        let a1: u32 = callee_cdecl!(1, u32,);
        if a1 & 0xff != 0 && *global::<u8>(0x17f5eb3) == 0 {
            let prev925 = *global::<u8>(0x103b925);
            let cur924 = *global::<u8>(0x103b924);
            *global::<u8>(0x103bff9) = *global::<u8>(0x103bff8);
            *global::<u8>(0x12dd5ed) = *global::<u8>(0x12dd5ec);
            *global::<u8>(0x103c0db) = *global::<u8>(0x103c0d9);
            let d = *global::<u8>(0x103c0da);
            *global::<u8>(0x103b925) = if cur924 != 5 { cur924 } else { prev925 };
            *global::<u8>(0x103c110) = d;
        }
        let a2: u32 = callee_cdecl!(2, u32,);
        if a2 & 0xff != 0 && *global::<u8>(0x17f5eb3) != 0 {
            *((this_obj.wrapping_add(0x28)) as *mut u8) = 1;
            return a2;
        }
        if *((this_obj.wrapping_add(0x28)) as *const u8) == 0 {
            return a2;
        }
        *global::<u8>(0x103b924) = *global::<u8>(0x103b925);
        *global::<u8>(0x103bff8) = *global::<u8>(0x103bff9);
        *global::<u8>(0x12dd5ec) = *global::<u8>(0x12dd5ed);
        *global::<u8>(0x103c0d9) = *global::<u8>(0x103c0db);
        let r110 = *global::<u8>(0x103c110);
        *global::<u8>(0x103c0da) = r110;
        *((this_obj.wrapping_add(0x28)) as *mut u8) = 0;
        (a2 & 0xffffff00) | (r110 as u32)
    }
});
