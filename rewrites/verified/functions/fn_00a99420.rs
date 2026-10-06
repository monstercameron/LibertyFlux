// original: 0x00a99420 init_segment_tables_and_register
/// Initializes a large segment-table region, configures a scratch object,
/// and registers the result in globals.
///
/// `this` points to a region of about 570KB. The function zeroes the header
/// word, a flag word and a trailing half-word, then builds three element
/// arrays through a helper call per element (2048 entries spaced 0x100
/// bytes apart, 128 spaced 0x120 apart, 32 spaced 0xA0 apart), clears a
/// 0x80-word trailer, runs a six-step configuration sequence on a scratch
/// slot, creates a handle through a factory call, and publishes the handle,
/// a zero counter, a 0x300 capacity and forty cleared stride-8 slots to
/// globals. Returns 0x140, the stride loop's end counter.
export!(thiscall, rw_00a99420(this_: u32) -> u32 {
    unsafe {
        (this_ as *mut u32).write_unaligned(0);
        (this_.wrapping_add(0x3010) as *mut u32).write_unaligned(0);
        (this_.wrapping_add(0x8eed8) as *mut u16).write_unaligned(0);
        let mut p = this_.wrapping_add(0x4820);
        for _ in 0..0x800 {
            lf_checker_rt::callee_thiscall!(1, u32, this_.wrapping_add(0x84820), p);
            p = p.wrapping_add(0x100);
        }
        p = this_.wrapping_add(0x84830);
        for _ in 0..0x80 {
            lf_checker_rt::callee_thiscall!(1, u32, this_.wrapping_add(0x8d830), p);
            p = p.wrapping_add(0x120);
        }
        p = this_.wrapping_add(0x8d840);
        for _ in 0..0x20 {
            lf_checker_rt::callee_thiscall!(1, u32, this_.wrapping_add(0x8ec40), p);
            p = p.wrapping_add(0xa0);
        }
        let trailer = this_.wrapping_add(0x8ecd8) as *mut u32;
        for i in 0..0x80 {
            trailer.add(i as usize).write_unaligned(0);
        }
        let mut slot: u32 = 0;
        let sa = &mut slot as *mut u32 as u32;
        lf_checker_rt::callee_thiscall!(2, u32, sa);
        lf_checker_rt::callee_thiscall!(3, u32, sa, 1, 6, 0);
        lf_checker_rt::callee_thiscall!(4, u32, sa, 1, 9);
        lf_checker_rt::callee_thiscall!(5, u32, sa, 0, 1, 5);
        lf_checker_rt::callee_thiscall!(6, u32, sa, 1, 6);
        lf_checker_rt::callee_thiscall!(7, u32, sa, 0, 1, 7);
        let h = lf_checker_rt::callee_cdecl!(8, u32, sa, 0, 0, 0, 0);
        lf_checker_rt::global::<u32>(0x1394c10).write_unaligned(h);
        lf_checker_rt::global::<u32>(0x1394d5c).write_unaligned(0);
        lf_checker_rt::global::<u32>(0x1394c14).write_unaligned(0x300);
        let mut i: u32 = 0;
        while i < 0x140 {
            lf_checker_rt::global::<u32>(0x1394c20 + i).write_unaligned(0);
            i += 8;
        }
        i
    }
});
