// original: 0x00E49AB0 E1_SELECT
/// Select-menu state handler.
///
/// Compares the current entry against three known slots and acts per slot.
/// Slot one selects a page index (raising the menu flags) or, for entry 5,
/// revalidates through a shared helper and tail-continues into it. Slot two
/// dispatches a sub-action: refresh the entry from the menu object (gated on
/// two ready flags), revalidate, or tail-continue. Slot three either resets
/// the frontend state or rebuilds the menu textures and rebinds the three
/// slots, then records the new state.
export!(thiscall, rw_00E49AB0(this: u32) -> u32 {
    let edi = this;
    let cur = unsafe { ((edi.wrapping_add(0x208)) as *const u32).read() };
    if cur == unsafe { ((edi.wrapping_add(0x1FC)) as *const u32).read() } {
        let sel = unsafe { ((edi.wrapping_add(0x1E0)) as *const u32).read() };
        if sel == 0 {
            unsafe { (relocated(0x11D6FEC) as *mut u32).write(relocated(0xF17300)) };
            unsafe { global::<u8>(0x1160C36).write(0) };
            unsafe { global::<u8>(0x1160C35).write(1) };
            unsafe { global::<u8>(0x18B677B).write(1) };
            sel
        } else if sel == 1 {
            unsafe { (relocated(0x11D6FEC) as *mut u32).write(relocated(0xF17304)) };
            unsafe { global::<u8>(0x1160C36).write(0) };
            unsafe { global::<u8>(0x1160C35).write(1) };
            unsafe { global::<u8>(0x18B677B).write(1) };
            sel
        } else if sel == 5 {
            let h = callee_thiscall!(1, u32, edi);
            if h != 0 {
                let _ = callee_thiscall!(2, u32, h);
            }
            callee_cdecl!(8, u32,)
        } else {
            sel
        }
    } else if cur == unsafe { ((edi.wrapping_add(0x200)) as *const u32).read() } {
        let sub = unsafe { ((edi.wrapping_add(0x1E4)) as *const u32).read() };
        if sub == 3 {
            let kept = callee_thiscall!(3, u32, edi);
            if unsafe { global::<u8>(0x18B6CA4).read() } == 0 {
                return kept;
            }
            if unsafe { global::<u8>(0x18B6E95).read() } == 0 {
                return kept;
            }
            let mode = callee_thiscall!(4, u32, edi);
            unsafe { ((edi.wrapping_add(0x208)) as *mut u32).write(mode) };
            unsafe { ((edi.wrapping_add(0x20C)) as *mut u32).write(mode) };
            mode
        } else if sub == 6 || sub == 4 {
            // The original tests the second helper's low byte here, but both
            // outcomes tail-continue identically, so only the calls matter.
            let h = callee_thiscall!(1, u32, edi);
            if h != 0 {
                let _ = callee_thiscall!(2, u32, h);
            }
            callee_cdecl!(8, u32,)
        } else if sub == 9 {
            callee_cdecl!(8, u32,)
        } else {
            unsafe { (relocated(0x11D6FEC) as *mut u32).write(relocated(0xF17308)) };
            unsafe { global::<u8>(0x1160C36).write(0) };
            unsafe { global::<u8>(0x1160C35).write(1) };
            unsafe { global::<u8>(0x18B677B).write(1) };
            sub
        }
    } else if cur == unsafe { ((edi.wrapping_add(0x204)) as *const u32).read() } {
        if unsafe { ((edi.wrapping_add(0x1E8)) as *const u32).read() } == 8 {
            callee_cdecl!(7, u32, 0x35)
        } else {
            let tex = callee_cdecl!(5, u32, relocated(0xF1730C));
            let c0 = unsafe { ((edi.wrapping_add(0x1F0)) as *const u32).read() };
            let _ = callee_thiscall!(6, u32, c0, tex, relocated(0xF17318));
            let c1 = unsafe { ((edi.wrapping_add(0x1F4)) as *const u32).read() };
            let _ = callee_thiscall!(6, u32, c1, tex, relocated(0xF17324));
            // Virtual slot 0x1E0 rebinds, one per slot object, going through
            // the fabricated objects exactly like the original.
            let o1 = unsafe { ((edi.wrapping_add(0x204)) as *const u32).read() };
            let t1 = unsafe { (((o1 as *const u32).read()).wrapping_add(0x1E0) as *const u32).read() };
            let f1: extern "thiscall" fn(u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(t1 as usize) };
            let _ = f1(o1, relocated(0xF17330), 0);
            unsafe { ((edi.wrapping_add(0x1E8)) as *mut u32).write(8) };
            let o2 = unsafe { ((edi.wrapping_add(0x1FC)) as *const u32).read() };
            let t2 = unsafe { (((o2 as *const u32).read()).wrapping_add(0x1E0) as *const u32).read() };
            let f2: extern "thiscall" fn(u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(t2 as usize) };
            let _ = f2(o2, relocated(0xF1733C), 0);
            unsafe { ((edi.wrapping_add(0x1E0)) as *mut u32).write(0) };
            let o3 = unsafe { ((edi.wrapping_add(0x200)) as *const u32).read() };
            let t3 = unsafe { (((o3 as *const u32).read()).wrapping_add(0x1E0) as *const u32).read() };
            let f3: extern "thiscall" fn(u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(t3 as usize) };
            let ans = f3(o3, relocated(0xF17348), 0);
            unsafe { ((edi.wrapping_add(0x1E4)) as *mut u32).write(3) };
            ans
        }
    } else {
        cur
    }
});
