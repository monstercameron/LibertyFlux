// original: 0x006f5a30 state_sync_update
/// Syncs the tracker from a snapshot, refreshing derived slots on the way.
///
/// When the snapshot's sequence trails the tracker's, marks every node of the
/// owned list. When its secondary sequence trails, clears slot 0x28. When its
/// primary sequence trails, re-stamps slot 0x24 from the shared tick source
/// (fast function-pointer path with a fallback). Then copies the snapshot's
/// five words over slots 0x30-0x40 and re-clears slots 0x24/0x28 whose fresh
/// sequence reads zero.
export!(thiscall, rw_006f5a30(this: u32, snap: u32) -> () {
    unsafe {
        let obj = this as *mut u32;
        let src = snap as *const u32;
        if *src.add(1) < *obj.add(0x34 / 4) {
            let mut node = *obj.add(0x44 / 4);
            while node != 0 {
                *((node + 0x24) as *mut u32) = 1;
                node = *((node + 4) as *const u32);
            }
        }
        if *src.add(3) < *obj.add(0x3c / 4) {
            *obj.add(0x28 / 4) = 0;
        }
        if *src.add(2) < *obj.add(0x38 / 4) {
            let raw = *global::<u32>(0x17ACD20);
            let tick = if raw == 0 {
                let fallback: extern "cdecl" fn() -> u32 =
                    core::mem::transmute(*global::<u32>(0xE73474));
                fallback()
            } else {
                let prime: extern "cdecl" fn() -> u32 =
                    core::mem::transmute(raw);
                prime();
                let convert: extern "cdecl" fn(*mut u32, *mut u32) -> u32 =
                    core::mem::transmute(*global::<u32>(0x17ACD00));
                let mut words = [0u32; 3];
                convert(words.as_mut_ptr().add(1), words.as_mut_ptr());
                words[0]
            };
            *obj.add(0x24 / 4) = tick.wrapping_sub(*obj.add(0x38 / 4));
        }
        let s0 = *src;
        let s1 = *src.add(1);
        let s2 = *src.add(2);
        let s3 = *src.add(3);
        let s4 = *src.add(4);
        *obj.add(0x30 / 4) = s0;
        *obj.add(0x34 / 4) = s1;
        *obj.add(0x38 / 4) = s2;
        *obj.add(0x3c / 4) = s3;
        *obj.add(0x40 / 4) = s4;
        if *obj.add(0x38 / 4) == 0 {
            *obj.add(0x24 / 4) = 0;
        }
        if *obj.add(0x3c / 4) == 0 {
            *obj.add(0x28 / 4) = 0;
        }
    }
});
