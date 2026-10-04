// original: 0x00cc0620 CPedMoveBlendOnFoot::vf1 (stage 1: entry + early tail-out)

/// First virtual-slot update of the on-foot move blend object (STAGE 1).
///
/// `this` (ECX) points to the blend object; its word at `+0x24` points to a
/// state object. Stage 1 covers the entry block through the early-out path
/// only, i.e. inputs where the state word at `+0x26c` has bit 2 set; any
/// other input fails loudly (see below) so a stage-1 contract can never
/// pass on unimplemented paths.
///
/// Entry block: `flag_obj` is loaded from `[state+0x6c]`. When it is null
/// or its byte at `+0x0e` is zero, the flag update is skipped. Otherwise,
/// when the state byte at `+0x219` is zero, bit 0x2000 of `[this+0x50]` is
/// cleared. When it is nonzero, a helper (thiscall, one stack word,
/// byte result) is asked up to three times in order (arguments 0xd3,
/// 0xfe, 0x11f) with `flag_obj+0x808` as its object; the first nonzero
/// answer clears bit 0x2000, while three zero answers set it. Bits
/// 0x200000/0x100000 of `[this+0x50]` are then always cleared and a global
/// frame flag is reset to 0.
///
/// Early-out path (state bit 2 set): a second helper (thiscall, argument
/// 1) runs against the state object, then `[this+0x48]` is set to -1,
/// `[this+0x30]`, `[this+0x5c]`, `[this+0x4c]` and `[this+0x20]` to 0,
/// `[this+0x1c]` to 3.0f bits, and control tail-jumps to the shared
/// finish routine with `this` still in ECX (forwarded through the checker
/// as a call whose answer is the return value).
///
/// The continue path (`+0x26c` bit 2 clear) is NOT implemented in stage 1:
/// reaching it aborts, which the checker reports as a loud rewrite-side
/// fault rather than a silent pass.
///
/// Original: 0x00cc0620 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00cc0620(this: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0x24;
        const FLAGS_OFF: u32 = 0x50;
        const FLAG_OBJ_OFF: u32 = 0x6c;
        const FLAG_BYTE_OFF: u32 = 0x0e;
        const GATE_BYTE_OFF: u32 = 0x219;
        const DISPATCH_OFF: u32 = 0x26c;
        const HELPER_OBJ_BIAS: u32 = 0x808;
        const SET_BIT: u32 = 0x2000;
        const CLEAR_BITS: u32 = 0x300000;
        const ASK1: u32 = 0xd3;
        const ASK2: u32 = 0xfe;
        const ASK3: u32 = 0x11f;
        const FRAME_FLAG: u32 = 0x171bf9c;
        const ASK_CALLEE_1: u32 = 1;
        const ASK_CALLEE_2: u32 = 2;
        const ASK_CALLEE_3: u32 = 3;
        const EARLY_CALLEE: u32 = 4;
        const TAIL_CALLEE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let state = rd32(this.wrapping_add(STATE_OFF));
        let flag_obj = rd32(state.wrapping_add(FLAG_OBJ_OFF));
        if flag_obj != 0 && rd8(flag_obj.wrapping_add(FLAG_BYTE_OFF)) != 0 {
            if rd8(state.wrapping_add(GATE_BYTE_OFF)) == 0 {
                wr32(
                    this.wrapping_add(FLAGS_OFF),
                    rd32(this.wrapping_add(FLAGS_OFF)) & !SET_BIT,
                );
            } else {
                let helper = flag_obj.wrapping_add(HELPER_OBJ_BIAS);
                let a1: u32 = lf_checker_rt::callee_thiscall!(ASK_CALLEE_1, u32, helper, ASK1);
                let mut all_zero = (a1 as u8) == 0;
                if all_zero {
                    let a2: u32 =
                        lf_checker_rt::callee_thiscall!(ASK_CALLEE_2, u32, helper, ASK2);
                    all_zero = (a2 as u8) == 0;
                    if all_zero {
                        let a3: u32 =
                            lf_checker_rt::callee_thiscall!(ASK_CALLEE_3, u32, helper, ASK3);
                        all_zero = (a3 as u8) == 0;
                    }
                }
                let flags = rd32(this.wrapping_add(FLAGS_OFF));
                if all_zero {
                    wr32(this.wrapping_add(FLAGS_OFF), flags | SET_BIT);
                } else {
                    wr32(this.wrapping_add(FLAGS_OFF), flags & !SET_BIT);
                }
            }
        }
        wr32(
            this.wrapping_add(FLAGS_OFF),
            rd32(this.wrapping_add(FLAGS_OFF)) & !CLEAR_BITS,
        );
        wr32(lf_checker_rt::relocated(FRAME_FLAG), 0);
        let state2 = rd32(this.wrapping_add(STATE_OFF));
        if rd32(state2.wrapping_add(DISPATCH_OFF)) & 4 == 0 {
            panic!("stage 1: continue path (+0x26c bit 2 clear) not implemented");
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(EARLY_CALLEE, u32, state2, 1);
        wr32(this.wrapping_add(0x48), 0xffff_ffff);
        wr32(this.wrapping_add(0x30), 0);
        wr32(this.wrapping_add(0x5c), 0);
        wr32(this.wrapping_add(0x4c), 0);
        wr32(this.wrapping_add(0x1c), 0x4040_0000);
        wr32(this.wrapping_add(0x20), 0);
        lf_checker_rt::callee_thiscall!(TAIL_CALLEE, u32, this)
    }
});
