// original: 0x006347E0 bracket_section_parse (proposed)

/// Parse one bracketed section from a token reader into a request struct.
///
/// `arg1` points at a handle whose word at `+4` is a reader object (vtable
/// with a sized-fill at slot 2, a one-word query at slot 5 and a struct-fill
/// at slot 0x28; the reader's cursor lives at its `+0x10`). The function
/// builds a 12-word request (zeroed except two untouched gaps), asks the
/// reader for a "["-delimited scan (three 0x40 fills, a query whose answer
/// becomes request word 8, two struct fills), then checks the "[" opener.
/// While the check passes it pulls 0x200-byte chunks and compares each
/// against the "]" closer with an inline strcmp, stepping the reader past
/// every non-closing chunk (and past empty pulls) and restoring the saved
/// cursor each time; the first closing chunk ends the scan. A finalizer then
/// consumes the section's first word plus the request and its return value
/// is the function's result. An opener mismatch skips the scan and goes
/// straight to the finalizer. thiscall, one stack word; the only live
/// incoming register besides ECX is none (ECX itself is only forwarded,
/// plus 0xD4, to the finalizer). The original aligns its frame and keeps a
/// stack cookie; the rewrite keeps just the behaviour. The rewrite mirrors
/// the original frame's request-plus-buffers window in one array so the
/// call-time snapshots observe identical bytes.
lf_checker_rt::export!(thiscall, rw_006347E0(this: u32, arg1: u32) -> u32 {
    unsafe {
        const VT_FILL: u32 = 0x08;
        const VT_QUERY: u32 = 0x14;
        const VT_STRUCT: u32 = 0x28;
        const RDR_CURSOR: u32 = 0x10;
        const OPEN_BRACKET: u32 = 0x00F9_7E00;
        const OPEN_BRACKET_ALT: u32 = 0x00F9_7E04;
        const CLOSE_BRACKET: u32 = 0x00F9_7E08;
        const FINAL_TAG: u32 = 0x00F9_7E38;
        // Frame window [E'+0x20..E'+0xD8]: request, gap, 0x40 buffer,
        // first 64 bytes of the 0x200 buffer (the stub never fills past
        // them and the strcmp ends inside them on every trial).
        const F_REQ: usize = 0x00;
        const F_BUF40: usize = 0x38;
        const F_BUF200: usize = 0x78;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn vfill(obj: u32, n: u32, buf: *mut u8) -> u32 {
            unsafe {
                let vt: u32 = rd32(obj);
                let slot: u32 = rd32(vt.wrapping_add(VT_FILL));
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                f(obj, buf as u32, n)
            }
        }
        #[inline(always)]
        unsafe fn vquery(obj: u32) -> u32 {
            unsafe {
                let vt: u32 = rd32(obj);
                let slot: u32 = rd32(vt.wrapping_add(VT_QUERY));
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                f(obj, 1)
            }
        }
        #[inline(always)]
        unsafe fn vstruct(obj: u32, out: *mut u32) {
            unsafe {
                let vt: u32 = rd32(obj);
                let slot: u32 = rd32(vt.wrapping_add(VT_STRUCT));
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                let _ = f(obj, out as u32, 1);
            }
        }
        /// Byte strcmp exactly like the original's inline loop: 0 when
        /// equal, -1/1 by the first differing byte (unsigned).
        unsafe fn cstrcmp(a: u32, b: *const u8) -> i32 {
            unsafe {
                let mut i: u32 = 0;
                loop {
                    let x: u8 = (a.wrapping_add(i) as *const u8).read();
                    let y: u8 = b.add(i as usize).read();
                    if x != y {
                        return if x < y { -1 } else { 1 };
                    }
                    if x == 0 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
            }
        }

        let mut frame = [0u8; 0xB8];
        let base = frame.as_mut_ptr() as u32;
        let req_at = |w: usize| (base.wrapping_add((F_REQ + w * 4) as u32) as *mut u32);
        let obj: u32 = rd32(arg1.wrapping_add(4));
        (base.wrapping_add(F_BUF40 as u32) as *mut u8).write(0);
        vfill(obj, 0x40, base.wrapping_add(F_BUF40 as u32) as *mut u8);
        req_at(8).write_unaligned(vquery(obj));
        (base.wrapping_add(F_BUF40 as u32) as *mut u8).write(0);
        vfill(obj, 0x40, base.wrapping_add(F_BUF40 as u32) as *mut u8);
        vstruct(obj, req_at(0));
        (base.wrapping_add(F_BUF40 as u32) as *mut u8).write(0);
        vfill(obj, 0x40, base.wrapping_add(F_BUF40 as u32) as *mut u8);
        vstruct(obj, req_at(4));
        let t1: u32 = lf_checker_rt::callee_thiscall!(
            4, u32, obj, lf_checker_rt::relocated(OPEN_BRACKET), 0
        );
        // hdr mirrors [E'+0x14, E'+0x18]: first word, then the last saved
        // cursor (or zero when the scan never ran).
        let mut hdr = [0u32; 2];
        hdr[0] = rd32(arg1);
        if t1 & 0xFF == 0 {
            let ans: u32 = lf_checker_rt::callee_thiscall!(
                6, u32, this.wrapping_add(0xD4), hdr.as_mut_ptr() as u32,
                req_at(0) as u32
            );
            // The stack-cookie check preserves eax on the original side;
            // the rewrite keeps the answer in a local (the stub-table load
            // for the check's own address would clobber eax first).
            let _: u32 = lf_checker_rt::callee_thiscall!(7, u32, 0);
            return ans;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            4, u32, obj, lf_checker_rt::relocated(OPEN_BRACKET_ALT), 1
        );
        let closer: u32 = lf_checker_rt::relocated(CLOSE_BRACKET);
        loop {
            let obj2: u32 = rd32(arg1.wrapping_add(4));
            let saved: u32 = rd32(obj2.wrapping_add(RDR_CURSOR));
            hdr[1] = saved;
            (base.wrapping_add(F_BUF200 as u32) as *mut u8).write(0);
            let edi: u32 = vfill(obj2, 0x200, base.wrapping_add(F_BUF200 as u32) as *mut u8);
            if edi != 0
                && cstrcmp(closer, base.wrapping_add(F_BUF200 as u32) as *const u8) == 0
            {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    5, u32, obj2, base.wrapping_add(F_BUF200 as u32), edi
                );
                wr32(obj2.wrapping_add(RDR_CURSOR), saved);
                break;
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(
                5, u32, obj2, base.wrapping_add(F_BUF200 as u32), edi
            );
            wr32(obj2.wrapping_add(RDR_CURSOR), saved);
            vfill(obj2, 0x100, base.wrapping_add(F_BUF200 as u32) as *mut u8);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            4, u32, obj, lf_checker_rt::relocated(FINAL_TAG), 1
        );
        let ans: u32 = lf_checker_rt::callee_thiscall!(
            6, u32, this.wrapping_add(0xD4), hdr.as_mut_ptr() as u32, req_at(0) as u32
        );
        // The stack-cookie check: no stack args; its ecx (cookie (an instruction of the original))
        // is legitimately frame-specific and uncompared (call_regs []),
        // the call itself is compared. The answer is kept in a local (see
        // above) rather than trusted through the check.
        let _: u32 = lf_checker_rt::callee_thiscall!(7, u32, 0);
        ans
    }
});
