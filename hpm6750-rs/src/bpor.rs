#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power on cause"]
    pub por_cause: crate::Reg<por_cause::POR_CAUSE_SPEC>,
    #[doc = "0x04 - Power on select"]
    pub por_select: crate::Reg<por_select::POR_SELECT_SPEC>,
    #[doc = "0x08 - Power on reset config"]
    pub por_config: crate::Reg<por_config::POR_CONFIG_SPEC>,
    #[doc = "0x0c - Power down control"]
    pub por_control: crate::Reg<por_control::POR_CONTROL_SPEC>,
}
#[doc = "POR_CAUSE register accessor: an alias for `Reg<POR_CAUSE_SPEC>`"]
pub type POR_CAUSE = crate::Reg<por_cause::POR_CAUSE_SPEC>;
#[doc = "Power on cause"]
pub mod por_cause;
#[doc = "POR_SELECT register accessor: an alias for `Reg<POR_SELECT_SPEC>`"]
pub type POR_SELECT = crate::Reg<por_select::POR_SELECT_SPEC>;
#[doc = "Power on select"]
pub mod por_select;
#[doc = "POR_CONFIG register accessor: an alias for `Reg<POR_CONFIG_SPEC>`"]
pub type POR_CONFIG = crate::Reg<por_config::POR_CONFIG_SPEC>;
#[doc = "Power on reset config"]
pub mod por_config;
#[doc = "POR_CONTROL register accessor: an alias for `Reg<POR_CONTROL_SPEC>`"]
pub type POR_CONTROL = crate::Reg<por_control::POR_CONTROL_SPEC>;
#[doc = "Power down control"]
pub mod por_control;
