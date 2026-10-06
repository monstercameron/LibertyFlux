// original: 0x008f7c70 input_list_refresh (proposed)

/// Resynchronize the element list when the generation changed.
///
/// `obj` is the input device object. When the stored generation at
/// `[obj+0x24]` differs from the global generation, the sync callee runs
/// first (thiscall, ECX = `obj`). The gate byte at `+0x471` is then set to
/// 1, and the value byte at `+0x470` cleared when it reads exactly 1. Each
/// of the UNSIGNED 16-bit count at `[obj+4]` elements is visited: while the
/// stored generation still differs, an element that is not the global skip
/// pointer goes to the touch callee; an element whose flag byte at `+0x558`
/// has bit 0 set goes to the mark callee. Three fixed callees then run in
/// order (scan, collect with ECX = `obj`, commit), the stored generation is
/// updated, and the commit answer is returned in EAX.
///
/// Thiscall: object in ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f7c70(obj: u32) -> u32 {
    unsafe {
        const C_SYNC: u32 = 1;
        const C_TOUCH: u32 = 2;
        const C_MARK: u32 = 3;
        const C_SCAN: u32 = 4;
        const C_COLLECT: u32 = 5;
        const C_COMMIT: u32 = 6;
        const ARRAY_OFF: u32 = 0x0;
        const COUNT_OFF: u32 = 0x4;
        const GEN_OFF: u32 = 0x24;
        const VALUE_OFF: u32 = 0x470;
        const GATE_OFF: u32 = 0x471;
        const FLAG_OFF: u32 = 0x558;
        const G_GEN: u32 = 0x1160e88;
        const G_SKIP: u32 = 0x18b6ef0;
        const SCAN_OBJ: u32 = 0x1161518;
        const COMMIT_OBJ: u32 = 0x11737d0;
        let gen = (lf_checker_rt::global::<u32>(G_GEN)).read_unaligned();
        if ((obj + GEN_OFF) as *const u32).read_unaligned() != gen {
            let _: u32 = lf_checker_rt::callee_thiscall!(C_SYNC, u32, obj);
        }
        ((obj + GATE_OFF) as *mut u8).write(1);
        if ((obj + VALUE_OFF) as *const u8).read() == 1 {
            ((obj + VALUE_OFF) as *mut u8).write(0);
        }
        let mut i = 0u32;
        loop {
            let n = ((obj + COUNT_OFF) as *const u16).read_unaligned() as u32;
            if i >= n {
                break;
            }
            if ((obj + GEN_OFF) as *const u32).read_unaligned() != gen {
                let array = (obj as *const u32).read_unaligned();
                let elem = ((array + i * 4) as *const u32).read_unaligned();
                if elem != (lf_checker_rt::global::<u32>(G_SKIP)).read_unaligned() {
                    let _: u32 = lf_checker_rt::callee_thiscall!(C_TOUCH, u32, elem);
                }
            }
            let array = (obj as *const u32).read_unaligned();
            let elem = ((array + i * 4) as *const u32).read_unaligned();
            if ((elem + FLAG_OFF) as *const u8).read() & 1 != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(C_MARK, u32, elem);
            }
            i += 1;
        }
        let _: u32 =
            lf_checker_rt::callee_thiscall!(C_SCAN, u32, lf_checker_rt::relocated(SCAN_OBJ));
        let _: u32 = lf_checker_rt::callee_thiscall!(C_COLLECT, u32, obj);
        let r: u32 =
            lf_checker_rt::callee_thiscall!(C_COMMIT, u32, lf_checker_rt::relocated(COMMIT_OBJ));
        ((obj + GEN_OFF) as *mut u32).write_unaligned(gen);
        r
    }
});
