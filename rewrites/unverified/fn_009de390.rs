// original: 0x009de390 framed_query_b
/// Build a float query from a triple, resolve it twice, dispatch all.
///
/// Packs the three floats at the incoming pointer with the incoming word
/// and flag byte into a frame parameter block and resolves it; copies the
/// 16-byte answer beside a second code address, resolves the same block
/// again into a second answer, then dispatches both answers with the
/// source pointer and a constant to the sink helper. Each dispatched
/// block's fourth word is the second code address: it lands there by
/// overlapping stores on the original's frame. Returns the sink's answer.
export!(thiscall, rw_009de390(this_: u32, src: *const u32, flag: u32, arg: u32) -> u32 {
    unsafe {
        let mut first = [0u32; 4];
        let params = [
            *src,
            *src.add(1),
            *src.add(2),
            0,
            arg,
            (flag as u8) as u32,
        ];
        let code = relocated(0x009deaa0);
        let code_pair = [code, code];
        callee_thiscall!(
            1,
            u32,
            first.as_mut_ptr() as u32,
            params.as_ptr() as u32,
            0,
            code_pair.as_ptr() as u32,
            4
        );
        let mut second = [0u32; 2];
        callee_thiscall!(
            2,
            u32,
            second.as_mut_ptr() as u32,
            params.as_ptr() as u32,
            0,
            &code as *const u32 as u32,
            4
        );
        callee_thiscall!(
            3,
            u32,
            this_,
            second[0],
            second[1],
            0,
            relocated(0x00404b80),
            first[0],
            first[1],
            first[2],
            relocated(0x00404b80),
            src as u32,
            0x42a00000
        )
    }
});
