// original: 0x008998c0 audio_pool_rebuild (proposed name)
/// Rebuild one audio pool's entry array after a size change.
///
/// `obj` is the pool header (entry count at +0x1c, capacity at +0x20, base
/// at +0x24), `arg1`/`arg2` are the old and new sizes. The function asks the
/// allocator for a buffer, copies the kept prefix through intercepted
/// helpers, rebases every entry by the buffer delta, validates each entry,
/// and stores the new base. When the header's base word is zero it takes a
/// shorter path that installs the fresh buffer directly.
///
/// Two original quirks are mirrored exactly: the entry-validation loop is
/// guarded by an early-exit test that adds a leftover register to the
/// count (harmless for the small counts used here, but observable in
/// principle), and the returned eax keeps the high bytes of whatever value
/// the last computation left behind with only the low byte forced to 1.
export!(thiscall, rw_008998c0(obj: *mut u8, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        let e20 = *(obj.add(0x20) as *const u32);
        let ebp: u32 = callee_thiscall!(1, u32, relocated(0x0115d9a0), e20.wrapping_add(arg2), 0x10);
        // Out-slot for the id2 stub: one count word stored through the
        // pointer argument (contract `writes`).
        let mut scratch = [0u32; 2];
        callee_thiscall!(2, u32, arg1, scratch.as_mut_ptr() as u32, 4);
        let s10 = scratch[0];
        // The buffer pointer and the spilled words all alias the id1 answer:
        // the original saves ebp over the same scratch the stub writes
        // beside, so every one of these reads observes ebp.
        let buf_ptr = ebp;
        let q: u32 = ebp;
        let e4 = *(obj.add(4) as *const u32);
        let e1c = *(obj.add(0x1c) as *const u32);
        if e4 != 0 {
            let edi_var = s10.wrapping_mul(4);
            let b4 = e1c.wrapping_mul(4);
            callee_cdecl!(4, u32, ebp, e4, b4);
            // The caller spills (capacity - kept) over its addend scratch.
            let s18b = e20.wrapping_sub(b4);
            // The buffer end advances past the kept region; every later use
            // of ebp observes the advanced value (the spilled copy keeps the
            // original answer and feeds the slots above).
            let ebp2 = ebp.wrapping_add(edi_var).wrapping_add(b4);
            callee_cdecl!(4, u32, ebp2, e4.wrapping_add(b4), s18b);
            callee_thiscall!(5, u32, (obj as u32).wrapping_add(0x10));
            callee_thiscall!(6, u32, relocated(0x0115d9a0), e4);
            callee_thiscall!(3, u32, arg1, q.wrapping_add(b4), edi_var);
            callee_thiscall!(
                3,
                u32,
                arg1,
                s18b.wrapping_add(edi_var).wrapping_add(b4).wrapping_add(buf_ptr),
                arg2.wrapping_sub(edi_var)
            );
            *(obj.add(0x24) as *mut u32) = buf_ptr;
            // Rebase the kept entries by the buffer delta.
            let e28 = *(obj.add(0x28) as *const u32);
            let delta = ebp2.wrapping_sub(e28);
            let mut dx: u32 = 0;
            while dx < e1c {
                let cell = (buf_ptr as *mut u32).add(dx as usize);
                *cell = (*cell).wrapping_add(delta);
                dx += 1;
            }
            // Rebase the appended entries; the leftover addend register is
            // whatever the last store left, tracked for the check below.
            let mut ecx_l2 = s10;
            let mut dx2 = e1c;
            let end2 = e1c.wrapping_add(s10);
            while dx2 < end2 {
                ecx_l2 = *((buf_ptr as *const u32).add(dx2 as usize));
                ecx_l2 = ecx_l2.wrapping_add(ebp2).wrapping_add(s18b);
                *((buf_ptr as *mut u32).add(dx2 as usize)) = ecx_l2;
                dx2 += 1;
            }
            // Every path reloads the spilled word, which is ebp (see above),
            // so the stored base word is ebp unconditionally.
            let ebx_m = q;
            // Early-exit quirk: the original adds the leftover ecx to the
            // count and skips validation when the sum wraps to zero.
            let skip_validation = e1c.wrapping_add(ecx_l2) == 0;
            if !skip_validation {
                let total3 = e1c.wrapping_add(s10);
                let mut di: u32 = 0;
                while di < total3 {
                    let cell_addr = (buf_ptr as u32).wrapping_add(di.wrapping_mul(4));
                    let ok =
                        callee_thiscall!(7, u32, (obj as u32).wrapping_add(0x10), cell_addr);
                    if ok == 0 {
                        // The original passes its incoming-arg slot holding
                        // (di & 0xffff); the contract skips that address and
                        // snapshots the content, which is mirrored here.
                        let mut slot: u32 = di & 0xffff;
                        callee_thiscall!(
                            8,
                            u32,
                            (obj as u32).wrapping_add(0x10),
                            cell_addr,
                            (&mut slot as *mut u32) as u32
                        );
                    }
                    di += 1;
                }
                *(obj.add(0x1c) as *mut u32) = e1c.wrapping_add(s10);
            } else {
                *(obj.add(0x1c) as *mut u32) = e1c.wrapping_add(ecx_l2);
            }
            *(obj.add(0x20) as *mut u32) = e20.wrapping_add(arg2);
            *(obj.add(4) as *mut u32) = ebx_m;
            *(obj.add(0x28) as *mut u32) = ebp2;
            // eax still holds arg2; only the low byte is forced to 1.
            (arg2 & 0xffffff00) | 1
        } else {
            callee_thiscall!(3, u32, arg1, ebp, arg2);
            let ecx0 = e1c.wrapping_add(s10);
            *(obj.add(0x28) as *mut u32) = ecx0.wrapping_mul(4).wrapping_add(ebp);
            let mut bx = e1c;
            let mut loop_ran = false;
            while bx < ecx0 {
                loop_ran = true;
                let f = bx.wrapping_mul(4).wrapping_add(ebp);
                let av = *(obj.add(0x28) as *const u32);
                *((f as *mut u32)) = ((f as *const u32).read()).wrapping_add(av);
                let mut slot: u32 = bx & 0xffff;
                callee_thiscall!(
                    8,
                    u32,
                    (obj as u32).wrapping_add(0x10),
                    f,
                    (&mut slot as *mut u32) as u32
                );
                bx = bx.wrapping_add(1);
            }
            *(obj.add(0x20) as *mut u32) = arg2;
            *(obj.add(4) as *mut u32) = ebp;
            *(obj.add(0x24) as *mut u32) = ebp;
            *(obj.add(0x1c) as *mut u32) = s10;
            let eax_res = if loop_ran {
                e1c.wrapping_add(s10)
            } else {
                ecx0.wrapping_mul(4).wrapping_add(ebp)
            };
            (eax_res & 0xffffff00) | 1
        }
    }
});
