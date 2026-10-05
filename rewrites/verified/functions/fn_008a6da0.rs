// original: 0x008A6DA0 audio_gather_slot_buffers (proposed)

/// Gather per-slot voice buffers, then select and mix through callee 3.
///
/// `this` is the audio manager, `arg` the voice request. The thread's audio
/// state index `t` comes from the TLS slot named by the global `TLS_INDEX`,
/// field `TLS_THREAD_FIELD`. For each slot `i` (0..5) with a nonzero dword
/// at `this + (t + IDX_BASE + i*4)*4` and bit `i` of `arg[OBJ_FLAGS]` set,
/// the first `BLOCK` bytes of `arg` are copied into the slot's frame buffer
/// (memcpy runs natively on the original side and as an identical word copy
/// here), the taken count increments, the gate dword is kept in `array[i]`,
/// and callee 2 runs with (this, buffer, i). Skipped slots store 0.
/// Finally callee 3 (the weight-and-mix routine, verified separately) runs
/// with (this, arg, array, taken, first buffer). (thiscall, one stack word;
/// the return value is caller-ignored scratch.)
///
/// The original aligns its stack frame; the rewrite uses plain locals, so
/// frame-derived pointer arguments are skipped in the contract while their
/// contents are snapshotted: each taken buffer (post-copy) at callee 2 and
/// the five array words at callee 3.
lf_checker_rt::export!(thiscall, rw_008A6DA0(this: u32, arg: u32) -> u32 {
    unsafe {
        const TLS_INDEX_GLOBAL: u32 = 0x017ABA14;
        const TLS_THREAD_FIELD: u32 = 0x70;
        const IDX_BASE: u32 = 0x560;
        const BLOCK_WORDS: usize = 56;
        const N_SLOTS: u32 = 5;
        const OBJ_FLAGS: u32 = 0xD3;
        const FILL_CALLEE: u32 = 2;
        const MIX_CALLEE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let slot = rd32(lf_checker_rt::relocated(TLS_INDEX_GLOBAL));
        let thread = lf_checker_rt::tls_slot(slot as usize);
        let t = rd32(thread.wrapping_add(TLS_THREAD_FIELD));
        let mut bufs = [[0u32; BLOCK_WORDS]; 5];
        let mut array = [0u32; 5];
        let mut taken = 0u32;
        for i in 0..N_SLOTS {
            let idx = t.wrapping_add(IDX_BASE + i.wrapping_mul(4));
            let entry = rd32(this.wrapping_add(idx.wrapping_mul(4)));
            let flags = ((arg.wrapping_add(OBJ_FLAGS)) as *const u8).read();
            if entry == 0 || flags & (1u8 << i) == 0 {
                array[i as usize] = 0;
                continue;
            }
            let dst = bufs[i as usize].as_mut_ptr() as u32;
            let mut w = 0usize;
            while w < BLOCK_WORDS {
                wr32(dst.wrapping_add((w as u32).wrapping_mul(4)), rd32(arg.wrapping_add((w as u32).wrapping_mul(4))));
                w += 1;
            }
            taken = taken.wrapping_add(1);
            array[i as usize] = entry;
            lf_checker_rt::callee_thiscall!(FILL_CALLEE, u32, this, dst, i);
        }
        lf_checker_rt::callee_thiscall!(
            MIX_CALLEE,
            u32,
            this,
            arg,
            array.as_mut_ptr() as u32,
            taken,
            bufs[0].as_mut_ptr() as u32
        );
        0
    }
});
