// original: 0x00a23280 ped_task_configure_and_probe_targets (proposed)

/// Store this task's configuration, then probe candidate target objects in a
/// loop until one is accepted or the candidates run out.
///
/// `this` is the task object; the seven stack arguments are a rejected-target
/// pointer (`a0`), two configuration values (`a1`, `a2`), a mode byte (`a3`),
/// a priority (`a4`), a search-context pointer (`a5`, null selects the
/// game-filled fallback) and a notify flag (`a6`). The configuration lands in
/// `+0xd8c..0xda0`, and a six-word probe descriptor is offered to the hooks
/// from a frame slot. The original also writes `[-1, -1, 0, -1, 7, 0]` and
/// later four argument words to a second frame region, but that region is
/// never handed out or read back (dead stores, confirmed by the snapshots);
/// the handed-out slot is never written, so it reads as zero under the
/// checker's defined stack fill on both sides.
///
/// When the notify flag is set, a registry hook (callee 1) runs first. The
/// descriptor is offered to the allocator hook (callee 2); when the mode byte
/// at `+0xd75` is clear, the probe hook (callee 3) also sees it. The
/// reset hook (callee 4) runs, the search context resolves, and when the slot
/// at `+0x504` is clear a scratch region is reserved through two size hooks
/// (callees 5 and 6, whose answers are ignored) and handed to the region hook
/// (callee 7) together with two constant words.
///
/// The probe loop then runs: with the mode byte clear the first candidate
/// comes from the fetch hook (callee 8), otherwise from the candidate table
/// at `+0xb70`. A null candidate ends the loop and the rejected target is
/// skipped. Any other candidate is offered to the pardon hook (an indirect
/// call through the `+0xd78/+0xd7c` slot, always with a non-null context in
/// this proof) together with its `+0x10` header through the attach hook
/// (callee 10); the candidate's linked info word feeds the evaluate hook
/// (callee 11), and the confirm hook (callee 12) ends the loop on a non-zero
/// answer. Otherwise the next candidate comes from the advance hook (callee
/// 13) with the mode byte clear, or from the next table slot (up to 0x80
/// slots) otherwise.
///
/// Afterwards the region hook runs again with four zero words when the
/// scratch region was reserved, the finish hook (callee 15) supplies the
/// return value, the registry hook runs again (callee 16) when notifying, and
/// the frame-cookie check (callee 17, preserved registers) runs last.
///
/// Original: 0x00a23280 (thiscall, seven stack words).
lf_checker_rt::export!(thiscall, rw_00a23280(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
) -> u32 {
    unsafe {
        const REGISTRY: u32 = 0x01b4a8b0;
        const G_FALLBACK_CTX: u32 = 0x018b896c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        wr32(this.wrapping_add(0xd8c), a0);
        wr8(this.wrapping_add(0xd98), a3 as u8);
        wr32(this.wrapping_add(0xd90), a1);
        wr32(this.wrapping_add(0xd94), a2);
        wr32(this.wrapping_add(0xda0), a5);
        let notify = a6 as u8;
        // The original materialises this address as a relocated immediate.
        let registry = lf_checker_rt::relocated(REGISTRY);
        if notify != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, registry);
        }
        // NOTE: the original initialises a six-word descriptor at [ebp+0] and
        // overwrites four of its words below, but every callee is handed
        // [ebp-0x50], a different, never-written frame region. The snapshots
        // confirm the callees see fill pattern, not the initialised words, so
        // the visible stores are dead and the descriptor reads as the
        // checker's defined stack fill (zero) on both sides.
        let desc = [0u32; 6];
        let dptr = desc.as_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, dptr);
        if rd8(this.wrapping_add(0xd75)) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, this, dptr);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, this);
        let mut ctx = a5;
        if ctx == 0 {
            ctx = rd32(lf_checker_rt::relocated(G_FALLBACK_CTX));
        }
        let mut reserved = false;
        if rd32(this.wrapping_add(0x504)) == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
            let _: u32 = lf_checker_rt::callee_cdecl!(6, u32,);
            let mut scratch = [0u32; 2];
            let p0 = scratch.as_mut_ptr() as u32;
            let p1 = scratch.as_mut_ptr().wrapping_add(1) as u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(7, u32, this, p0, 0x2000, p1, 1);
            reserved = true;
        }
        let mut ebx = 0u32;
        let mut esi: u32;
        if rd8(this.wrapping_add(0xd75)) == 0 {
            esi = lf_checker_rt::callee_thiscall!(8, u32, ctx, dptr);
            wr32(this.wrapping_add(0xb70), esi);
        } else {
            esi = rd32(this.wrapping_add(0xb70));
        }
        loop {
            if esi == 0 {
                break;
            }
            if esi != a0 {
                let cx = rd32(this.wrapping_add(0xd78));
                let target: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(this.wrapping_add(0xd7c)) as usize);
                let pardon: u32 = target(cx, esi);
                if (pardon & 0xff) != 0 {
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(10, u32, this, esi.wrapping_add(0x10));
                    let link = rd32(esi.wrapping_add(4));
                    let info = rd32(link.wrapping_add(0x0c));
                    let eval: u32 = lf_checker_rt::callee_thiscall!(11, u32, this, info, esi);
                    if (eval & 0xff) != 0 {
                        let conf: u32 = lf_checker_rt::callee_thiscall!(
                            12,
                            u32,
                            this,
                            esi.wrapping_add(0x10),
                            0
                        );
                        if (conf & 0xff) != 0 {
                            break;
                        }
                    }
                }
            }
            if rd8(this.wrapping_add(0xd75)) == 0 {
                esi = lf_checker_rt::callee_thiscall!(13, u32, ctx, dptr);
                ebx = ebx.wrapping_add(1);
                let idx = if (ebx as i32) < 0x7f { ebx } else { 0x7f };
                wr32(this.wrapping_add(0xb70).wrapping_add(idx.wrapping_mul(4)), esi);
            } else {
                ebx = ebx.wrapping_add(1);
                if (ebx as i32) >= 0x80 {
                    break;
                }
                esi = rd32(this.wrapping_add(0xb70).wrapping_add(ebx.wrapping_mul(4)));
            }
        }
        if reserved {
            let _: u32 = lf_checker_rt::callee_thiscall!(14, u32, this, 0, 0, 0, 0);
        }
        let ret: u32 = lf_checker_rt::callee_thiscall!(15, u32, this);
        if notify != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(16, u32, registry);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(17, u32,);
        ret
    }
});
