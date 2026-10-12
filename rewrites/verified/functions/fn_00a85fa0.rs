// original: 0x00A85FA0 default_task_float_by_index

/// Return one of four fixed single-precision defaults selected by an index.
///
/// The sole cdecl stack argument is an unsigned 32-bit index. Values zero
/// through three select `4.0`, `8.0`, `16.0`, and `1000.0`; every larger
/// index selects `0.0`. The result is returned on the x87 stack.
lf_checker_rt::export!(cdecl, rw_00a85fa0(index: u32) -> f32 {
    match index {
        0 => 4.0,
        1 => 8.0,
        2 => 16.0,
        3 => 1000.0,
        _ => 0.0,
    }
});
