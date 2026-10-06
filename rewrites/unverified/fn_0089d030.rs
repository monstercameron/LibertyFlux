// original: 0x0089d030 audio_rebase_driver (proposed)

/// Drives the key-rebase passes and publishes the rebased bases to globals.
///
/// Seeds global `G_A` with `G_A+a6`, allocates the block (callee 1,
/// thiscall/2 on the pool with (`G_A+a6`, 16)), then chains: the scan
/// (callee 2, cdecl/3) on (`a0`, `a1`, block), the index pass (callee 3,
/// cdecl/5) on (block, scan, `a0`, `a1`, block+10*scan), and the two rebase
/// passes (callees 4 and 5, cdecl/5) on (block, scan, `a2`, `a3`,
/// block+10*scan+14*index) and (block, scan, `a4`, `a5`,
/// `SZ1+a3+block+10*scan+14*index`). Finally it publishes the derived bases
/// to the globals `G_A`..`G_G` and runs the commit (callee 6, thiscall/1 on
/// the pool with old `G824`), returning the commit's answer. All address
/// arithmetic wraps mod 2^32. Incoming `a6`'s stack slot is reused to hold
/// the block across the calls, so the stack check is off.
///
/// Original: 0x0089d030 (cdecl, seven stack words).
lf_checker_rt::export!(cdecl, rw_0089d030(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    unsafe {
        const POOL: u32 = 0x0115d9a0;
        const G824: u32 = 0x0115f824;
        const G_A: u32 = 0x0115f828;
        const G82C: u32 = 0x0115f82c;
        const G830: u32 = 0x0115f830;
        const G834: u32 = 0x0115f834;
        const G838: u32 = 0x0115f838;
        const G83C: u32 = 0x0115f83c;
        const G840: u32 = 0x0115f840;
        const ALLOC: u32 = 1;
        const SCAN: u32 = 2;
        const INDEX: u32 = 3;
        const PASS1: u32 = 4;
        const PASS2: u32 = 5;
        const COMMIT: u32 = 6;
        let seed = lf_checker_rt::global::<u32>(G_A).read_unaligned().wrapping_add(a6);
        lf_checker_rt::global::<u32>(G_A).write_unaligned(seed);
        let blk: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, lf_checker_rt::relocated(POOL), seed, 0x10);
        let sc: u32 = lf_checker_rt::callee_cdecl!(SCAN, u32, a0, a1, blk);
        let b1 = blk.wrapping_add(sc.wrapping_mul(10));
        let ix: u32 = lf_checker_rt::callee_cdecl!(INDEX, u32, blk, sc, a0, a1, b1);
        let b2 = b1.wrapping_add(ix.wrapping_mul(14));
        lf_checker_rt::callee_cdecl!(PASS1, u32, blk, sc, a2, a3, b2);
        let sz1 = lf_checker_rt::global::<u32>(G838).read_unaligned();
        let b3 = sz1.wrapping_add(a3).wrapping_add(b2);
        lf_checker_rt::callee_cdecl!(PASS2, u32, blk, sc, a4, a5, b3);
        let sz2 = lf_checker_rt::global::<u32>(G840).read_unaligned();
        let b4 = sz2.wrapping_add(a5);
        let fin = b4.wrapping_sub(blk).wrapping_add(b3);
        lf_checker_rt::global::<u32>(G_A).write_unaligned(fin);
        lf_checker_rt::global::<u32>(G82C).write_unaligned(sc);
        lf_checker_rt::global::<u32>(G830).write_unaligned(blk);
        lf_checker_rt::global::<u32>(G834).write_unaligned(b2);
        lf_checker_rt::global::<u32>(G838).write_unaligned(b3.wrapping_sub(b2));
        lf_checker_rt::global::<u32>(G83C).write_unaligned(b3);
        lf_checker_rt::global::<u32>(G840).write_unaligned(b4);
        let old = lf_checker_rt::global::<u32>(G824).read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(COMMIT, u32, lf_checker_rt::relocated(POOL), old);
        lf_checker_rt::global::<u32>(G824).write_unaligned(blk);
        r
    }
});
