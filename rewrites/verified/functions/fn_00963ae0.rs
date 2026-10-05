// original: 0x00963AE0 replay_image_target_open (proposed)
//
// Checked-in text of this file is in out/rewrites/fn_00963AE0.rs; the mutant
// below it is in-crate only, never shipped.

/// Open a replay image target: gate on global state, resolve a name, build a
/// target object through helpers, and publish it.
///
/// `arg`, when non-null, is a ready-made name passed straight to the name
/// setter; when null the name is looked up from the globals and copied into a
/// local buffer, truncated at the first "." when the search helper finds one
/// (otherwise copied whole including the NUL). A second block repeats the
/// copy for a derived name unless gated off.
///
/// Gates, in order: byte `GATE_A` must be 0; then either `MODE_FLAG` is 1 and
/// `HANDLE` is non-null (fast path), or the `STATE`/`STATE_REF`/`SUBSTATE`
/// triple checks out and the `SLOT`/`SLOT_VAL` pair is empty-or-unset.
/// Anything else exits quietly, returning the last loaded state word.
///
/// The main sequence calls the allocator helper (eight constant arguments),
/// a 0x42-kind query helper, a bind helper, an init helper taking a pointer
/// to a scratch struct, and a probe helper; when the probe reports 0 the
/// width comes from `456.0 / measured + 0.5` truncated toward zero (invalid
/// results become 0x80000000, exactly like the original's `cvttss2si`),
/// otherwise it stays 0x100. A hook object reached through `HOOK_HOLDER`
/// (vtable slot at +0x38) then receives ("REPLAY_IMAGE_TARGET", 3, 0x1C8,
/// width, 0x20, param-block pointer) where the param block is the eight words
/// [0,0,1,1,1,0,1,0]. When the allocator returned null the hook result is
/// released; otherwise a null hook result drops a reference count (freeing at
/// zero) and a live one is published to `SLOT`/`SLOT_VAL` with a follow-up id
/// stored at `SLOT_ID`. Every exit runs the security-cookie check and returns
/// what the original leaves in eax on that path.
///
/// The two search/copy helpers are stubbed by the checker, so the copy path
/// leaves the buffer as it was (zeroed, then possibly holding the first
/// string when the second copy runs); the byte-copy path is a real loop and
/// is replicated exactly.
///
/// Original: 0x00963AE0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00963AE0(arg: u32) -> u32 {
    unsafe {
        const GATE_A: u32 = 0x01037759;
        const GATE_B: u32 = 0x0103775A;
        const SUBSTATE: u32 = 0x01037720;
        const OBJ_THIS: u32 = 0x012088B8;
        const STATE: u32 = 0x012088B4;
        const SLOT: u32 = 0x012088B8;
        const SLOT_VAL: u32 = 0x012088BC;
        const SLOT_ID: u32 = 0x012088D4;
        const HANDLE: u32 = 0x011F66A0;
        const NAME_SRC: u32 = 0x011F6954;
        const NAME_ARG: u32 = 0x011F6F34;
        const MODE_FLAG: u32 = 0x011F7060;
        const OBJ_SINGLETON: u32 = 0x0118D7F0;
        const HOOK_HOLDER: u32 = 0x017F5630;
        const DOT1: u32 = 0x00E8A7F8;
        const DOT2: u32 = 0x00E8A7FC;
        const HOOK_NAME: u32 = 0x00E8A888;
        const HOOK_SLOT: u32 = 0x38;
        const QUERY_KIND: u32 = 0x42;
        const DEFAULT_WIDTH: u32 = 0x100;
        const HEIGHT: u32 = 0x1C8;
        const STATE_REF: u32 = 0x00F1C040;
        const EXPECT_SUBSTATE: u32 = 0x12;
        const DIVIDEND: f32 = 456.0;
        const ROUND_HALF: f32 = 0.5;
        const INVALID_CVT: u32 = 0x80000000;

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
        unsafe fn cookie() {
            unsafe { lf_checker_rt::callee_cdecl!(19, u32,) };
        }
        /// Byte copy including the NUL, exactly the original's loop.
        #[inline(always)]
        unsafe fn strcpy_into(dst: *mut u8, mut src: u32) {
            unsafe {
                let mut d = dst;
                loop {
                    let b = rd8(src);
                    d.write(b);
                    d = d.add(1);
                    src = src.wrapping_add(1);
                    if b == 0 {
                        break;
                    }
                }
            }
        }
        /// The original's `cvttss2si`: truncate toward zero, 0x80000000 for
        /// NaN, infinities and out-of-range values (Rust's `as` saturates
        /// instead, so the invalid domain is checked explicitly).
        #[inline(always)]
        fn cvt(x: f32) -> u32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                INVALID_CVT
            } else {
                (x as i32) as u32
            }
        }

        let r = lf_checker_rt::relocated;
        if rd8(r(GATE_A)) != 0 {
            // Pinned off by the contract: the original returns the
            // security-cookie (an instruction of the original)
            // reproduce (see narrowed). The check call keeps call parity.
            cookie();
            return 0;
        }
        // Gate lattice: MODE_FLAG==1 jumps straight to the handle check
        // (L1); otherwise the STATE word is loaded and, on mismatch against
        // STATE_REF, control goes to L1, while on match the SUBSTATE is
        // checked (mismatch -> slot check L2, match -> L1). L1 proceeds on
        // a live handle and falls to L2 otherwise; L2 proceeds when the
        // slot pair is empty-or-unset and otherwise exits quietly, returning
        // the loaded STATE word. The one corner where no STATE load ran
        // (MODE_FLAG==1 with a null handle) is pinned off by the
        // contract's correlated gates.
        let mut loaded_state: u32 = 0;
        let mut have_state = false;
        let at_l1: bool;
        if rd32(r(MODE_FLAG)) == 1 {
            at_l1 = true;
        } else {
            loaded_state = rd32(r(STATE));
            have_state = true;
            if loaded_state != rd32(r(STATE_REF)) {
                at_l1 = true;
            } else if rd32(r(SUBSTATE)) != EXPECT_SUBSTATE {
                at_l1 = false;
            } else {
                at_l1 = true;
            }
        }
        let to_slot_check = if at_l1 { rd32(r(HANDLE)) == 0 } else { true };
        if to_slot_check {
            let proceed = rd32(r(SLOT)) == 0 || rd32(r(SLOT_VAL)) == 0;
            if !proceed {
                cookie();
                return if have_state { loaded_state } else { 0 };
            }
        }
        let this = r(OBJ_THIS);
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);

        // Shared 40-byte name buffer (zeroed; the original zeroes bytes
        // 8..40 explicitly and the rest is the checker's zero fill).
        let mut name_buf = [0u8; 40];
        if arg != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, arg);
        } else {
            let name_arg = rd32(r(NAME_ARG));
            let name_src = rd32(r(NAME_SRC));
            let found: u32 = lf_checker_rt::callee_thiscall!(4, u32, name_src, name_arg);
            if found == 0 {
                cookie();
                return 0;
            }
            let hit: u32 = lf_checker_rt::callee_cdecl!(5, u32, found, r(DOT1));
            if hit != 0 {
                // Search helper stubbed: the copy it implies does not
                // happen; the call itself is still observed.
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    6,
                    u32,
                    name_buf.as_mut_ptr() as u32,
                    found,
                    hit.wrapping_sub(found)
                );
            } else {
                strcpy_into(name_buf.as_mut_ptr(), found);
            }
            let _: u32 =
                lf_checker_rt::callee_thiscall!(3, u32, this, name_buf.as_mut_ptr() as u32);
            if rd8(r(GATE_B)) == 0 && rd32(r(NAME_ARG)) == 0 {
                let derived = rd32(r(NAME_SRC)).wrapping_add(0x2C);
                let hit2: u32 = lf_checker_rt::callee_cdecl!(5, u32, derived, r(DOT2));
                if hit2 != 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        6,
                        u32,
                        name_buf.as_mut_ptr() as u32,
                        derived,
                        hit2.wrapping_sub(derived)
                    );
                } else {
                    strcpy_into(name_buf.as_mut_ptr(), derived);
                }
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(7, u32, this, name_buf.as_mut_ptr() as u32);
            }
        }

        let target: u32 =
            lf_checker_rt::callee_cdecl!(8, u32, HEIGHT, DEFAULT_WIDTH, 1, 1, 0, 0, 0, 0);
        let scratch = [0u32; 2];
        let bound_ptr: u32 =
            lf_checker_rt::callee_cdecl!(9, u32, scratch.as_ptr() as u32, QUERY_KIND);
        let bound = rd32(bound_ptr);
        let _: u32 = lf_checker_rt::callee_thiscall!(10, u32, target, bound);
        let init_scratch = [0u32; 4];
        let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, init_scratch.as_ptr() as u32, 0);
        let params = [0u32, 0, 1, 1, 1, 0, 1, 0];
        let probe: u32 = lf_checker_rt::callee_thiscall!(12, u32, r(OBJ_SINGLETON));
        let mut width = DEFAULT_WIDTH;
        if probe == 0 {
            let measured: f32 = lf_checker_rt::callee_thiscall!(13, f32, r(OBJ_SINGLETON), 0);
            let q = core::hint::black_box(DIVIDEND) / core::hint::black_box(measured);
            let s = core::hint::black_box(q) + core::hint::black_box(ROUND_HALF);
            width = cvt(s);
        }
        let holder = rd32(r(HOOK_HOLDER));
        let hook_addr = rd32(rd32(holder).wrapping_add(HOOK_SLOT));
        let hook: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(hook_addr as usize) };
        let hooked = hook(
            holder,
            r(HOOK_NAME),
            3,
            HEIGHT,
            width,
            0x20,
            params.as_ptr() as u32,
        );
        if target == 0 {
            if hooked == 0 {
                cookie();
                return 0;
            }
            let released: u32 = lf_checker_rt::callee_thiscall!(17, u32, hooked);
            cookie();
            return released;
        }
        if hooked == 0 {
            let left = rd32(target.wrapping_add(0x20)).wrapping_sub(1);
            wr32(target.wrapping_add(0x20), left);
            if left != 0 {
                cookie();
                return 0;
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(15, u32, target);
            let freed: u32 = lf_checker_rt::callee_cdecl!(18, u32, target);
            cookie();
            return freed;
        }
        wr32(r(SLOT), target);
        wr32(r(SLOT_VAL), hooked);
        let id: u32 = lf_checker_rt::callee_cdecl!(14, u32,);
        wr32(r(SLOT_ID), id);
        cookie();
        id
    }
});
