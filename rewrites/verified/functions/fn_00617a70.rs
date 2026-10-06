// original: 0x00617A70 init_files_mem_g (proposed)

/// Lazily create and register the files-memory descriptor for slot G.
///
/// Slot G is one of a family of near-identical one-time initializers (one
/// global each). On the first call it allocates a 0x40-byte allocator block
/// through the thread-local allocator object, stamps the descriptor fields,
/// publishes a set of small offsets into shared tables, clears the
/// descriptor's link words, runs the descriptor through the register callee
/// and finally hands it to the link callee together with the address of its
/// own global. Later calls return at once: the global is non-null.

/// Slot G additionally links the parent descriptor (null-checked,
/// initialised on demand) into `+0x08` and merges the parent flag
/// bits into its own flag word.
/// Arguments: none (cdecl/0). Return value: none (the original leaves
/// whatever the last callee returned in EAX on the init path and entry EAX
/// on the early path; callers use neither).
///
/// Descriptor layout (offsets from the block the allocator answers):
/// `+0x04` file id, `+0x08`/`+0x0c` link words, `+0x10` span length,
/// `+0x1c` flag word, `+0x20`/`+0x28` zero words, `+0x24`/`+0x2c` table
/// pointers. All comparisons are against zero/null only; no value is
/// compared as signed or unsigned.
///
/// Edge cases: a null allocator answer, or a null answer from the
/// constructor callee, stores null in the global and then faults on the
/// first field store, exactly like the original; the checker compares the
/// fault. A non-null global on entry takes the early path with no calls.
///
/// Stack shape (verified by probe): the link callee takes its object in
/// ECX plus two stack words and pops both words itself (callee cleanup);
/// the original's trailing stack adjustment then restores the frame, so
/// both paths return balanced. A first attempt modelled the link as
/// caller-cleanup and both sides returned into a stack slot instead.
lf_checker_rt::export!(cdecl, rw_00617A70() -> u32 {
    unsafe {
        const GLOBAL: u32 = 0x018B_7484;
        const FILE_ID: u32 = 0x00F9_56C0;
        const SPAN: u32 = 0x80;
        const VTABLE_A: u32 = 0x0061_B470;
        const VTABLE_B: u32 = 0x0061_B460;
        const TABLE_ARG: u32 = 0x0108_1D3C;
        const REGISTRY: u32 = 0x01BB_5520;
        const FLAG_KEEP: u32 = 0xFFFF_0000;
        const ALLOC_SIZE: u32 = 0x40;
        const ALLOC_ALIGN: u32 = 0x10;
        const REG_THIS_OFF: u32 = 0x18;
        const F_ID: u32 = 0x04;
        const F_LINK0: u32 = 0x08;
        const F_LINK1: u32 = 0x0C;
        const F_SPAN: u32 = 0x10;
        const F_FLAGS: u32 = 0x1C;
        const F_FLAGS_HI: u32 = 0x1E;
        const F_ZERO0: u32 = 0x20;
        const F_VTA: u32 = 0x24;
        const F_ZERO1: u32 = 0x28;
        const F_VTB: u32 = 0x2C;
        const CCTOR: u32 = 2;
        const CREG: u32 = 3;
        const CLINK: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        use lf_checker_rt::{callee_cdecl, callee_thiscall, global, relocated, tls_slot};
        if rd32(global::<u32>(GLOBAL) as u32) != 0 {
            return 0;
        }
        let slot0 = tls_slot(0);
        let obj = rd32(slot0.wrapping_add(8));
        let vt = rd32(obj);
        let tgt = rd32(vt.wrapping_add(8));
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let blk = alloc(obj, ALLOC_SIZE, ALLOC_ALIGN, 0);
        let blk = if blk != 0 { callee_thiscall!(CCTOR, u32, blk) } else { 0 };
        wr32(global::<u32>(GLOBAL) as u32, blk);
        wr32(blk.wrapping_add(F_ID), relocated(FILE_ID));
        wr32(blk.wrapping_add(F_SPAN), SPAN);
        wr32(blk.wrapping_add(F_ZERO0), 0);
        wr32(blk.wrapping_add(F_VTA), relocated(VTABLE_A));
        wr32(blk.wrapping_add(F_ZERO1), 0);
        wr32(blk.wrapping_add(F_VTB), relocated(VTABLE_B));
        // Parent descriptor at 0x018B_74C4: run its init if null, link it
        // into +0x08, then merge its flag bits 6..15 into ours.
        let mut par = rd32(global::<u32>(0x018B_74C4) as u32);
        if par == 0 {
            callee_cdecl!(5, u32,);
            par = rd32(global::<u32>(0x018B_74C4) as u32);
        }
        wr32(blk.wrapping_add(F_LINK0), par);
        wr32(blk.wrapping_add(F_LINK1), 0);
        let masked = rd32(par.wrapping_add(F_FLAGS)) & 0xFFC0;
        let diff = (rd32(blk.wrapping_add(F_FLAGS)) ^ masked) & 0xFFFF;
        wr32(
            blk.wrapping_add(F_FLAGS),
            rd32(blk.wrapping_add(F_FLAGS)) ^ diff,
        );
        ((blk.wrapping_add(F_FLAGS_HI)) as *mut u16)
            .write_unaligned((masked >> 16) as u16);
        wr32(global::<u32>(0x0108_1F54) as u32, 0x10);
        wr32(global::<u32>(0x0108_1DA8) as u32, 0x20);
        wr32(global::<u32>(0x0108_1B90) as u32, 0x24);
        callee_thiscall!(CREG, u32, blk, relocated(TABLE_ARG));
        let mut slot0w = relocated(GLOBAL);
        let mut slot1w = rd32(blk.wrapping_add(F_ID));
        let reg_this = rd32(global::<u32>(REGISTRY) as u32).wrapping_add(REG_THIS_OFF);
        // Argument order mirrors the original's pushes: it pushes &slot0
        // first, so &slot1 lands in the first arg slot and &slot0 in the
        // second.
        callee_thiscall!(
            CLINK,
            u32,
            reg_this,
            &mut slot1w as *mut u32 as u32,
            &mut slot0w as *mut u32 as u32
        );
        0
    }
});
