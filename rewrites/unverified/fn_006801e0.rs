// original: 0x006801E0 rage::crAnimation::dtor
//! Replace the animation object's vtable with the base-class table, run the
//! list cleanup helper, release each enabled non-null channel group through
//! allocator TLS slot zero's vtable slot 3, then install the final base
//! vtable. Both 16-bit flags gate their own pointer field. The destructor has
//! no specified return value.
const ANIMATION_VTABLE_VA: u32 = 0x00FE3854;
const BASE_VTABLE_VA: u32 = 0x00E86AFC;
const FIRST_GROUP_OFFSET: usize = 0x14;
const FIRST_FLAG_OFFSET: usize = 0x1a;
const SECOND_GROUP_OFFSET: usize = 0x20;
const SECOND_FLAG_OFFSET: usize = 0x26;
const ALLOCATOR_OFFSET: usize = 8;
const RELEASE_SLOT_OFFSET: usize = 12;

unsafe fn release_group(allocator_owner: *const u32, group: u32) {
    let allocator = unsafe {
        core::ptr::read_volatile(allocator_owner.add(ALLOCATOR_OFFSET / 4))
    };
    let vtable = unsafe { core::ptr::read_volatile(allocator as *const u32) };
    let release_address = unsafe {
        core::ptr::read_volatile((vtable as *const u32).add(RELEASE_SLOT_OFFSET / 4))
    };
    let release: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(release_address as usize) };
    let _release_result = release(allocator, group);
}

#[no_mangle]
pub unsafe extern "thiscall" fn fn_006801E0(animation: *mut u8) {
    unsafe {
        core::ptr::write_volatile(
            animation as *mut u32,
            lf_checker_rt::relocated(ANIMATION_VTABLE_VA),
        );
    }
    let _ = unsafe { lf_checker_rt::callee_thiscall!(1, u32, animation as u32) };

    let allocator_owner = lf_checker_rt::tls_slot(0) as *const u32;
    let second_enabled = unsafe {
        core::ptr::read_volatile((animation.add(SECOND_FLAG_OFFSET)) as *const u16) != 0
    };
    if second_enabled {
        let group = unsafe {
            core::ptr::read_volatile((animation.add(SECOND_GROUP_OFFSET)) as *const u32)
        };
        if group != 0 {
            unsafe { release_group(allocator_owner, group) };
        }
    }

    let first_enabled = unsafe {
        core::ptr::read_volatile((animation.add(FIRST_FLAG_OFFSET)) as *const u16) != 0
    };
    if first_enabled {
        let group = unsafe {
            core::ptr::read_volatile((animation.add(FIRST_GROUP_OFFSET)) as *const u32)
        };
        if group != 0 {
            unsafe { release_group(allocator_owner, group) };
        }
    }

    unsafe {
        core::ptr::write_volatile(
            animation as *mut u32,
            lf_checker_rt::relocated(BASE_VTABLE_VA),
        );
    }
}
