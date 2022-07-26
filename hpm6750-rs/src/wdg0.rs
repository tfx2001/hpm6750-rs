#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x14 - Restart Register"]
    pub restart: crate::Reg<restart::RESTART_SPEC>,
    #[doc = "0x18 - Write Protection Register"]
    pub wren: crate::Reg<wren::WREN_SPEC>,
    #[doc = "0x1c - Status Register"]
    pub st: crate::Reg<st::ST_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "RESTART register accessor: an alias for `Reg<RESTART_SPEC>`"]
pub type RESTART = crate::Reg<restart::RESTART_SPEC>;
#[doc = "Restart Register"]
pub mod restart;
#[doc = "WREN register accessor: an alias for `Reg<WREN_SPEC>`"]
pub type WREN = crate::Reg<wren::WREN_SPEC>;
#[doc = "Write Protection Register"]
pub mod wren;
#[doc = "ST register accessor: an alias for `Reg<ST_SPEC>`"]
pub type ST = crate::Reg<st::ST_SPEC>;
#[doc = "Status Register"]
pub mod st;
