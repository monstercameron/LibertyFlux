// original: 0x008e0690 for_each_live_slot

/// Sweep every pool slot and refresh the live ones: for each index below the
/// table count, skip dead slots (flag high bit), probe the slot, and unless
/// `only_live` is set, read the slot's data word and its byte probe: slots
/// whose probe passes the channel mask are refreshed, the rest are left.
///
/// `only_live`: when nonzero the data-word and probe checks are skipped and
/// every live, probed slot is refreshed directly. The table context is
/// reloaded after each slot (the callees may replace it) and the loop bound
/// is re-read every iteration.
///
/// Original: cdecl (flag byte); no return value.
lf_checker_rt::export!(cdecl, rw_008e0690(only_live: u8) -> () {
    unsafe {
        let ctx = *lf_checker_rt::global::<u32>(0x11764C0) as *mut u32;
        if *ctx.add(2) <= 0 {
            return;
        }
        let mut i = 0u32;
        loop {
            let live = {
                let c = *lf_checker_rt::global::<u32>(0x11764C0) as *mut u32;
                let flags = *c.add(1) as *const u8;
                *flags.add(i as usize) & 0x80 == 0
            };
            if live {
                let c = *lf_checker_rt::global::<u32>(0x11764C0) as *mut u32;
                let base = *c as *mut u8;
                let stride = *c.add(3);
                let entry = base.byte_add(i.wrapping_mul(stride) as usize);
                if !entry.is_null() && lf_checker_rt::callee_cdecl!(1, u32, i) != 0 {
                    if only_live != 0 {
                        lf_checker_rt::callee_cdecl!(4, u32, i);
                    } else if lf_checker_rt::callee_cdecl!(2, u32, i) == 0 {
                        let probe =
                            lf_checker_rt::callee_cdecl!(3, u32, i, *lf_checker_rt::global::<u32>(0x1032F58));
                        if probe as u8 & 0xC6 == 0 {
                            lf_checker_rt::callee_cdecl!(4, u32, i);
                        }
                    }
                }
            }
            i += 1;
            if i >= *ctx.add(2) {
                break;
            }
        }
    }
});
