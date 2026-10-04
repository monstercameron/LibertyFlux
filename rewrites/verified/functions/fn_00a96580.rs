// original: 0x00a96580 fade_dispatch
/// Dispatches one channel tick to the handler matching the channel state.
///
/// Three global mode words gate the tick: it runs unless the first is set
/// while the other two both differ from 0x34. A failed readiness probe can
/// still be saved by the progress-dirty flag. With a zero tick flag only an
/// idle channel runs (through the idle handler); otherwise states 2, 3 and
/// 4 each run their own handler and every other state idles.
export!(thiscall, rw_00a96580(this: u32, flag: u32) -> u32 {
    unsafe {
        let g1 = *(global::<u8>(0x11609F6) as *const u8);
        let g2 = *(global::<u32>(0x1160C24) as *const u32);
        let g3 = *(global::<u32>(0x1030B7C) as *const u32);
        if g1 != 0 && g2 != 0x34 && g3 != 0x34 {
            return 0;
        }
        if callee_thiscall!(1, u32, this) & 0xFF == 0 {
            if *((this + 0x20) as *const u8) & 4 == 0 {
                return 0;
            }
        }
        let st = *(this as *const u32);
        if flag & 0xFF == 0 {
            if st == 0 {
                callee_thiscall!(5, u32, this);
            }
            return 0;
        }
        match st {
            2 => {
                callee_thiscall!(2, u32, this);
            }
            3 => {
                callee_thiscall!(3, u32, this);
            }
            4 => {
                callee_thiscall!(4, u32, this);
            }
            _ => {}
        }
        0
    }
});
