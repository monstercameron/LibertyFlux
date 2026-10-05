// original: 0x00c24650 cam_handler_select (proposed)
/// Select a camera-mode handler: query both objects through their virtual
/// slot +0x28, then dispatch on the first answer (jump table over
/// `r1 - 1`) with a chained inner dispatch on the second answer in each
/// live case. The map below is read from the tables: r1=1 tests
/// r2 in {1, 2, 16}, r1=2 tests {1, 16}, r1=6 tests {7, 6, ...chain},
/// r1=7 tests {6, ...chain}, r1=14 tests {14, 7, 6, ...chain}, everything
/// else yields the default handler. Returned handler addresses carry
/// relocations and go through `relocated`.
///
/// Original: 0x00c24650 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00c24650(obj_a: u32, obj_b: u32) -> u32 {
    unsafe {
        const VT_SLOT: u32 = 0x28;
        const H_DEFAULT: u32 = 0x00c25280;
        const H_CHAIN_A: u32 = 0x00c252d0;
        const H_CHAIN_B: u32 = 0x00c25320;
        const H_TAIL: u32 = 0x00c25370;
        let r = lf_checker_rt::relocated;
        let vta = (obj_a as *const u32).read_unaligned();
        let f1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((vta.wrapping_add(VT_SLOT) as *const u32).read_unaligned() as usize);
        let r1 = f1(obj_a);
        let vtb = (obj_b as *const u32).read_unaligned();
        let f2: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((vtb.wrapping_add(VT_SLOT) as *const u32).read_unaligned() as usize);
        let r2 = f2(obj_b);
        // Shared inner chain (entered directly and by fallthrough).
        let chain = |x: u32| -> u32 {
            if x == 1 {
                r(H_CHAIN_A)
            } else if x == 2 {
                r(H_CHAIN_B)
            } else if x == 16 {
                r(H_CHAIN_B)
            } else {
                r(H_DEFAULT)
            }
        };
        match r1.wrapping_sub(1) {
            0 => chain(r2),
            1 => {
                if r2 == 1 || r2 == 16 {
                    r(H_CHAIN_B)
                } else {
                    r(H_DEFAULT)
                }
            }
            5 => {
                if r2 == 7 || r2 == 6 {
                    r(H_DEFAULT)
                } else {
                    chain(r2)
                }
            }
            6 => {
                if r2 == 6 {
                    r(H_DEFAULT)
                } else {
                    chain(r2)
                }
            }
            13 => {
                if r2 == 14 {
                    r(H_TAIL)
                } else if r2 == 7 || r2 == 6 {
                    r(H_DEFAULT)
                } else {
                    chain(r2)
                }
            }
            _ => r(H_DEFAULT),
        }
    }
});
