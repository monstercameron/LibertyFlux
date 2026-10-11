// original: 0x00668050 rage::ptxEvent::~ptxEvent

/// Writes the relocated destructor vtable to offset +0 and releases a non-null child handle at +0x0c through the direct helper and TLS slot 0 manager vtable slot +0x0c. If bit 0 of the incoming flags word is set, it also releases this. It returns this; the proof observes the vtable write, helper and manager calls, return, stack adjustment, and faults.
lf_checker_rt::export!(thiscall, rw_00668050(this: u32, flags: u32) -> u32 {
    const CHILD_HANDLE: u32 = 0x0c;
    const RELEASE_SLOT: u32 = 0x0c;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
    let child = unsafe { read_u32(this.wrapping_add(CHILD_HANDLE)) };
    unsafe { write_u32(this, lf_checker_rt::relocated(0x00FE35B4)) };
    let tls_root = lf_checker_rt::tls_slot(0);
    let manager = unsafe { read_u32(tls_root.wrapping_add(0x08)) };
    let manager_vtable = unsafe { read_u32(manager) };
    let target = unsafe { read_u32(manager_vtable.wrapping_add(RELEASE_SLOT)) };
    let release: extern "thiscall" fn(u32, u32) -> u32 = unsafe { core::mem::transmute(target as usize) };
    if child != 0 {
        let _ignored_helper_result = lf_checker_rt::callee_thiscall!(1, u32, child);
        let _ignored_result = release(manager, child);
    }
    if flags & 1 != 0 {
        let _ignored_result = release(manager, this);
    }
    this
});
