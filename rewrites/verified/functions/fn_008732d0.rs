// original: 0x008732d0 crmtObserverFunctorNoRefCount::vf3
use lf_checker_rt::{export};

/// `crmtObserverFunctorNoRefCount::vf3`: dispatch the matching observer.
///
/// Scans the strided entry array at +0x10 (counted by the halfword at
/// +0x14) for the first entry whose tag word equals the key's tag, then
/// invokes its callback (cdecl/3) with the entry, the key and this.
/// An empty array or no match ends the call silently. EAX is left over
/// from the entry state or the callback, so no return channel.
export!(thiscall, rw_008732d0(this_: u32, key: u32) -> () {
    unsafe {
        let count = *((this_.wrapping_add(0x14)) as *const u16) as u32;
        if (count as i32) <= 0 {
            return;
        }
        let tag = *(key as *const u32);
        let mut entry = *((this_.wrapping_add(0x10)) as *const u32);
        let mut i = 0u32;
        loop {
            if *(entry as *const u32) == tag {
                break;
            }
            i += 1;
            entry = entry.wrapping_add(0x14);
            if i >= count {
                return;
            }
        }
        let callback: extern "cdecl" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(*((entry.wrapping_add(0x10)) as *const u32));
        callback(entry.wrapping_add(4), key, this_);
    }
});
