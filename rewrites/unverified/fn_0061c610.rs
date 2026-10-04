// original: 0x0061C610 net_table_record_build (proposed, staged: entry)
// Stage 1 of 0x0061C010 (see below): guards, select call, record
// clearing, frame setup, both loop-skips and the empty tail. Loop bodies
// ( stages 2-3) are pinned to zero trips by the contract.
//
// original: 0x0061C610 net_table_record_build (proposed, staged: entry)
//
// cdecl(a0, a1, flag): guard on the select flag at `a1 + 0x68`, a null
// `a0`, and the slot count at `a0 + 0x4000` (exit unless below 0x40
// signed); resolve the record at `a0 + count * 0x100` through the select
// callee (exit unless its low byte is zero); clear the 256-byte record
// with two memset calls and set its flag byte at `+0xf8`; stage seven
// floats and a flag byte from `a0 + 0x5511..0x5538` into the frame. Past
// this point the original scans two data graphs (stages 2-3, unbuilt
// here); with both loop counts pinned to non-positive values it skips to
// the tail, which exits with 0 when no record was accepted. Returns 1 on
// the success path (stages 2-3) and the incoming EAX with its low byte
// cleared on early exits (see the exit table in the report).
//
// Original: 0x0061C610 (cdecl, three stack words; the flag word is read
// only on the success tail).
unsafe fn fn4_stage1(a0: u32, a1: u32, flag: u32, mode: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x4000;
        const COUNT_MAX: i32 = 0x40;
        const REC_SHIFT: u32 = 8;
        const REC_FLAG: u32 = 0xf8;
        const C_SELECT: u32 = 1;
        const C_MEMSET: u32 = 2;
        const C_COOKIE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        let exit = |eax: u32| -> u32 {
            let _: u32 = lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            eax & 0xffff_ff00
        };
        if rd8(a1 + 0x68) == 0 {
            return exit(a0);
        }
        if a0 == 0 {
            return exit(0);
        }
        let count = rd32(a0 + COUNT_OFF) as i32;
        if count >= COUNT_MAX {
            return exit(a0);
        }
        let rec = a0.wrapping_add((count as u32).wrapping_shl(REC_SHIFT));
        if rec == 0 {
            return exit(a0);
        }
        let mut scratch = core::mem::MaybeUninit::<[u8; 96]>::uninit();
        let sc = scratch.as_mut_ptr() as *mut u8;
        // Snapped fill word: unwritten stack (zero under the fill) on the
        // original side; written explicitly here (r-b184 precedent).
        (sc.add(0x40) as *mut u32).write_unaligned(0);
        let sel: u32 = lf_checker_rt::callee_thiscall!(C_SELECT, u32, a1, sc.add(0x40) as u32, a0);
        if sel & 0xff != 0 {
            return exit(sel);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(C_MEMSET, u32, rec, 0, 0x80);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_MEMSET, u32, rec.wrapping_add(0x80), 0, 0x80);
        if mode == 0 {
            (rec.wrapping_add(REC_FLAG) as *mut u8).write(1);
        }
        // Frame staging (kept as explicit reads plus dead stores so the
        // read order and fault behavior match the original exactly).
        let mut frame = [0u32; 8];
        frame[0] = rd32(a0 + 0x5528);
        frame[1] = rd32(a0 + 0x5534);
        frame[2] = rd32(a0 + 0x5538);
        frame[3] = rd32(a0 + 0x5520);
        frame[4] = rd32(a0 + 0x5524);
        frame[5] = rd8(a0 + 0x5511) as u32;
        frame[6] = rd32(a0 + 0x5530);
        core::hint::black_box(&frame);
        // Loop 1 skip: pinned non-positive by the contract (stage 2 builds
        // the body).
        let n1 = rd8(a1 + 0x26) as i8 as i32;
        if n1 > 0 {
            // Unreachable in stage 1: fault honestly rather than continue
            // into unbuilt behavior.
            core::hint::black_box(n1);
            return exit(0xeeee_ee00);
        }
        // Post-loop-1 global scaling (dead in stage 1: no accepted count).
        let g0 = rdf(lf_checker_rt::relocated(0x0110eb10));
        let g1 = rdf(lf_checker_rt::relocated(0x0110eb14));
        let g2 = rdf(lf_checker_rt::relocated(0x0110eb18));
        core::hint::black_box((g0, g1, g2));
        // Loop 2 skip (stage 3 builds the body).
        let n2 = rd8(a1 + 0x24) as i8 as i32;
        if n2 > 0 {
            core::hint::black_box(n2);
            return exit(0xeeee_ee00);
        }
        // Empty tail: accepted count is 0, exit with 0.
        let _ = flag;
        exit(0)
    }
}

lf_checker_rt::export!(cdecl, rw_0061C610(a0: u32, a1: u32, flag: u32) -> u32 {
    unsafe { fn4_stage1(a0, a1, flag, 0) }
});
