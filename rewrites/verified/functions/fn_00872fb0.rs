// original: 0x00872FB0 crmt_global_array_grow_and_append

/// Growable global dword array (base at 0x1b4af2c, count at +0x30, capacity at +0x32): when count equals capacity, raise capacity by 16, allocate the new array through the thread manager, copy the old words over, free the old array and install the new one; then hand out the slot at `base + count*4` and bump the count. Returns the new slot pointer.
///
/// Original: 0x00872FB0 (stdcall, one ignored stack word).
lf_checker_rt::export!(stdcall, rw_00872fb0(_arg: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const ALLOC_SLOT: u32 = 8;
    const ARR_BASE: u32 = 0x001b4af2c;
    const ARR_COUNT: u32 = 0x001b4af30;
    const ARR_CAP: u32 = 0x001b4af32;
    unsafe {
        let count = lf_checker_rt::global::<u16>(ARR_COUNT).read_unaligned();
        let cap = lf_checker_rt::global::<u16>(ARR_CAP).read_unaligned();
        if count != cap {
            let base = lf_checker_rt::global::<u32>(ARR_BASE).read_unaligned();
            lf_checker_rt::global::<u16>(ARR_COUNT).write_unaligned(count.wrapping_add(1));
            return base + (count as u32) * 4;
        }
        let newcap = cap.wrapping_add(0x10);
        lf_checker_rt::global::<u16>(ARR_CAP).write_unaligned(newcap);
        let tls0 = lf_checker_rt::tls_slot(0);
        let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
        let vtable = (manager as *const u32).read_unaligned();
        let target = ((vtable as *const u8).add(ALLOC_SLOT as usize) as *const u32)
            .read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let fresh = alloc(manager, (newcap as u32) * 4, 0x10, 0);
        let old = lf_checker_rt::global::<u32>(ARR_BASE).read_unaligned();
        let mut i = 0u32;
        while i < count as u32 {
            let v = ((old + i * 4) as *const u32).read_unaligned();
            ((fresh + i * 4) as *mut u32).write_unaligned(v);
            i += 1;
        }
        if old != 0 {
            let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(FREE_SLOT as usize) as *const u32)
                .read_unaligned();
            let free_it: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            free_it(manager, old);
        }
        lf_checker_rt::global::<u32>(ARR_BASE).write_unaligned(fresh);
        lf_checker_rt::global::<u16>(ARR_COUNT).write_unaligned(count.wrapping_add(1));
        fresh + (count as u32) * 4
    }
});
