#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bandgap config"]
    pub vbg_cfg: crate::Reg<vbg_cfg::VBG_CFG_SPEC>,
    #[doc = "0x04 - LDO config"]
    pub ldo_cfg: crate::Reg<ldo_cfg::LDO_CFG_SPEC>,
    #[doc = "0x08 - On-chip 32k oscillator config"]
    pub irc32k_cfg: crate::Reg<irc32k_cfg::IRC32K_CFG_SPEC>,
    #[doc = "0x0c - XTAL 32K config"]
    pub xtal32k_cfg: crate::Reg<xtal32k_cfg::XTAL32K_CFG_SPEC>,
    #[doc = "0x10 - Clock config"]
    pub clk_cfg: crate::Reg<clk_cfg::CLK_CFG_SPEC>,
}
#[doc = "VBG_CFG register accessor: an alias for `Reg<VBG_CFG_SPEC>`"]
pub type VBG_CFG = crate::Reg<vbg_cfg::VBG_CFG_SPEC>;
#[doc = "Bandgap config"]
pub mod vbg_cfg;
#[doc = "LDO_CFG register accessor: an alias for `Reg<LDO_CFG_SPEC>`"]
pub type LDO_CFG = crate::Reg<ldo_cfg::LDO_CFG_SPEC>;
#[doc = "LDO config"]
pub mod ldo_cfg;
#[doc = "IRC32K_CFG register accessor: an alias for `Reg<IRC32K_CFG_SPEC>`"]
pub type IRC32K_CFG = crate::Reg<irc32k_cfg::IRC32K_CFG_SPEC>;
#[doc = "On-chip 32k oscillator config"]
pub mod irc32k_cfg;
#[doc = "XTAL32K_CFG register accessor: an alias for `Reg<XTAL32K_CFG_SPEC>`"]
pub type XTAL32K_CFG = crate::Reg<xtal32k_cfg::XTAL32K_CFG_SPEC>;
#[doc = "XTAL 32K config"]
pub mod xtal32k_cfg;
#[doc = "CLK_CFG register accessor: an alias for `Reg<CLK_CFG_SPEC>`"]
pub type CLK_CFG = crate::Reg<clk_cfg::CLK_CFG_SPEC>;
#[doc = "Clock config"]
pub mod clk_cfg;
