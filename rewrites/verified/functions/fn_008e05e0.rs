// original: 0x008e05e0 pin_global_resource (proposed)

/// Pin the shared global resource: bump the use count, publish the current
/// resource pointer into the pinned slot, and take a reference on it.
///
/// Reads the resource from `CURRENT`, increments `USE_COUNT`, stores the
/// pointer into `PINNED_COPY`, and when it is non-null increments the
/// reference count word at `+0x0c` of the resource. Returns the resource
/// pointer (left in eax by the read). Cdecl, no arguments, no calls.
lf_checker_rt::export!(cdecl, rw_008e05e0() -> u32 {
    unsafe {
        const CURRENT: u32 = 0x01bb_5554;
        const USE_COUNT: u32 = 0x0117_64a8;
        const PINNED_COPY: u32 = 0x0117_64ac;
        const REFCOUNT_OFF: u32 = 0x0c;
        let res = lf_checker_rt::global::<u32>(CURRENT).read_unaligned();
        let uses = lf_checker_rt::global::<u32>(USE_COUNT);
        uses.write_unaligned(uses.read_unaligned().wrapping_add(1));
        lf_checker_rt::global::<u32>(PINNED_COPY).write_unaligned(res);
        if res != 0 {
            let rc = (res + REFCOUNT_OFF) as *mut u32;
            rc.write_unaligned(rc.read_unaligned().wrapping_add(1));
        }
        res
    }
});
