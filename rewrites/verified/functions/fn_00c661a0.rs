// original: 0x00c661a0 CCutsceneObject::vf18
/// Notify then hand off to the member's successor.
///
/// Takes the object pointer in ECX and one word on the stack. Issues a
/// two-party notification call carrying the object and the word, then reads
/// the member link one page-plus in. When the link is null the notification
/// answer is the result; otherwise control passes to the routine stored in
/// the member's table, called with the member and the same word, and its
/// answer is the result. The successor cleans the forwarded word, matching
/// the plain-return path's stack adjustment.
export!(thiscall, rw_00c661a0(this: u32, word: u32) -> u32 {
    unsafe {
        const MEMBER_OFF: u32 = 0x310;
        const SUCCESSOR_SLOT: u32 = 0x20;
        let first = callee_thiscall!(1, u32, this, word);
        let member = *((this.wrapping_add(MEMBER_OFF)) as *const u32);
        if member == 0 {
            return first;
        }
        let table = *(member as *const u32);
        let successor: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((table.wrapping_add(SUCCESSOR_SLOT)) as *const u32) as usize);
        successor(member, word)
    }
});
