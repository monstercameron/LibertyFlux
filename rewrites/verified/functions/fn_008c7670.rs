// original: 0x008C7670 stream_channel_bringup
/// Bring up streaming channel `hint` when the request state is idle.
///
/// Unless the state global reads 0 with the done flag clear the result is
/// 0 or 2 at once. Otherwise, with the cached-names flag set the channel
/// name is copied from the name table slot `hint` into the request block;
/// with it clear a negative hint is first resolved through the resolve
/// callee, then the bind callee runs. The started flag is cleared, done
/// and state are recorded, the announce callee runs, and the result is 0.
/// Original: cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_008c7670(hint: u32) -> u32 {
    unsafe {
        const RESOLVE_CALLEE: u32 = 1;
        const BIND_CALLEE: u32 = 2;
        const ANNOUNCE_CALLEE: u32 = 3;
        const NAME_STRIDE: u32 = 0x134;
        const NAME_TABLE_FILE_VA: u32 = 0x116D398;
        const REQ_BASE_FILE_VA: u32 = 0x1173100;
        match *lf_checker_rt::global::<u32>(0x11730F8) {
            0 => {}
            _ => return 2,
        }
        if *lf_checker_rt::global::<u32>(0x11730FC) != 0 {
            return 0;
        }
        if *lf_checker_rt::global::<u8>(0x1031F2C) != 0 {
            let table = lf_checker_rt::relocated(NAME_TABLE_FILE_VA);
            let dst = lf_checker_rt::relocated(REQ_BASE_FILE_VA);
            let mut src = table.wrapping_add(
                (hint as i32).wrapping_mul(NAME_STRIDE as i32) as u32);
            let mut d = dst;
            loop {
                let b = (src as *const u8).read();
                (d as *mut u8).write(b);
                if b == 0 {
                    break;
                }
                src = src.wrapping_add(1);
                d = d.wrapping_add(1);
            }
        } else {
            let mut id = hint;
            if (hint as i32) < 0 {
                id = lf_checker_rt::callee_cdecl!(RESOLVE_CALLEE, u32,);
            }
            lf_checker_rt::callee_cdecl!(BIND_CALLEE, u32, id);
        }
        *lf_checker_rt::global::<u8>(0x1172F53) = 0;
        *lf_checker_rt::global::<u32>(0x11730FC) = 1;
        *lf_checker_rt::global::<u32>(0x11730F8) = 2;
        lf_checker_rt::callee_cdecl!(ANNOUNCE_CALLEE, u32,);
        0
    }
});
