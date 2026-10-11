// original: 0x00DE5200 UIGTAScrollingRow::vf94
/// Forward the global row manager to the base routine with its first stack
/// argument replaced by the constant two. The base call supplies the return
/// value; this wrapper uses thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_de5200(_this: u32, _argument: u32) -> u32 {
    const ROW_MANAGER: u32 = 0x018B6C8C;
    unsafe {
        let manager = lf_checker_rt::global::<u32>(ROW_MANAGER).read_unaligned();
        lf_checker_rt::callee_thiscall!(1, u32, manager, 2)
    }
});

