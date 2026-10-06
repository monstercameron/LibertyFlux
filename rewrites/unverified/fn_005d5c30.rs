// original: 0x005d5c30 input_sink_configure (proposed)
//
// Configure an input sink according to its kind tag.
//
// `this` is the sink, the stack word is a context object. A base call
// runs first with the context, the vtable is stamped, and when the word
// at `+0xe0` is nonzero a direct callee folds its answer into it. Then
// the kind at `+0xd8` selects one of three paths. Kind 0x27 publishes a
// global descriptor pair (a code address and the global object, or zeroes
// when the global is null, with one unstored scratch word that reads as
// the zero fill on both sides), notifies the global object, iterates the
// `+0xc` pointer array `+0x10` times calling two direct callees per
// element, clears the global flag and runs a final direct callee.
// Kind 0x15 resolves a name through a direct callee, probes a second one
// with a stack out-word (skipping the rest when its first byte is zero),
// hashes the word through two direct callees, and indexes a global
// pointer table by the hash: a set high bit in the selected byte faults
// on a null dereference exactly as the original does, otherwise the
// entry yields the target object, whose vtable slots `+0x20` and `+0x24`
// refresh the `+0x18` and `+0x1c` floats (each skipped when already
// equal to the -1.0 constant) before they are copied to `+0x20` and
// `+0x24`. Any other kind does nothing. The security-cookie check is
// stubbed with register preservation.
//
// Original: 0x005d5c30 (thiscall, one stack word; returns `this`).
lf_checker_rt::export!(thiscall, rw_005d5c30(this: u32, ctx: u32) -> u32 {
    unsafe {
        const ID_BASE: u32 = 1;
        const ID_FOLD: u32 = 2;
        const ID_NOTIFY: u32 = 3;
        const ID_ELEM_A: u32 = 4;
        const ID_ELEM_B: u32 = 5;
        const ID_FINISH: u32 = 6;
        const ID_NAME: u32 = 7;
        const ID_PROBE: u32 = 8;
        const ID_HASH1: u32 = 9;
        const ID_HASH2: u32 = 10;
        const ID_LOOKUP: u32 = 11;
        const ID_TARGET: u32 = 12;
        const ID_COOKIE: u32 = 15;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(ID_BASE, u32, this, ctx);
        wr32(this, lf_checker_rt::relocated(0x00fe0b9c));
        let e0 = rd32(this.wrapping_add(0xe0));
        if e0 != 0 {
            let f: u32 = lf_checker_rt::callee_thiscall!(ID_FOLD, u32, ctx, e0);
            wr32(this.wrapping_add(0xe0), e0.wrapping_add(f));
        }
        let kind = rd32(this.wrapping_add(0xd8));
        if kind == 0x27 {
            let gob = rd32(lf_checker_rt::relocated(0x018b6fa8));
            let gbase = lf_checker_rt::relocated(0x019e8658);
            if gob != 0 {
                wr32(gbase, lf_checker_rt::relocated(0x005d9cd0));
                wr32(gbase.wrapping_add(4), 0);
                wr32(gbase.wrapping_add(8), gob);
            } else {
                wr32(gbase, 0);
                wr32(gbase.wrapping_add(4), 0);
                wr32(gbase.wrapping_add(8), 0);
            }
            wr32(gbase.wrapping_add(12), lf_checker_rt::relocated(0x00404b80));
            let _: u32 = lf_checker_rt::callee_thiscall!(ID_NOTIFY, u32, gob, this);
            wr8(lf_checker_rt::relocated(0x018b6fa4), 1);
            let n = rd16(this.wrapping_add(0x10)) as u32;
            let mut i = 0u32;
            while i < n {
                let arr = rd32(this.wrapping_add(0x0c));
                let elem = rd32(arr.wrapping_add(i.wrapping_mul(4)));
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    ID_ELEM_A, u32, elem.wrapping_add(0x14), this.wrapping_add(0x14)
                );
                let arr2 = rd32(this.wrapping_add(0x0c));
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    ID_ELEM_B, u32, gob, rd32(arr2.wrapping_add(i.wrapping_mul(4)))
                );
                i = i.wrapping_add(1);
            }
            wr8(lf_checker_rt::relocated(0x018b6fa4), 0);
            let _: u32 = lf_checker_rt::callee_cdecl!(ID_FINISH, u32,);
        } else if kind == 0x15 {
            let nm: u32 = lf_checker_rt::callee_thiscall!(ID_NAME, u32, this);
            let mut out: u32 = 0;
            let _: u32 = lf_checker_rt::callee_thiscall!(
                ID_PROBE, u32, this, &mut out as *mut u32 as u32, this
            );
            if out & 0xff != 0 {
                let h1: u32 =
                    lf_checker_rt::callee_cdecl!(ID_HASH1, u32, &mut out as *mut u32 as u32);
                let h2: u32 = lf_checker_rt::callee_cdecl!(ID_HASH2, u32, h1);
                let tab = rd32(lf_checker_rt::relocated(0x011764c0));
                let b = ((h2.wrapping_add(rd32(tab.wrapping_add(4)))) as *const u8).read();
                // A set high bit leaves the target null, faulting on the
                // dereference below exactly as the original does.
                let tgt = if b & 0x80 != 0 {
                    0
                } else {
                    rd32(tab.wrapping_add(0x0c))
                        .wrapping_mul(h2)
                        .wrapping_add(rd32(tab))
                };
                let lk: u32 = lf_checker_rt::callee_cdecl!(ID_LOOKUP, u32, nm, 0);
                let tobj: u32 = lf_checker_rt::callee_thiscall!(ID_TARGET, u32, rd32(tgt), lk);
                if tobj != 0 {
                    let cmin = f32::from_bits(rd32(lf_checker_rt::relocated(0x00fe8d94)));
                    // Each refresh runs only on ordered equality with the
                    // constant (the original's jp skips on inequality).
                    if rdf(this.wrapping_add(0x18)) == cmin {
                        let v20: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                            rd32(rd32(tobj).wrapping_add(0x20)) as usize,
                        );
                        let a: u32 = v20(tobj);
                        wrf(this.wrapping_add(0x18), core::hint::black_box(a as i32 as f32));
                    }
                    if rdf(this.wrapping_add(0x1c)) == cmin {
                        let v24: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                            rd32(rd32(tobj).wrapping_add(0x24)) as usize,
                        );
                        let a: u32 = v24(tobj);
                        wrf(this.wrapping_add(0x1c), core::hint::black_box(a as i32 as f32));
                    }
                    wr32(this.wrapping_add(0x20), rd32(this.wrapping_add(0x18)));
                    wr32(this.wrapping_add(0x24), rd32(this.wrapping_add(0x1c)));
                }
            }
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(ID_COOKIE, u32,);
        this
    }
});
