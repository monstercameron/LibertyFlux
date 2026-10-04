// original: 0x00875390 rage::crmtRequestAddN::vf2
/// Build the summed output of an additive blend motion request.
///
/// Creates the output object through the allocator helper (stubbed),
/// moves the two child references into its slots `0x124` and `0x120`
/// (taking a reference on each new child through vtable slot 1 and
/// releasing each displaced child through vtable slot 2), runs the
/// combine helper (stubbed), then gathers the live source pairs into
/// the output: for each of the thirty-two lanes whose guard word at
/// `0x14..` is non-zero, the words at `0x94..` and `0x114..` are packed
/// consecutively from output offset `0x20`. Returns the output object.
export!(thiscall, rw_00875390(this: u32, tag: u32, finish: u32) -> u32 {
    unsafe {
        const SLOT_HI: usize = 0x198 / 4;
        const SLOT_LO: usize = 0x194 / 4;
        const OUT_HI: usize = 0x124 / 4;
        const OUT_LO: usize = 0x120 / 4;
        const GUARD: usize = 0x14 / 4;
        const SRC0: usize = 0x94 / 4;
        const SRC1: usize = 0x114 / 4;
        const OUT_FIRST: usize = 0x20 / 4;
        const LANES: usize = 32;
        let out: u32 = callee_cdecl!(1, u32, tag);
        let base = this as *const u32;
        let hi = base.add(SLOT_HI).read();
        if hi != 0 {
            let vt = (hi as *const u32).read();
            let target = (vt as *const u32).add(1).read();
            let addref: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let _: u32 = addref(hi);
        }
        let dst = out as *mut u32;
        let old_hi = dst.add(OUT_HI).read();
        if old_hi != 0 {
            let vt = (old_hi as *const u32).read();
            let target = (vt as *const u32).add(2).read();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let _: u32 = release(old_hi);
        }
        dst.add(OUT_HI).write(hi);
        let lo = base.add(SLOT_LO).read();
        if lo != 0 {
            let vt = (lo as *const u32).read();
            let target = (vt as *const u32).add(1).read();
            let addref: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let _: u32 = addref(lo);
        }
        let old_lo = dst.add(OUT_LO).read();
        if old_lo != 0 {
            let vt = (old_lo as *const u32).read();
            let target = (vt as *const u32).add(2).read();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let _: u32 = release(old_lo);
        }
        dst.add(OUT_LO).write(lo);
        let _: u32 = callee_thiscall!(4, u32, this, tag, finish, out);
        let mut lane: usize = 0;
        let mut pair = dst.add(OUT_FIRST);
        while lane < LANES {
            if base.add(GUARD + lane).read() != 0 {
                pair.add(0).write(base.add(SRC0 + lane).read());
                pair.add(1).write(base.add(SRC1 + lane).read());
                pair = pair.add(2);
            }
            lane += 1;
        }
        out
    }
});
