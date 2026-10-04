// original: 0x00946c00 pool_pair_init
/// Initialise a two-element pool in place and raise its ready flag.
///
/// Runs the element initialiser (a stubbed callee taking the element
/// pointer in ECX) over two consecutive elements with a stride of 0xBD0
/// bytes, then sets the ready byte at offset 0x1920. Returns the second
/// call's answer.
export!(thiscall, rw_00946c00(obj: *mut u8) -> u32 {
    unsafe {
        let mut answer = 0;
        for i in 0..2usize {
            answer = callee_thiscall!(1, u32, obj.add(i * 0xBD0) as u32);
        }
        *obj.add(0x1920) = 1;
        answer
    }
});
