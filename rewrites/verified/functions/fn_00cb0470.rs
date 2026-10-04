// original: 0x00cb0470 CTaskComplexMoveAroundCoverPoints::vf20

/// Decide the next step of the move-around-cover task: keep or drop the
/// current cover approach depending on a float probe, a position check and
/// the pedestrian's state flags.
///
/// `this` is the task object; `ped` points at the pedestrian (vtable at `+0`,
/// aux block at `+AUX_BLK`, state block at `+STATE_BLK`). Returns the
/// sub-task at `+SUBTASK`, except on the shared release path which updates
/// the sub-task's flag word and returns 0.
///
/// Behaviour:
/// - Always notify the pedestrian (callee id 1) and set flag `AUX_FLAG` on
///   the aux block. If the state block is missing, take the release path.
/// - Otherwise query `kind(subtask)` (id 10, vtable slot `+0xc`): remember
///   the sub-task when it answers `KIND_COVER`, and latch the gate byte from
///   callee id 2.
/// - When a sub-task was remembered, run the float probe: virtual call
///   id 11 on the cover handle (slot `+8`, object at `subtask+HANDLE`) whose
///   single-precision result is classified by callee id 3. Answer 1, or a
///   set gate byte, continues below; otherwise take the swap path.
/// - With no remembered sub-task, a set gate byte takes the release path,
///   a clear one the swap path.
/// - Continue path: fetch a position through the pedestrian's virtual slot
///   `+POS_SLOT` (id 12; its buffer argument points at uninitialised stack
///   holding the defined fill, only the pointed-to word is compared). Unless
///   the squared length `((x*x + y*y) + z*z)` exceeds `FAR2` and the state
///   attribute has exactly the `ATTR_BITS` bits, take the release path;
///   otherwise tap the handle (id 13) and return the sub-task.
/// - Swap path: when `kind(subtask) == KIND_COVER`, re-seat the handle
///   through id 14 `(ped, this+SCRATCH, 0)` and confirm through id 13 with
///   code 2 (the same slot id 13 taps above); return the sub-task.
/// - Release path: when bit 0 of the sub-task's flag word `+FLAGS` is clear,
///   call its virtual slot `+0x14` (id 16) and set bit 1 of the flags when
///   that answers true; return 0.
///
/// Original: 0x00cb0470 (thiscall, one stack word). Single-precision SSE in
/// the original's operand order; no globals except a pristine constant.
fn decide_00cb0470<const KIND_BUMP: bool>(this: u32, ped: u32) -> u32 {
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read_unaligned() }
    }
    fn kind_of(obj: u32) -> u32 {
        unsafe {
            let vtable = (obj as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(0x0c) as *const u32).read_unaligned();
            let query: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            query(obj)
        }
    }
    #[inline(always)]
    fn fmul(x: f32, y: f32) -> f32 {
        core::hint::black_box(x) * core::hint::black_box(y)
    }
    #[inline(always)]
    fn fadd(x: f32, y: f32) -> f32 {
        core::hint::black_box(x) + core::hint::black_box(y)
    }
    fn release(task: u32, ped: u32) -> u32 {
        unsafe {
            const FLAGS: u32 = 0x0c;
            if rd8(task + FLAGS) & 1 == 0 {
                let vtable = rd32(task);
                let target = rd32(vtable + 0x14);
                let free: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                if free(task, ped, 2, 0) as u8 != 0 {
                    wr32(task + FLAGS, rd32(task + FLAGS) | 2);
                }
            }
            0
        }
    }
    unsafe {
        const SUBTASK: u32 = 0x08;
        const SCRATCH: u32 = 0x20;
        const HANDLE: u32 = 0x14;
        const AUX_BLK: u32 = 0xa80;
        const AUX_FLAG_AT: u32 = 0x50;
        const AUX_FLAG: u32 = 0x1_0000;
        const STATE_BLK: u32 = 0xd68;
        const POS_SLOT: u32 = 0xec;
        const KIND_COVER: u32 = 0x384;
        const ATTR_BITS: u32 = 0x40;
        const ATTR_MASK: u32 = 0x60;
        const FAR2_GVA: u32 = 0x00fe_88e8;
        const PROBE_OK: u32 = 1;
        let kind_want = if KIND_BUMP { KIND_COVER + 1 } else { KIND_COVER };
        lf_checker_rt::callee_thiscall!(1, u32, ped, 1);
        let aux = rd32(ped + AUX_BLK);
        wr32(aux + AUX_FLAG_AT, rd32(aux + AUX_FLAG_AT) | AUX_FLAG);
        let task = rd32(this + SUBTASK);
        if rd32(ped + STATE_BLK) == 0 {
            return release(task, ped);
        }
        let mut res: u32 = 0;
        if kind_of(task) == kind_want {
            res = task;
        }
        let gate = lf_checker_rt::callee_thiscall!(2, u32, this, ped) as u8;
        if res != 0 {
            let handle_at = res.wrapping_add(HANDLE);
            let handle = rd32(handle_at);
            let target = rd32(handle + 8);
            let probe: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(target as usize);
            let h = probe(handle_at);
            if lf_checker_rt::callee_cdecl!(3, u32, h.to_bits()) != PROBE_OK && gate == 0 {
                if kind_of(task) == kind_want {
                    let o = task.wrapping_add(HANDLE);
                    let ov = rd32(o);
                    let tgt = rd32(ov + 0x24);
                    let reseat: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(tgt as usize);
                    reseat(o, ped, this + SCRATCH, 0);
                    let a2 = rd32(o);
                    let tgt2 = rd32(a2 + 0x0c);
                    let confirm: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(tgt2 as usize);
                    confirm(o, 2);
                }
                return task;
            }
            let v = rd32(ped);
            let t = rd32(v + POS_SLOT);
            let mut buf: u32 = 0;
            let buf_ptr = (&mut buf as *mut u32) as u32;
            let fetch: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(t as usize);
            let r2 = fetch(ped, buf_ptr);
            let x = ((r2) as *const f32).read_unaligned();
            let y = ((r2 + 4) as *const f32).read_unaligned();
            let z = ((r2 + 8) as *const f32).read_unaligned();
            let d2 = fadd(fadd(fmul(x, x), fmul(y, y)), fmul(z, z));
            let far2 = lf_checker_rt::global::<f32>(FAR2_GVA).read_unaligned();
            if !(d2 > far2) {
                return release(task, ped);
            }
            let attr = rd8(rd32(ped + STATE_BLK));
            if attr & (ATTR_MASK as u8) != ATTR_BITS as u8 {
                return release(task, ped);
            }
            let c = task.wrapping_add(HANDLE);
            let cv = rd32(c);
            let tgt = rd32(cv + 0x0c);
            let tap: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            tap(c, 1);
            return task;
        }
        if gate != 0 {
            return release(task, ped);
        }
        if kind_of(task) == kind_want {
            let o = task.wrapping_add(HANDLE);
            let ov = rd32(o);
            let tgt = rd32(ov + 0x24);
            let reseat: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            reseat(o, ped, this + SCRATCH, 0);
            let a2 = rd32(o);
            let tgt2 = rd32(a2 + 0x0c);
            let confirm: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt2 as usize);
            confirm(o, 2);
        }
        task
    }
}

lf_checker_rt::export!(thiscall, rw_00cb0470(this: u32, ped: u32) -> u32 {
    decide_00cb0470::<false>(this, ped)
});
