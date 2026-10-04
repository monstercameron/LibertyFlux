// original: 0x00b45e30 CEventMeleeAction::~CEventMeleeAction
use lf_checker_rt::{callee_thiscall, export, relocated};
/// Destructor for CEventMeleeAction::~CEventMeleeAction: stamp the vtable, release 2 guarded members, tail into the base destructor.
///
/// Stamps the vtable pointer, then for each guarded member slot (0x18, 0x1c) calls the member cleanup with the slot address when the guard word is non-zero. Forwards to the base destructor (modelled as a tail call).
export!(thiscall, rw_00b45e30(obj: *mut u8) -> u32 {
    unsafe {
        *(obj as *mut u32) = relocated(0xead8d4);
        let member1 = *(obj.add(0x18) as *const u32);
        if member1 != 0 {
            callee_thiscall!(1, u32, member1, (obj as u32).wrapping_add(0x18));
        }
        let member2 = *(obj.add(0x1c) as *const u32);
        if member2 != 0 {
            callee_thiscall!(1, u32, member2, (obj as u32).wrapping_add(0x1c));
        }
        callee_thiscall!(9, u32, obj as u32)
    }
});
