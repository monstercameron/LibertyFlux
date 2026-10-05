// original: 0x00b66d80 veh_rebind_slot_tail
/// Rebind a vehicle slot, tear down the old binding, and tail into reset.
///
/// Installs the slot vtable at `[this]`, then, when `[this+0x2024]` holds a
/// record, a mode global reads at least 2, the kind word at `[this+0x2e]`
/// matches one of two kind globals, and bit 2 of `[this+0x1eb0]` is set:
/// resolves through two chained lookups, and unless the result is null or
/// the handle lookup answers negative, notifies the table slot for the
/// handle and clears the bit. Then tears down a live `[this+0x2024]` record
/// (two calls) and zeroes the slot, flushes a live `[this+0x2020]` record
/// (one call) and zeroes that slot, issues a reset call, and tail-calls the
/// successor with ECX set to `this`, returning its answer.
///
/// Original: thiscall, ECX only, tail jump on exit.
lf_checker_rt::export!(thiscall, rw_00b66d80(this: u32) -> u32 {
    unsafe {
        const V_GET: u32 = 1;
        const H_GET: u32 = 2;
        const USE: u32 = 3;
        const TEARDOWN: u32 = 4;
        const RELEASE: u32 = 5;
        const FLUSH: u32 = 6;
        const RESET: u32 = 7;
        const TAIL: u32 = 8;
        const MODE_VA: u32 = 0x011d6fd4;
        const KIND_A_VA: u32 = 0x012f9ef4;
        const KIND_B_VA: u32 = 0x012fa668;
        const TABLE_VA: u32 = 0x01295cd8;
        const VTABLE_VA: u32 = 0x00eb1534;
        let esi = this;
        (esi as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_VA));
        let p = ((esi + 0x2024) as *const u32).read_unaligned();
        if p != 0 {
            let mode = lf_checker_rt::global::<i32>(MODE_VA).read_unaligned();
            if mode >= 2 {
                let kind = ((esi + 0x2e) as *const u16).read_unaligned() as i16 as i32;
                let ka = lf_checker_rt::global::<i32>(KIND_A_VA).read_unaligned();
                let mut hit = kind == ka;
                if !hit {
                    let kb = lf_checker_rt::global::<i32>(KIND_B_VA).read_unaligned();
                    hit = kind == kb;
                }
                if hit && ((esi + 0x1eb0) as *const u8).read() & 4 != 0 {
                    let key = ((p + 0x18) as *const u32).read_unaligned();
                    let a: u32 = lf_checker_rt::callee_cdecl!(V_GET, u32, key);
                    let b: u32 = lf_checker_rt::callee_cdecl!(
                        V_GET, u32, ((a + 0xa8) as *const u32).read_unaligned());
                    if b != 0 {
                        let h: u32 = lf_checker_rt::callee_thiscall!(H_GET, u32, b);
                        if (h as i32) > -1 {
                            let entry = lf_checker_rt::relocated(TABLE_VA)
                                .wrapping_add(h.wrapping_mul(4));
                            let slot = (entry as *const u32).read_unaligned();
                            let _: u32 = lf_checker_rt::callee_thiscall!(USE, u32, slot);
                            let f = ((esi + 0x1eb0) as *const u8).read();
                            ((esi + 0x1eb0) as *mut u8).write(f & 0xfb);
                        }
                    }
                }
            }
        }
        let q = ((esi + 0x2024) as *const u32).read_unaligned();
        if q != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(TEARDOWN, u32, q);
            let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, q);
        }
        ((esi + 0x2024) as *mut u32).write_unaligned(0);
        let r = ((esi + 0x2020) as *const u32).read_unaligned();
        if r != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(FLUSH, u32, r, esi + 0x2020);
            ((esi + 0x2020) as *mut u32).write_unaligned(0);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, esi, 0);
        lf_checker_rt::callee_thiscall!(TAIL, u32, esi)
    }
});

/// Wrong version of rw_00b66d80: the `[this+0x2024]` slot is not zeroed.
lf_checker_rt::export!(thiscall, mut_00b66d80(this: u32) -> u32 {
    unsafe {
        const V_GET: u32 = 1;
        const H_GET: u32 = 2;
        const USE: u32 = 3;
        const TEARDOWN: u32 = 4;
        const RELEASE: u32 = 5;
        const FLUSH: u32 = 6;
        const RESET: u32 = 7;
        const TAIL: u32 = 8;
        const MODE_VA: u32 = 0x011d6fd4;
        const KIND_A_VA: u32 = 0x012f9ef4;
        const KIND_B_VA: u32 = 0x012fa668;
        const TABLE_VA: u32 = 0x01295cd8;
        const VTABLE_VA: u32 = 0x00eb1534;
        let esi = this;
        (esi as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_VA));
        let p = ((esi + 0x2024) as *const u32).read_unaligned();
        if p != 0 {
            let mode = lf_checker_rt::global::<i32>(MODE_VA).read_unaligned();
            if mode >= 2 {
                let kind = ((esi + 0x2e) as *const u16).read_unaligned() as i16 as i32;
                let ka = lf_checker_rt::global::<i32>(KIND_A_VA).read_unaligned();
                let mut hit = kind == ka;
                if !hit {
                    let kb = lf_checker_rt::global::<i32>(KIND_B_VA).read_unaligned();
                    hit = kind == kb;
                }
                if hit && ((esi + 0x1eb0) as *const u8).read() & 4 != 0 {
                    let key = ((p + 0x18) as *const u32).read_unaligned();
                    let a: u32 = lf_checker_rt::callee_cdecl!(V_GET, u32, key);
                    let b: u32 = lf_checker_rt::callee_cdecl!(
                        V_GET, u32, ((a + 0xa8) as *const u32).read_unaligned());
                    if b != 0 {
                        let h: u32 = lf_checker_rt::callee_thiscall!(H_GET, u32, b);
                        if (h as i32) > -1 {
                            let entry = lf_checker_rt::relocated(TABLE_VA)
                                .wrapping_add(h.wrapping_mul(4));
                            let slot = (entry as *const u32).read_unaligned();
                            let _: u32 = lf_checker_rt::callee_thiscall!(USE, u32, slot);
                            let f = ((esi + 0x1eb0) as *const u8).read();
                            ((esi + 0x1eb0) as *mut u8).write(f & 0xfb);
                        }
                    }
                }
            }
        }
        let q = ((esi + 0x2024) as *const u32).read_unaligned();
        if q != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(TEARDOWN, u32, q);
            let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, q);
        }
        // MUTANT: zeroing of [this+0x2024] omitted.
        let r = ((esi + 0x2020) as *const u32).read_unaligned();
        if r != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(FLUSH, u32, r, esi + 0x2020);
            ((esi + 0x2020) as *mut u32).write_unaligned(0);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, esi, 0);
        lf_checker_rt::callee_thiscall!(TAIL, u32, esi)
    }
});
