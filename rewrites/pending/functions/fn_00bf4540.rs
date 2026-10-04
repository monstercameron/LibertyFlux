// original: 0x00bf4540 tune_entity_response (proposed name)
/// Tune an entity session's response curves from live measurements.
///
/// `this_` carries configuration (key at +8, floats at +0x28/+0x2c/+0x30),
/// `arg1` the entity and `arg2` a blend weight. A null entity or a failed
/// session lookup returns zero. A zero weight takes the direct path
/// (prepare, push, two weighted calls, row copy); a nonzero weight may
/// run a gated probe-and-forward block. Both join at a virtual fetch of
/// a 3-float measurement whose length, scaled and clamped to one, drives
/// two more weighted calls. A flag byte then decides between returning
/// the last answer and flushing the session. Returns the last answer, or
/// zero on the early exits.
export!(thiscall, rw_00bf4540(this_: *mut u8, arg1: *mut u8, arg2: f32) -> u32 {
    unsafe {
        if arg1.is_null() {
            return 0;
        }
        let a8 = *(this_.add(8) as *const u32);
        let mut flag = [0u32; 2];
        let a1p10 = (arg1 as u32).wrapping_add(10);
        let esi = callee_thiscall!(1, u32, relocated(0x01394D60), a1p10, a8,
            flag.as_mut_ptr() as u32, 0, 0);
        if esi == 0 {
            return 0;
        }
        if arg2 == 0.0 {
            let mut s20 = [0u32; 4];
            callee_thiscall!(2, u32, this_ as u32, s20.as_mut_ptr() as u32);
            callee_thiscall!(3, u32, esi, s20.as_mut_ptr() as u32);
            let f28 = *(this_.add(0x28) as *const f32);
            callee_thiscall!(4, u32, esi, relocated(0x00EBBE14), f28.to_bits());
            let f2c = *(this_.add(0x2c) as *const f32);
            callee_thiscall!(4, u32, esi, relocated(0x00EBBE20), f2c.to_bits());
            *((esi + 0x1a0) as *mut u32) = *(this_.add(0x30) as *const u32);
        } else if (flag[0] as u8) == 0 {
            let t = callee_thiscall!(5, u32, this_ as u32);
            if (t as u8) != 0 {
                let mut se = [0u32; 4];
                callee_stdcall!(6, u32, se.as_mut_ptr() as u32);
                // The original reads its middle argument from a scratch
                // word no call writes (uninitialized stack, i.e. the
                // checker's stack fill); it also clobbers its own saved
                // register slot with the weight, which no one re-reads.
                callee_thiscall!(7, u32, this_ as u32, esi, 0, arg2.to_bits());
            }
        }
        let vt = *(arg1 as *const u32);
        let slot_v = ((vt + 0xec) as *const u32).read();
        let vcall: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot_v as usize);
        let mut sj = [0u32; 4];
        let ans = vcall(arg1 as u32, sj.as_mut_ptr() as u32);
        let x = *(ans as *const f32);
        let y = *((ans + 4) as *const f32);
        let z = *((ans + 8) as *const f32);
        let mut d = x * x;
        d += y * y;
        d += z * z;
        let r0 = d.sqrt() * f32::from_bits(0x3D4CCCCD);
        let r = if 1.0f32 > r0 { r0 } else { 1.0f32 };
        callee_thiscall!(4, u32, esi, relocated(0x00EBBE28), r.to_bits());
        let m0 = *(arg1.add(0x1ed4) as *const f32);
        let m = if 1.0f32 > m0 { m0 } else { 1.0f32 };
        let ans4 = callee_thiscall!(4, u32, esi, relocated(0x00EBBE30), m.to_bits());
        if (flag[0] as u8) == 0 {
            return ans4;
        }
        callee_thiscall!(8, u32, esi)
    }
});
