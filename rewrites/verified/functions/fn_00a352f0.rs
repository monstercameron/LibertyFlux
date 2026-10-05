// original: 0x00a352f0 vehicle_best_entry (proposed)

/// Pick the first of 16 table entries the observer accepts into a new node.
///
/// Takes a node from the allocator (id 1, cdecl/0) -- null answers null --
/// stamps `x` at `+0` and the global stamp at `+0x1c`, then scans the 16
/// `0x60`-byte entries at `arr`: an entry whose head word is zero is
/// skipped, else when `flags & 1` the observer (id 2, thiscall/1 through
/// the global object's vtable slot `+0x14`) examines its `+0x48` word and
/// a set bit 1 of the answer's `+0x38` word -- or a set bit 27 of the
/// entry word -- skips it. When `flags & 2` is clear the entry is taken;
/// else the observer runs again and the entry is taken unless its answer
/// has bit 0 of `+0x38` set or the entry word has bit 26 set. A taken
/// entry copies its six words (`+0x10`..`+0x28`) into the node
/// (`+0x4`..`+0x18`) and sets bit 0 of `+0x20`; an empty scan clears that
/// bit instead. Cdecl/3, returns the node.
lf_checker_rt::export!(cdecl, rw_00a352f0(x: u32, flags: u32, arr: u32) -> u32 {
    unsafe {
        const STAMP: u32 = 0x0117_35B4;
        const OBSERVER_GLOBAL: u32 = 0x018B_8968;
        const VTABLE_SLOT: u32 = 0x14;
        const FLAG: u32 = 0x20;
        const ALLOC: u32 = 1;
        const STRIDE: u32 = 0x60;
        const COUNT: u32 = 16;
        const SRC: [u32; 6] = [0x10, 0x14, 0x18, 0x20, 0x24, 0x28];
        let node: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32,);
        if node == 0 {
            return 0;
        }
        core::ptr::write_unaligned(node as *mut u32, x);
        let stamp = core::ptr::read(lf_checker_rt::global::<u32>(STAMP));
        core::ptr::write_unaligned((node + 0x1C) as *mut u32, stamp);
        let obj = core::ptr::read(lf_checker_rt::global::<u32>(OBSERVER_GLOBAL));
        let slot = core::ptr::read_unaligned(
            (core::ptr::read_unaligned(obj as *const u32) + VTABLE_SLOT) as *const u32,
        );
        let observe: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(slot as usize);
        for i in 0..COUNT {
            let e = arr.wrapping_add(i.wrapping_mul(STRIDE));
            if core::ptr::read_unaligned(e as *const u32) == 0 {
                continue;
            }
            let w = core::ptr::read_unaligned((e + 0x48) as *const u32);
            if flags & 1 != 0 {
                let o = observe(obj, w);
                if (core::ptr::read_unaligned((o + 0x38) as *const u32) >> 1) & 1 != 0 {
                    continue;
                }
                if (w >> 27) & 1 != 0 {
                    continue;
                }
            }
            let mut take = false;
            if flags & 2 == 0 {
                take = true;
            } else {
                let o = observe(obj, w);
                if core::ptr::read((o + 0x38) as *const u8) & 1 == 0 && (w >> 26) & 1 == 0 {
                    take = true;
                }
            }
            if take {
                for (k, off) in SRC.iter().enumerate() {
                    let v = core::ptr::read_unaligned((e + off) as *const u32);
                    core::ptr::write_unaligned((node + 4 + k as u32 * 4) as *mut u32, v);
                }
                let f = core::ptr::read_unaligned((node + FLAG) as *const u32);
                core::ptr::write_unaligned((node + FLAG) as *mut u32, f | 1);
                return node;
            }
        }
        let f = core::ptr::read_unaligned((node + FLAG) as *const u32);
        core::ptr::write_unaligned((node + FLAG) as *mut u32, f & 0xFFFF_FFFE);
        node
    }
});
