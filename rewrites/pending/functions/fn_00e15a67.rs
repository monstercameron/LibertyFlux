// original: 0x00e15a67 open_file2_or_fallback
/// Open a file, preferring the extended routine when it resolves.
///
/// When the availability check (cdecl/0, stubbed as id 1) reports present,
/// the module handle is fetched through the module slot (stdcall/1, planted
/// stub id 2) and the extended open routine is resolved through the lookup
/// slot (stdcall/2, planted stub id 3); a null resolution returns -1. A
/// second availability check then selects the path: when set, the extended
/// routine is invoked through the resolved pointer (stdcall/5, stub id 4
/// by scripted answer) with the name, access, share and disposition
/// arguments plus a six-word parameter block built on the stack (size
/// `0x18`, the remaining three arguments, two zero words); when clear, the
/// classic seven-argument open routine is invoked through its slot
/// (stdcall/7, planted stub id 5) with the merged flags word. Returns the
/// open routine's answer.
export!(cdecl, rw_00e15a67(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    unsafe {
        const MODNAME: u32 = 0xF0CD2C;
        const PROCNAME: u32 = 0xF13AD8;
        const MOD_SLOT: u32 = 0xE732EC;
        const LOOKUP_SLOT: u32 = 0xE73210;
        const OPEN_SLOT: u32 = 0xE73154;
        const PARAM_LEN: u32 = 0x18;
        let mut create2 = 0u32;
        let present: u32 = callee_cdecl!(1, u32,);
        if present != 0 {
            let gm = *global::<u32>(MOD_SLOT);
            let getmod: extern "stdcall" fn(u32) -> u32 =
                core::mem::transmute(gm as usize);
            let h = getmod(relocated(MODNAME));
            let lk = *global::<u32>(LOOKUP_SLOT);
            let getproc: extern "stdcall" fn(u32, u32) -> u32 =
                core::mem::transmute(lk as usize);
            create2 = getproc(h, relocated(PROCNAME));
            if create2 == 0 {
                return 0xFFFFFFFF;
            }
        }
        let again: u32 = callee_cdecl!(1, u32,);
        if again != 0 {
            let mut ex = [0u32; 6];
            ex[0] = PARAM_LEN;
            ex[1] = a5;
            ex[2] = a6;
            ex[3] = 0;
            ex[4] = a3;
            ex[5] = 0;
            let f: extern "stdcall" fn(u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(create2 as usize);
            f(a0, a1, a2, a4, ex.as_mut_ptr() as u32)
        } else {
            let op = *global::<u32>(OPEN_SLOT);
            let open: extern "stdcall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(op as usize);
            open(a0, a1, a2, a3, a4, a5 | a6, 0)
        }
    }
});
