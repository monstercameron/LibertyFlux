// original: 0x008f7f00 input_device_attach (proposed)

/// Attach a device object: find, open and register its three units.
///
/// `obj` is the input device object. The probe callee runs first (thiscall,
/// ECX = `obj`). Three finder callees then run (cdecl, two constant words
/// each); a null answer registers 0, otherwise the unit is opened: the
/// first through the open callee (thiscall with 0x1D, answer registered),
/// the second in place (its header is stamped with two constants and the
/// comparator address while its sequence word is mixed with the global
/// sequence counter, which increments), the third through the start callee.
/// Every outcome, null or not, goes to the register callee (cdecl, one
/// word). A tail callee with no arguments runs next (its ECX is the
/// previous stub's scratch on both sides, so the contract does not compare
/// it). Finally each of the UNSIGNED 16-bit count at `[obj+0x3C]` entries
/// of the array at `[obj+0x38]` that is non-null has its virtual slot at
/// vtable `+0x1C` called (thiscall, ECX = element), while a null entry runs
/// the diag callee (cdecl, the table address and the index). EAX holds the
/// count at return.
///
/// Thiscall: object in ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f7f00(obj: u32) -> u32 {
    unsafe {
        const C_PROBE: u32 = 1;
        const C_FIND1: u32 = 2;
        const C_OPEN: u32 = 3;
        const C_REG: u32 = 4;
        const C_FIND2: u32 = 5;
        const C_FIND3: u32 = 6;
        const C_START: u32 = 7;
        const C_TAIL: u32 = 8;
        const C_DIAG: u32 = 9;
        const C_VIRT: u32 = 10;
        const ARRAY_OFF: u32 = 0x38;
        const COUNT_OFF: u32 = 0x3c;
        const VT_SLOT: u32 = 0x1c;
        const G_SEQ: u32 = 0x10327a0;
        const HDR_TMP: u32 = 0xe7e048;
        const HDR_DONE: u32 = 0xe7e080;
        const COMPARE_FN: u32 = 0x8f7fd0;
        const DIAG_TAB: u32 = 0xe83e54;
        const SEQ_MASK: u32 = 0x3fff;
        let _: u32 = lf_checker_rt::callee_thiscall!(C_PROBE, u32, obj);
        let e1: u32 = lf_checker_rt::callee_cdecl!(C_FIND1, u32, 0x10, 1);
        let r1 = if e1 != 0 {
            lf_checker_rt::callee_thiscall!(C_OPEN, u32, e1, 0x1d)
        } else {
            0
        };
        let _: u32 = lf_checker_rt::callee_cdecl!(C_REG, u32, r1);
        let e2: u32 = lf_checker_rt::callee_cdecl!(C_FIND2, u32, 0xc, 0);
        let r2 = if e2 != 0 {
            let c = ((e2 + 4) as *const u32).read_unaligned();
            (e2 as *mut u32).write_unaligned(lf_checker_rt::relocated(HDR_TMP));
            let g = (lf_checker_rt::global::<u32>(G_SEQ)).read_unaligned();
            let mix = (c ^ g) & SEQ_MASK;
            let w = ((e2 + 4) as *const u32).read_unaligned();
            ((e2 + 4) as *mut u32).write_unaligned(w ^ mix);
            (lf_checker_rt::global::<u32>(G_SEQ)).write_unaligned(g.wrapping_add(1));
            (e2 as *mut u32).write_unaligned(lf_checker_rt::relocated(HDR_DONE));
            ((e2 + 8) as *mut u32).write_unaligned(lf_checker_rt::relocated(COMPARE_FN));
            e2
        } else {
            0
        };
        let _: u32 = lf_checker_rt::callee_cdecl!(C_REG, u32, r2);
        let e3: u32 = lf_checker_rt::callee_cdecl!(C_FIND3, u32, 8, 0);
        let r3 = if e3 != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(C_START, u32, e3);
            e3
        } else {
            0
        };
        let _: u32 = lf_checker_rt::callee_cdecl!(C_REG, u32, r3);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_TAIL, u32,);
        let mut i = 0u32;
        loop {
            let n = ((obj + COUNT_OFF) as *const u16).read_unaligned() as u32;
            if i >= n {
                return n;
            }
            let array = ((obj + ARRAY_OFF) as *const u32).read_unaligned();
            let elem = ((array + i * 4) as *const u32).read_unaligned();
            if elem != 0 {
                let vt = (elem as *const u32).read_unaligned();
                let target = ((vt + VT_SLOT) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                let _ = f(elem);
            } else {
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    C_DIAG, u32, lf_checker_rt::relocated(DIAG_TAB), i);
            }
            i += 1;
        }
    }
});
