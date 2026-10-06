// original: 0x009de2f0 framed_query_a
/// Build a float query from a triple, resolve it, dispatch the answer.
///
/// Packs the three floats at the incoming pointer with the incoming word
/// and a flag byte into a frame parameter block and hands it to the
/// resolver with a code address and constants; the resolver fills an
/// 8-byte frame answer. Then dispatches the answer words with a second
/// code address, the source pointer and a constant to the sink helper.
/// Returns the sink's answer.
export!(thiscall, rw_009de2f0(this_: u32, src: *const u32, arg: u32) -> u32 {
    unsafe {
        let mut answer = [0u32; 2];
        let params = [
            *src,
            *src.add(1),
            *src.add(2),
            0,
            arg,
            1,
        ];
        let code = relocated(0x009deaa0);
        callee_thiscall!(
            1,
            u32,
            answer.as_mut_ptr() as u32,
            params.as_ptr() as u32,
            0,
            &code as *const u32 as u32,
            4
        );
        callee_thiscall!(
            2,
            u32,
            this_,
            answer[0],
            answer[1],
            0,
            relocated(0x00404b80),
            src as u32,
            0x44160000
        )
    }
});
