// original: 0x00b45b50 event_vtable_init
use lf_checker_rt::{export, relocated};
/// Initialise an event object: stamp its vtable pointer and zero three field words.
///
/// Writes the vtable pointer at +0x00 and zeroes +0x04, +0x0c and +0x10 (+0x08 is left untouched). Returns the object pointer.
export!(thiscall, rw_00b45b50(obj: *mut u8) -> u32 {
    unsafe {
        *(obj as *mut u32) = relocated(0xead86c);
        *(obj.add(0x4) as *mut u32) = 0;
        *(obj.add(0xc) as *mut u32) = 0;
        *(obj.add(0x10) as *mut u32) = 0;
        obj as u32
    }
});
