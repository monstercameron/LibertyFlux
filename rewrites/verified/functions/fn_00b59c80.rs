// original: 0x00B59C80 clear_matching_stream_request_and_forward (proposed)
/// Clears the two stream-request state words when the input is null or matches
/// the current owner, then forwards the unchanged input to the shared helper.
///
/// The cdecl wrapper reads one stack word and does not dereference it. The
/// state is held at two separate global dwords. All branches tail-transfer to
/// the same helper, so the return value is the helper's exact 32-bit result.
lf_checker_rt::export!(cdecl, rw_00B59C80(input: u32) -> u32 {
    unsafe {
        const OWNER_WORD_VA: u32 = 0x0166_DA70;
        const REQUEST_WORD_VA: u32 = 0x0166_DA74;
        const FORWARD_CALLEE: u32 = 1;

        let owner = lf_checker_rt::global::<u32>(OWNER_WORD_VA);
        if input == 0 || owner.read() == input {
            owner.write(0);
            lf_checker_rt::global::<u32>(REQUEST_WORD_VA).write(0);
        }
        lf_checker_rt::callee_cdecl!(FORWARD_CALLEE, u32, input)
    }
});
