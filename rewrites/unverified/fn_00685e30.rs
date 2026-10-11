// original: 0x00685E30 rage::crFrame::vf1

/// Forward the low byte of the sole stack argument to the frame-record
/// factory in ECX and return its pointer unchanged. The original instance
/// register is otherwise unused. This thiscall method pops its one argument.
lf_checker_rt::export!(thiscall, rw_00685e30(_this: u32, kind_input: u32) -> u32 {
    unsafe {
        let kind = kind_input & 0xFF;
        lf_checker_rt::callee_fastcall!(1, u32, kind, 0)
    }
});
