// original: 0x00bbb5e0 NativeImpl_CAN_PED_SHIMMY_IN_DIRECTION
/// Report whether a ped's shimmy target accepts a direction query.
///
/// Resolves the ped through the ped pool (a null resolution returns zero),
/// then asks the ped's task manager first for the slot-3 shimmy target and,
/// when that is empty, for the slot-4 one; an empty second answer also
/// returns zero. Classifies the target's inner object and continues only for
/// classes 2, 3 and 4, otherwise returning the class word with its low byte
/// cleared. Finally queries the inner object through its own function table
/// and returns that answer with its low byte cleared.
export!(cdecl, rw_00bbb5e0(handle: u32) -> u32 {
    unsafe {
        let pool = *global::<u32>(0x18B6F1C);
        let ped: u32 = callee_thiscall!(1, u32, pool, handle);
        if ped == 0 {
            return 0;
        }
        let mgr = (*((ped + 0x224) as *const u32)).wrapping_add(0x44);
        let mut task: u32 = callee_thiscall!(2, u32, mgr, 3, 0xD3);
        if task == 0 {
            let mgr2 = (*((ped + 0x224) as *const u32)).wrapping_add(0x44);
            task = callee_thiscall!(3, u32, mgr2, 4, 0xD3);
            if task == 0 {
                return 0;
            }
        }
        let inner = *((task + 8) as *const u32);
        let cls: u32 = callee_cdecl!(4, u32, inner);
        if cls != 2 && cls != 3 && cls != 4 {
            return cls & 0xFFFFFF00;
        }
        let obj = *((task + 8) as *const u32);
        if obj == 0 {
            return 0;
        }
        let vtab = *(obj as *const u32);
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vtab + 0xC) as *const u32));
        query(obj) & 0xFFFFFF00
    }
});
