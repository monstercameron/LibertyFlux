// original: 0x00cfb180 climb_ladder_setup_anims (proposed)

/// Look up eight ladder-animation slots and arm them, then finish the setup.
///
/// `obj` (at `+0x78`) holds a collection searched once per slot id in
/// `LOOKUP_FIXED` (0x8c, 0x8d, 0x87, 0x88, 0x89, 0x8a): each found handle gets
/// the default weight -2.0. Slot 0x8b instead takes a weight from a global
/// (-8.0 when the mode word at `this+0x1c` is 4, else -2.0). Slot 0x86, when
/// found, is attached to the collection. A null handle skips its slot.
///
/// Then the outer object at `this+0x60` is examined: null returns the last
/// lookup answer; otherwise, when its flag byte (`+0x46`, bit 5) is clear,
/// the finish callee runs with the -8.0 global. When the flag is set, the
/// value at `+0x5c` is compared against the 0.0 global with an unordered-aware
/// compare (only an exact zero, plus or minus, takes the read path; NaN takes
/// the else path): on equality the finish callee runs with a value read
/// through the outer object, else with the -8.0 global. Returns the last
/// callee answer observed on the taken path.
///
/// Original: 0x00cfb180 (thiscall, one stack argument, returns eax).
lf_checker_rt::export!(thiscall, rw_00cfb180(this: u32, obj: u32) -> u32 {
    unsafe {
        const COLL_OFF: u32 = 0x78;
        const MODE_OFF: u32 = 0x1c;
        const MODE_ALT: u32 = 4;
        const OUTER_OFF: u32 = 0x60;
        const FLAG_OFF: u32 = 0x46;
        const FLAG_BIT: u8 = 0x20;
        const VAL_OFF: u32 = 0x5c;
        const LOOKUP_FIXED: [u32; 6] = [0x8c, 0x8d, 0x87, 0x88, 0x89, 0x8a];
        const LOOKUP_ALT: u32 = 0x8b;
        const LOOKUP_TAIL: u32 = 0x86;
        const DEFAULT_WEIGHT: u32 = 0xc000_0000;
        const G_ALT_WEIGHT: u32 = 0x00fe_8dd8;
        const G_BASE_WEIGHT: u32 = 0x00fe_8db0;
        const G_THRESHOLD: u32 = 0x00fe_8628;
        const FIND: u32 = 1;
        const SETW: u32 = 2;
        const ATTACH: u32 = 3;
        const READV: u32 = 4;
        const FINISH: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdglobal(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }

        let coll = rd32(obj + COLL_OFF);
        let mut ans = 0u32;
        for slot in LOOKUP_FIXED {
            let h: u32 = lf_checker_rt::callee_thiscall!(FIND, u32, coll, slot);
            ans = h;
            if h != 0 {
                lf_checker_rt::callee_thiscall!(SETW, u32, h, DEFAULT_WEIGHT);
            }
        }
        let h: u32 = lf_checker_rt::callee_thiscall!(FIND, u32, coll, LOOKUP_ALT);
        ans = h;
        if h != 0 {
            let w = if rd32(this + MODE_OFF) == MODE_ALT {
                rdglobal(G_ALT_WEIGHT)
            } else {
                rdglobal(G_BASE_WEIGHT)
            };
            lf_checker_rt::callee_thiscall!(SETW, u32, h, w);
        }
        let h: u32 = lf_checker_rt::callee_thiscall!(FIND, u32, coll, LOOKUP_TAIL);
        ans = h;
        if h != 0 {
            ans = lf_checker_rt::callee_thiscall!(ATTACH, u32, coll, h);
        }
        let outer = rd32(this + OUTER_OFF);
        if outer == 0 {
            return ans;
        }
        if rd8(outer + FLAG_OFF) & FLAG_BIT == 0 {
            let w = rdglobal(G_ALT_WEIGHT);
            return lf_checker_rt::callee_thiscall!(FINISH, u32, this, w);
        }
        let f = f32::from_bits(rd32(outer + VAL_OFF));
        let g = f32::from_bits(rdglobal(G_THRESHOLD));
        if f == g {
            let v: f32 = lf_checker_rt::callee_thiscall!(READV, f32, outer);
            lf_checker_rt::callee_thiscall!(FINISH, u32, this, v.to_bits())
        } else {
            let w = rdglobal(G_ALT_WEIGHT);
            lf_checker_rt::callee_thiscall!(FINISH, u32, this, w)
        }
    }
});
