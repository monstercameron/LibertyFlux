// original: 0x00d10c20 task_geometry_update (proposed)

/// Blend a task object's position toward an input point, run the acceptance
/// chain, and write four solution blocks, returning 1 on success.
///
/// `obj` is a task object (position block at `+0x20` with floats at
/// `+0x30/+0x34/+0x38`, a helper base at `+0x224`, a gate word at `+0xd68`).
/// `input` points at three floats, `target` at three floats whose third is
/// raised by one half on the success path. `out_a`, `out_b` and `out_c` each
/// receive four floats and `out_len` one float.
///
/// Algorithm: helper 0 must decline (non-zero returns 0) and helper 1 (run
/// on the helper base plus `0x44` with word `0x3ae`) must return non-zero.
/// Helper 2 fills two scratch areas; when it reports success the middle
/// point is its three words, otherwise it is the object position pulled
/// halfway past itself away from the input
/// (`pos + (pos - input) * 0.5`, per lane). Helper 3 takes the averaged
/// point (`(pos + middle) * 0.5`) and must return non-zero; helpers 4 and 5
/// attach the object and its gate. Helper 6 refines from `middle - pos`
/// with no output area, helper 7 checks the gate, and helper 8 (whose
/// floating result arrives on the x87 stack) scales the run. The outputs
/// are: `out_len = |pos - middle| * helper8`, `out_a = target' - pos`,
/// `out_b = target' - (middle - pos)`, `out_c = middle - pos`, each with
/// helper 2's spare word appended (`target'` is the raised target). Any
/// rejection detaches (helper 9, except the first three gates which return
/// directly) and yields 0.
///
/// Float order is the original's throughout; the x87 result is exact for
/// every scripted 32-bit value.
///
/// Original: 0x00d10c20 (cdecl, seven stack words, byte result in `al`).
lf_checker_rt::export!(cdecl, rw_00d10c20(
    obj: u32, input: u32, target: u32, out_a: u32, out_b: u32, out_len: u32, out_c: u32,
) -> u32 {
    unsafe {
        const POS_PTR: u32 = 0x20;
        const GATE: u32 = 0xd68;
        const HELPER_BASE: u32 = 0x224;
        const HELPER_BASE_OFF: u32 = 0x44;
        const HELPER_TAG: u32 = 0x3ae;
        const PX: u32 = 0x30;
        const PY: u32 = 0x34;
        const PZ: u32 = 0x38;
        const HALF: f32 = 0.5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let esi = obj;
        let al_a: u32 = lf_checker_rt::callee_cdecl!(0, u32, esi);
        if (al_a & 0xff) != 0 {
            return 0;
        }
        let base_b = rd32(esi + HELPER_BASE);
        let eax_b: u32 =
            lf_checker_rt::callee_thiscall!(1, u32, base_b.wrapping_add(HELPER_BASE_OFF), HELPER_TAG);
        if eax_b == 0 {
            return 0;
        }
        let mut buf20 = [0u32; 8];
        let mut buf10 = [0u32; 3];
        let al_c: u32 = lf_checker_rt::callee_thiscall!(
            2, u32, eax_b, esi, buf20.as_mut_ptr() as u32, buf10.as_mut_ptr() as u32, 0
        );
        let apos = rd32(esi + POS_PTR);
        let ax = rdf(apos + PX);
        let ay = rdf(apos + PY);
        let az = rdf(apos + PZ);
        let (mx, my, mz);
        if (al_c & 0xff) == 0 {
            let vx = rdf(input);
            let vy = rdf(input + 4);
            let vz = rdf(input + 8);
            mx = add(mul(sub(ax, vx), HALF), ax);
            my = add(mul(sub(ay, vy), HALF), ay);
            mz = add(mul(sub(az, vz), HALF), az);
            let _dead = rdf(apos + 0x3c);
        } else {
            mx = f32::from_bits(buf10[0]);
            my = f32::from_bits(buf10[1]);
            mz = f32::from_bits(buf10[2]);
        }
        let w3c = buf20[7];
        let t0 = mul(add(ax, mx), HALF);
        let t1 = mul(add(ay, my), HALF);
        let t2 = mul(add(az, mz), HALF);
        let tvec = [t0.to_bits(), t1.to_bits(), t2.to_bits()];
        let eax_d: u32 =
            lf_checker_rt::callee_cdecl!(3, u32, tvec.as_ptr() as u32, input, 2, 0, 0, 1);
        if eax_d == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, esi, eax_d);
        let esi_gate = rd32(esi + GATE);
        let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, esi_gate, esi);
        let gx = sub(mx, ax);
        let gy = sub(my, ay);
        let gz = sub(mz, az);
        let gvec = [gx.to_bits(), gy.to_bits(), gz.to_bits()];
        let al_g: u32 =
            lf_checker_rt::callee_cdecl!(6, u32, esi_gate, esi, gvec.as_ptr() as u32, target, 0);
        if (al_g & 0xff) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, esi);
            return 0;
        }
        let al_h: u32 = lf_checker_rt::callee_cdecl!(7, u32, esi, esi_gate, target);
        if (al_h & 0xff) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, esi);
            return 0;
        }
        let e8n = add(rdf(target + 8), HALF);
        wrf(target + 8, e8n);
        let avec = [ax.to_bits(), ay.to_bits(), az.to_bits()];
        let st0: f32 =
            lf_checker_rt::callee_thiscall!(8, f32, avec.as_ptr() as u32, target, gvec.as_ptr() as u32);
        let dx = sub(ax, mx);
        let dy = sub(ay, my);
        let dz = sub(az, mz);
        let len2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        wrf(out_len, mul(len2.sqrt(), st0));
        let f0 = rdf(target);
        let f4 = rdf(target + 4);
        wrf(out_a, sub(f0, ax));
        wrf(out_a + 4, sub(f4, ay));
        wrf(out_a + 8, sub(e8n, az));
        wrf(out_a + 12, f32::from_bits(w3c));
        wrf(out_b, sub(f0, gx));
        wrf(out_b + 4, sub(f4, gy));
        wrf(out_b + 8, sub(e8n, gz));
        wrf(out_b + 12, f32::from_bits(w3c));
        wrf(out_c, sub(mx, ax));
        wrf(out_c + 4, sub(my, ay));
        wrf(out_c + 8, sub(mz, az));
        wrf(out_c + 12, f32::from_bits(w3c));
        1
    }
});
