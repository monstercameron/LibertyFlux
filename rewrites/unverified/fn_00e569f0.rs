// original: 0x00E569F0 UIRawClipViewer::vf112
/// Run the four channel rounds in order, first passing round wins (original 0x00E569F0).
///
/// `this` is the viewer, holding four channel-object pointers at `+0x1E0`,
/// `+0x1E4`, `+0x1EC` and `+0x1E8` (in that order). Each non-null channel is
/// polled once with argument 1; null channels are skipped. Then each channel
/// runs the same three gates in round order: a ready flag through vtable
/// slot `+0x124` (low byte only), a check through slot `+0x1D4` (zero fails),
/// and, through the channel's auxiliary object at `+0x1E0` resolved via slot
/// `+0x224`, a done flag through slot `+0x1C` (low byte set means the round
/// is already done and fails). The first round to pass all three runs its
/// tail; if every round fails the function tail-dispatches with no argument.
///
/// Tails: round 0 fetches an item through slot `+0x1E0` with argument 0,
/// fires it through slot `+0x18` with argument 1, then runs the shared
/// finish; round 1 pairs the channel with arguments (0, 1), re-resolves the
/// auxiliary and sends it through slot `+0x22C` with (0, 1), returning the
/// send's answer; rounds 2 and 3 fetch and fire like round 0 and join the
/// shared finish. The shared finish fetches a worker through slot `+0x1E0`,
/// runs it twice through the same slot, finalizes the first result through
/// slot `+0x1AC`, and returns the second fire's answer.
///
/// Original: thiscall, no stack arguments; the all-fail path is an E9 tail
/// jump, the winning paths plain `ret`.
lf_checker_rt::export!(thiscall, rw_e569f0(this: u32) -> u32 {
    unsafe {
        /// Channel slots in round order.
        const CH: [u32; 4] = [0x1e0, 0x1e4, 0x1ec, 0x1e8];
        /// Vtable slots.
        const GATE_SLOT: u32 = 0x124;
        const CHECK_SLOT: u32 = 0x1d4;
        const INNER_SLOT: u32 = 0x224;
        const FLAG_SLOT: u32 = 0x1c;
        const GET_SLOT: u32 = 0x1e0;
        const FIRE_SLOT: u32 = 0x18;
        const FIN_SLOT: u32 = 0x1ac;
        const SEND_SLOT: u32 = 0x22c;
        /// Auxiliary object within a channel.
        const AUX: u32 = 0x1e0;
        /// Direct-callee ids, matching the contract.
        const POLL: u32 = 1;
        const PAIR: u32 = 9;
        const TAIL: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        /// Call a thiscall/0 vtable slot.
        unsafe fn slot0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt + slot) as usize);
                f(obj)
            }
        }
        /// Call a thiscall/1 vtable slot.
        unsafe fn slot1(obj: u32, slot: u32, arg: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt + slot) as usize);
                f(obj, arg)
            }
        }

        /// The shared finish for rounds 0, 2 and 3; returns the second fire.
        unsafe fn shared(ch: u32) -> u32 {
            unsafe {
                let worker = slot1(ch, GET_SLOT, 0);
                let first = slot1(worker, GET_SLOT, 0);
                slot0(first, FIN_SLOT);
                let second = slot1(worker, GET_SLOT, 0);
                slot1(second, FIRE_SLOT, 1)
            }
        }

        for k in 0..4 {
            let ch = rd32(this + CH[k]);
            if ch != 0 {
                lf_checker_rt::callee_thiscall!(POLL, u32, ch, 1);
            }
        }
        for k in 0..4 {
            let ch = rd32(this + CH[k]);
            if (slot0(ch, GATE_SLOT) as u8) == 0 {
                continue;
            }
            if slot0(ch, CHECK_SLOT) == 0 {
                continue;
            }
            let obj = slot0(rd32(ch + AUX), INNER_SLOT);
            if (slot0(obj, FLAG_SLOT) as u8) != 0 {
                continue;
            }
            if k == 1 {
                lf_checker_rt::callee_thiscall!(PAIR, u32, ch, 0, 1);
                let obj2 = slot0(rd32(ch + AUX), INNER_SLOT);
                let vt = rd32(obj2);
                let send: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt + SEND_SLOT) as usize);
                return send(obj2, 0, 1);
            }
            let item = slot1(ch, GET_SLOT, 0);
            slot1(item, FIRE_SLOT, 1);
            return shared(ch);
        }
        lf_checker_rt::callee_thiscall!(TAIL, u32, this)
    }
});
