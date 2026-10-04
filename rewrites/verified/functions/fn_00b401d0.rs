// original: 0x00b401d0 publish_task_config (proposed)

/// Publish one task-configuration snapshot into the module's globals.
///
/// Copies four words from `p`, four float arguments, one integer argument
/// and six groups of four floats from `q` (at `+0x310` stepping `0x10`) into
/// fixed global slots. The fourth word of each `q` group lands in a compact
/// side table while the first three land in the matching vector slot whose
/// fourth word is set from a stack slot the original never writes: under
/// the checker's defined stack fill that value is 0, which is what this
/// rewrite stores. The whole body is skipped when the initialised flag byte
/// is already nonzero, and sets the published flag byte otherwise. All
/// copies are bitwise; no arithmetic is done.
///
/// Original: 0x00B401D0 (cdecl, seven stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00b401d0(p: u32, q: u32, fa: u32, fb: u32, fc: u32, fd: u32, ival: u32) -> u32 {
    unsafe {
        const FLAG_DONE: u32 = 0x016B7BF2;
        const FLAG_PUBLISHED: u32 = 0x016B7BF3;
        const CENTER: u32 = 0x016C85A0;
        const PARAMS: u32 = 0x016B7C74;
        const MODE: u32 = 0x016B7C94;
        const VECTORS: u32 = 0x016B8FD0;
        const SIDES: u32 = 0x016B7C5C;
        const Q_BASE: u32 = 0x310;
        const GROUPS: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gw32(va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(va).write_unaligned(v) }
        }

        if unsafe { lf_checker_rt::global::<u8>(FLAG_DONE).read() } != 0 {
            return 0;
        }
        unsafe { lf_checker_rt::global::<u8>(FLAG_PUBLISHED).write(1) };
        for i in 0..4u32 {
            gw32(CENTER.wrapping_add(i * 4), rd32(p.wrapping_add(i * 4)));
        }
        gw32(PARAMS, fc);
        gw32(PARAMS.wrapping_add(4), fd);
        gw32(PARAMS.wrapping_add(8), fa);
        gw32(PARAMS.wrapping_add(12), fb);
        gw32(MODE, ival);
        for g in 0..GROUPS {
            let src = q.wrapping_add(Q_BASE).wrapping_add(g * 0x10);
            let dst = VECTORS.wrapping_add(g * 0x10);
            for i in 0..3u32 {
                gw32(dst.wrapping_add(i * 4), rd32(src.wrapping_add(i * 4)));
            }
            // The original copies an uninitialised stack slot here; the
            // contract defines that slot as 0 (stack_fill).
            gw32(dst.wrapping_add(12), 0);
            gw32(SIDES.wrapping_add(g * 4), rd32(src.wrapping_add(12)));
        }
        0
    }
});
