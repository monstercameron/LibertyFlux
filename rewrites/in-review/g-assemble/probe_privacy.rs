mod kid {
    pub fn pub_f() -> u32 {
        1
    }
    fn priv_f() -> u32 {
        2
    }
}
use self::kid::*;

pub fn probe_pub() -> u32 {
    pub_f()
}

pub fn probe_priv() -> u32 {
    priv_f()
}
