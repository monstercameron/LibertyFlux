// original: 0x00997370 audio_dispatch_voice_params (proposed)

/// Dispatch voice parameters down the main or the fallback path.
///
/// `this` is the voice object, `arg0` an opaque word forwarded to the
/// combiner call, `arg1` a pointer to the input struct (floats at `+0x24`
/// and `+0x2C`). The root object comes from the writable
/// global at file VA 0x12832AC: a null root returns immediately (leaving
/// the entry EAX in place). Otherwise the sub-object at
/// `[this+0x820]+0xF50` is inspected: a null pointer, a set byte at
/// `+0x218`, or a clear byte at `+0x219` selects the fallback path.
///
/// Main path: four virtual calls through slot `+0x10` (callees 21-24, one
/// id per site) build two event structs. Each struct gets the flag byte
/// from the seeded global at 0x12832B4 at `+0x14`, the tag 8
/// at `+0x10`, two words from `[this+0x9DC]` at `+0x00`/`+0x08`, and a
/// mapped float product at `+0x04` (three `map` calls, callee 25). The
/// first struct's slots are cleared first (`[r1+8] = 0`). Then the
/// combiner (callee 26: object `this+0x488`, constant 1.0, `arg0`) answers
/// `f4`; with `K1` (relocated read-only constant at 0xFE88E8) the pair
/// `(f4, K1 - f4)` is posted (callee 27) and acknowledged (callee 28, arg
/// 1), the seeded counter at 0x128329C is incremented, and
/// the acknowledge result is returned.
///
/// Fallback path: the combiner runs with constant 0 instead of 1.0,
/// answering `f5`. A voice index is resolved from `[this+0xA90]` (bytes
/// `+4` and `+0x40`) through the seeded table base at
/// 0x115D988 (row stride 0x6F40, cell at `+0x6F14`, plus byte-4 times the
/// seeded factor at 0x115D968), or zero when byte-4 is
/// 0xFF. The pair `(f5, K1 - f5)` goes to the alternate sink (callee 29);
/// a positive `f5` increments the same counter and raises the flag word
/// passed with the re-resolved index to callee 30, whose result is
/// returned.
///
/// Original: 0x00997370 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00997370(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const VCALL1: u32 = 21;
        const VCALL2: u32 = 22;
        const VCALL3: u32 = 23;
        const VCALL4: u32 = 24;
        const MAP_CALLEE: u32 = 25;
        const COMB_CALLEE: u32 = 26;
        const POST_CALLEE: u32 = 27;
        const ACK_CALLEE: u32 = 28;
        const ALT1_CALLEE: u32 = 29;
        const ALT2_CALLEE: u32 = 30;
        const VTABLE_SLOT: u32 = 0x10;
        const ROOT_VA: u32 = 0x0128_32AC;
        const FLAG_VA: u32 = 0x0128_32B4;
        const COUNT_VA: u32 = 0x0128_329C;
        const FACTOR_VA: u32 = 0x0115_D968;
        const TABLE_VA: u32 = 0x0115_D988;
        const K1_VA: u32 = 0x00FE_88E8;
        const ROW_STRIDE: u32 = 0x6F40;
        const CELL_OFF: u32 = 0x6F14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn vcall(id: u32, obj: u32) -> u32 {
            unsafe {
                let vtab = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vtab + VTABLE_SLOT) as usize);
                let _ = id;
                f(obj)
            }
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
        unsafe fn comb(obj: u32, k: f32, a: u32) -> f32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32) -> f32 = core::mem::transmute(
                    lf_checker_rt::callee_addr(COMB_CALLEE) as usize,
                );
                f(obj, k.to_bits(), a)
            }
        }
        /// Resolve the fallback voice index from the `+0xA90` object, or
        /// zero when its byte-4 is 0xFF. The table base and factor are
        /// seeded globals.
        #[inline(always)]
        unsafe fn voice_index(a90: u32) -> u32 {
            unsafe {
                let b4 = rd8(a90 + 4) as u32;
                if b4 == 0xFF {
                    0
                } else {
                    let c40 = rd8(a90 + 0x40) as u32;
                    let factor = g32(FACTOR_VA);
                    let table = g32(TABLE_VA);
                    factor
                        .wrapping_mul(b4)
                        .wrapping_add(rd32(table.wrapping_add(c40.wrapping_mul(ROW_STRIDE)).wrapping_add(CELL_OFF)))
                }
            }
        }

        unsafe {
            let _ = (VCALL1, VCALL2, VCALL3, VCALL4);
            let root = g32(ROOT_VA);
            if root == 0 {
                // The original returns with the entry EAX untouched; the
                // probe contract pins entry EAX to 0 so that 0 is the
                // early-path result on both sides.
                return 0;
            }
            let q = rd32(rd32(this + 0x820) + 0xF50);
            let main = q != 0 && rd8(q + 0x218) == 0 && rd8(q + 0x219) != 0;
            let k1 = f32::from_bits(g32(K1_VA));
            if main {
                let r1 = vcall(VCALL1, root);
                wr32(r1 + 8, 0);
                let r2 = vcall(VCALL2, root);
                wr8(r2 + 0x11, g8(FLAG_VA));
                let mid = rd32(root + 8);
                let r3 = vcall(VCALL3, mid);
                let f1 = map(this + 0x258, rdf(arg1 + 0x2c));
                let flag = g8(FLAG_VA);
                wr8(r3 + 0x14, flag);
                wr32(r3 + 0x10, 8);
                let n9dc = rd32(this + 0x9dc);
                wr32(r3, rd32(n9dc + 0x58));
                wr32(r3 + 8, rd32(n9dc + 0x5c));
                let f2 = map(this + 0x208, rdf(arg1 + 0x24));
                wr32(r3 + 4, mul(f2, f1).to_bits());
                let tail = rd32(mid + 8);
                let r4 = vcall(VCALL4, tail);
                wr8(r4 + 0x14, g8(FLAG_VA));
                wr32(r4 + 0x10, 8);
                wr32(r4, rd32(n9dc + 0x64));
                wr32(r4 + 8, rd32(n9dc + 0x68));
                let f3 = map(this + 0x230, rdf(arg1 + 0x24));
                let f4 = comb(this + 0x488, 1.0, arg0);
                wr32(r4 + 4, mul(f3, f1).to_bits());
                let d1 = sub(k1, f4);
                let sink = rd32(this + 0xa90);
                let post: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(
                        lf_checker_rt::callee_addr(POST_CALLEE) as usize,
                    );
                post(sink, f4.to_bits(), d1.to_bits());
                let ack: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    lf_checker_rt::callee_addr(ACK_CALLEE) as usize,
                );
                let r = ack(sink, 1);
                let cp = lf_checker_rt::global::<u32>(COUNT_VA);
                cp.write_unaligned(cp.read_unaligned().wrapping_add(1));
                r
            } else {
                let f5 = comb(this + 0x488, 0.0, arg0);
                let a90 = rd32(this + 0xa90);
                let idx = voice_index(a90);
                let d2 = sub(k1, f5);
                let s1: extern "thiscall" fn(u32, u32, u32) -> u32 = core::mem::transmute(
                    lf_checker_rt::callee_addr(ALT1_CALLEE) as usize,
                );
                s1(idx, f5.to_bits(), d2.to_bits());
                let mut flag: u32 = 0;
                if f5 > 0.0 {
                    let cp = lf_checker_rt::global::<u32>(COUNT_VA);
                    cp.write_unaligned(cp.read_unaligned().wrapping_add(1));
                    flag = 1;
                }
                let idx2 = voice_index(rd32(this + 0xa90));
                let s2: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    lf_checker_rt::callee_addr(ALT2_CALLEE) as usize,
                );
                // When byte-4 is 0xFF the original passes a zero object.
                s2(idx2, flag)
            }
        }
    }
});
