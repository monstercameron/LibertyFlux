// original: 0x00D4F580 task_build_chained_pair (proposed)

// Builds a chained pair of objects through the game allocator.
///
/// Allocates a block (intercepted callee 1); on success constructs the first
/// object on it through intercepted callee 2 with (2, arg0, 0.5, const, -1,
/// 1, 0, 0, 0, 1),
/// 1, 0, 0, 0), where const is the float stored in the image. Then allocates
/// a second block (intercepted callee 3); on success constructs the second
/// object through intercepted callee 4 with (first, 0, 0, 0) and returns its
/// result. Either allocation failure yields null (first) or returns null
/// (second).
///
/// Original: 0x00D4F580 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00d4f580(arg0: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167E2A0;
        const BLEND_CONST: u32 = 0x00EE1EB4;
        const HALF_BITS: u32 = 0x3F000000; // 0.5f
        const ALLOC1: u32 = 1;
        const MAKE1: u32 = 2;
        const ALLOC2: u32 = 3;
        const MAKE2: u32 = 4;
        let alloc = lf_checker_rt::global::<u32>(ALLOCATOR_SLOT).read();
        let block: u32 = lf_checker_rt::callee_thiscall!(ALLOC1, u32, alloc);
        let mut first: u32 = 0;
        if block != 0 {
            let k = lf_checker_rt::global::<u32>(BLEND_CONST).read();
            first = lf_checker_rt::callee_thiscall!(
                MAKE1, u32, block, 2, arg0, HALF_BITS, k, 0xFFFFFFFF, 1, 0, 0, 0, 1);
        }
        let alloc2 = lf_checker_rt::global::<u32>(ALLOCATOR_SLOT).read();
        let block2: u32 = lf_checker_rt::callee_thiscall!(ALLOC2, u32, alloc2);
        if block2 == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(MAKE2, u32, block2, first, 0, 0, 0)
    }
});
