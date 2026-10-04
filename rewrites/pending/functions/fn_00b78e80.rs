// original: 0x00b78e80 CTaskComplexMove::CTaskComplexMove
/// Construct a `CTaskComplexMove` with a float parameter.
///
/// Runs the `CTaskComplex` constructor, stores the float argument into the
/// embedded move-interface subobject at offset `0x18`, clears its flag bit
/// at `0x1c`, and installs the derived virtual tables. Returns the object.
export!(thiscall, rw_00b78e80(this: u32, arg: f32) -> u32 {
    let _: u32 = callee_thiscall!(1, u32, this);
    unsafe {
        let base = this as *mut u32;
        *base.add(0x1c / 4) &= !1;
        *base.add(0x14 / 4) = relocated(0x00EB2B98);
        *(base.add(0x18 / 4) as *mut f32) = arg;
        *base = relocated(0x00EB2D5C);
        *base.add(0x14 / 4) = relocated(0x00EB2DB4);
    }
    this
});
