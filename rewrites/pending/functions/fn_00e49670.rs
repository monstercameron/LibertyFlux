// original: 0x00E49670 UITitleMenu::vf110
/// Title-menu input handler (vtable slot 110).
///
/// Polls two frontend toggle groups and refreshes the menu mode from
/// whichever group answers, then dispatches on a saved lookup word: value 5
/// runs the mode-5 action, value 0x16 asks the selected object for its quit
/// token and either resets the frontend state (token matches) or raises the
/// quit flag, and any other value is left as the return value.
export!(thiscall, rw_00E49670(this: u32) -> u32 {
    let esi = this;
    // Lookup record: 12 bytes, of which only the first word is ever read
    // back. All 12 are read here, in the original's order, so a partially
    // mapped record faults on both sides alike.
    let rec = callee_thiscall!(1, u32, relocated(0x19D2E08), 0x400F04, 1);
    let head = unsafe { (rec as *const u64).read_unaligned() };
    let _tail = unsafe { ((rec.wrapping_add(8)) as *const u32).read() };
    let saved = head as u32;
    // First toggle group: code 3, retried with code 1 when it stays silent.
    let first = callee_cdecl!(2, u32, 3, 1, 0, 0, 1, 0, 0);
    let mut active = (first as u8) != 0;
    if !active {
        let retry = callee_cdecl!(3, u32, 1, 1, 0, 0, 1, 0, 0);
        active = (retry as u8) != 0;
    }
    if active {
        let mode = callee_thiscall!(6, u32, esi);
        unsafe { ((esi.wrapping_add(0x1F0)) as *mut u32).write(mode) };
        let _ = callee_thiscall!(7, u32, relocated(0x19D2E08), 1);
        let _ = callee_thiscall!(8, u32, relocated(0x1176888), relocated(0xF16D44));
    }
    // Second toggle group: code 2, retried with code 0 when it stays silent.
    let second = callee_cdecl!(4, u32, 2, 1, 0, 0, 1, 0, 0);
    let mut active2 = (second as u8) != 0;
    if !active2 {
        let retry = callee_cdecl!(5, u32, 0, 1, 0, 0, 1, 0, 0);
        active2 = (retry as u8) != 0;
    }
    if active2 {
        let mode = callee_thiscall!(9, u32, esi);
        unsafe { ((esi.wrapping_add(0x1F0)) as *mut u32).write(mode) };
        let _ = callee_thiscall!(7, u32, relocated(0x19D2E08), 1);
        let _ = callee_thiscall!(8, u32, relocated(0x1176888), relocated(0xF16D5C));
    }
    if saved == 5 {
        let _ = callee_thiscall!(11, u32, esi);
        callee_thiscall!(7, u32, relocated(0x19D2E08), 1)
    } else if saved == 0x16 {
        let obj = unsafe { ((esi.wrapping_add(0x1EC)) as *const u32).read() };
        let vtbl = unsafe { (obj as *const u32).read() };
        let target = unsafe { ((vtbl.wrapping_add(0x20C)) as *const u32).read() };
        let query: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        let token = query(obj);
        // Byte-wise string equality: the only observable outcome of the
        // original's small string comparison (both of its callers only test
        // the result for zero).
        let want = relocated(0xF16D74);
        let mut a = token;
        let mut b = want;
        let equal = loop {
            let x = unsafe { (a as *const u8).read() };
            let y = unsafe { (b as *const u8).read() };
            if x != y {
                break false;
            }
            if x == 0 {
                break true;
            }
            a = a.wrapping_add(1);
            b = b.wrapping_add(1);
        };
        if equal {
            let _ = callee_cdecl!(12, u32, 0x35);
            callee_thiscall!(7, u32, relocated(0x19D2E08), 1)
        } else {
            unsafe { global::<u8>(0x18B6E8A).write(1) };
            callee_thiscall!(7, u32, relocated(0x19D2E08), 1)
        }
    } else {
        saved
    }
});
