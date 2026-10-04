// original: 0x0061C010 net_obj_sync_apply (proposed, staged: entry path)
// Staged entry-path probe for 0x0061C010 (see below). The full function
// runs ~30 calls past the validator; only the entry path through the
// validator's early exit is built here, which is enough to confirm the
// 16-word-buffer/8-word-snap gap on this function.
//
// original: 0x0061C010 net_obj_sync_apply (proposed, staged: entry path)
//
// fastcall(obj, src, a0): copy twelve floats out of the 64-byte record at
// `a0` into a frame buffer (four groups of three, skipping the pad word
// of each 16-byte group) and run the buffer through the copy and
// validation helpers. On validator rejection the function returns the
// validator's full EAX unchanged (this probe pins rejection, so only that
// path is covered). Past validation the original continues into a large
// body (~28 further calls, x87 float returns, table dispatches) that this
// probe does not build; the buffer gap below blocks full verification
// regardless.
//
// Original: 0x0061C010 (fastcall argument arrival, ECX, EDX, one stack
// word, plain ret: caller cleanup, so the contract leaves esp off).
unsafe fn fn3_entry(a0: u32, mode: u32) -> u32 {
    unsafe {
        const C_COPY: u32 = 1;
        const C_VALID: u32 = 2;

        // Zeroed: the original leaves the buffer pads and the scratch word
        // as unwritten stack (zero under the contract's fill); the rewrite
        // writes the zero explicitly (r-b184 precedent).
        let mut buf = [0u32; 16];
        let b = buf.as_mut_ptr() as *mut u32;
        let mut gi = 0usize;
        while gi < 4 {
            let s = (a0 + gi as u32 * 16) as *const u32;
            let d = b.add(gi * 4);
            d.write_unaligned(s.read_unaligned());
            d.add(1).write_unaligned(s.add(1).read_unaligned());
            d.add(2).write_unaligned(if mode == 1 && gi == 3 {
                0
            } else {
                s.add(2).read_unaligned()
            });
            gi += 1;
        }
        let mut scratch = core::mem::MaybeUninit::<[u8; 96]>::uninit();
        let sc = scratch.as_mut_ptr() as *mut u8;
        (sc.add(0x40) as *mut u32).write_unaligned(0);
        let _: u32 = lf_checker_rt::callee_cdecl!(C_COPY, u32, sc.add(0x40) as u32, a0);
        let valid: u32 = lf_checker_rt::callee_thiscall!(C_VALID, u32, sc as u32, b as u32);
        valid
    }
}

lf_checker_rt::export!(fastcall, rw_0061C010(obj: u32, src: u32, a0: u32) -> u32 {
    unsafe {
        let _ = (obj, src);
        fn3_entry(a0, 0)
    }
});

lf_checker_rt::export!(fastcall, mut_0061C010(obj: u32, src: u32, a0: u32) -> u32 {
    unsafe {
        let _ = (obj, src);
        fn3_entry(a0, 1)
    }
});
