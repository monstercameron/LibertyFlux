// original: 0x00c40ac0 timing_attach_b (proposed)

/// Attach a timing client of the second kind, resolving through a node list.
///
/// Same shape as its sibling `0x00c40990` with a different node id
/// (`0x2e2`), a different probe handshake (the probe must answer exactly
/// `0x13`) and the object taken from the client itself (word at `+0xb30`
/// when flag bit 2 at `+0x26c` is set) instead of a resolver callee. The
/// object's id, table float and state checks are the same, as are the
/// fetch and lookup callees. The handle comes from a final callee's
/// `+0x48` word and must be non-null with its target's word at `+4`
/// differing from 2 (an equal word fails outright). Success (1) needs all
/// three fields non-null.
///
/// Original: 0x00c40ac0 (thiscall, one stack word). Returns 0 or 1 in
/// `al`; the upper bits of `eax` are leftovers.
lf_checker_rt::export!(thiscall, rw_00c40ac0(this: u32, arg: u32) -> u32 {
    unsafe { a0ac0_core(this, arg, false) }
});

unsafe fn a0ac0_core(this: u32, arg: u32, flip_tail_check: bool) -> u32 {
    unsafe {
        const F_OBJ: u32 = 0x14;
        const F_HANDLE: u32 = 0x18;
        const F_CLIENT: u32 = 0x1c;
        const NODE_ID: u32 = 0x2e2;
        const C_PROBE: u32 = 1;
        const C_FETCH: u32 = 2;
        const C_LOOK1: u32 = 3;
        const C_LOOK2: u32 = 4;
        const C_HANDLE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn tail2(this: u32) -> u32 {
            unsafe {
                let c = rd32(this + F_CLIENT);
                let o = rd32(this + F_OBJ);
                let h = rd32(this + F_HANDLE);
                if c != 0 && o != 0 && h != 0 {
                    1
                } else {
                    0
                }
            }
        }

        wr32(this + F_CLIENT, 0);
        wr32(this + F_OBJ, 0);
        wr32(this + F_HANDLE, 0);
        if arg == 0 {
            return tail2(this);
        }
        wr32(this + F_CLIENT, arg);
        let edi = rd32(arg + 0x224);
        let mut node = rd32(edi.wrapping_add(0x2e0));
        if node == 0 {
            return 0;
        }
        loop {
            // Same always-taken self-comparison as the sibling.
            let _class = (rd32(node + 8) >> 1) & 7;
            if rd32(node + 4) == NODE_ID {
                break;
            }
            node = rd32(node + 0xc);
            if node == 0 {
                return 0;
            }
        }
        let anchor = edi.wrapping_add(0x2e0);
        let probe: u32 = lf_checker_rt::callee_thiscall!(C_PROBE, u32, anchor, NODE_ID, 5);
        if probe != 0x13 {
            return 0;
        }
        let client = rd32(this + F_CLIENT);
        let obj = if (client as *const u8).add(0x26c).read() & 4 != 0 {
            rd32(client + 0xb30)
        } else {
            0
        };
        wr32(this + F_OBJ, obj);
        if obj == 0 {
            return tail2(this);
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
            return tail2(this);
        }
        let q: u32 = lf_checker_rt::callee_thiscall!(C_LOOK1, u32, rd32(this + F_OBJ), idx);
        let z: u32 = lf_checker_rt::callee_thiscall!(C_LOOK2, u32, q, 0);
        if z != 0 {
            return tail2(this);
        }
        let base = rd32(rd32(this + F_CLIENT) + 0x224).wrapping_add(0x44);
        let u: u32 = lf_checker_rt::callee_thiscall!(C_HANDLE, u32, base, NODE_ID);
        let h = rd32(u + 0x48);
        wr32(this + F_HANDLE, h);
        if h == 0 {
            return tail2(this);
        }
        let n = rd32(h + 0x22c);
        let tail_word = rd32(n + 4);
        // NOTE the inversion: an equal word fails outright, anything else
        // proceeds to the all-nonzero tail (which then succeeds).
        if (tail_word == 2) != flip_tail_check {
            return 0;
        }
        tail2(this)
    }
}
