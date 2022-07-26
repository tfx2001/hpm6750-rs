#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - flag indicate reset source"]
    pub reset_flag: crate::Reg<reset_flag::RESET_FLAG_SPEC>,
    #[doc = "0x04 - reset source status"]
    pub reset_status: crate::Reg<reset_status::RESET_STATUS_SPEC>,
    #[doc = "0x08 - reset hold attribute"]
    pub reset_hold: crate::Reg<reset_hold::RESET_HOLD_SPEC>,
    #[doc = "0x0c - reset source enable"]
    pub reset_enable: crate::Reg<reset_enable::RESET_ENABLE_SPEC>,
    #[doc = "0x10 - reset type triggered by reset"]
    pub reset_hot: crate::Reg<reset_hot::RESET_HOT_SPEC>,
    #[doc = "0x14 - reset type attribute"]
    pub reset_cold: crate::Reg<reset_cold::RESET_COLD_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Software reset counter"]
    pub software_reset: crate::Reg<software_reset::SOFTWARE_RESET_SPEC>,
}
#[doc = "RESET_FLAG register accessor: an alias for `Reg<RESET_FLAG_SPEC>`"]
pub type RESET_FLAG = crate::Reg<reset_flag::RESET_FLAG_SPEC>;
#[doc = "flag indicate reset source"]
pub mod reset_flag;
#[doc = "RESET_STATUS register accessor: an alias for `Reg<RESET_STATUS_SPEC>`"]
pub type RESET_STATUS = crate::Reg<reset_status::RESET_STATUS_SPEC>;
#[doc = "reset source status"]
pub mod reset_status;
#[doc = "RESET_HOLD register accessor: an alias for `Reg<RESET_HOLD_SPEC>`"]
pub type RESET_HOLD = crate::Reg<reset_hold::RESET_HOLD_SPEC>;
#[doc = "reset hold attribute"]
pub mod reset_hold;
#[doc = "RESET_ENABLE register accessor: an alias for `Reg<RESET_ENABLE_SPEC>`"]
pub type RESET_ENABLE = crate::Reg<reset_enable::RESET_ENABLE_SPEC>;
#[doc = "reset source enable"]
pub mod reset_enable;
#[doc = "RESET_HOT register accessor: an alias for `Reg<RESET_HOT_SPEC>`"]
pub type RESET_HOT = crate::Reg<reset_hot::RESET_HOT_SPEC>;
#[doc = "reset type triggered by reset"]
pub mod reset_hot;
#[doc = "RESET_COLD register accessor: an alias for `Reg<RESET_COLD_SPEC>`"]
pub type RESET_COLD = crate::Reg<reset_cold::RESET_COLD_SPEC>;
#[doc = "reset type attribute"]
pub mod reset_cold;
#[doc = "SOFTWARE_RESET register accessor: an alias for `Reg<SOFTWARE_RESET_SPEC>`"]
pub type SOFTWARE_RESET = crate::Reg<software_reset::SOFTWARE_RESET_SPEC>;
#[doc = "Software reset counter"]
pub mod software_reset;
