// original: 0x00d6a580 replay_button_bar_dtor
/// Destructor of the replay button bar (original 0x00D6A580, thiscall/0).
///
/// Stamps the bar vtable (file VA 0x00EEAD04), then walks the array at
/// `this+0x1c` for the `count` entries named by the word at `this+0x20`:
/// each non-null element is released through virtual slot 0 (callee 1,
/// thiscall/1 with flags 1)
/// and every visited slot is cleared. Frees the array (callee 2), clears
/// `this+0x1c` and the `this+0x20` dword, and stamps the dead vtable (file VA
/// 0x00EEA88C). Loop entry tests 0 against the count as UNSIGNED (`jae`,
/// so only count 0 skips); the continue test is signed (`jl`) on the
/// zero-extended count, which agrees with unsigned for every value since
/// both operands are non-negative. Counts above 3 are untested (see below).
/// Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d6a580(this_ptr: u32) -> u32 {
    unsafe {
        const VTABLE_LIVE: u32 = 0x00EEAD04;
        const VTABLE_DEAD: u32 = 0x00EEA88C;
        const ARRAY_OFF: u32 = 0x1c;
        const COUNT_OFF: u32 = 0x20;
        (this_ptr as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_LIVE));
        let count = ((this_ptr + COUNT_OFF) as *const u16).read_unaligned();
        if (count as u32) > 0 {
            let base =
                ((this_ptr + ARRAY_OFF) as *const u32).read_unaligned();
            let mut i: u32 = 0;
            loop {
                let obj = ((base + i * 4) as *const u32).read_unaligned();
                if obj != 0 {
                    let vtab = (obj as *const u32).read_unaligned();
                    let target = (vtab as *const u32).read_unaligned();
                    let f: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(target as usize);
                    f(obj, 1);
                }
                ((base + i * 4) as *mut u32).write_unaligned(0);
                i += 1;
                if !((i as i32) < (count as i32)) {
                    break;
                }
            }
        }
        let arr = ((this_ptr + ARRAY_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(2, u32, arr);
        ((this_ptr + ARRAY_OFF) as *mut u32).write_unaligned(0);
        ((this_ptr + COUNT_OFF) as *mut u32).write_unaligned(0);
        (this_ptr as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_DEAD));
        0
    }
});
