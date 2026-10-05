// original: 0x0097F0E0 ped_task_flagged_event (proposed)

/// Fire a flag-indexed ped audio event, reporting whether it fired.
///
/// `this` is the task object and `flag` selects one of its event bits.
/// Besides the shared audio-ready guards the clock must have reached the
/// task's baseline at `+0x158` plus 0x1388 ticks. If the selected bit is
/// already set in the mask at `+0x148` the event is skipped; otherwise a
/// ped-state test decides: the event is skipped only when the ped word at
/// `+0x224` is set while the state bytes say idle (`+0x210` clear, or a
/// non-speech kind at `+0xA74`) with `+0x218` clear and `+0x219` set.
///
/// On the fire path a scratch buffer (word 3 = ped pointer biased by
/// 0x780, word 8 = sub-object word at `+0x8`) goes through the shared
/// resolve/derive/arbitrate chain (callees 1-4, this = task, table word
/// `table[flag]` first); a zero low byte releases the handle through
/// callee 6, otherwise callee 5 posts the event with a (0, -1, 0x36)
/// descriptor. The bit is then set in the mask unless the ped word at
/// `+0x224` says the ped is busy, and the return is 1 iff the event fired.
///
/// Original: 0x0097F0E0 (thiscall, one stack argument, boolean in AL).
lf_checker_rt::export!(thiscall, rw_0097F0E0(this: u32, flag: u32) -> u32 {
    unsafe {
        const G_QUIT: u32 = 0x011F7060;
        const G_SESS_A: u32 = 0x012088B4;
        const G_SESS_B: u32 = 0x00F1C040;
        const G_MODE: u32 = 0x01037720;
        const G_CLOCK: u32 = 0x011735B4;
        const SKIP_MODE: u32 = 0x12;
        const BASE_DELAY: u32 = 0x1388;
        const TABLE: u32 = 0x01231618;
        const OFF_MASK: u32 = 0x148;
        const OFF_BASE: u32 = 0x158;
        const OFF_PED: u32 = 0x120;
        const OFF_SUB: u32 = 0x08;
        const PED_STATE: u32 = 0x210;
        const PED_KIND: u32 = 0xA74;
        const PED_BUSY: u32 = 0x224;
        const PED_IDLE0: u32 = 0x218;
        const PED_IDLE1: u32 = 0x219;
        const PED_BIAS: u32 = 0x780;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn gget(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }

        if gget(G_QUIT) == 1 {
            return 0;
        }
        if gget(G_SESS_A) != gget(G_SESS_B) {
            return 0;
        }
        if gget(G_MODE) == SKIP_MODE {
            return 0;
        }
        if gget(G_CLOCK) < rd32(this.wrapping_add(OFF_BASE)).wrapping_add(BASE_DELAY) {
            return 0;
        }
        let bit = 1u32 << (flag & 31);
        let mut fired = 0u32;
        if rd32(this.wrapping_add(OFF_MASK)) & bit == 0 {
            let ped = rd32(this.wrapping_add(OFF_PED));
            let mut skip = false;
            if rd8(ped.wrapping_add(PED_STATE)) != 0 {
                let kind = rd32(ped.wrapping_add(PED_KIND));
                if kind != 1 && kind != 2 {
                    skip = rd32(ped.wrapping_add(PED_BUSY)) != 0
                        && rd8(ped.wrapping_add(PED_IDLE0)) == 0
                        && rd8(ped.wrapping_add(PED_IDLE1)) != 0;
                }
            } else if rd32(ped.wrapping_add(PED_BUSY)) != 0
                && rd8(ped.wrapping_add(PED_IDLE0)) == 0
                && rd8(ped.wrapping_add(PED_IDLE1)) != 0
            {
                skip = true;
            }
            if !skip {
                let mut buf = [0u32; 16];
                let buf_ptr = buf.as_mut_ptr() as u32;
                let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, buf_ptr);
                buf[8] = rd32(this.wrapping_add(OFF_SUB));
                buf[3] = ped.wrapping_add(PED_BIAS);
                let handle: u32 = lf_checker_rt::callee_thiscall!(2, u32, buf_ptr);
                let param: u32 = lf_checker_rt::callee_cdecl!(3, u32, handle);
                let word = rd32(lf_checker_rt::relocated(TABLE).wrapping_add(flag.wrapping_mul(4)));
                let arb: u32 = lf_checker_rt::callee_thiscall!(
                    4, u32, this, word, buf_ptr, handle, param, 0
                );
                if (arb & 0xFF) == 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(6, u32, handle);
                } else {
                    let mut desc = [0u32, 0xFFFF_FFFF, 0x36];
                    let desc_ptr = desc.as_mut_ptr() as u32;
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        5, u32, word, 0, 0, 1, buf_ptr, desc_ptr, ped, handle
                    );
                }
                fired = 1;
            }
        }
        let ped = rd32(this.wrapping_add(OFF_PED));
        let mut set_bit = rd32(ped.wrapping_add(PED_BUSY)) == 0;
        if rd8(ped.wrapping_add(PED_STATE)) != 0 {
            let kind = rd32(ped.wrapping_add(PED_KIND));
            if kind == 1 || kind == 2 {
                set_bit = true;
            } else {
                set_bit = rd32(ped.wrapping_add(PED_BUSY)) == 0;
            }
        }
        if set_bit {
            let m = this.wrapping_add(OFF_MASK);
            (m as *mut u32).write_unaligned(rd32(m) | bit);
        }
        fired
    }
});
