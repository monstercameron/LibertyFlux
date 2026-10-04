// original: 0x00952f10 resolve_scaled_metric
/// Resolve the object's scaled metric through its hook pair.
///
/// Only runs when the mode word reads 2 and the object's key matches one of
/// the two accepted keys. Each of two rounds calls the slot-40 hook twice and
/// feeds the second answer through its slot-56 hook (a null first answer
/// falls back to the object's spare field); a null round result aborts. When
/// the final header count is positive the loader runs and its header word,
/// times 20, is the result.
export!(cdecl, rw_00952f10(obj: u32) -> u32 {
    unsafe {
        if *global::<u32>(0x11D6FD4) != 2 {
            return 0;
        }
        let narrow = *((obj.wrapping_add(0x2E)) as *const i16) as i32 as u32;
        if narrow != *global::<u32>(0x12FA4F4) && narrow != *global::<u32>(0x12FA404) {
            return 0;
        }
        let round = |o: u32| -> u32 {
            let vtable = *(o as *const u32);
            let slot = *((vtable.wrapping_add(0xA0)) as *const u32);
            let hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
            if hook(o) == 0 {
                return *((o.wrapping_add(0x100)) as *const u32);
            }
            let second = hook(o);
            let vtable2 = *(second as *const u32);
            let slot2 = *((vtable2.wrapping_add(0xE0)) as *const u32);
            let tail: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot2 as usize);
            tail(second)
        };
        if round(obj) == 0 {
            return 0;
        }
        let final_ref = round(obj);
        if *((final_ref.wrapping_add(0x10)) as *const i32) <= 0 {
            return 0;
        }
        let loaded: u32 = callee_thiscall!(2, u32, obj);
        (*((loaded.wrapping_add(0x10)) as *const u32)).wrapping_mul(20)
    }
});
