// original: 0x00e16940 quicktime_resolve_components
/// Resolve the three media-component entry points and store them.
///
/// Looks each of the three fixed names up through the loader routine reached
/// via the module slot (stdcall/2, planted stub id 1), passing the stored
/// module key and the relocated name pointer, and stores each answer in its
/// module global. The first failure aborts with the fail code; when all three
/// resolve, the ready flag is cleared and 0 is returned. Only the low 16
/// bits of the result are defined.
export!(cdecl, rw_00e16940() -> u16 {
    unsafe {
        const LOOKUP_SLOT: u32 = 0xE73210;
        const KEY_GLOBAL: u32 = 0x17AC450;
        const OUT_A: u32 = 0x1059528;
        const OUT_B: u32 = 0x1059524;
        const OUT_C: u32 = 0x1059520;
        const READY_FLAG: u32 = 0x1059514;
        const NAME_A: u32 = 0xF13B30;
        const NAME_B: u32 = 0xF13B20;
        const NAME_C: u32 = 0xF13B08;
        const FAIL_CODE: u16 = 0xF7D2;
        let target = *global::<u32>(LOOKUP_SLOT);
        let lookup: extern "stdcall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let a = lookup(*global::<u32>(KEY_GLOBAL), relocated(NAME_A));
        *global::<u32>(OUT_A) = a;
        if a == 0 {
            return FAIL_CODE;
        }
        let b = lookup(*global::<u32>(KEY_GLOBAL), relocated(NAME_B));
        *global::<u32>(OUT_B) = b;
        if b == 0 {
            return FAIL_CODE;
        }
        let c = lookup(*global::<u32>(KEY_GLOBAL), relocated(NAME_C));
        *global::<u32>(OUT_C) = c;
        if c == 0 {
            return FAIL_CODE;
        }
        *global::<u32>(READY_FLAG) = 0;
        0
    }
});
