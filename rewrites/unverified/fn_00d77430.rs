// original: 0x00d77430 CRenderPhaseMirrorReflection::vf3

/// Refresh one mirror-reflection render phase: gate on two global flags and
/// the phase parameters, rebuild the reflection target, run a two-part hook
/// on the current target object, briefly swap the phase parameters from a
/// global table around a commit call, then tail-transfer to the phase footer.
///
/// `this` points to the render-phase object: the target handle lives at
/// `+0x938`, a pointer to the parameter block at `+0x940`, and three
/// descriptor words at `+0x8f8`/`+0x8fc`/`+0x900`. Behaviour:
/// - Return without doing anything when gate 0 is clear, gate 1 is set, the
///   parameter block's selector (`+0x30`) is zero, or its count (`+0x38`) is
///   not positive. These paths return whatever value the caller left in the
///   accumulator, so the rewrite's return is unchecked on them.
/// - Otherwise fetch the target handle through the session object, rebuild
///   the reflection target from the descriptors, and look up the current
///   target object. When one exists, run its hook slot (`+0x30`) with
///   argument 1, remembering bit 5 of its flags (`+0x24`); otherwise remember
///   1. The remembered bit is kept in the low byte of a copy of `this`.
/// - When the swap flag is set and the parameter block exists, save its
///   count and selector, overwrite them from entry `index * 3` of the global
///   parameter table (count from the entry's first word, selector from the
///   second), commit the saved target handle, then write the saved values
///   back unless the block vanished or the saved selector is zero.
/// - Run the target object's hook slot again with the remembered word, then
///   tail-transfer to the phase footer, whose answer becomes the return
///   value (likewise unchecked: see above).
///
/// Original: 0x00d77430 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d77430(this: u32) -> u32 {
    unsafe {
        const GATE0: u32 = 0x166DA20;
        const GATE1: u32 = 0x166DA21;
        const SWAP_FLAG: u32 = 0x166DA68;
        const SWAP_INDEX: u32 = 0x166DA6C;
        const TABLE_COUNT: u32 = 0x166DA3C;
        const TABLE_SELECTOR: u32 = 0x166DA40;
        const SESSION: u32 = 0x1614C90;
        const PARAMS: u32 = 0x940;
        const TARGET: u32 = 0x938;
        const DESC0: u32 = 0x900;
        const DESC1: u32 = 0x8f8;
        const DESC2: u32 = 0x8fc;
        const PARAM_SELECTOR: u32 = 0x30;
        const PARAM_COUNT: u32 = 0x38;
        const OBJ_FLAGS: u32 = 0x24;
        const VT_HOOK: u32 = 0x30;
        const HOOK_FIRST_ARG: u32 = 1;
        const FETCH_TARGET: u32 = 1;
        const REBUILD: u32 = 2;
        const LOOKUP: u32 = 3;
        const HOOK: u32 = 4;
        const COMMIT: u32 = 5;
        const FOOTER: u32 = 6;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn hook(obj: u32, arg: u32) -> u32 {
            unsafe {
                let target: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VT_HOOK) as usize);
                target(obj, arg)
            }
        }

        if rd8(lf_checker_rt::relocated(GATE0)) == 0 {
            return 0;
        }
        if rd8(lf_checker_rt::relocated(GATE1)) != 0 {
            return 0;
        }
        let params = rd32(this.wrapping_add(PARAMS));
        if rd32(params.wrapping_add(PARAM_SELECTOR)) == 0 {
            return 0;
        }
        if (rd32(params.wrapping_add(PARAM_COUNT)) as i32) <= 0 {
            return 0;
        }

        let target: u32 =
            lf_checker_rt::callee_thiscall!(FETCH_TARGET, u32, lf_checker_rt::relocated(SESSION));
        wr32(this.wrapping_add(TARGET), target);
        lf_checker_rt::callee_cdecl!(
            REBUILD,
            u32,
            this.wrapping_add(DESC0),
            this.wrapping_add(DESC1),
            this.wrapping_add(DESC2),
            this,
            1u32,
            0u32,
        );
        let obj: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, 0u32);
        // Low byte of a copy of `this`: the flag bit when an object exists,
        // otherwise 1.
        let mut remembered = (this & 0xffff_ff00) | HOOK_FIRST_ARG;
        if obj != 0 {
            let bit = (rd32(obj.wrapping_add(OBJ_FLAGS)) >> 5) & 1;
            remembered = (this & 0xffff_ff00) | bit;
            hook(obj, HOOK_FIRST_ARG);
        }

        let (mut saved_selector, mut saved_count) = (0u32, 0u32);
        if rd32(lf_checker_rt::relocated(SWAP_FLAG)) != 0 {
            let block = rd32(this.wrapping_add(PARAMS));
            if block != 0 {
                let entry = rd32(lf_checker_rt::relocated(SWAP_INDEX)).wrapping_mul(3);
                saved_count = rd32(block.wrapping_add(PARAM_COUNT));
                saved_selector = rd32(block.wrapping_add(PARAM_SELECTOR));
                wr32(
                    block.wrapping_add(PARAM_COUNT),
                    rd32(
                        lf_checker_rt::relocated(TABLE_COUNT)
                            .wrapping_add(entry.wrapping_mul(4)),
                    ),
                );
                let fresh = rd32(this.wrapping_add(PARAMS));
                wr32(
                    fresh.wrapping_add(PARAM_SELECTOR),
                    rd32(
                        lf_checker_rt::relocated(TABLE_SELECTOR)
                            .wrapping_add(entry.wrapping_mul(4)),
                    ),
                );
            }
        }

        lf_checker_rt::callee_cdecl!(COMMIT, u32, rd32(this.wrapping_add(TARGET)));
        let after = rd32(this.wrapping_add(PARAMS));
        if after != 0 && saved_selector != 0 {
            wr32(after.wrapping_add(PARAM_COUNT), saved_count);
            wr32(
                rd32(this.wrapping_add(PARAMS)).wrapping_add(PARAM_SELECTOR),
                saved_selector,
            );
        }

        if obj != 0 {
            hook(obj, remembered);
        }
        lf_checker_rt::callee_cdecl!(FOOTER, u32,)
    }
});
