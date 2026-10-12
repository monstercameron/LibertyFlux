//! The verified rewrites of the instances, each behind one function of the same shape.
//!
//! Entry `i` calls the rewrite of entry `i` of `lf_core::word_copy::instances::COPIES`.

/// The rewrite of `store_float_to_1b4b330`, called once; it copies one word and returns zero.
pub fn call_0() -> u32 {
    crate::rewrites::m_00e5be20::rs188_00e5be20()
}

/// The rewrite of `store_float_to_1b4b2f4`, called once; it copies one word and returns zero.
pub fn call_1() -> u32 {
    crate::rewrites::m_00e5be40::rs188_00e5be40()
}

/// The rewrite of `store_float_to_1b4b2c8`, called once; it copies one word and returns zero.
pub fn call_2() -> u32 {
    crate::rewrites::m_00e5be60::rs188_00e5be60()
}

/// The rewrite of `store_float_to_1b4b2ec`, called once; it copies one word and returns zero.
pub fn call_3() -> u32 {
    crate::rewrites::m_00e5be80::rs188_00e5be80()
}

/// The rewrite of `copy_timing_factor_1`, called once; it copies one word and returns zero.
pub fn call_4() -> u32 {
    crate::rewrites::m_00e5fec0::rw_00e5fec0()
}

/// The rewrite of `task_float_copy_00E66FC0`, called once; it copies one word and returns zero.
pub fn call_5() -> u32 {
    crate::rewrites::m_00e66fc0::rw_00E66FC0()
}

/// The rewrite of `task_float_copy_00E67010`, called once; it copies one word and returns zero.
pub fn call_6() -> u32 {
    crate::rewrites::m_00e67010::rw_00E67010()
}

/// The rewrite of `task_float_copy_00E670C0`, called once; it copies one word and returns zero.
pub fn call_7() -> u32 {
    crate::rewrites::m_00e670c0::rw_00E670C0()
}

/// The rewrite of `float_const_copy`, called once; it copies one word and returns zero.
pub fn call_8() -> u32 {
    crate::rewrites::m_00e6af50::rw_00e6af50()
}

/// The rewrite of `veh_float_copy_50`, called once; it copies one word and returns zero.
pub fn call_9() -> u32 {
    crate::rewrites::m_00e6c750::rw_00e6c750()
}

/// The rewrite of `veh_float_copy_90`, called once; it copies one word and returns zero.
pub fn call_10() -> u32 {
    crate::rewrites::m_00e6c890::rw_00e6c890()
}

/// The call table, in the order of the instances table.
pub const CALLS: [fn() -> u32; 11] = [call_0, call_1, call_2, call_3, call_4, call_5, call_6, call_7, call_8, call_9, call_10];
