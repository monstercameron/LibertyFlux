// original: 0x00bf9500 task_handle_cascade (proposed)

/// Ensure the task handle, run one of two setup cascades, then dispatch on
/// two flag bits and stamp the handle with the current tick.
///
/// `this` carries a dword at `+0x08`, a dword at `+0x20`, a dword at `+0x24`
/// and a flag byte at `+0x28`; `a` selects the long cascade (with the flag)
/// and is passed on to two of its calls.
///
/// Behaviour: the manager object is asked for the handle for (`this.+0x08`,
/// 0, 0) while `this` is saved in a frame slot; a null handle returns 0. When
/// bit 1 of the flag byte is set and `a` is non-null, the long cascade runs:
/// a fetch call, a query call whose answer becomes a scratch value, a vector
/// call and a register call on the handle, a copy call, and two transform
/// calls sharing one scratch object; then the saved `this` is restored. The
/// short cascade instead runs two gather calls, a matrix call filling a
/// fifteen-word block, and an apply call on that block. The tail copies
/// sixteen bytes from one frame slot to another (matrix words on the short
/// path, never-stored zeros on the long path), selects one of two global
/// floats by bit 0 of the flag byte, and dispatches on bits 2-3: values 1
/// and 2 each run a clear call plus a five-argument dispatch (handle, copied
/// block pointer, selected float, 3.0, -1.0) to different callees, value 3
/// runs a sixteen-byte fill whose first eight bytes go to handle `+0x00`
/// while the next four go to handle `+0x08` with a trailing constant word,
/// and value 0 runs nothing. Then `this.+0x24` is stored at handle `+0x1ec`,
/// a post call runs, and the stamp slot (`+0x1d4`) gets the global tick,
/// bumped by one when the tick-check call's answer equals the global compare
/// word. The stamp is also the return value.
///
/// Original: 0x00bf9500 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00bf9500(this: u32, a: u32) -> u32 {
    unsafe {
        const TASK_MGR: u32 = 0x01394d60;
        const G_SEL0: u32 = 0x00fe8a94;
        const G_SEL1: u32 = 0x00fe8b20;
        const TICK_CMP: u32 = 0x011f702c;
        const TICK_SRC: u32 = 0x011f70c4;
        const TYPE_ID: u32 = 0x08;
        const THIS_W20: u32 = 0x20;
        const THIS_W24: u32 = 0x24;
        const THIS_FLAG: u32 = 0x28;
        const HANDLE_V3HI: u32 = 0x1ec;
        const HANDLE_STAMP: u32 = 0x1d4;
        const THREE_BITS: u32 = 0x40400000;
        const NEGONE_BITS: u32 = 0xbf800000;
        const MAKE_CONST: u32 = 0x00c19080;
        const TRAIL_CONST: u32 = 0x0062e7a0;
        const C_ACQUIRE: u32 = 1;
        const C_FETCH: u32 = 2;
        const C_QUERY: u32 = 3;
        const C_VEC: u32 = 4;
        const C_REG: u32 = 5;
        const C_COPY: u32 = 6;
        const C_X1: u32 = 7;
        const C_X2: u32 = 8;
        const C_G1: u32 = 9;
        const C_G2: u32 = 10;
        const C_MAT: u32 = 11;
        const C_APPLY: u32 = 12;
        const C_CLEAR: u32 = 13;
        const C_D1: u32 = 14;
        const C_D2: u32 = 15;
        const C_MAKE: u32 = 16;
        const C_POST: u32 = 17;
        const C_TICK2: u32 = 18;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let type_id = rd32(this.wrapping_add(TYPE_ID));
        let handle = lf_checker_rt::callee_thiscall!(
            C_ACQUIRE,
            u32,
            lf_checker_rt::relocated(TASK_MGR),
            type_id,
            0,
            0
        );
        if handle == 0 {
            return 0;
        }
        let flag = (rd32(this.wrapping_add(THIS_FLAG)) & 0xff) as u8;
        // Sixteen-byte slot shared by the tail copy: matrix words on the
        // short path, never-stored zeros on the long path.
        let mut slot60 = [0u32; 4];
        if flag & 2 != 0 && a != 0 {
            let mut cell80 = [0u32; 1];
            lf_checker_rt::callee_thiscall!(C_FETCH, u32, this, cell80.as_mut_ptr() as u32);
            let mut cell_b: u32 = 1;
            let q = lf_checker_rt::callee_cdecl!(
                C_QUERY,
                u32,
                a,
                rd32(this.wrapping_add(THIS_W20)),
                1,
                (&mut cell_b as *mut u32) as u32
            );
            lf_checker_rt::callee_thiscall!(C_VEC, u32, handle, q);
            lf_checker_rt::callee_thiscall!(
                C_REG,
                u32,
                lf_checker_rt::relocated(TASK_MGR),
                handle,
                a,
                0
            );
            lf_checker_rt::callee_thiscall!(C_COPY, u32, handle, cell80.as_mut_ptr() as u32);
            let mut obj30 = [0u32; 1];
            lf_checker_rt::callee_thiscall!(
                C_X1,
                u32,
                obj30.as_mut_ptr() as u32,
                cell80.as_mut_ptr() as u32
            );
            lf_checker_rt::callee_thiscall!(C_X2, u32, obj30.as_mut_ptr() as u32, q);
        } else {
            let mut cell20 = [0u32; 3];
            lf_checker_rt::callee_thiscall!(C_G1, u32, this, cell20.as_mut_ptr() as u32);
            let mut cell70 = [0u32; 3];
            lf_checker_rt::callee_thiscall!(C_G2, u32, this, cell70.as_mut_ptr() as u32);
            let mut cell30 = [0u32; 15];
            lf_checker_rt::callee_cdecl!(
                C_MAT,
                u32,
                cell30.as_mut_ptr() as u32,
                cell70.as_mut_ptr() as u32,
                cell20.as_mut_ptr() as u32,
                0
            );
            lf_checker_rt::callee_thiscall!(C_APPLY, u32, handle, cell30.as_mut_ptr() as u32);
            slot60[0] = cell30[12];
            slot60[1] = cell30[13];
            slot60[2] = cell30[14];
            // slot60[3] is never stored (contract stack_fill 0).
        }
        let mut cell10 = slot60;
        let sel = if flag & 1 != 0 {
            rd32(lf_checker_rt::relocated(G_SEL1))
        } else {
            rd32(lf_checker_rt::relocated(G_SEL0))
        };
        match (flag.wrapping_shr(2)) & 3 {
            1 => {
                lf_checker_rt::callee_thiscall!(C_CLEAR, u32, handle);
                lf_checker_rt::callee_cdecl!(
                    C_D1,
                    u32,
                    handle,
                    cell10.as_mut_ptr() as u32,
                    sel,
                    THREE_BITS,
                    NEGONE_BITS
                );
            }
            2 => {
                lf_checker_rt::callee_thiscall!(C_CLEAR, u32, handle);
                lf_checker_rt::callee_cdecl!(
                    C_D2,
                    u32,
                    handle,
                    cell10.as_mut_ptr() as u32,
                    sel,
                    THREE_BITS,
                    NEGONE_BITS
                );
            }
            3 => {
                let mut cell20c = [0u32; 4];
                lf_checker_rt::callee_thiscall!(
                    C_MAKE,
                    u32,
                    cell20c.as_mut_ptr() as u32,
                    0,
                    MAKE_CONST,
                    0,
                    0
                );
                wr32(handle, cell20c[0]);
                wr32(handle.wrapping_add(4), cell20c[1]);
                wr32(handle.wrapping_add(8), cell20c[2]);
                wr32(handle.wrapping_add(12), TRAIL_CONST);
            }
            _ => {}
        }
        wr32(
            handle.wrapping_add(HANDLE_V3HI),
            rd32(this.wrapping_add(THIS_W24)),
        );
        lf_checker_rt::callee_thiscall!(C_POST, u32, handle);
        let tick_answer = lf_checker_rt::callee_cdecl!(C_TICK2, u32,);
        let cmp = rd32(lf_checker_rt::relocated(TICK_CMP));
        let tick = rd32(lf_checker_rt::relocated(TICK_SRC));
        let stamped = if cmp != tick_answer {
            tick
        } else {
            tick.wrapping_add(1)
        };
        wr32(handle.wrapping_add(HANDLE_STAMP), stamped);
        stamped
    }
});
