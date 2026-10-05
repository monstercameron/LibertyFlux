// original: 0x0093B530 stream_obj_init_list (proposed)

/// Tear down both streaming lists and clear the small globals.
///
/// Unlinks every node of the first list head-first, running teardown on
/// each and freeing it, then runs the two middle passes. Unlinks every
/// node of the second list the same way, zeroing its mark byte, short
/// and count first. Clears the two tail globals and answers 0.
lf_checker_rt::export!(cdecl, rw_0093b530() -> u32 {
    unsafe {
        const TEARDOWN: u32 = 1;
        const FREE: u32 = 2;
        const FIRST: u32 = 3;
        const SECOND: u32 = 4;
        const A_HEAD: u32 = 0x11A4EE0;
        const B_HEAD: u32 = 0x11A4EE4;
        const TAIL_A: u32 = 0x11A4EEC;
        const TAIL_B: u32 = 0x11A4EE8;
        const MARK: u32 = 4;
        const SHORT: u32 = 0x108;
        const COUNT: u32 = 0x104;
        let ha = lf_checker_rt::global::<u32>(A_HEAD);
        let mut node = ha.read_unaligned();
        while node != 0 {
            let next = (node as *const u32).read_unaligned();
            ha.write_unaligned(next);
            let _: u32 = lf_checker_rt::callee_thiscall!(TEARDOWN, u32, node);
            let _: u32 = lf_checker_rt::callee_cdecl!(FREE, u32, node);
            node = ha.read_unaligned();
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(FIRST, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(SECOND, u32,);
        let hb = lf_checker_rt::global::<u32>(B_HEAD);
        let mut cur = hb.read_unaligned();
        while cur != 0 {
            let next = (cur as *const u32).read_unaligned();
            hb.write_unaligned(next);
            ((cur + MARK) as *mut u8).write(0);
            ((cur + SHORT) as *mut u16).write_unaligned(0);
            ((cur + COUNT) as *mut u32).write_unaligned(0);
            let _: u32 = lf_checker_rt::callee_cdecl!(FREE, u32, cur);
            cur = hb.read_unaligned();
        }
        lf_checker_rt::global::<u32>(TAIL_A).write_unaligned(0);
        lf_checker_rt::global::<u32>(TAIL_B).write_unaligned(0);
        0
    }
});
