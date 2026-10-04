// original: 0x009256f0 view_system_init (proposed)

/// Initialize the view system: targets, eight row blocks and the copy table.
///
/// Takes no arguments. Runs the two start-up helpers (callees 1 and 2); the
/// second returns a mode pointer that is null when the mode query found
/// nothing. Clears the active-mode byte at 0x1036780. When the mode is
/// non-null and the mode check (callee 3) answers nonzero, the mode is
/// installed (byte = 1) and the render targets are recreated for it
/// (callee 4, the 0x00925430 function, with `(mode, 0x20, 8)`); otherwise
/// the targets are recreated with `(1, 0x20, 8)`.
///
/// Then eight row blocks are initialized. Block `k` (base
/// `0x119F1C0 + k*0x110`) gets: words +0x00/+0x04/+0x08 = 0, +0x0C = `g`,
/// +0x10/+0x14/+0x18 = 0, +0x1C = `g`, byte +0x2D = 0, word +0x40 = 0, and
/// the 16-byte mask `FFFFFFFF FFFFFFFF 00000000 FFFFFFFF` (image block at
/// 0xE863D0) at +0x30. `g` is a word the original reads from its own
/// uninitialized stack area; the rewrite reads its own uninitialized stack
/// slot the same way, and the contract's `stack_fill: 0` makes both read 0
/// (in the game this value is whatever the stack held).
///
/// Finally sixteen copy-table entries are built: entry `k` copies the row at
/// `0x119F320 + (k&1)*0x110` to `0x11A0B10 + k*0x100` through the record
/// copier (callee 5, the 0x00924b00 function) and writes
/// `0, 0, -1, -1, -1, 0` at `0x11A0BEC + k*0x100`. Returns whatever the last
/// record-copy call returned (in the game, the last destination address).
///
/// Original: 0x009256f0 (cdecl, no stack words, plain `ret`).
lf_checker_rt::export!(cdecl, rw_009256f0() -> u32 {
    unsafe {
        // The original's uninitialized stack word, read before anything else
        // so this slot holds only the worker's reset fill, exactly like the
        // original's slot. Volatile: the read itself is the behaviour.
        let slot = core::mem::MaybeUninit::<u32>::uninit();
        let g = (slot.as_ptr() as *const u32).read_volatile();

        lf_checker_rt::callee_cdecl!(1, u32,);
        let mode = lf_checker_rt::callee_cdecl!(2, u32,);
        let active = lf_checker_rt::global::<u8>(0x1036780);
        active.write(0);
        if mode != 0 {
            let ok = lf_checker_rt::callee_cdecl!(3, u32,);
            if (ok & 0xFF) as u8 != 0 {
                active.write(1);
                lf_checker_rt::callee_cdecl!(4, u32, mode, 0x20u32, 8u32);
            } else {
                lf_checker_rt::callee_cdecl!(4, u32, 1u32, 0x20u32, 8u32);
            }
        } else {
            lf_checker_rt::callee_cdecl!(4, u32, 1u32, 0x20u32, 8u32);
        }
        let mask = (lf_checker_rt::relocated(0x00E863D0) as *const [u32; 4]).read_unaligned();
        for k in 0..8u32 {
            let b = 0x119F1C0u32.wrapping_add(k.wrapping_mul(0x110));
            let w = |off: u32| lf_checker_rt::global::<u32>(b.wrapping_add(off));
            w(0x00).write_unaligned(0);
            w(0x04).write_unaligned(0);
            w(0x08).write_unaligned(0);
            w(0x0C).write_unaligned(g);
            w(0x10).write_unaligned(0);
            w(0x14).write_unaligned(0);
            w(0x18).write_unaligned(0);
            w(0x1C).write_unaligned(g);
            (lf_checker_rt::global::<u8>(b.wrapping_add(0x2D)) as *mut u8).write(0);
            w(0x40).write_unaligned(0);
            (lf_checker_rt::global::<u32>(b.wrapping_add(0x30)) as *mut [u32; 4])
                .write_unaligned(mask);
        }
        let mut last = 0u32;
        for k in 0..16u32 {
            let src = if k & 1 != 0 { 0x119F430u32 } else { 0x119F320u32 };
            let dst = 0x11A0B10u32.wrapping_add(k.wrapping_mul(0x100));
            last = lf_checker_rt::callee_thiscall!(5, u32, lf_checker_rt::relocated(dst), lf_checker_rt::relocated(src));
            let e = 0x11A0BECu32.wrapping_add(k.wrapping_mul(0x100));
            let w = |off: u32| lf_checker_rt::global::<u32>(e.wrapping_add(off));
            w(0x00).write_unaligned(0);
            w(0x04).write_unaligned(0);
            w(0x08).write_unaligned(0xFFFFFFFF);
            w(0x0C).write_unaligned(0xFFFFFFFF);
            w(0x10).write_unaligned(0xFFFFFFFF);
            w(0x14).write_unaligned(0);
        }
        last
    }
});
