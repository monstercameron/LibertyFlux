// original: 0x00d2af60 task_teardown (proposed)
/// Destructor chain: stamp the two vtable slots, free the handle at `+0x6c`
/// (clearing bit 2 at `+0xd8`) and the reference at `+0x70` when set, then
/// when the block at `+0x74` is set release its handle at `+0x10`, its slot
/// at `+0x14`, free the block and clear it, and tail-jump to the base
/// destructor, forwarding `this` and its result.
///
/// Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00d2af60(this: u32) -> u32 {
    unsafe {
        const VT0V: u32 = 0x00ee1f94;
        const VT14V: u32 = 0x00ee1fec;
        const KEEP_MASK: u32 = 0xfffffffb;
        const FREE: u32 = 1;
        const UNREF: u32 = 2;
        const RELEASE: u32 = 3;
        const BLK_FREE: u32 = 4;
        const TAIL: u32 = 5;
        const MGR_GLOB: u32 = 0x0179d114;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT0V));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(VT14V));
        let h = ((this + 0x6c) as *const u32).read_unaligned();
        if h != 0 {
            lf_checker_rt::callee_cdecl!(FREE, u32, h);
            let f = ((this + 0xd8) as *const u32).read_unaligned();
            ((this + 0xd8) as *mut u32).write_unaligned(f & KEEP_MASK);
            ((this + 0x6c) as *mut u32).write_unaligned(0);
        }
        let r = ((this + 0x70) as *const u32).read_unaligned();
        if r != 0 {
            let mgr = lf_checker_rt::global::<u32>(MGR_GLOB).read_unaligned();
            lf_checker_rt::callee_thiscall!(UNREF, u32, mgr, r);
        }
        let blk = ((this + 0x74) as *const u32).read_unaligned();
        if blk != 0 {
            let bh = ((blk + 0x10) as *const u32).read_unaligned();
            if bh != 0 {
                let mgr = lf_checker_rt::global::<u32>(MGR_GLOB).read_unaligned();
                lf_checker_rt::callee_thiscall!(UNREF, u32, mgr, bh);
            }
            if ((blk + 0x14) as *const u32).read_unaligned() != 0 {
                lf_checker_rt::callee_stdcall!(RELEASE, u32, blk + 0x14);
                ((blk + 0x14) as *mut u32).write_unaligned(0);
            }
            lf_checker_rt::callee_cdecl!(BLK_FREE, u32, blk);
            ((this + 0x74) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(TAIL, u32, this)
    }
});
