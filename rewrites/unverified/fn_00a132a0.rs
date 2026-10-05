// original: 0x00a132a0 subtype_code_store (proposed)
/// Store a subtype code derived from the object's kind word.
///
/// Reads the word at `obj + 0x1304` and dispatches through a jump table:
/// 0 stores 0, 1 stores 1 (7 when the global selector is 1), 2 stores 4,
/// 4 stores 2 (8 when the selector is 2), 5 stores 3. Kind 3 and anything
/// above 5 store nothing. Returns 1 when a code was stored, else 0 (low
/// byte only, so the contract compares `al`). Stdcall, two arguments.
export!(stdcall, rw_00a132a0(obj: u32, out: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x1304;
        const SELECTOR: u32 = 0x011d6fd4;
        let code: u32 = match ((obj + KIND_OFF) as *const u32).read_unaligned() {
            0 => 0,
            1 => {
                if *global::<u32>(SELECTOR) == 1 {
                    7
                } else {
                    1
                }
            }
            2 => 4,
            4 => {
                if *global::<u32>(SELECTOR) == 2 {
                    8
                } else {
                    2
                }
            }
            5 => 3,
            _ => return 0,
        };
        (out as *mut u32).write_unaligned(code);
        1
    }
});
