#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - flag indicate reset source"]
    pub reset_flag: RESET_FLAG,
    #[doc = "0x04 - reset source status"]
    pub reset_status: RESET_STATUS,
    #[doc = "0x08 - reset hold attribute"]
    pub reset_hold: RESET_HOLD,
    #[doc = "0x0c - reset source enable"]
    pub reset_enable: RESET_ENABLE,
    #[doc = "0x10 - reset type triggered by reset"]
    pub reset_hot: RESET_HOT,
    #[doc = "0x14 - reset type attribute"]
    pub reset_cold: RESET_COLD,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Software reset counter"]
    pub software_reset: SOFTWARE_RESET,
}
#[doc = "RESET_FLAG (rw) register accessor: an alias for `Reg<RESET_FLAG_SPEC>`"]
pub type RESET_FLAG = crate::Reg<reset_flag::RESET_FLAG_SPEC>;
#[doc = "flag indicate reset source"]
pub mod reset_flag;
#[doc = "RESET_STATUS (rw) register accessor: an alias for `Reg<RESET_STATUS_SPEC>`"]
pub type RESET_STATUS = crate::Reg<reset_status::RESET_STATUS_SPEC>;
#[doc = "reset source status"]
pub mod reset_status;
#[doc = "RESET_HOLD (rw) register accessor: an alias for `Reg<RESET_HOLD_SPEC>`"]
pub type RESET_HOLD = crate::Reg<reset_hold::RESET_HOLD_SPEC>;
#[doc = "reset hold attribute"]
pub mod reset_hold;
#[doc = "RESET_ENABLE (rw) register accessor: an alias for `Reg<RESET_ENABLE_SPEC>`"]
pub type RESET_ENABLE = crate::Reg<reset_enable::RESET_ENABLE_SPEC>;
#[doc = "reset source enable"]
pub mod reset_enable;
#[doc = "RESET_HOT (rw) register accessor: an alias for `Reg<RESET_HOT_SPEC>`"]
pub type RESET_HOT = crate::Reg<reset_hot::RESET_HOT_SPEC>;
#[doc = "reset type triggered by reset"]
pub mod reset_hot;
#[doc = "RESET_COLD (rw) register accessor: an alias for `Reg<RESET_COLD_SPEC>`"]
pub type RESET_COLD = crate::Reg<reset_cold::RESET_COLD_SPEC>;
#[doc = "reset type attribute"]
pub mod reset_cold;
#[doc = "SOFTWARE_RESET (rw) register accessor: an alias for `Reg<SOFTWARE_RESET_SPEC>`"]
pub type SOFTWARE_RESET = crate::Reg<software_reset::SOFTWARE_RESET_SPEC>;
#[doc = "Software reset counter"]
pub mod software_reset;
