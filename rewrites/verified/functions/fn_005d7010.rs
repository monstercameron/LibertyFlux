// original: 0x005D7010 HTMD10 (merged name)

/// Formats a diagnostic record or allocates a fallback object, then
/// rotates the allocator cursors.
///
/// `this` is the owner; the three stack arguments are a source word,
/// a tag word and a flags word (callee pops 12). Session state
/// lives in TLS slot 0 (`tls0`): a counter at `[tls0+0x68]` is
/// decremented, or, when zero, the spare allocator at `[tls0+0x64]` is
/// installed at `[tls0+8]` and the spare slot cleared.
///
/// A non-zero low byte of the flags word runs callee 3 (cdecl: table
/// constant, 0, 0; answer ignored). Callee 4 (cdecl: link global, 0,
/// two table constants, 0) then answers null or a record: null takes
/// the fallback path below, otherwise the record's word at `+0xDC`
/// points at a key and callee 5 (cdecl: key, table constant) is probed.
/// A zero probe answer formats: callee 6 (cdecl: scratch buffer,
/// `0x200`, format word, source, tag, 0, 0; first-meg, patched) fills
/// a 128-word frame buffer and callee 7 (thiscall: `this+0x20`, buffer)
/// consumes it; only zero/non-zero of the probe answer matters
/// (`test`/`je`). A non-zero probe answer, like a null record, takes
/// the fallback: the allocator at `[tls0+8]` is called as `tls[0] ->
/// [+8] -> vtable[+8]` (thiscall: allocator, `0xE0, 0x10, 0`; the twin
/// stub answers on spare-installed trials), a null answer clears the
/// tag while callee 8 (thiscall: fresh object, source, tag) otherwise
/// replaces it, and the allocator object kept live in ECX across those
/// calls (the stubs preserve it, as the caller requires) is passed with
/// the link slot to callee 9 (thiscall: link slot + 12, last live ECX),
/// whose answer points at the word receiving the tag; the tag links
/// back at `+8` (a null tag faults on both sides here).
///
/// Finally `[tls0+8]` and `[tls0+0x10]` are compared for equality: equal
/// increments the counter, otherwise the first word rotates into the
/// spare slot and the second becomes current. Returns the current
/// allocator word `[tls0+8]`. Only the low byte of the flags word is
/// read. The trailing security-cookie check runs natively in the
/// original and is not intercepted: it always passes and writes
/// nothing.
///
/// Original: 0x005D7010 (thiscall, three stack words, callee pops 12).
lf_checker_rt::export!(thiscall, rw_005D7010(this: u32, esi_in: u32, ebx_in: u32, flag_in: u32) -> u32 {
    unsafe {
        const TLS_SLOT: usize = 0;
        const HEAPOBJ_OFF: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const LINK_VA: u32 = 0x018B6FAC;
        const C_TBL_VA: u32 = 0x00F907B4;
        const D_T0_VA: u32 = 0x0114E780;
        const D_T1_VA: u32 = 0x0114E798;
        const DF_TBL_VA: u32 = 0x00F907C8;
        const FMT_STR_VA: u32 = 0x00F1C728;
        const C: u32 = 3;
        const D: u32 = 4;
        const DF: u32 = 5;
        const FMT: u32 = 6;
        const USE: u32 = 7;
        const F: u32 = 8;
        const LINK: u32 = 9;
        const FMT_WORDS: u32 = 128;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let tls0 = lf_checker_rt::tls_slot(TLS_SLOT);
        let rc = rd32(tls0.wrapping_add(0x68));
        if rc != 0 {
            wr32(tls0.wrapping_add(0x68), rc.wrapping_sub(1));
        } else {
            let spare = rd32(tls0.wrapping_add(0x64));
            wr32(tls0.wrapping_add(8), spare);
            wr32(tls0.wrapping_add(0x64), 0);
        }
        if (flag_in as u8) != 0 {
            let _ = lf_checker_rt::callee_cdecl!(C, u32, lf_checker_rt::relocated(C_TBL_VA), 0, 0);
        }
        let link = rd32(lf_checker_rt::relocated(LINK_VA));
        let d = lf_checker_rt::callee_cdecl!(
            D,
            u32,
            link,
            0,
            lf_checker_rt::relocated(D_T0_VA),
            lf_checker_rt::relocated(D_T1_VA),
            0
        );
        let heap_now = rd32(tls0.wrapping_add(8));
        let mut fmt_done = false;
        if d != 0 {
            let fobj = rd32(d.wrapping_add(0xDC));
            let key = rd32(fobj);
            let r = lf_checker_rt::callee_cdecl!(DF, u32, key, lf_checker_rt::relocated(DF_TBL_VA));
            if r == 0 {
                let mut buf = [0u32; FMT_WORDS as usize];
                let bp = buf.as_mut_ptr() as u32;
                let _ = lf_checker_rt::callee_cdecl!(
                    FMT,
                    u32,
                    bp,
                    0x200,
                    lf_checker_rt::relocated(FMT_STR_VA),
                    esi_in,
                    ebx_in,
                    0,
                    0
                );
                let _ = lf_checker_rt::callee_thiscall!(USE, u32, this.wrapping_add(0x20), bp);
                fmt_done = true;
            }
        }
        if !fmt_done {
            let vt = rd32(heap_now);
            let site: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vt.wrapping_add(ALLOC_SLOT)) as usize) };
            let r = site(heap_now, 0xE0, 0x10, 0);
            let mut ebx = ebx_in;
            let last_ecx: u32;
            if r != 0 {
                let fr = lf_checker_rt::callee_thiscall!(F, u32, r, esi_in, ebx_in);
                ebx = fr;
                last_ecx = r;
            } else {
                ebx = 0;
                last_ecx = heap_now;
            }
            let esi2 = rd32(lf_checker_rt::relocated(LINK_VA));
            let lr = lf_checker_rt::callee_thiscall!(LINK, u32, esi2.wrapping_add(0xC), last_ecx);
            wr32(lr, ebx);
            wr32(ebx.wrapping_add(8), esi2);
        }
        let ea = rd32(tls0.wrapping_add(8));
        let ec = rd32(tls0.wrapping_add(0x10));
        if ea == ec {
            let c = rd32(tls0.wrapping_add(0x68));
            wr32(tls0.wrapping_add(0x68), c.wrapping_add(1));
        } else {
            wr32(tls0.wrapping_add(0x64), ea);
            wr32(tls0.wrapping_add(8), ec);
        }
        ea
    }
});
