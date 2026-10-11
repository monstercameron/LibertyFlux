// original: 0x00B3BC40 process_task_registries
/// Visit both task registries in reverse index order. The first registry
/// selects objects whose virtual predicate succeeds and whose pending byte
/// is clear; the second selects records with no active auxiliary object.
/// Selected objects are forwarded with the enabled flag. Registry storage
/// is described by four dwords: record base, byte flags, count, and stride.
/// The function has no stack arguments and does not return a semantic value.
lf_checker_rt::export!(cdecl, rw_00b3bc40() -> () {
    unsafe {
        const REGISTRY_PRIMARY: u32 = 0x018b6f1c;
        const REGISTRY_SECONDARY: u32 = 0x018b6f10;
        const OBJECT_VTABLE: u32 = 0x00;
        const OBJECT_PREDICATE_SLOT: u32 = 0x34;
        const OBJECT_PENDING: u32 = 0x219;
        const OBJECT_AUXILIARY: u32 = 0x6c;
        const AUXILIARY_FLAGS: u32 = 0x0e;
        const SKIP_FLAG: u8 = 0x80;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        for (registry_address, primary) in [
            (REGISTRY_PRIMARY, true),
            (REGISTRY_SECONDARY, false),
        ] {
            let registry = lf_checker_rt::global::<u32>(registry_address)
                .read_unaligned();
            if registry == 0 {
                continue;
            }
            let record_base = read_u32(registry);
            let flag_base = read_u32(registry.wrapping_add(4));
            let count = read_u32(registry.wrapping_add(8));
            let stride = read_u32(registry.wrapping_add(12));
            if count == 0 {
                continue;
            }

            let mut index = count.wrapping_sub(1);
            loop {
                if read_u8(flag_base.wrapping_add(index)) & SKIP_FLAG == 0 {
                    let object = record_base.wrapping_add(stride.wrapping_mul(index));
                    if object != 0 {
                        let selected = if primary {
                            let vtable = read_u32(object.wrapping_add(OBJECT_VTABLE));
                            let target = read_u32(vtable.wrapping_add(OBJECT_PREDICATE_SLOT));
                            let predicate: extern "thiscall" fn(u32) -> u32 =
                                core::mem::transmute(target as usize);
                            predicate(object) as u8 != 0
                                && read_u8(object.wrapping_add(OBJECT_PENDING)) == 0
                        } else {
                            let auxiliary = read_u32(object.wrapping_add(OBJECT_AUXILIARY));
                            auxiliary == 0
                                || read_u8(auxiliary.wrapping_add(AUXILIARY_FLAGS)) == 0
                        };
                        if selected {
                            let _ = lf_checker_rt::callee_cdecl!(2, u32, object, 1);
                        }
                    }
                }
                if index == 0 {
                    break;
                }
                index = index.wrapping_sub(1);
            }
        }
    }
});
