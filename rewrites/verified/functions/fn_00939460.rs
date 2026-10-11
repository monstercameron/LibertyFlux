// original: 0x00939460 streaming_reset_and_continue

/// Clears the streaming request and phase globals, enables streaming, then
/// invokes its setup helper and forwards the helper's return value from the
/// continuation routine. The function takes no stack arguments and uses the
/// cdecl convention. Global values are addressed relative to the relocated
/// image base through the checker runtime.
lf_checker_rt::export!(cdecl, rw_00939460() -> u32 {
    unsafe {
        const STREAMING_PHASE: u32 = 0x11A4_EF4;
        const REQUEST_INDEX: u32 = 0x11A4_EFC;
        const REQUEST_COUNT: u32 = 0x11A4_F00;
        const LOADING_FLAGS: u32 = 0x11A4_F04;
        const ACTIVE_REQUESTS: u32 = 0x11A4_F08;
        const STREAMING_ENABLED: u32 = 0x0103_6EF0;
        const NEXT_REQUEST: u32 = 0x11A4_F0C;
        const REQUEST_LIMIT: u32 = 0x11A4_F14;
        const REQUEST_CURSOR: u32 = 0x11A4_F10;

        lf_checker_rt::callee_cdecl!(1, u32,);
        lf_checker_rt::global::<u32>(STREAMING_PHASE).write_unaligned(0);
        lf_checker_rt::global::<u32>(REQUEST_INDEX).write_unaligned(0);
        lf_checker_rt::global::<u32>(REQUEST_COUNT).write_unaligned(0);
        lf_checker_rt::global::<u8>(LOADING_FLAGS).write(0);
        lf_checker_rt::global::<u32>(ACTIVE_REQUESTS).write_unaligned(0);
        lf_checker_rt::global::<u8>(STREAMING_ENABLED).write(1);
        lf_checker_rt::global::<u32>(NEXT_REQUEST).write_unaligned(0);
        lf_checker_rt::global::<u32>(REQUEST_LIMIT).write_unaligned(0);
        lf_checker_rt::global::<u16>(REQUEST_CURSOR).write_unaligned(0);
        lf_checker_rt::global::<u8>(LOADING_FLAGS + 1).write(0);
        lf_checker_rt::callee_cdecl!(2, u32,);
        lf_checker_rt::callee_cdecl!(3, u32,)
    }
});
