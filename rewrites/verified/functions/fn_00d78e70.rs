// original: 0x00d78e70 mode_dispatch_with_guards (proposed)

/// Dispatch on the object's mode word, then run a chain of guard checks.
///
/// `obj` points to an object with a mode word at `MODE`. The mode word is
/// read and the status word at `STATUS` cleared first, always. Mode 1 writes
/// a fixed triple of outputs; mode 4 compares the float at `LIMIT` against a
/// threshold, calling callee 5 above it and zeroing four words otherwise;
/// mode 5 calls callee 4 with the object. Every other mode takes the default
/// path: clear the flag bit at `FLAG_BIT`, preset three outputs, then call
/// the virtual slot `VT_SLOT` on the object itself (callee 1) and, unless it
/// or the flag byte at `GUARD_FLAG`, the kind byte at `KIND` (callee 3 runs
/// only for kind 2), the linked object at `LINK` (null skips the rest of the
/// chain) or the deadline check says stop, call the same virtual slot on the
/// linked object (callee 2) and compare the tick global against the linked
/// deadline plus `GRACE`. If the whole chain passes, the word at `DIAL` is
/// split into quotient and remainder by 100; the low ten bits scaled and
/// biased become the float output, and the remainder selects one of four
/// (flag bit, output pair) endings.
///
/// Original: 0x00d78e70 (cdecl, one stack word; no return value).
lf_checker_rt::export!(cdecl, rw_00d78e70(obj: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x1304;
        const STATUS: u32 = 0x0e3c;
        const VTABLE: u32 = 0x0;
        const VT_SLOT: u32 = 0x128;
        const FLAG_BYTE: u32 = 0x0f14;
        const FLAG_BIT: u8 = 0x80;
        const OUT_A: u32 = 0x1088;
        const OUT_B: u32 = 0x1078;
        const OUT_C: u32 = 0x107c;
        const PRESET_C: u32 = 0x3dc2_8f5c;
        const MODE1_C: u32 = 0x3d4c_cccd;
        const GUARD_FLAG: u32 = 0x011a;
        const KIND: u32 = 0x10b8;
        const LINK: u32 = 0x0f50;
        const LINK_KIND: u32 = 0x0a60;
        const LINK_DEADLINE: u32 = 0x0d3c;
        const TICK_GLOBAL: u32 = 0x0117_35b4;
        const GRACE: u32 = 0x1f40;
        const DIAL: u32 = 0x2c;
        const LIMIT: u32 = 0x1ed4;
        const ZERO_BASE: u32 = 0x1eb4;
        const VSELF: u32 = 1;
        const VLINK: u32 = 2;
        const KIND_CHECK: u32 = 3;
        const MODE5_CALLEE: u32 = 4;
        const MODE4_CALLEE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn vcall(slot_id: u32, this: u32) -> u32 {
            unsafe {
                let _ = slot_id;
                let table = rd32(this.wrapping_add(VTABLE));
                let target: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(table.wrapping_add(VT_SLOT)) as usize);
                target(this)
            }
        }

        let mode = rd32(obj.wrapping_add(MODE));
        wr32(obj.wrapping_add(STATUS), 0);
        if mode == 1 {
            wr8(obj.wrapping_add(FLAG_BYTE), rd8(obj.wrapping_add(FLAG_BYTE)) & !FLAG_BIT);
            wr32(obj.wrapping_add(OUT_A), 0);
            wr32(obj.wrapping_add(OUT_C), MODE1_C);
            wr32(obj.wrapping_add(OUT_B), 0);
            return 0;
        }
        if mode == 4 {
            const THRESHOLD: f32 = f32::from_bits(0x3dcc_cccd);
            if rdf(obj.wrapping_add(LIMIT)) > THRESHOLD {
                lf_checker_rt::callee_cdecl!(MODE4_CALLEE, u32, obj);
            } else {
                wr32(obj.wrapping_add(ZERO_BASE + 4), 0);
                wr32(obj.wrapping_add(ZERO_BASE), 0);
                wr32(obj.wrapping_add(ZERO_BASE + 8), 0);
                wr32(obj.wrapping_add(ZERO_BASE + 12), 0);
            }
            return 0;
        }
        if mode == 5 {
            lf_checker_rt::callee_cdecl!(MODE5_CALLEE, u32, obj);
            return 0;
        }

        // Default path.
        wr8(obj.wrapping_add(FLAG_BYTE), rd8(obj.wrapping_add(FLAG_BYTE)) & !FLAG_BIT);
        wr32(obj.wrapping_add(OUT_A), 0);
        wr32(obj.wrapping_add(OUT_B), 0);
        wr32(obj.wrapping_add(OUT_C), PRESET_C);
        if vcall(VSELF, obj) & 0xff != 0 {
            return 0;
        }
        if rd8(obj.wrapping_add(GUARD_FLAG)) & 1 != 0 {
            return 0;
        }
        if rd8(obj.wrapping_add(KIND)) == 2 {
            if lf_checker_rt::callee_cdecl!(KIND_CHECK, u32,) & 0xff != 0 {
                return 0;
            }
        }
        let linked = rd32(obj.wrapping_add(LINK));
        if linked != 0 {
            if rd8(linked.wrapping_add(LINK_KIND)) == 2 {
                return 0;
            }
            if vcall(VLINK, linked) & 0xff != 0 {
                return 0;
            }
            let tick: u32 =
                unsafe { (lf_checker_rt::global::<u32>(TICK_GLOBAL) as *const u32).read_unaligned() };
            if tick > rd32(linked.wrapping_add(LINK_DEADLINE)).wrapping_add(GRACE) {
                return 0;
            }
        }

        // Dial word: quotient and remainder by 100 drive the outputs.
        const SCALE: f32 = f32::from_bits(0x3a4c_cccd);
        const BIAS: f32 = f32::from_bits(0x3ecc_cccd);
        let dial = rd16(obj.wrapping_add(DIAL));
        let rem = dial % 100;
        wrf(obj.wrapping_add(OUT_A), sub(mul((dial & 0x3ff) as f32, SCALE), BIAS));
        if rem < 0x14 {
            wr8(obj.wrapping_add(FLAG_BYTE), rd8(obj.wrapping_add(FLAG_BYTE)) & !FLAG_BIT);
            wr32(obj.wrapping_add(OUT_C), 0);
            wr32(obj.wrapping_add(OUT_B), 0);
        } else if rem < 0x28 {
            wr8(obj.wrapping_add(FLAG_BYTE), rd8(obj.wrapping_add(FLAG_BYTE)) | FLAG_BIT);
            wr32(obj.wrapping_add(OUT_C), PRESET_C);
            wr32(obj.wrapping_add(OUT_B), 0);
        } else {
            wr8(obj.wrapping_add(FLAG_BYTE), rd8(obj.wrapping_add(FLAG_BYTE)) & !FLAG_BIT);
            if rem < 0x3c {
                wr32(obj.wrapping_add(OUT_B), 0x3f80_0000);
                wr32(obj.wrapping_add(OUT_C), 0);
            } else {
                wr32(obj.wrapping_add(OUT_C), PRESET_C);
                wr32(obj.wrapping_add(OUT_B), 0);
            }
        }
        0
    }
});
