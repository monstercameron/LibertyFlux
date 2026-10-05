// original: 0x00c91890 peds_task_init_c91890 (proposed)

/// Initialise a ped task/IK member object: vtable, scalar fields, flag bits,
/// an optional 4-word vector, then two member calls.
///
/// `this` points to the object (at least 0x68 bytes). Twelve stack words
/// follow (`thiscall`, callee cleans 0x30 bytes); the first (`_a0`) is never
/// read. Stored: `a8`/`a9` at `+0x24`/`+0x28`, the vtable at `+0x0`,
/// `0xffffffff` at `+0x20`/`+0x2c`, zero at `+0x30`..`+0x48`, `a11` at
/// `+0x58`, `a1` at `+0x64`, `a2` at `+0x18`, zero at `+0x50`, `1.0f` at
/// `+0x54`, `a3` at `+0x1c`, `a4` at `+0x14`, `a5` at `+0x2c`, `a7` at
/// `+0x20` (again), `a8`/`a9` again, the low byte of `a10` at `+0x60`,
/// `a11` again at `+0x58`.
///
/// Flag word at `+0x5c`: the incoming value is masked with `~0xb`, bit 2 is
/// set, then bit 1 is set when `a4` is non-null and cleared when it is null.
/// When `a4` is non-null, callee 1 runs as `thiscall(a4, this+0x14)`.
///
/// Four words at `+0x30`..`+0x3c`: when `a6` is non-null they are copied
/// bitwise from `[a6..a6+0xc]`; when it is null the first three are zeroed
/// and the last receives a word the original reads from uninitialised stack
/// scratch (its aligned-frame spill area, never written), which is 0 under
/// the checker's zero stack fill. Narrowing: that word is matched to the
/// fill, not to real game stack garbage.
///
/// Finally callee 2 runs as `thiscall(this)` and callee 3 as
/// `thiscall(this, [this+0x24])`; the function returns `this`.
lf_checker_rt::export!(thiscall, rw_00c91890(this: u32, _a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32, a11: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ED_6EB8;
        const ONE_F: u32 = 0x3F80_0000;
        const FLAG_KEEP: u32 = 0xFFFF_FFF4;
        const FLAG_SET: u32 = 0x4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(this.wrapping_add(0x24), a8);
        wr32(this.wrapping_add(0x28), a9);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        wr32(this.wrapping_add(0x20), 0xFFFF_FFFF);
        wr32(this.wrapping_add(0x2C), 0xFFFF_FFFF);
        for off in [0x30u32, 0x34, 0x38, 0x40, 0x44, 0x48] {
            wr32(this.wrapping_add(off), 0);
        }
        let flags = (rd32(this.wrapping_add(0x5C)) & FLAG_KEEP) | FLAG_SET;
        wr32(this.wrapping_add(0x58), a11);
        wr32(this.wrapping_add(0x64), a1);
        wr32(this.wrapping_add(0x18), a2);
        wr32(this.wrapping_add(0x50), 0);
        wr32(this.wrapping_add(0x54), ONE_F);
        wr32(this.wrapping_add(0x5C), flags);
        // Byte +0x60 is set to 3 here and overwritten by a10's low byte
        // below; only the final value is observable, so one store.
        wr32(this.wrapping_add(0x1C), a3);
        wr32(this.wrapping_add(0x14), a4);
        if a4 != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, a4, this.wrapping_add(0x14));
            wr32(this.wrapping_add(0x5C), rd32(this.wrapping_add(0x5C)) | 2);
        } else {
            wr32(this.wrapping_add(0x5C), flags & 0xFFFF_FFFD);
        }
        wr32(this.wrapping_add(0x2C), a5);
        if a6 != 0 {
            wr32(this.wrapping_add(0x30), rd32(a6));
            wr32(this.wrapping_add(0x34), rd32(a6.wrapping_add(4)));
            wr32(this.wrapping_add(0x38), rd32(a6.wrapping_add(8)));
            wr32(this.wrapping_add(0x3C), rd32(a6.wrapping_add(0xC)));
        } else {
            wr32(this.wrapping_add(0x30), 0);
            wr32(this.wrapping_add(0x34), 0);
            wr32(this.wrapping_add(0x38), 0);
            wr32(this.wrapping_add(0x3C), 0);
        }
        wr32(this.wrapping_add(0x20), a7);
        wr32(this.wrapping_add(0x24), a8);
        wr32(this.wrapping_add(0x28), a9);
        (this as *mut u8).add(0x60).write((a10 & 0xFF) as u8);
        wr32(this.wrapping_add(0x58), a11);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this);
        let last_arg = rd32(this.wrapping_add(0x24));
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, this, last_arg);
        this
    }
});
