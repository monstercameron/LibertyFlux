// original: 0x0061ded0 rage::snEventGetGamerData::snEventGetGamerData
/// Construct a `snEventGetGamerData` network event object.
///
/// The original stores the vtable pointer and a self link in the object
/// header, runs two initialisers over the embedded member at offset 0x10
/// (both intercepted by the checker), marks two state words, zeroes the
/// float/union area, forwards the incoming argument to a third member
/// routine, sets the trailing size words and returns the object address.
/// The redundant word-zeroing after the double-quadword stores changes no
/// byte and is not repeated here.
export!(thiscall, rw_0061ded0(this_obj: u32, arg: u32) -> u32 {
    unsafe {
        /// Vtable installed in the object header (file VA).
        const VTABLE: u32 = 0x00FE227C;
        /// Offset of the embedded initialised member.
        const MEMBER_OFF: u32 = 0x10;
        let obj = this_obj as *mut u32;
        *obj.add(2) = 0;
        *obj.add(3) = 0;
        *obj.add(1) = this_obj;
        *obj = relocated(VTABLE);
        let member = this_obj.wrapping_add(MEMBER_OFF);
        let _: u32 = callee_thiscall!(1, u32, member);
        *obj.add(0x14) = 0xFFFF_FFFF;
        *obj.add(0x15) = 0xFFFF_FFFF;
        *obj.add(0x16) = 0;
        *obj.add(0x17) = 0;
        *obj.add(0x18) = 0;
        *obj.add(0x19) = 0;
        let _: u32 = callee_thiscall!(2, u32, member);
        let _: u32 = callee_thiscall!(3, u32, member, arg);
        *obj.add(0xA0) = 0x200;
        *obj.add(0xA1) = 0;
        this_obj
    }
});
