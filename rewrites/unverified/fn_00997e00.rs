// original: 0x00997E00 audio_fill_voice_params (proposed)

/// Fill the 14-word voice parameter struct at `out`.
///
/// `this` is the voice object. The seed value `v0` is the float at
/// `[[this+0x820]+0x1EC0]`; it is stored at `out+0x24` and mapped through
/// eleven `map` calls (callee 11, thiscall, one float arg, float in ST0)
/// whose object pointers are fixed offsets from `this`. Three conversions
/// (callee 12) each consume one map result and answer an integer; one
/// register-indirect call through the table at `[[this+0x820]+0xFC]`
/// (callee 13, planted stub, float in ST0) gives the rotor term; one `peek`
/// (callee 14) and six `gen` calls (callee 15) finish the mix.
///
/// Two reads decide the rotor term and are unrelocated absolute reads of
/// writable data (file VAs 0x12832AA, 0x1038C1C): when the flag byte is
/// clear the indirect-call result is used, otherwise the stored float. The
/// term is scaled by the unrelocated read-only constant at 0xFE86B4 and
/// clamped to `[0, K1]` (`K1` at 0xFE88E8, NaN stays NaN) before being
/// stored at `out+0x30` and mapped once more. The `gen` results are added
/// onto five of the earlier map outputs. Words `out+0x28`/`+0x2C` are never
/// written (`+0x2C` is read as the eleventh map input). Returns the last
/// `gen` result's bits (the intercepted stub leaves the float bits in EAX).
///
/// Original: 0x00997E00 (thiscall, one stack word). Unverifiable on the
/// stock v5 worker and under read-only shadowing alike: the original reads
/// unrelocated writable data on every path.
lf_checker_rt::export!(thiscall, rw_00997E00(this: u32, out: u32) -> u32 {
    unsafe {
        const MAP_CALLEE: u32 = 11;
        const CONV_CALLEE: u32 = 12;
        const PEEK_CALLEE: u32 = 14;
        const GEN_CALLEE: u32 = 15;
        const VTABLE_SLOT: u32 = 0xFC;
        const SEED_PTR_OFF: u32 = 0x820;
        const SEED_VAL_OFF: u32 = 0x1EC0;
        const FLAG_VA: u32 = 0x0128_32AA;
        const STORED_VA: u32 = 0x0103_8C1C;
        const SCALE_VA: u32 = 0x00FE_86B4;
        const K1_VA: u32 = 0x00FE_88E8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { (lf_checker_rt::global::<u8>(va) as *const u8).read() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn map(obj: u32, x: f32) -> f32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> f32 = core::mem::transmute(
                    lf_checker_rt::callee_addr(MAP_CALLEE) as usize,
                );
                f(obj, x.to_bits())
            }
        }
        #[inline(always)]
        unsafe fn conv(obj: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                    lf_checker_rt::callee_addr(CONV_CALLEE) as usize,
                );
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn peek(x: f32) -> f32 {
            unsafe {
                let f: extern "cdecl" fn(u32) -> f32 = core::mem::transmute(
                    lf_checker_rt::callee_addr(PEEK_CALLEE) as usize,
                );
                f(x.to_bits())
            }
        }
        #[inline(always)]
        unsafe fn gen(obj: u32) -> f32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> f32 = core::mem::transmute(
                    lf_checker_rt::callee_addr(GEN_CALLEE) as usize,
                );
                f(obj, 0)
            }
        }

        unsafe {
            let inner = rd32(this + SEED_PTR_OFF);
            let v0 = rdf(inner + SEED_VAL_OFF);
            wrf(out + 0x24, v0);

            let o1 = map(this + 0xc8, v0);
            wrf(out + 0x0c, o1);
            let o2 = map(this + 0xc8, v0);
            wrf(out + 0x04, o2);
            let o3 = map(this + 0x140, v0);
            core::hint::black_box(o3);
            wr32(out + 0x08, conv(this + 0x140));
            let o4 = map(this + 0xf0, v0);
            wrf(out + 0x20, o4);
            let o5 = map(this + 0x118, v0);
            wrf(out + 0x10, o5);
            let o6 = map(this + 0x168, v0);
            wrf(out + 0x00, o6);
            let o7 = map(this + 0x1e0, v0);
            wrf(out + 0x18, o7);

            let vtab = rd32(inner);
            let slot: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(rd32(vtab + VTABLE_SLOT) as usize);
            let vt = slot(inner);
            let x = if g8(FLAG_VA) != 0 {
                f32::from_bits(g32(STORED_VA))
            } else {
                vt
            };
            let scale = f32::from_bits(g32(SCALE_VA));
            let k1 = f32::from_bits(g32(K1_VA));
            let z = mul(x, scale);
            let clamped = if z < 0.0 {
                0.0
            } else if z > k1 {
                k1
            } else {
                z
            };
            wrf(out + 0x30, clamped);

            let y8 = peek(map(this + 0x370, x));
            wrf(out + 0x34, y8);
            let _o9 = map(this + 0x190, v0);
            core::hint::black_box(_o9);
            wr32(out + 0x1c, conv(this + 0x190));
            let _o10 = map(this + 0x1b8, v0);
            core::hint::black_box(_o10);
            wr32(out + 0x14, conv(this + 0x1b8));
            let t11 = map(this + 0xa0, rdf(out + 0x2c));

            let r1 = gen(this + 0x780);
            let r2 = gen(this + 0x5f0);
            wrf(out + 0x0c, add(add(r1, r2), o1));
            let r3 = gen(this + 0x6e0);
            let r4 = gen(this + 0x5f0);
            wrf(out + 0x04, add(add(r3, r4), o2));
            let r5 = gen(this + 0x730);
            wrf(out + 0x20, add(r5, o4));
            let r6 = gen(this + 0x640);
            wrf(out + 0x10, add(r6, o5));
            wrf(out + 0x18, add(o7, t11));
            r6.to_bits()
        }
    }
});
