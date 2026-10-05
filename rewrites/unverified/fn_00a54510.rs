// original: 0x00a54510 fn_00a54510
//! Vector probe with virtual fallback through the world pick dispatcher.
//!
//! Reads three words from `p0` and one word `a1` from the stack, builds a
//! six-word frame `[p0[0], p0[1], p0[2], p0[0], p0[1], a1]` in an aligned
//! scratch area, then resolves a value from `obj`: a null object gives 0,
//! otherwise slot 0xa0 of the object's vtable is called with the object as
//! `this` and its return value is used unless it is 0, in which case the
//! word at `obj+0x38` is used instead. Finally an 8-argument dispatcher
//! call (global at 0x012b9c78 as `this`) runs with the frame pointer as
//! first argument and the function returns whether that call was nonzero.
//! cdecl; stack frame is 16-byte aligned.

use lf_checker_rt as rt;

const VT_SLOT: u32 = 0xa0;
const OBJ_FLAT: u32 = 0x38;

unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

rt::export!(cdecl, rw_00a54510(p0: u32, a1: u32, obj: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        let mut st = [0u32; 7];
        st[0] = rd32(p0);
        st[1] = rd32(p0.wrapping_add(4));
        st[2] = rd32(p0.wrapping_add(8));
        st[4] = st[0];
        st[5] = st[1];
        st[6] = a1;
        let frame = st.as_mut_ptr() as u32;
        let v: u32 = if obj == 0 {
            0
        } else {
            let vt = rd32(obj);
            let target = rd32(vt.wrapping_add(VT_SLOT));
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let t = f(obj);
            if t != 0 { t } else { rd32(obj.wrapping_add(OBJ_FLAT)) }
        };
        let g = rd32(rt::relocated(0x012b9c78));
        let r: u32 = rt::callee_thiscall!(
            2, u32, g, frame, a3, v, a4, 0xffff_ffff, 7, 1, 0);
        u32::from(r != 0)
    }
});
