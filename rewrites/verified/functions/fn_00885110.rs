// original: 0x00885110 streaming_reader_query_and_subtract

/// Query the shared streaming reader twice with the same argument. Virtual
/// slot 6 supplies a quantity that is subtracted with 32-bit wrapping from
/// the running total global; virtual slot 3 supplies the return value. The
/// manager pointer and total occupy adjacent globals. This is a cdecl routine
/// with one 32-bit stack argument.
const STREAM_READER_POINTER_VA: u32 = 0x0115_a3f8;
const STREAM_READER_TOTAL_VA: u32 = 0x0115_a3fc;

lf_checker_rt::export!(cdecl, rw_00885110(argument: u32) -> u32 {
    unsafe {
        let reader = lf_checker_rt::global::<u32>(STREAM_READER_POINTER_VA).read_unaligned();
        let consumed = lf_checker_rt::callee_thiscall!(1, u32, reader, argument);
        let total = lf_checker_rt::global::<u32>(STREAM_READER_TOTAL_VA).read_unaligned();
        lf_checker_rt::global::<u32>(STREAM_READER_TOTAL_VA)
            .write_unaligned(total.wrapping_sub(consumed));
        lf_checker_rt::callee_thiscall!(2, u32, reader, argument)
    }
});
