// original: 0x00a536b0 vehicle_ray_pick (proposed)

/// Pick through the ray struct, returning whether the probe hit.
///
/// Copies two float triples (from the first two arguments) into a scratch
/// struct, selects a value by the object's kind (null yields 0; kind 3
/// prefers the alternate word when the flag word has bit 6 and it is
/// nonzero; kind 2 calls the vtable slot at +0xa0 through the fabricated
/// object; anything else takes the word at +0x38), then calls the probe
/// callee with the scratch, the fourth and sixth arguments, the value, the
/// flag word, -1, 7 and 0, with the global object in ecx. The scratch
/// address is skipped and its bytes snapshotted. Nonzero probe answer
/// yields 1. Cdecl, six stack words, two callees, 0 or 1 in eax.
lf_checker_rt::export!(cdecl, rw_00a536b0(p0: u32, p1: u32, obj: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const VT_CALL: u32 = 1;
        const FINAL: u32 = 2;
        const G_VA: u32 = 0x012b9c78;
        const VT_SLOT: u32 = 0xa0;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut st = [0u32; 7];
        st[0] = rd32(p0);
        st[1] = rd32(p0.wrapping_add(4));
        st[2] = rd32(p0.wrapping_add(8));
        st[4] = rd32(p1);
        st[5] = rd32(p1.wrapping_add(4));
        st[6] = rd32(p1.wrapping_add(8));
        let frame = st.as_mut_ptr() as u32;
        let v: u32;
        if obj == 0 {
            v = 0;
        } else {
            let kind = (rd32(obj.wrapping_add(0x28)) >> 6) & 0xf;
            if kind == 3 {
                let b = rd32(obj.wrapping_add(0x7b0));
                if (a4 & 0x40) != 0 {
                    let c = rd32(obj.wrapping_add(0x7b4));
                    v = if c != 0 { c } else { b };
                } else {
                    v = b;
                }
            } else if kind == 2 {
                let vt = rd32(obj);
                let target = rd32(vt.wrapping_add(VT_SLOT));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                v = f(obj);
            } else {
                v = rd32(obj.wrapping_add(0x38));
            }
        }
        let g = rd32(lf_checker_rt::relocated(G_VA));
        let r: u32 = lf_checker_rt::callee_thiscall!(
            FINAL, u32, g, frame, a3, v, a4, 0xffff_ffff, 7, a5, 0);
        if r != 0 {
            1
        } else {
            0
        }
    }
});
