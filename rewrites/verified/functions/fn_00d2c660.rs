// original: 0x00d2c660 CTaskComplexMoveGetOntoMainNavMesh::vf20
/// Poll the sub-task at `+8`: unless its virtual kind (slot `+0xc`) is
/// 0x387 and the arm byte (`+0x4c`) is set, return it at once. Otherwise
/// refresh the start time (`+0x44`) from the global clock when the dirty
/// byte (`+0x4d`) is set, and return the sub-task unless start+span
/// (`+0x48`, signed) has reached the clock. Then, when its flag word
/// (`+0xc`) already has bit 0, return 0 at once; else run its virtual
/// check (slot `+0x14`) with (`arg`, 1, 0), returning the sub-task when
/// that answers zero, and otherwise setting bit 1 of the flag word and
/// returning 0.
///
/// Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00d2c660(this: u32, arg: u32) -> u32 {
    unsafe {
        const WANT_KIND: u32 = 0x387;
        const TIME_GLOB: u32 = 0x011735b4;
        const VT_KIND: u32 = 0x0c;
        const VT_CHECK: u32 = 0x14;
        let sub = ((this + 8) as *const u32).read_unaligned();
        let vt = (sub as *const u32).read_unaligned();
        let kind_tgt = ((vt + VT_KIND) as *const u32).read_unaligned();
        let kind: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(kind_tgt as usize);
        if kind(sub) != WANT_KIND {
            return sub;
        }
        if ((this + 0x4c) as *const u8).read() == 0 {
            return sub;
        }
        if ((this + 0x4d) as *const u8).read() != 0 {
            let now = lf_checker_rt::global::<u32>(TIME_GLOB).read_unaligned();
            ((this + 0x44) as *mut u32).write_unaligned(now);
            ((this + 0x4d) as *mut u8).write(0);
        }
        let span = ((this + 0x48) as *const u32).read_unaligned();
        let start = ((this + 0x44) as *const u32).read_unaligned();
        let end = span.wrapping_add(start);
        let now = lf_checker_rt::global::<u32>(TIME_GLOB).read_unaligned();
        if (end as i32) > (now as i32) {
            return sub;
        }
        let s2 = ((this + 8) as *const u32).read_unaligned();
        if ((s2 + 0x0c) as *const u32).read_unaligned() & 1 != 0 {
            return 0;
        }
        let vt2 = (s2 as *const u32).read_unaligned();
        let chk_tgt = ((vt2 + VT_CHECK) as *const u32).read_unaligned();
        let check: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(chk_tgt as usize);
        if check(s2, arg, 1, 0) & 0xff == 0 {
            return sub;
        }
        let f = ((s2 + 0x0c) as *const u32).read_unaligned();
        ((s2 + 0x0c) as *mut u32).write_unaligned(f | 2);
        0
    }
});
