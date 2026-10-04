// original: 0x009d0d60 locked_field_getter_3e4
/// Locked read of one field of the shared context object.
///
/// Takes the shared critical section when the lock flag is set, reads the
/// word at `this + 0x3e4`, releases the lock, and returns the word.
export!(thiscall, rw_009d0d60(this: u32) -> u32 {
    const LOCK_FLAG: u32 = 0x0103AD58;
    const LOCK_CS: u32 = 0x0129588C;
    let flag = global::<u8>(LOCK_FLAG);
    let locked = unsafe { flag.read() };
    if locked != 0 {
        callee_stdcall!(1, u32, relocated(LOCK_CS));
    }
    let guarded = unsafe { flag.read() };
    let v = unsafe { (this.wrapping_add(0x3e4) as *const u32).read() };
    if guarded != 0 {
        callee_stdcall!(2, u32, relocated(LOCK_CS));
    }
    v
});
