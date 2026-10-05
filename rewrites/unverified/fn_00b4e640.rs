// original: 0x00b4e640 ped_task_clone_chain_a (proposed)

/// Walk the entry chain for `key`, cloning each entry through the owner.
///
/// `this` points to an object whose word at `+0x78` is the owner (manager)
/// passed in ECX to the two maker calls. `key` is an opaque lookup value
/// passed in ECX to the find-first / find-next calls. The chain holds entries
/// with flag/aux words at `+0x04`/`+0x08`, selector words at `+0x0c`/`+0x10`,
/// alternate selector words at `+0x14`/`+0x18`, and three payload floats (as
/// bits) at `+0x4c`/`+0x54`/`+0x58`.
///
/// For each entry the flag word is or-ed with `FLAG_EXTRA`, then one maker
/// runs: both selectors valid (auxiliary zero, or neither selector the
/// `NONE` sentinel) uses the full maker with (y, x, flags, aux, priority,
/// `NONE`); a `NONE` selector with nonzero auxiliary uses the alternate
/// maker with (alt_a, alt_b, flags, aux, priority). A null product skips the
/// entry. Otherwise the first payload is set through the product's setter,
/// the second payload is stored at product `+0x54`, the third through the
/// other setter, and `FLAG_DONE` is or-ed into the product's word at `+0x04`.
///
/// Always returns 0: the loop only exits on a null find result, and the
/// empty chain returns null immediately. Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00b4e640(this: u32, key: u32) -> u32 {
    unsafe {
        const OWNER: u32 = 0x78;
        const E_FLAGS: u32 = 0x04;
        const E_AUX: u32 = 0x08;
        const E_X: u32 = 0x0c;
        const E_Y: u32 = 0x10;
        const E_ALT_A: u32 = 0x14;
        const E_ALT_B: u32 = 0x18;
        const E_F1: u32 = 0x4c;
        const E_F2: u32 = 0x54;
        const E_F3: u32 = 0x58;
        const P_FLAGS: u32 = 0x04;
        const P_F2: u32 = 0x54;
        const FLAG_EXTRA: u32 = 0x4000;
        const FLAG_DONE: u32 = 0x0040_0000;
        const PRIORITY: u32 = 0xC100_0000;
        const NONE: u32 = 0xFFFF_FFFF;
        const FIND_FIRST: u32 = 1;
        const MAKE_FULL: u32 = 2;
        const MAKE_ALT: u32 = 3;
        const SET_F1: u32 = 4;
        const SET_F3: u32 = 5;
        const FIND_NEXT: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let owner = rd32(this + OWNER);
        let mut node: u32 = lf_checker_rt::callee_thiscall!(FIND_FIRST, u32, key);
        while node != 0 {
            let f1 = rd32(node + E_F1);
            let flags = rd32(node + E_FLAGS) | FLAG_EXTRA;
            let aux = rd32(node + E_AUX);
            let product: u32 = if aux == 0 {
                lf_checker_rt::callee_thiscall!(
                    MAKE_FULL, u32, owner,
                    rd32(node + E_Y), rd32(node + E_X),
                    flags, aux, PRIORITY, NONE
                )
            } else {
                let y = rd32(node + E_Y);
                if y == NONE {
                    lf_checker_rt::callee_thiscall!(
                        MAKE_ALT, u32, owner,
                        rd32(node + E_ALT_A), rd32(node + E_ALT_B),
                        flags, aux, PRIORITY
                    )
                } else {
                    let x = rd32(node + E_X);
                    if x == NONE {
                        lf_checker_rt::callee_thiscall!(
                            MAKE_ALT, u32, owner,
                            rd32(node + E_ALT_A), rd32(node + E_ALT_B),
                            flags, aux, PRIORITY
                        )
                    } else {
                        lf_checker_rt::callee_thiscall!(
                            MAKE_FULL, u32, owner, y, x, flags, aux, PRIORITY, NONE
                        )
                    }
                }
            };
            if product != 0 {
                lf_checker_rt::callee_thiscall!(SET_F1, u32, product, f1);
                wr32(product + P_F2, rd32(node + E_F2));
                lf_checker_rt::callee_thiscall!(SET_F3, u32, product, rd32(node + E_F3));
                wr32(product + P_FLAGS, rd32(product + P_FLAGS) | FLAG_DONE);
            }
            node = lf_checker_rt::callee_thiscall!(FIND_NEXT, u32, key);
        }
        0
    }
});
