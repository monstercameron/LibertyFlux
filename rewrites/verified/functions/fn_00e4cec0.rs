
// original: 0x00E4CEC0 pause_menu_set_visible (proposed)

/// Show or hide the pause menu panel and (re)build its bar widgets.
///
/// `this` is the pause menu object, `mode` selects the branch by its low
/// byte, which is stored at `this+0x434`. A zero byte takes the show path,
/// any other value the hide path.
///
/// Both paths start by notifying the three sub-objects at `+0x1ec`, `+0x1f4`
/// and `+0x1e0` through their virtual slot `+0x120` (one stack word: 1 on
/// the show path, 0 on the hide path) and the bar host at `+0x1f0` through
/// slot `+0xfc`, then rebuild bars through a shared emplace step: callee 5
/// builds a 24-byte descriptor from two float words, its 24 bytes are copied
/// to a temporary, and virtual slot `+0x100` of the bar host takes nine
/// stack words (two small integers, a widget-name string, and the six
/// descriptor words by value); callee 7 then releases the built descriptor.
/// The show path emplaces three bars ("PauseMenuTopBar" with (4, 0x10),
/// "PauseMenuBottomBar" with (0x10, 4), "PauseMenuLeftBar" with (2, 8)) from
/// (0, 10.0), (0, -10.0) and (0, 0); the hide path emplaces four from
/// (0, 0) with (4, 4), (0x10, 0x10), (2, 2) and (8, 8).
///
/// The show path additionally derives a scale factor: callee 3 answers which
/// of two width globals to use and callee 4 which of two height globals,
/// the signed integers convert to float and divide; a set override byte
/// replaces the quotient with a constant. The host's `+0x1e0` word is set to
/// 0xff242424, and after the bars the factor feeds slot `+0x94` of the host,
/// multiplied by two constants in the original's order. The hide path
/// instead copies a (1024.0, 576.0) descriptor into the host's `+0xa4` words,
/// sets `+0x1d8` to 1, and stores the colour callee 9 returns for slot 2 at
/// `+0x1e0`.
///
/// Calles 3 and 4 take no arguments and answer in the low byte; callee 5
/// returns the descriptor bytes; callee 9 takes (slot-pointer, 2) and
/// returns a pointer to the colour. The slot pointer addresses the
/// function's own incoming argument slot, which a rewrite cannot form, so
/// the contract skips that argument and this rewrite passes 0 there. The
/// show path also keeps its factor in that same incoming slot, so the
/// contract compares no stack for this function; the factor is still proven
/// bit-exact through the slot `+0x94` argument.
///
/// Returns slot `+0x94`'s answer on the show path and callee 7's answer on
/// the hide path. Original: 0x00E4CEC0 (thiscall, one stack word). Callee
/// ids: 1 slot +0x120, 2 slot +0xfc, 3 width select, 4 height select,
/// 5 descriptor build, 6 slot +0x100, 7 descriptor release, 8 slot +0x94,
/// 9 colour lookup.
lf_checker_rt::export!(thiscall, rw_00E4CEC0(this: u32, mode: u32) -> u32 {
    unsafe {
        const SUB_A: u32 = 0x1ec;
        const SUB_B: u32 = 0x1f4;
        const SUB_C: u32 = 0x1e0;
        const HOST: u32 = 0x1f0;
        const MODE_OFF: u32 = 0x434;
        const SHOWN_OFF: u32 = 0x435;
        const HOST_COLOUR: u32 = 0x1e0;
        const HOST_VEC: u32 = 0xa4;
        const HOST_READY: u32 = 0x1d8;
        const VSLOT_NOTIFY: u32 = 0x120;
        const VSLOT_SYNC: u32 = 0xfc;
        const VSLOT_EMPLACE: u32 = 0x100;
        const VSLOT_SCALE: u32 = 0x94;
        const SHOW_COLOUR: u32 = 0xff242424;
        const W0: u32 = 0x0105c884;
        const W1: u32 = 0x0105c888;
        const H0: u32 = 0x0105c880;
        const H1: u32 = 0x0105c87c;
        const OVERRIDE_FLAG: u32 = 0x0118dc44;
        const OVERRIDE_VAL: u32 = 0x00fe89b8;
        const SCALE_A: u32 = 0x00fe8848;
        const SCALE_B: u32 = 0x00f186ec;
        const STR_TOP: u32 = 0x00f1846c;
        const STR_BOTTOM: u32 = 0x00f1847c;
        const STR_LEFT: u32 = 0x00f18490;
        const STR_TOP2: u32 = 0x00f18424;
        const STR_BOTTOM2: u32 = 0x00f18434;
        const STR_LEFT2: u32 = 0x00f18448;
        const STR_TOP3: u32 = 0x00f1845c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { (lf_checker_rt::relocated(va) as *const u8).read() }
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        unsafe fn notify(recv: u32, v: u32) {
            unsafe {
                let hook: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(recv) + VSLOT_NOTIFY) as usize);
                hook(recv, v);
            }
        }
        unsafe fn sync(recv: u32) {
            unsafe {
                let hook: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(recv) + VSLOT_SYNC) as usize);
                hook(recv);
            }
        }
        unsafe fn emplace(host: u32, f0: u32, f1: u32, a: u32, s: u32, b: u32) {
            unsafe {
                let mut out = [0u32; 6];
                let p: u32 = lf_checker_rt::callee_thiscall!(
                    5,
                    u32,
                    out.as_mut_ptr() as u32,
                    f0,
                    f1
                );
                let w = [
                    rd32(p),
                    rd32(p.wrapping_add(4)),
                    rd32(p.wrapping_add(8)),
                    rd32(p.wrapping_add(12)),
                    rd32(p.wrapping_add(16)),
                    rd32(p.wrapping_add(20)),
                ];
                let hook: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(host) + VSLOT_EMPLACE) as usize);
                hook(host, a, s, b, w[0], w[1], w[2], w[3], w[4], w[5]);
                lf_checker_rt::callee_thiscall!(7, u32, out.as_mut_ptr() as u32);
            }
        }

        let host = rd32(this.wrapping_add(HOST));
        wr8(this.wrapping_add(MODE_OFF), mode as u8);
        if mode as u8 != 0 {
            notify(rd32(this.wrapping_add(SUB_A)), 0);
            notify(rd32(this.wrapping_add(SUB_B)), 0);
            notify(rd32(this.wrapping_add(SUB_C)), 0);
            sync(host);
            emplace(host, 0, 0, 4, lf_checker_rt::relocated(STR_TOP2), 4);
            emplace(host, 0, 0, 0x10, lf_checker_rt::relocated(STR_BOTTOM2), 0x10);
            emplace(host, 0, 0, 2, lf_checker_rt::relocated(STR_LEFT2), 2);
            emplace(host, 0, 0, 8, lf_checker_rt::relocated(STR_TOP3), 8);
            let mut out = [0u32; 6];
            let _p: u32 = lf_checker_rt::callee_thiscall!(
                5,
                u32,
                out.as_mut_ptr() as u32,
                0x44800000,
                0x44100000
            );
            let mut tmp = [0u32; 6];
            let mut i = 0usize;
            while i < 6 {
                tmp[i] = out[i];
                wr32(host.wrapping_add(HOST_VEC).wrapping_add(i as u32 * 4), out[i]);
                i += 1;
            }
            wr32(host.wrapping_add(HOST_READY), 1);
            lf_checker_rt::callee_thiscall!(7, u32, tmp.as_mut_ptr() as u32);
            let cp: u32 = lf_checker_rt::callee_cdecl!(9, u32, 0, 2);
            wr32(host.wrapping_add(HOST_COLOUR), rd32(cp));
            lf_checker_rt::callee_thiscall!(7, u32, out.as_mut_ptr() as u32)
        } else {
            wr8(this.wrapping_add(SHOWN_OFF), 1);
            notify(rd32(this.wrapping_add(SUB_A)), 1);
            notify(rd32(this.wrapping_add(SUB_B)), 1);
            notify(rd32(this.wrapping_add(SUB_C)), 1);
            sync(host);
            let b1: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
            let num = if b1 & 0xff != 0 { g32(W1) } else { g32(W0) };
            let b2: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
            let den = if b2 & 0xff != 0 { g32(H1) } else { g32(H0) };
            let mut ratio = div((num as i32) as f32, (den as i32) as f32);
            if g8(OVERRIDE_FLAG) != 0 {
                ratio = f32::from_bits(g32(OVERRIDE_VAL));
            }
            wr32(host.wrapping_add(HOST_COLOUR), SHOW_COLOUR);
            emplace(host, 0, 0x41200000, 4, lf_checker_rt::relocated(STR_TOP), 0x10);
            emplace(host, 0, 0xc1200000, 0x10, lf_checker_rt::relocated(STR_BOTTOM), 4);
            emplace(host, 0, 0, 2, lf_checker_rt::relocated(STR_LEFT), 8);
            let q = mul(
                mul(ratio, f32::from_bits(g32(SCALE_A))),
                f32::from_bits(g32(SCALE_B)),
            );
            let hook: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(host) + VSLOT_SCALE) as usize);
            hook(host, q.to_bits())
        }
    }
});
