// original: 0x00694E00 rage::crFrameFilterN::vf6

/// Clear the single word addressed by the stdcall argument and return zero.
///
/// The function takes one pointer in its stack argument area, writes a zero
/// word through it, removes that argument, and returns zero in EAX.
lf_checker_rt::export!(stdcall, rw_00694E00(destination: u32) -> u32 {
    unsafe {
        (destination as *mut u32).write_unaligned(0);
        0
    }
});
