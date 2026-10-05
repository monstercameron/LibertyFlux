// original: 0x00b96fb0 script_vec_store_b (proposed)

/// Stores six float arguments into two global float blocks.
///
/// Same shape as `rw_00b96f50`: the first three arguments go to the first
/// three of four consecutive global slots starting at `BLOCK0`; the last
/// two arguments go to a pair starting at `BLOCK1`. The fourth slot of the
/// first block is NOT an argument: the original loads it from below its own
/// aligned frame (`[esp+0x0c]` after `and esp, ~0x0f`), which is
/// uninitialized scratch. The contract pins the scratch fill to zero
/// (`stack_fill: 0`) and this rewrite stores zero for that slot; see the
/// `narrowed` note.
///
/// Original: 0x00B96FB0 (cdecl, five stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b96fb0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const BLOCK0: u32 = 0x012DD8F0;
        const BLOCK1: u32 = 0x012DD8D0;
        lf_checker_rt::global::<u32>(BLOCK0).write(a0);
        lf_checker_rt::global::<u32>(BLOCK0 + 4).write(a1);
        lf_checker_rt::global::<u32>(BLOCK0 + 8).write(a2);
        // Uninitialized-scratch slot: the original reads below its frame.
        lf_checker_rt::global::<u32>(BLOCK0 + 12).write(0);
        lf_checker_rt::global::<u32>(BLOCK1).write(a3);
        lf_checker_rt::global::<u32>(BLOCK1 + 4).write(a4);
    }
    0
});
