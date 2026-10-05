// original: 0x00c3e910 train_convert_by_global_mode (proposed)
/// Convert the argument through the converter selected by a global mode.
///
/// `this` (ECX) is the car, `arg` an input word. Reads the global mode,
/// subtracts one, and dispatches (the original through a jump table over
/// pristine code bytes, verified from the file to list the cases in
/// order): modes 1..5 call first-converters id 1..5 (declared cdecl/1:
/// the original passes its scratch pointer in ECX, which cannot be
/// compared across different frames, so only the stack argument is), any other mode calls id 6. Each
/// path then calls copier id 7 (thiscall/2: this car, scratch pointer,
/// constant 1). The contract does not compare the scratch addresses
/// (each side's own frame) and the stubbed callees never observe the
/// scratch contents, so the proof covers the dispatch, the argument,
/// the constant and the final answer. Returns id 7's answer.
///
/// Original: 0x00c3e910 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c3e910(this: u32, arg: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x16d2194;
        const COPY: u32 = 7;
        let mut scratch = [0u32; 16];
        let sp = scratch.as_mut_ptr() as u32;
        let mode = lf_checker_rt::global::<u32>(MODE).read_unaligned();
        let case = mode.wrapping_sub(1);
        match case {
            0 => {
                let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, arg);
            }
            1 => {
                let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, arg);
            }
            2 => {
                let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, arg);
            }
            3 => {
                let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, arg);
            }
            4 => {
                let _: u32 = lf_checker_rt::callee_cdecl!(5, u32, arg);
            }
            _ => {
                let _: u32 = lf_checker_rt::callee_cdecl!(6, u32, arg);
            }
        }
        lf_checker_rt::callee_thiscall!(COPY, u32, this, sp, 1)
    }
});
