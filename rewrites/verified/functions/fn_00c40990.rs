// original: 0x00c40990 timing_attach_a (proposed)

/// Attach a timing client to its channel, resolving through a node list.
///
/// `arg` points to a client (or is null) and `this` receives three fields:
/// the client at `+0x1c`, a resolved object at `+0x14` and a handle at
/// `+0x18`. The node list hanging off the client at `+0x224`/`+0x2e0` is
/// walked for the node whose id word at `+4` is `0x2de`, gated by a
/// predecessor class check (a node's 3-bit class must not rise from a
/// predecessor at 2 or above); misses return 0. A probe callee must answer
/// in `0x15..=0x1a`, a resolver callee
/// then supplies the object, whose signed 16-bit id at `+0x2e` must avoid
/// two excluded ids, whose table float at `+0x38` must not exceed 1.75 and
/// whose state at `+0x1300` must be neither 1 nor 2. A fetch callee answers
/// an index that must be non-negative; two lookup callees must then answer
/// null, and a final callee supplies the handle. Success (1) needs all
/// three fields non-null with the handle differing from the client.
///
/// Original: 0x00c40990 (thiscall, one stack word). Returns 0 or 1 in
/// `al`; the upper bits of `eax` are leftovers.
lf_checker_rt::export!(thiscall, rw_00c40990(this: u32, arg: u32) -> u32 {
    unsafe { a0990_core(this, arg, false) }
});

unsafe fn a0990_core(this: u32, arg: u32, zul_eq_ok: bool) -> u32 {
    unsafe {
        const F_OBJ: u32 = 0x14;
        const F_HANDLE: u32 = 0x18;
        const F_CLIENT: u32 = 0x1c;
        const NODE_ID: u32 = 0x2de;
        const C_PROBE: u32 = 1;
        const C_RESOLVE: u32 = 2;
        const C_FETCH: u32 = 3;
        const C_LOOK1: u32 = 4;
        const C_LOOK2: u32 = 5;
        const C_HANDLE: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(this + F_CLIENT, 0);
        wr32(this + F_OBJ, 0);
        wr32(this + F_HANDLE, 0);
        if arg == 0 {
            return tail(this, zul_eq_ok);
        }
        wr32(this + F_CLIENT, arg);
        let anchor = rd32(arg + 0x224).wrapping_add(0x2e0);
        let mut node = rd32(anchor);
        if node == 0 {
            return 0;
        }
        // Class gate: the previous node's 3-bit class (bits 1..3 of the
        // word at +8) must not be below the current node's unless below 2.
        // The loop re-enters past the first computation, so on the first
        // node both sides are the same value (always taken) while later
        // nodes compare against their predecessor.
        let mut prev = (rd32(node + 8) >> 1) & 7;
        loop {
            let cur = (rd32(node + 8) >> 1) & 7;
            if prev < cur && prev >= 2 {
                return 0;
            }
            if rd32(node + 4) == NODE_ID {
                break;
            }
            prev = cur;
            node = rd32(node + 0xc);
            if node == 0 {
                return 0;
            }
        }
        let probe: u32 = lf_checker_rt::callee_thiscall!(C_PROBE, u32, anchor, NODE_ID, 5);
        if probe.wrapping_sub(0x15) > 5 {
            return 0;
        }
        let obj: u32 = lf_checker_rt::callee_thiscall!(C_RESOLVE, u32, anchor, NODE_ID, 5);
        wr32(this + F_OBJ, obj);
        if obj == 0 {
            return 0;
        }
        let id = (obj as *const u16).add(0x2e / 2).read_unaligned() as i16 as i32;
        if id == lf_checker_rt::global::<i32>(0x12fa65c).read()
            || id == lf_checker_rt::global::<i32>(0x12fa290).read()
        {
            return 0;
        }
        let entry = rd32(
            lf_checker_rt::relocated(0x1295cd8).wrapping_add((id as u32).wrapping_mul(4)),
        );
        if f32::from_bits(rd32(entry + 0x38)) > f32::from_bits(0x3fe0_0000) {
            return 0;
        }
        let state = rd32(obj + 0x1300);
        if state == 1 || state == 2 {
            return 0;
        }
        let idx: u32 = lf_checker_rt::callee_thiscall!(C_FETCH, u32, anchor, NODE_ID, 5);
        if (idx as i32) < 0 {
            return tail(this, zul_eq_ok);
        }
        let q: u32 = lf_checker_rt::callee_thiscall!(C_LOOK1, u32, rd32(this + F_OBJ), idx);
        let z: u32 = lf_checker_rt::callee_thiscall!(C_LOOK2, u32, q, 0);
        if z != 0 {
            return tail(this, zul_eq_ok);
        }
        let h: u32 = lf_checker_rt::callee_thiscall!(C_HANDLE, u32, rd32(this + F_OBJ), 0);
        wr32(this + F_HANDLE, h);
        tail(this, zul_eq_ok)
    }
}

#[inline(always)]
unsafe fn tail(this: u32, zul_eq_ok: bool) -> u32 {
    unsafe {
        let c = (this as *const u32).add(0x1c / 4).read_unaligned();
        let o = (this as *const u32).add(0x14 / 4).read_unaligned();
        let h = (this as *const u32).add(0x18 / 4).read_unaligned();
        if c != 0 && o != 0 && h != 0 && (zul_eq_ok || h != c) {
            1
        } else {
            0
        }
    }
}
