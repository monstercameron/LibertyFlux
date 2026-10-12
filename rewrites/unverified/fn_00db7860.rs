// original: 0x00DB7860 music_viewer_build_panel
/// Initializes the viewer's timing record, forwards its six returned words
/// with the three caller arguments and two mode flags, then disposes of the
/// temporary record. The timing helper receives `this` in ECX and two stack
/// words (zero and the bit pattern for 5.0f). Its returned pointer supplies
/// three packed pairs; the last pair is also forwarded in XMM0. The final
/// cleanup receives a pointer into the caller frame; this function does not
/// initialize the six words in that record before the call. This is a
/// 32-bit MSVC thiscall wrapper with three stack arguments and no result.
lf_checker_rt::export!(thiscall, rw_00db7860(
    this_ptr: u32,
    first_argument: u32,
    second_argument: u32,
    third_argument: u32,
) -> () {
    const ZERO_MODE: u32 = 0;
    const FIVE_SECONDS_BITS: u32 = 0x40a00000;
    const PANEL_MODE: u32 = 0;
    const PANEL_ENABLED: u32 = 1;

    let mut output_record = [0u32; 6];
    let output_pointer = output_record.as_mut_ptr() as usize as u32;
    let returned_pointer = lf_checker_rt::callee_thiscall!(
        1,
        u32,
        output_pointer,
        ZERO_MODE,
        FIVE_SECONDS_BITS,
    );

    let returned_words = returned_pointer as *const u32;
    let read_word = |index: usize| unsafe { returned_words.add(index).read_unaligned() };
    let panel_values = [
        read_word(0),
        read_word(1),
        read_word(2),
        read_word(3),
        read_word(4),
        read_word(5),
    ];

    type PanelCall = extern "thiscall" fn(
        u32,
        core::arch::x86::__m128i,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
    ) -> u32;
    let final_pair = unsafe {
        core::mem::transmute::<[u32; 4], core::arch::x86::__m128i>([
            panel_values[4],
            panel_values[5],
            0,
            0,
        ])
    };
    let panel_call: PanelCall = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(2) as usize)
    };
    let _ = panel_call(
        this_ptr,
        final_pair,
        first_argument,
        second_argument,
        third_argument,
        panel_values[0],
        panel_values[1],
        panel_values[2],
        panel_values[3],
        panel_values[4],
        panel_values[5],
        PANEL_MODE,
        PANEL_ENABLED,
    );

    let mut temporary_record = [0u32; 6];
    let _ = lf_checker_rt::callee_thiscall!(
        3,
        u32,
        temporary_record.as_mut_ptr() as usize as u32,
    );
});
