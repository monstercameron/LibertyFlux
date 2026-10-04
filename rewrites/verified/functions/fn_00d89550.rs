// original: 0x00d89550 proposed_range_gate_store
// Range-gate check over two transform blocks, then a length check.
//
// `a` and `d` are objects whose word at +0x20 points at a row of
// floats. The function gates on the dot product of one row pair, on a
// projected difference falling inside a fixed interval, and on the
// magnitude of another projection, then calls the object's slot-0xec
// method and stores marker 5 at `out+0x26` when the returned vector is
// long enough. Comparison thresholds come from the image's constant
// pool. Returns the second block pointer on early exits, the method's
// answer when the length gate fails, and `out` when the marker stores.

use lf_checker_rt::export;

#[inline(always)]
fn rf(base: u32, off: u32) -> f32 {
    unsafe { ((base + off) as *const f32).read() }
}


export!(cdecl, rw_d89550(a: u32, d: u32, out: u32) -> u32 {
    unsafe {
        if d == 0 {
            return 0;
        }
        let a20 = ((a + 0x20) as *const u32).read();
        let c20 = ((d + 0x20) as *const u32).read();
        // dot = (c14*a14 + c10*a10) + c18*a18, in the original's order.
        let p10 = rf(c20, 0x10) * rf(a20, 0x10);
        let p14 = rf(c20, 0x14) * rf(a20, 0x14);
        let dot = (p14 + p10) + rf(c20, 0x18) * rf(a20, 0x18);
        if 0.8f32 > dot {
            return a20;
        }
        let d30 = rf(c20, 0x30) - rf(a20, 0x30);
        let d34 = rf(c20, 0x34) - rf(a20, 0x34);
        let d38 = rf(c20, 0x38) - rf(a20, 0x38);
        let s = (rf(a20, 0x14) * d34 + rf(a20, 0x10) * d30) + rf(a20, 0x18) * d38;
        if s > -10.0f32 {
            return a20;
        }
        if -80.0f32 > s {
            return a20;
        }
        let t = (rf(a20, 0x04) * d34 + rf(a20, 0x00) * d30) + rf(a20, 0x08) * d38;
        if t.abs() > 20.0f32 {
            return a20;
        }
        let vt = (d as *const u32).read();
        let tgt = ((vt + 0xec) as *const u32).read();
        let mut buf = [0u32; 4];
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let r = f(d, buf.as_mut_ptr() as u32);
        let x = ((r + 0) as *const f32).read();
        let y = ((r + 4) as *const f32).read();
        let z = ((r + 8) as *const f32).read();
        let len = ((x * x + y * y) + z * z).sqrt();
        if 10.0f32 > len {
            return r;
        }
        ((out + 0x26) as *mut u8).write(5);
        out
    }
});
