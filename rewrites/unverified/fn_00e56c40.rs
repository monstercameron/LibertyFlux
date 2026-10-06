// original: 0x00E56C40 UIRawClipViewer::vf111
/// Poll four channel objects, then dispatch on the hub's class word (original 0x00E56C40).
///
/// `this` is the viewer, holding four channel-object pointers at `+0x1E0`,
/// `+0x1E4`, `+0x1EC` and `+0x1E8` (in that call order). Each non-null channel
/// is polled once with argument 1; null channels are skipped. The hub service
/// is then asked for a 12-byte status block whose first word is an integer
/// class compared for equality (never ordered, so signedness is moot):
/// class `0x17` selects the A path, `0x18` the B path, anything else returns
/// the class word itself with no further calls.
///
/// On the A/B paths the channels are asked in the same order, through vtable
/// slot `+0x124`, for a ready flag (low byte only: `AL != 0` wins). The first
/// ready channel wins; its child at `+0x1EC` is selected with argument 1
/// (service A on the A path, service B on the B path). With no ready channel
/// the select call is skipped. Both paths end by releasing the hub with
/// argument 1 and returning the release call's answer.
///
/// Original: thiscall, no stack arguments, plain `ret`.
lf_checker_rt::export!(thiscall, rw_e56c40(this: u32) -> u32 {
    unsafe {
        /// Channel slots in poll order.
        const CH: [u32; 4] = [0x1e0, 0x1e4, 0x1ec, 0x1e8];
        /// Vtable slot of the ready-flag query.
        const READY_SLOT: u32 = 0x124;
        /// Child selected from the winning channel.
        const CHILD: u32 = 0x1ec;
        /// Hub object passed in ECX to the status and release calls.
        const HUB: u32 = 0x019D_2E08;
        /// Status-service selector pushed as the second argument.
        const STATUS_SEL: u32 = 0xC000;
        /// Class words selecting the A and B paths.
        const CLASS_A: u32 = 0x17;
        const CLASS_B: u32 = 0x18;
        /// Callee ids, matching the contract.
        const POLL: u32 = 1;
        const STATUS: u32 = 2;
        const SELECT_A: u32 = 4;
        const RELEASE: u32 = 5;
        const SELECT_B: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }

        /// Ask one channel whether it is ready (vtable slot READY_SLOT).
        unsafe fn ready(ch: u32) -> bool {
            unsafe {
                let vt = rd32(ch);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute((rd32(vt + READY_SLOT)) as usize);
                (f(ch) as u8) != 0
            }
        }

        // Poll every live channel once with argument 1.
        for k in 0..4 {
            let ch = rd32(this + CH[k]);
            if ch != 0 {
                lf_checker_rt::callee_thiscall!(POLL, u32, ch, 1);
            }
        }
        // Fetch the status block; the first word is the class.
        let p = lf_checker_rt::callee_thiscall!(STATUS, u32, lf_checker_rt::relocated(HUB), STATUS_SEL, 1);
        let mut scratch = [0u32; 3];
        scratch[0] = rd32(p);
        scratch[1] = rd32(p + 4);
        scratch[2] = rd32(p + 8);
        let class = scratch[0];

        /// Run the A/B ready scan; returns the winning channel or 0.
        unsafe fn scan(this: u32) -> u32 {
            unsafe {
                for k in 0..4 {
                    let ch = rd32(this + CH[k]);
                    if ready(ch) {
                        return ch;
                    }
                }
                0
            }
        }

        if class == CLASS_A {
            let winner = scan(this);
            if winner != 0 {
                let child = rd32(winner + CHILD);
                lf_checker_rt::callee_thiscall!(SELECT_A, u32, child, 1);
            }
            lf_checker_rt::callee_thiscall!(RELEASE, u32, lf_checker_rt::relocated(HUB), 1)
        } else if class == CLASS_B {
            let winner = scan(this);
            if winner != 0 {
                let child = rd32(winner + CHILD);
                lf_checker_rt::callee_thiscall!(SELECT_B, u32, child, 1);
            }
            lf_checker_rt::callee_thiscall!(RELEASE, u32, lf_checker_rt::relocated(HUB), 1)
        } else {
            class
        }
    }
});
