// original: 0x00b45dc0 CEventGunShot::~CEventGunShot
use lf_checker_rt::{callee_thiscall, export, relocated};
/// Destructor for CEventGunShot::~CEventGunShot: stamp the vtable, release 1 guarded member, tail into the base destructor.
///
/// Stamps the vtable pointer, then for each guarded member slot (0x18) calls the member cleanup with the slot address when the guard word is non-zero. Forwards to the base destructor (modelled as a tail call).
export!(thiscall, rw_00b45dc0(obj: *mut u8) -> u32 {
    unsafe {
        *(obj as *mut u32) = relocated(0xead934);
        let member = *(obj.add(0x18) as *const u32);
        if member != 0 {
            callee_thiscall!(1, u32, member, (obj as u32).wrapping_add(0x18));
        }
        callee_thiscall!(9, u32, obj as u32)
    }
});
