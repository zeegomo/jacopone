use clap::arg_enum;
use jacopone::*;

arg_enum! {
    pub enum JMode {
        CTR,
        ECB,
    }
}

impl Default for JMode {
    fn default() -> JMode {
        JMode::CTR
    }
}

impl JMode {
    pub fn to_jacopone(&self) -> Mode {
        match self {
            Self::CTR => Mode::CTR,
            Self::ECB => Mode::ECB,
        }
    }
}

arg_enum! {
    pub enum JFunction {
        Sha3,
        Sha2,
    }
}

impl Default for JFunction {
    fn default() -> JFunction {
        JFunction::Sha3
    }
}

impl JFunction {
    pub fn to_jacopone(&self) -> Function {
        match self {
            Self::Sha3 => Function::Sha3,
            Self::Sha2 => Function::Sha2,
        }
    }
}

arg_enum! {
    pub enum JScheduler {
        Dummy
    }
}

impl Default for JScheduler {
    fn default() -> JScheduler {
        JScheduler::Dummy
    }
}

impl JScheduler {
    pub fn to_jacopone(&self) -> Scheduler {
        match self {
            Self::Dummy => Scheduler::Dummy,
        }
    }
}

arg_enum! {
    pub enum JPadding {
        PKCS7
    }
}

impl Default for JPadding {
    fn default() -> JPadding {
        JPadding::PKCS7
    }
}

impl JPadding {
    pub fn to_jacopone(&self) -> Padding {
        match self {
            Self::PKCS7 => Padding::PKCS7,
        }
    }
}
