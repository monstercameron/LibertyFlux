
// original: 0x00DABB50 ped_probe_gated (proposed)
//
// Vtable-gated probe check with a float epilogue.
//
// Arguments (cdecl, two stack words): `obj` points to an object whose vtable
// slot 0xEC is a thiscall taking one out-pointer; `aux` points to a small
// record whose float at +8 is used twice. Returns 1 in AL on success, else 0.
//
// Behaviour: calls the vtable slot with a 16-byte scratch out-slot and reads
// the word at +8 (`f`). If `f` is above zero or NaN the result is 0. Otherwise
// it builds the same probe descriptor as its neighbours (three copies of the
// global direction vector plus header/footer) and calls the 6-argument probe
// `(aux, aux[8]-4.0, 0, descriptor, 0x8e, 4)`; a zero answer means 0. On a
// nonzero answer: `t0 = -(rate * f)`, `t1 = aux[8] - dir_z - 1.0`; `t0` is
// clamped at zero from below (`t0 > 0` keeps it, else 0, NaN included), then
// `t0 += 0.3`; if `t0 > t1` the result is 1, else the low nibble of the byte
// at `obj+0x1E2` decides (below 2 means 0, else 1). Float operations are in
// the original's operand order, pinned against reassociation; NaN follows the
// original's unordered-compare paths (`jb` fails on NaN, `ja` does not take).
lf_checker_rt::export!(cdecl, rw_00dabb50(obj: u32, aux: u32) -> u32 {
    unsafe {
        const PROBE_ID: u32 = 2;
        const VT_SLOT: u32 = 0xEC;
        const GDIR_X: u32 = 0x01B4B320;
        const GDIR_Y: u32 = 0x01B4B324;
        const GDIR_Z: u32 = 0x01B4B328;
        const RATE: u32 = 0x011735BC;
        const FOUR: u32 = 0x00FE8AB8;
        const ONE: u32 = 0x00FE88E8;
        const THREE_TENTHS: u32 = 0x00FE87E8;
        const SIGN_BIT: u32 = 0x80000000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let mut out = [0u32; 4];
        let vtable = rd32(obj);
        let gate: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + VT_SLOT) as usize);
        gate(obj, out.as_mut_ptr() as u32);
        let f = f32::from_bits(out[2]);
        // comiss 0, f / jb: fail when f > 0 or f is NaN.
        if !(f <= 0.0) {
            return 0;
        }

        let gx = rd32(lf_checker_rt::relocated(GDIR_X));
        let gy = rd32(lf_checker_rt::relocated(GDIR_Y));
        let gz_bits = rd32(lf_checker_rt::relocated(GDIR_Z));
        let gz = f32::from_bits(gz_bits);
        let k_four = f32::from_bits(rd32(lf_checker_rt::relocated(FOUR)));
        let k_one = f32::from_bits(rd32(lf_checker_rt::relocated(ONE)));
        let k_03 = f32::from_bits(rd32(lf_checker_rt::relocated(THREE_TENTHS)));
        let rate = f32::from_bits(rd32(lf_checker_rt::relocated(RATE)));
        let aux8 = f32::from_bits(rd32(aux + 8));

        let adj = sub(aux8, k_four);
        let mut desc = [0u32; 21];
        desc[4] = gx;
        desc[5] = gy;
        desc[6] = gz_bits;
        desc[8] = gx;
        desc[9] = gy;
        desc[10] = gz_bits;
        desc[12] = gx;
        desc[13] = gy;
        desc[14] = gz_bits;
        desc[19] = 0xFFFF;
        let r: u32 = lf_checker_rt::callee_cdecl!(
            PROBE_ID, u32, aux, adj.to_bits(), 0, desc.as_mut_ptr() as u32, 0x8E, 4
        );
        if r & 0xFF == 0 {
            return 0;
        }
        let mut t0 = mul(rate, f);
        let mut t1 = sub(aux8, gz);
        t0 = f32::from_bits(t0.to_bits() ^ SIGN_BIT);
        t1 = sub(t1, k_one);
        // comiss t0, +0 / ja: keep t0 only when strictly above zero.
        if !(t0 > 0.0) {
            t0 = 0.0;
        }
        t0 = add(t0, k_03);
        if t0 > t1 {
            return 1;
        }
        let nibble = ((obj + 0x1E2) as *const u8).read() & 0xF;
        if nibble < 2 { 0 } else { 1 }
    }
});
