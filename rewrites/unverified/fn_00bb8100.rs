// original: 0x00BB8100 None
/// Forward a three-component position to a scene-streaming method.
///
/// The native handler receives three raw float words, copies them into a
/// temporary vector, and passes its address as the method's sole stack
/// argument. The receiver is the relocated scene-streaming object; the
/// method's EAX result is returned unchanged. Raw words preserve signed zero
/// and NaN payloads.
lf_checker_rt::export!(cdecl, rw_00bb8100(x: u32, y: u32, z: u32) -> u32 {
    const RECEIVER_VA: u32 = 0x0117_374C;
    const VECTOR_WORDS: usize = 3;
    let position = [x, y, z];
    let position_ptr = position.as_ptr() as usize as u32;
    let receiver = lf_checker_rt::relocated(RECEIVER_VA);
    lf_checker_rt::callee_thiscall!(1, u32, receiver, position_ptr)
});
