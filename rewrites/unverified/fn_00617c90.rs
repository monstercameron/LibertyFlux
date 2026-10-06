// original: 0x00617C90 files_mem_dtor (proposed)

/// Tear down a files-memory owner object: release its allocator block, drop
/// six reference-counted members, shut down three sub-objects and restore
/// the base vtable.
///
/// `this` points to an object of at least 0x2EC bytes. The teardown runs in
/// this order: stamp the derived vtable; if the allocator block at
/// `+0x2CC` is set, notify the free callee of its `+0x04` word, release the
/// two objects at block `+0x08`/`+0x0C` through vtable slot 0 when set, and
/// return the block through the thread-local allocator's free slot; then
/// drop the six members at `+0x2E8`, `+0x2E4`, `+0x2E0`, `+0x2DC`,
/// `+0x2D8`, `+0x2D4` in that order; then shut down the object at `+0x2C4`
/// when set (and null the cell), the sub-object at `+0x250` and the one at
/// `+0x10`; finally stamp the base vtable.
///
/// Member rule (all six alike): a null member is skipped; a zero 16-bit
/// refcount at member `+0x0A` is skipped; otherwise the refcount is
/// decremented and stored back, and when it reaches zero the member is
/// released through vtable slot 0 with argument 1 only if the flag byte at
/// member `+0x08` is 2 or 4. All comparisons are equality against
/// zero/two/four; nothing is compared as signed or unsigned.
///
/// Arguments: thiscall/0 (object in ECX). Return value: none (the original
/// leaves the last shutdown callee's answer in EAX; callers use nothing).
///
/// Edge cases: every pointer is null-checked before use, so with all cells
/// null the function makes exactly the two unconditional shutdown calls.
/// The allocator-block path is the only one that touches thread-local
/// state; it is skipped entirely when the block cell is null.
lf_checker_rt::export!(thiscall, rw_00617C90(this: u32) -> u32 {
    unsafe {
        const VT_DERIVED: u32 = 0x00EF_4220;
        const VT_BASE: u32 = 0x00E8_6AFC;
        const O_BLOCK: u32 = 0x2CC;
        const O_SUB3: u32 = 0x2C4;
        const O_SUB2: u32 = 0x250;
        const O_SUB1: u32 = 0x10;
        const M_FLAG: u32 = 0x08;
        const M_REF: u32 = 0x0A;
        const CFREE_NOTIFY: u32 = 1;
        const CSHUT3: u32 = 4;
        const CSHUT2: u32 = 5;
        const CSHUT1: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn release(obj: u32) {
            unsafe {
                let vt = rd32(obj);
                let tgt = rd32(vt);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                f(obj, 1);
            }
        }
        #[inline(always)]
        unsafe fn drop_member(m: u32) {
            unsafe {
                if m == 0 {
                    return;
                }
                let rc = ((m.wrapping_add(M_REF)) as *const u16).read_unaligned();
                if rc == 0 {
                    return;
                }
                let flag = ((m.wrapping_add(M_FLAG)) as *const u8).read();
                let next = rc.wrapping_sub(1);
                ((m.wrapping_add(M_REF)) as *mut u16).write_unaligned(next);
                if next == 0 && (flag == 2 || flag == 4) {
                    release(m);
                }
            }
        }

        use lf_checker_rt::{callee_cdecl, callee_thiscall, relocated, tls_slot};
        let blk = rd32(this.wrapping_add(O_BLOCK));
        wr32(this, relocated(VT_DERIVED));
        if blk != 0 {
            callee_cdecl!(CFREE_NOTIFY, u32, rd32(blk.wrapping_add(4)));
            let e8 = rd32(blk.wrapping_add(8));
            if e8 != 0 {
                release(e8);
            }
            let ec = rd32(blk.wrapping_add(0x0C));
            if ec != 0 {
                release(ec);
            }
            let slot0 = tls_slot(0);
            let obj = rd32(slot0.wrapping_add(8));
            let vt = rd32(obj);
            let tgt = rd32(vt.wrapping_add(0x0C));
            let give_back: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            give_back(obj, blk);
        }
        drop_member(rd32(this.wrapping_add(0x2E8)));
        drop_member(rd32(this.wrapping_add(0x2E4)));
        drop_member(rd32(this.wrapping_add(0x2E0)));
        drop_member(rd32(this.wrapping_add(0x2DC)));
        drop_member(rd32(this.wrapping_add(0x2D8)));
        drop_member(rd32(this.wrapping_add(0x2D4)));
        let s3 = rd32(this.wrapping_add(O_SUB3));
        if s3 != 0 {
            callee_thiscall!(CSHUT3, u32, s3);
            wr32(this.wrapping_add(O_SUB3), 0);
        }
        callee_thiscall!(CSHUT2, u32, this.wrapping_add(O_SUB2));
        callee_thiscall!(CSHUT1, u32, this.wrapping_add(O_SUB1));
        wr32(this, relocated(VT_BASE));
        0
    }
});
