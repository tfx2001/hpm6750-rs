#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x0c - Configuration Register"]
    pub rx_cfgr: crate::Reg<rx_cfgr::RX_CFGR_SPEC>,
    #[doc = "0x10 - RX Slot Control Register"]
    pub rxslt: crate::Reg<rxslt::RXSLT_SPEC>,
    #[doc = "0x14 - HPF A Coef Register"]
    pub hpf_ma: crate::Reg<hpf_ma::HPF_MA_SPEC>,
    #[doc = "0x18 - HPF B Coef Register"]
    pub hpf_b: crate::Reg<hpf_b::HPF_B_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "RX_CFGR register accessor: an alias for `Reg<RX_CFGR_SPEC>`"]
pub type RX_CFGR = crate::Reg<rx_cfgr::RX_CFGR_SPEC>;
#[doc = "Configuration Register"]
pub mod rx_cfgr;
#[doc = "RXSLT register accessor: an alias for `Reg<RXSLT_SPEC>`"]
pub type RXSLT = crate::Reg<rxslt::RXSLT_SPEC>;
#[doc = "RX Slot Control Register"]
pub mod rxslt;
#[doc = "HPF_MA register accessor: an alias for `Reg<HPF_MA_SPEC>`"]
pub type HPF_MA = crate::Reg<hpf_ma::HPF_MA_SPEC>;
#[doc = "HPF A Coef Register"]
pub mod hpf_ma;
#[doc = "HPF_B register accessor: an alias for `Reg<HPF_B_SPEC>`"]
pub type HPF_B = crate::Reg<hpf_b::HPF_B_SPEC>;
#[doc = "HPF B Coef Register"]
pub mod hpf_b;
