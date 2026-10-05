// original: 0x00a476e0 CHeli::vf46
/// Collect up to five related objects into the array at `a0`, notifying each.
///
/// Forwards (`a0`..`a3`) to the base collector (callee 1, thiscall, four
/// stack words), then appends the object at `this+0xF50` when non-null, then
/// four times calls the prober (callee 2, thiscall, one stack word: the\n/// loop index 0..3) and
/// appends its answer when non-null and different from `this+0xF50`
/// (thiscall, four stack words). Each append writes the pointer at
/// `a0 + [a1]*4` and increments `[a1]`, but only while `[a1] < a2` (signed);
/// each appended object is notified through its slot-0xB8 virtual (callee 3,
/// thiscall, four stack words: `a0`, `a1`, `a2`, 0). Returns the last call
/// answer (or the last probe answer when its append was skipped).
export!(thiscall, rw_00a476e0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const FIRST_OFF: u32 = 0xf50;
        const VT_SLOT: u32 = 0xb8;
        const ITERS: u32 = 4;
        let vf = |obj: u32, p0: u32, p1: u32, p2: u32, p3: u32| -> u32 {
            let vt = (obj as *const u32).read_unaligned();
            let faddr = (vt.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(faddr as usize);
            f(obj, p0, p1, p2, p3)
        };
        // EAX flow mirrors the original exactly: the prober answer stays in
        // EAX only until the count reload (\(an instruction of the original)) overwrites it.
        let mut last: u32 = callee_thiscall!(1, u32, this, a0, a1, a2, a3);
        let r0 = (this.wrapping_add(FIRST_OFF) as *const u32).read_unaligned();
        if r0 != 0 {
            let n = (a1 as *const u32).read_unaligned();
            if (n as i32) < (a2 as i32) {
                ((a0.wrapping_add(n.wrapping_mul(4))) as *mut u32).write_unaligned(r0);
                (a1 as *mut u32).write_unaligned(n.wrapping_add(1));
                last = vf(r0, a0, a1, a2, 0);
            } else {
                last = n;
            }
        }
        let mut k = 0u32;
        while k < ITERS {
            // The original pushes its loop counter, not a constant 0.
            let r: u32 = callee_thiscall!(2, u32, this, k);
            last = r;
            if r != 0 && r != (this.wrapping_add(FIRST_OFF) as *const u32).read_unaligned() {
                let n = (a1 as *const u32).read_unaligned();
                if (n as i32) < (a2 as i32) {
                    ((a0.wrapping_add(n.wrapping_mul(4))) as *mut u32).write_unaligned(r);
                    (a1 as *mut u32).write_unaligned(n.wrapping_add(1));
                    last = vf(r, a0, a1, a2, 0);
                } else {
                    last = n;
                }
            }
            k += 1;
        }
        last
    }
});
