// original: 0x00936C80 forward_network_transition_defaults

/// Call the network transition helper on its fixed global object, forwarding
/// the three leading stack arguments and the final stack argument around two
/// fixed mode words (`0` and `1`). Argument order is preserved as passed on
/// the caller's stack. This cdecl wrapper has four stack arguments and no
/// defined return value.
lf_checker_rt::export!(cdecl, rw_00936c80(
    first_value: u32,
    second_value: u32,
    third_value: u32,
    final_value: u32
) -> () {
    const TRANSITION_OBJECT: u32 = 0x011a_4070;
    let transition_object = lf_checker_rt::relocated(TRANSITION_OBJECT);
    let _ = lf_checker_rt::callee_thiscall!(
        1,
        u32,
        transition_object,
        first_value,
        second_value,
        third_value,
        0,
        1,
        final_value
    );
});
