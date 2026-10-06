// original: 0x00ae02a0 input_ui_object_dispatch (proposed)

/// Dispatch an input record by its mode byte, then run the shared tail.
///
/// Thiscall: `obj` in ecx, two stack words `a0`/`a1`, the callee pops 8 bytes. Returns the
/// tail call's answer on every path. The record layout (from the object's
/// base): `+0x1a` table index (u16), `+0x24` key (unread here), `+0x25` mode,
/// `+0x26` spare, `+0x27` sub-index (0 selects the plain arm, nonzero adds
/// `sub << 4` to the base as a sub-object), `+0x29` bit 0x10 (extra pre- and
/// post-calls), `+0x2a` bit 8 (extra pre-call).
///
/// After a setup call on the object, a jump table on the mode runs: modes 0
/// and 1 share the long main path below (split again on `mode == 1`),
/// modes 2-5 each run one two-argument arm method on the object, and any
/// higher mode skips straight to the tail, which is a single no-argument
/// method on the object.
///
/// Main path: when bit 8 is set, a resolver runs on the object and the
/// returned object's slot-0x10 method runs. When bit 0x10 is set, an extra
/// no-argument call runs (its ecx is ambient stub residue on both sides and
/// is not compared). A data-table pointer is loaded from `TABLE[idx]`, then:
/// mode 1 reads that entry's `+8` object and, unless its byte at `+0xb4` is
/// zero, publishes `a1`/`a0` to the `OUT1`/`OUT0` globals, runs a filler
/// with a zeroed two-word frame buffer, then either a nine-argument report
/// (with the table pointer, the sub-object, the buffer and a `-10.0f`-headed
/// constants block) or a three-argument method on the `+8` object, and
/// finally resets the outputs to `0xff`/`0`. Any other mode (0 here) runs a
/// no-argument method on the table pointer; a null answer ends at the tail,
/// otherwise when `a1 & 3` is set, or a polled answer is nonzero, one pair
/// of virtual methods (slots 0x30/0x2c, with/without the sub-object) runs,
/// else the other pair (slots 0x28/0x20) runs. The tail repeats the bit-0x10
/// call (also ambient ecx) and the shared tail method.
///
/// Virtual calls reach the checker through their callee ids; the loaded
/// vtable pointers themselves are unobservable address arithmetic. All
/// compares are equality or bit tests; nothing here depends on signedness.
/// Frame buffers stay zero: the filler is stubbed without writes on both
/// sides, and every buffer's pre-call words are snapshotted.
lf_checker_rt::export!(thiscall, rw_00ae02a0(obj: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x1295cd8;
        const OUT0: u32 = 0x15dbdc0;
        const OUT1: u32 = 0x15dbdc4;
        const NEG_TEN: u32 = 0xc1200000;
        const CAL_SETUP: u32 = 1;
        const CAL_RESOLVE: u32 = 2;
        const CAL_VT10: u32 = 3;
        const CAL_PRE: u32 = 4;
        const CAL_FILL: u32 = 5;
        const CAL_REPORT: u32 = 6;
        const CAL_M3: u32 = 7;
        const CAL_GET: u32 = 8;
        const CAL_POLL: u32 = 9;
        const CAL_VT28: u32 = 10;
        const CAL_VT20: u32 = 11;
        const CAL_VT30: u32 = 12;
        const CAL_VT2C: u32 = 13;
        const CAL_POST: u32 = 14;
        const CAL_TAIL: u32 = 15;
        const CAL_ARM2: u32 = 16;
        const CAL_ARM3: u32 = 17;
        const CAL_ARM4: u32 = 18;
        const CAL_ARM5: u32 = 19;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        unsafe fn g32(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn set(file_va: u32, v: u32) {
            unsafe { wr32(lf_checker_rt::relocated(file_va), v) }
        }
        #[inline(always)]
        unsafe fn tail(obj: u32) -> u32 {
            unsafe { lf_checker_rt::callee_thiscall!(CAL_TAIL, u32, obj) }
        }
        #[inline(always)]
        unsafe fn arm(id: u32, obj: u32, a0: u32, a1: u32) -> u32 {
            unsafe {
                let _: u32 = lf_checker_rt::callee_thiscall!(id, u32, obj, a0, a1);
                tail(obj)
            }
        }
        /// Shared tail of the main path (extra call only under bit 0x10).
        #[inline(always)]
        unsafe fn main_tail(obj: u32) -> u32 {
            unsafe {
                if rd8(obj.wrapping_add(0x29)) & 0x10 != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(CAL_POST, u32, obj);
                }
                tail(obj)
            }
        }
        /// Second virtual pair (slots 0x30 with the sub-object, else 0x2c
        /// with a frame buffer).
        #[inline(always)]
        unsafe fn second_pair(h1: u32, edi: u32, a0: u32, obj: u32) {
            unsafe {
                if edi != 0 {
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_VT30, u32, h1, 0, edi, 0, 1, a0);
                } else {
                    let mut buf = [0u32; 2];
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_FILL, u32, obj, buf.as_mut_ptr() as u32
                    );
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_VT2C, u32, h1, buf.as_mut_ptr() as u32, 0, a0, 0
                    );
                }
            }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_SETUP, u32, obj);
        let mode = rd8(obj.wrapping_add(0x25));
        if mode > 5 {
            return tail(obj);
        }
        if mode >= 2 {
            let id = match mode {
                2 => CAL_ARM2,
                3 => CAL_ARM3,
                4 => CAL_ARM4,
                _ => CAL_ARM5,
            };
            return arm(id, obj, a0, a1);
        }
        // Main path (modes 0 and 1).
        if rd8(obj.wrapping_add(0x2a)) & 8 != 0 {
            let o: u32 = lf_checker_rt::callee_thiscall!(CAL_RESOLVE, u32, obj);
            let _vt = rd32(o);
            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_VT10, u32, o);
        }
        if rd8(obj.wrapping_add(0x29)) & 0x10 != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_PRE, u32, obj);
        }
        let sub = rd8(obj.wrapping_add(0x27));
        let edi = if sub != 0 {
            obj.wrapping_add((sub as u32) << 4)
        } else {
            0
        };
        let idx = rd16(obj.wrapping_add(0x1a)) as u32;
        let entry = rd32(lf_checker_rt::relocated(TABLE).wrapping_add(idx * 4));
        if mode == 1 {
            let t2 = rd32(entry.wrapping_add(8));
            if rd8(t2.wrapping_add(0xb4)) != 0 {
                set(OUT1, a1);
                set(OUT0, a0);
                let mut buf = [0u32; 2];
                let buf_ptr = buf.as_mut_ptr() as u32;
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_FILL, u32, obj, buf_ptr);
                if edi != 0 {
                    let consts = [NEG_TEN, entry, t2, 0u32];
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        CAL_REPORT, u32, entry, edi, buf_ptr, 0, 0, 0, 0,
                        consts.as_ptr() as u32, 0
                    );
                } else {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_M3, u32, t2, buf_ptr, 0, 0
                    );
                }
                set(OUT1, 0xff);
                set(OUT0, 0);
            }
            return main_tail(obj);
        }
        // Mode 0.
        let h1: u32 = lf_checker_rt::callee_thiscall!(CAL_GET, u32, entry);
        if h1 != 0 {
            if a1 & 3 != 0 {
                second_pair(h1, edi, a0, obj);
            } else {
                let q: u32 = lf_checker_rt::callee_thiscall!(CAL_POLL, u32, h1);
                if q != 0 {
                    second_pair(h1, edi, a0, obj);
                } else if edi != 0 {
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_VT28, u32, h1, 0, edi, a0, 0);
                } else {
                    let mut buf = [0u32; 2];
                    let buf_ptr = buf.as_mut_ptr() as u32;
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_FILL, u32, obj, buf_ptr);
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_VT20, u32, h1, buf_ptr, a0, 0, 0
                    );
                }
            }
        }
        main_tail(obj)
    }
});
