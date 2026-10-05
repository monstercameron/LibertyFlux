// original: 0x00dd1d80 input_list_row_fetch_insert (proposed)

/// Fetch (or build) one row object for a UI list and insert it into the
/// list's row array, returning the row.
///
/// `this` is the list controller. `a2` is a tag that must equal the answer
/// of the tag helper (callee 1) for `TAG`, else the function returns 0
/// immediately. `a1` is handed to the row factory (callee 5) as its first
/// word. `a3`'s low byte selects the tail: zero takes the main path,
/// non-zero the alternate tail (callee 7 instead of callee 6); the whole
/// of `a3` is later reported to callee 14.
///
/// Main path: allocate a `ROW_SIZE`-byte block (callee 3; a null answer
/// skips the factory and faults on the first use of the missing row,
/// symmetrically on both sides), read two cached values through the list
/// object (slot `+0x48` twice), format a request (callee 4, cdecl/3: the
/// first pushed value stays on the stack as a third factory word below),
/// then resolve the row through the row factory (callee 5, thiscall/3
/// with the block, `a1`, the format answer and the first cached value).
/// Three rounds of virtual calls refresh values from the controller
/// (`+0x4c`) into the row (`+0x170/+0x17c/+0x188`), then three
/// single-argument row updates (`+0x28` with 1, `+0x44` with 1, `+0x1f0`
/// with 3). The controller's sequence counter at `+0x200` is incremented.
///
/// If the stored row index at `+0x204` is negative it is recomputed: a
/// lookup object (controller `+0x1f4`, slot `+0xf8`) yields a candidate
/// whose word at `+8` selects one of two refresh sequences, both ending
/// in the index provider (list object, slot `+0x1e4`, fed the refresh
/// answer); a `-1` answer is stored as 0, otherwise the answer (plus one
/// on the second path).
///
/// The array holder (list object `+0x1e0`, slot `+0x1d0`, three calls)
/// gives an array struct (base pointer at `+0`, 16-bit count at `+4`):
/// the count is decremented and restored around a one-slot upward shift
/// of the elements above the stored index, the count is incremented, and
/// the row is stored at the index. The row is then announced (slot
/// `+0x23c`) and reported to one of two sinks (callee 6 with four words,
/// or callee 7 with three).
///
/// After the tails rejoin: a float is read from the sink answer `+0x44`
/// and passed by value to the row (`+0x224`); a float answer (callee 8,
/// x87 return) is passed by value to the controller (`+0x54`); a registry
/// object is fetched (callee 12, fed the `+0x54` answer) and released
/// (callee 13). A still-negative index takes the provider at slot `+0x1d4`
/// (decremented) before the three-word report (callee 14: row, index word,
/// `a3`); the list object is asked through its computed slot `+0x1c` (a
/// `(an instruction of the original)` site, answered 0 or not) and, on zero, through slot `+0x1ac`;
/// a finaliser runs (callee 15); two float constants (`F_A`, `F_B`) go to
/// the row (`+0x80`); and a last lookup (callee 9) and release (callee 10)
/// run. Returns the row pointer.
///
/// Layout read: controller words at `+0x1e0/+0x1ec/+0x1f4/+0x1f8` (child
/// objects), `+0x200` (sequence) and `+0x204` (index); row words at
/// `+0x310/+0x314` (report pair). All virtual targets are fabrication
/// addresses planted in those objects, so both sides call the same
/// scripted stubs in the same order. No floating-point arithmetic is
/// done here; floats are only moved and passed by value, hence bit-exact.
///
/// Stack discipline (verified against the ending the callee pops 0xc bytes): every
/// single-push virtual call pops its word, the factory pops three, the
/// report call pops three, and the `+0x80` call's eight reserved bytes
/// are popped by that callee.
///
/// Original: 0x00DD1D80 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00dd1d80(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x00EF_AFCC;
        const AUX_THIS: u32 = 0x0117_37D0;
        const ROW_SIZE: u32 = 0x31c;
        const ROW_ARG: u32 = 0x00EF_AFD8;
        const REG_THIS: u32 = 0x0198_1A4C;
        const F_A: f32 = f32::from_bits(0x4378_0000);
        const F_B: f32 = f32::from_bits(0x431B_0000);

        const C_TAG: u32 = 1;
        const C_AUX: u32 = 2;
        const C_ALLOC: u32 = 3;
        const C_FMT: u32 = 4;
        const C_FACTORY: u32 = 5;
        const C_SINK4: u32 = 6;
        const C_SINK3: u32 = 7;
        const C_FVAL: u32 = 8;
        const C_LOOKUP: u32 = 9;
        const C_RELEASE: u32 = 10;
        const C_LIST: u32 = 11;
        const C_REG: u32 = 12;
        const C_REGREL: u32 = 13;
        const C_REPORT: u32 = 14;
        const C_FIN: u32 = 15;

        const S_GET48: u32 = 0x48;
        const S_SELF4C: u32 = 0x4c;
        const S_SETF: u32 = 0x54;
        const S_ROW80: u32 = 0x80;
        const S_ROW28: u32 = 0x28;
        const S_ROW44: u32 = 0x44;
        const S_ROW224: u32 = 0x224;
        const S_ROW23C: u32 = 0x23c;
        const S_ROW170: u32 = 0x170;
        const S_ROW17C: u32 = 0x17c;
        const S_ROW188: u32 = 0x188;
        const S_ROW1F0: u32 = 0x1f0;
        const S_LOOKUPF8: u32 = 0xf8;
        const S_LIST1AC: u32 = 0x1ac;
        const S_LIST1C: u32 = 0x1c;
        const S_LIST1D4: u32 = 0x1d4;
        const S_LIST1E4: u32 = 0x1e4;
        const S_ARR1D0: u32 = 0x1d0;
        const S_CAND4C: u32 = 0x4c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        /// Call through a fabricated object exactly like the original's
        /// `(an instruction of the original)`: load the vtable, load the slot, call it.
        #[inline(always)]
        unsafe fn icall0(obj: u32, off: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + off) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn icall1(obj: u32, off: u32, x: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + off) as usize);
                f(obj, x)
            }
        }
        #[inline(always)]
        unsafe fn icall2(obj: u32, off: u32, x: u32, y: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + off) as usize);
                f(obj, x, y)
            }
        }

        if lf_checker_rt::callee_cdecl!(C_TAG, u32, lf_checker_rt::relocated(TAG)) != a2 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(C_AUX, u32, lf_checker_rt::relocated(AUX_THIS), 0);
        let block = lf_checker_rt::callee_cdecl!(C_ALLOC, u32, ROW_SIZE);
        let list = rd32(this + 0x1e0);
        let row = if block != 0 {
            let g1 = icall0(list, S_GET48);
            let g2 = icall0(list, S_GET48);
            let seq = rd32(this + 0x200);
            let fmt = lf_checker_rt::callee_cdecl!(
                C_FMT, u32, lf_checker_rt::relocated(ROW_ARG), g2, seq);
            lf_checker_rt::callee_thiscall!(C_FACTORY, u32, block, a1, fmt, g1)
        } else {
            // The original skips the factory and faults on the first use
            // of the missing row; fault identically (volatile: the read
            // must survive optimisation).
            core::ptr::read_volatile(0 as *const u32)
        };
        // Touch the row exactly where the original does (before any
        // further call).
        core::ptr::read_volatile(row as *const u32);
        let t = icall0(this, S_SELF4C);
        icall1(row, S_ROW170, t);
        let t = icall0(this, S_SELF4C);
        icall1(row, S_ROW17C, t);
        let t = icall0(this, S_SELF4C);
        icall1(row, S_ROW188, t);
        icall1(row, S_ROW28, 1);
        icall1(row, S_ROW44, 1);
        icall1(row, S_ROW1F0, 3);
        wr32(this + 0x200, rd32(this + 0x200).wrapping_add(1));
        let sink = if (a3 & 0xff) == 0 {
            if (rd32(this + 0x204) as i32) < 0 {
                let lookup = rd32(this + 0x1f4);
                let cand = icall0(lookup, S_LOOKUPF8);
                let cand_obj = rd32(rd32(cand).wrapping_add(8));
                let z = lf_checker_rt::callee_thiscall!(C_LIST, u32, cand_obj);
                if rd32(cand_obj) == rd32(cand_obj + 0xc) {
                    let r = icall0(z, S_CAND4C);
                    let idx = icall1(list, S_LIST1E4, r);
                    if idx == 0xffff_ffff {
                        wr32(this + 0x204, 0);
                    } else {
                        wr32(this + 0x204, idx);
                    }
                } else {
                    let r = icall0(z, S_CAND4C);
                    let idx = icall1(list, S_LIST1E4, r);
                    wr32(this + 0x204, idx.wrapping_add(1));
                }
            }
            let b = rd32(rd32(this + 0x1e0) + 0x1e0);
            let arr = icall0(b, S_ARR1D0);
            wr16(arr + 4, rd16(arr + 4).wrapping_add(0xffff));
            let b = rd32(rd32(this + 0x1e0) + 0x1e0);
            let arr = icall0(b, S_ARR1D0);
            let idx = rd32(this + 0x204) as i32;
            let mut pos = rd16(arr + 4) as i32;
            let base = rd32(arr);
            while pos > idx {
                pos -= 1;
                wr32(
                    base.wrapping_add((pos as u32).wrapping_mul(4)).wrapping_add(4),
                    rd32(base.wrapping_add((pos as u32).wrapping_mul(4))),
                );
            }
            wr16(arr + 4, rd16(arr + 4).wrapping_add(1));
            let b = rd32(rd32(this + 0x1e0) + 0x1e0);
            let arr = icall0(b, S_ARR1D0);
            let at = rd32(this + 0x204);
            wr32(rd32(arr).wrapping_add(at.wrapping_mul(4)), row);
            let w = icall0(row, S_ROW23C);
            lf_checker_rt::callee_thiscall!(
                C_SINK4, u32, rd32(this + 0x1f8), w,
                rd32(this + 0x204), rd32(row + 0x310), rd32(row + 0x314))
        } else {
            let w = icall0(row, S_ROW23C);
            lf_checker_rt::callee_thiscall!(
                C_SINK3, u32, rd32(this + 0x1f8), w,
                rd32(row + 0x310), rd32(row + 0x314))
        };
        icall1(row, S_ROW224, rd32(sink + 0x44));
        let inlet: f32 = lf_checker_rt::callee_thiscall!(C_FVAL, f32, rd32(this + 0x1f8));
        let answer = icall1(this, S_SETF, inlet.to_bits());
        let h = lf_checker_rt::callee_thiscall!(
            C_REG, u32, lf_checker_rt::relocated(REG_THIS), answer);
        lf_checker_rt::callee_thiscall!(C_REGREL, u32, h);
        let cur = rd32(this + 0x204);
        let word = if (cur as i32) < 0 {
            icall0(list, S_LIST1D4).wrapping_sub(1)
        } else {
            cur
        };
        lf_checker_rt::callee_thiscall!(C_REPORT, u32, rd32(this + 0x1ec), row, word, a3);
        let al = icall0(list, S_LIST1C);
        if (al & 0xff) == 0 {
            icall0(list, S_LIST1AC);
        }
        lf_checker_rt::callee_thiscall!(C_FIN, u32, list);
        icall2(row, S_ROW80, F_A.to_bits(), F_B.to_bits());
        let found = lf_checker_rt::callee_stdcall!(C_LOOKUP, u32, rd32(this + 0x1f8));
        lf_checker_rt::callee_thiscall!(C_RELEASE, u32, found);
        row
    }
});
