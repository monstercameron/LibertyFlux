// original: 0x00672CB0 close_file_slot
//! Release a non-null indexed file-slot object through the allocator supplied
//! in TLS slot zero, then clear the table entry. The release method is vtable
//! slot 3 and receives the allocator as `this` plus the slot object as its
//! stack argument. The native function has no specified return value.
const SLOT_OBJECTS_VA: u32 = 0x019F13F0;
const ALLOCATOR_OFFSET: usize = 8;
const RELEASE_SLOT_OFFSET: usize = 12;

#[no_mangle]
pub unsafe extern "stdcall" fn fn_00672CB0(slot_index: u32) {
    let slots = lf_checker_rt::global::<u32>(SLOT_OBJECTS_VA);
    let slot_object = unsafe { core::ptr::read_volatile(slots.add(slot_index as usize)) };
    if slot_object == 0 {
        return;
    }

    let allocator_owner = lf_checker_rt::tls_slot(0) as *const u32;
    let allocator = unsafe {
        core::ptr::read_volatile(allocator_owner.add(ALLOCATOR_OFFSET / 4))
    };
    let vtable = unsafe { core::ptr::read_volatile(allocator as *const u32) };
    let release_address = unsafe {
        core::ptr::read_volatile((vtable as *const u32).add(RELEASE_SLOT_OFFSET / 4))
    };
    let release: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(release_address as usize) };
    let _release_result = release(allocator, slot_object);

    unsafe { core::ptr::write_volatile(slots.add(slot_index as usize), 0) };
}
