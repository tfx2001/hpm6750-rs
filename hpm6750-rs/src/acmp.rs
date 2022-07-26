#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configure Register"]
    pub channel_chn0_cfg: crate::Reg<channel_chn0_cfg::CHANNEL_CHN0_CFG_SPEC>,
    #[doc = "0x04 - DAC configure register"]
    pub channel_chn0_daccfg: crate::Reg<channel_chn0_daccfg::CHANNEL_CHN0_DACCFG_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Status register"]
    pub channel_chn0_sr: crate::Reg<channel_chn0_sr::CHANNEL_CHN0_SR_SPEC>,
    #[doc = "0x14 - Interrupt request enable register"]
    pub channel_chn0_irqen: crate::Reg<channel_chn0_irqen::CHANNEL_CHN0_IRQEN_SPEC>,
    #[doc = "0x18 - DMA request enable register"]
    pub channel_chn0_dmaen: crate::Reg<channel_chn0_dmaen::CHANNEL_CHN0_DMAEN_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x20 - Configure Register"]
    pub channel_chn1_cfg: crate::Reg<channel_chn1_cfg::CHANNEL_CHN1_CFG_SPEC>,
    #[doc = "0x24 - DAC configure register"]
    pub channel_chn1_daccfg: crate::Reg<channel_chn1_daccfg::CHANNEL_CHN1_DACCFG_SPEC>,
    _reserved7: [u8; 0x08],
    #[doc = "0x30 - Status register"]
    pub channel_chn1_sr: crate::Reg<channel_chn1_sr::CHANNEL_CHN1_SR_SPEC>,
    #[doc = "0x34 - Interrupt request enable register"]
    pub channel_chn1_irqen: crate::Reg<channel_chn1_irqen::CHANNEL_CHN1_IRQEN_SPEC>,
    #[doc = "0x38 - DMA request enable register"]
    pub channel_chn1_dmaen: crate::Reg<channel_chn1_dmaen::CHANNEL_CHN1_DMAEN_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x40 - Configure Register"]
    pub channel_chn2_cfg: crate::Reg<channel_chn2_cfg::CHANNEL_CHN2_CFG_SPEC>,
    #[doc = "0x44 - DAC configure register"]
    pub channel_chn2_daccfg: crate::Reg<channel_chn2_daccfg::CHANNEL_CHN2_DACCFG_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x50 - Status register"]
    pub channel_chn2_sr: crate::Reg<channel_chn2_sr::CHANNEL_CHN2_SR_SPEC>,
    #[doc = "0x54 - Interrupt request enable register"]
    pub channel_chn2_irqen: crate::Reg<channel_chn2_irqen::CHANNEL_CHN2_IRQEN_SPEC>,
    #[doc = "0x58 - DMA request enable register"]
    pub channel_chn2_dmaen: crate::Reg<channel_chn2_dmaen::CHANNEL_CHN2_DMAEN_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x60 - Configure Register"]
    pub channel_chn3_cfg: crate::Reg<channel_chn3_cfg::CHANNEL_CHN3_CFG_SPEC>,
    #[doc = "0x64 - DAC configure register"]
    pub channel_chn3_daccfg: crate::Reg<channel_chn3_daccfg::CHANNEL_CHN3_DACCFG_SPEC>,
    _reserved17: [u8; 0x08],
    #[doc = "0x70 - Status register"]
    pub channel_chn3_sr: crate::Reg<channel_chn3_sr::CHANNEL_CHN3_SR_SPEC>,
    #[doc = "0x74 - Interrupt request enable register"]
    pub channel_chn3_irqen: crate::Reg<channel_chn3_irqen::CHANNEL_CHN3_IRQEN_SPEC>,
    #[doc = "0x78 - DMA request enable register"]
    pub channel_chn3_dmaen: crate::Reg<channel_chn3_dmaen::CHANNEL_CHN3_DMAEN_SPEC>,
}
#[doc = "CHANNEL_CHN0_CFG register accessor: an alias for `Reg<CHANNEL_CHN0_CFG_SPEC>`"]
pub type CHANNEL_CHN0_CFG = crate::Reg<channel_chn0_cfg::CHANNEL_CHN0_CFG_SPEC>;
#[doc = "Configure Register"]
pub mod channel_chn0_cfg;
#[doc = "CHANNEL_CHN0_DACCFG register accessor: an alias for `Reg<CHANNEL_CHN0_DACCFG_SPEC>`"]
pub type CHANNEL_CHN0_DACCFG = crate::Reg<channel_chn0_daccfg::CHANNEL_CHN0_DACCFG_SPEC>;
#[doc = "DAC configure register"]
pub mod channel_chn0_daccfg;
#[doc = "CHANNEL_CHN0_SR register accessor: an alias for `Reg<CHANNEL_CHN0_SR_SPEC>`"]
pub type CHANNEL_CHN0_SR = crate::Reg<channel_chn0_sr::CHANNEL_CHN0_SR_SPEC>;
#[doc = "Status register"]
pub mod channel_chn0_sr;
#[doc = "CHANNEL_CHN0_IRQEN register accessor: an alias for `Reg<CHANNEL_CHN0_IRQEN_SPEC>`"]
pub type CHANNEL_CHN0_IRQEN = crate::Reg<channel_chn0_irqen::CHANNEL_CHN0_IRQEN_SPEC>;
#[doc = "Interrupt request enable register"]
pub mod channel_chn0_irqen;
#[doc = "CHANNEL_CHN0_DMAEN register accessor: an alias for `Reg<CHANNEL_CHN0_DMAEN_SPEC>`"]
pub type CHANNEL_CHN0_DMAEN = crate::Reg<channel_chn0_dmaen::CHANNEL_CHN0_DMAEN_SPEC>;
#[doc = "DMA request enable register"]
pub mod channel_chn0_dmaen;
#[doc = "CHANNEL_CHN1_CFG register accessor: an alias for `Reg<CHANNEL_CHN1_CFG_SPEC>`"]
pub type CHANNEL_CHN1_CFG = crate::Reg<channel_chn1_cfg::CHANNEL_CHN1_CFG_SPEC>;
#[doc = "Configure Register"]
pub mod channel_chn1_cfg;
#[doc = "CHANNEL_CHN1_DACCFG register accessor: an alias for `Reg<CHANNEL_CHN1_DACCFG_SPEC>`"]
pub type CHANNEL_CHN1_DACCFG = crate::Reg<channel_chn1_daccfg::CHANNEL_CHN1_DACCFG_SPEC>;
#[doc = "DAC configure register"]
pub mod channel_chn1_daccfg;
#[doc = "CHANNEL_CHN1_SR register accessor: an alias for `Reg<CHANNEL_CHN1_SR_SPEC>`"]
pub type CHANNEL_CHN1_SR = crate::Reg<channel_chn1_sr::CHANNEL_CHN1_SR_SPEC>;
#[doc = "Status register"]
pub mod channel_chn1_sr;
#[doc = "CHANNEL_CHN1_IRQEN register accessor: an alias for `Reg<CHANNEL_CHN1_IRQEN_SPEC>`"]
pub type CHANNEL_CHN1_IRQEN = crate::Reg<channel_chn1_irqen::CHANNEL_CHN1_IRQEN_SPEC>;
#[doc = "Interrupt request enable register"]
pub mod channel_chn1_irqen;
#[doc = "CHANNEL_CHN1_DMAEN register accessor: an alias for `Reg<CHANNEL_CHN1_DMAEN_SPEC>`"]
pub type CHANNEL_CHN1_DMAEN = crate::Reg<channel_chn1_dmaen::CHANNEL_CHN1_DMAEN_SPEC>;
#[doc = "DMA request enable register"]
pub mod channel_chn1_dmaen;
#[doc = "CHANNEL_CHN2_CFG register accessor: an alias for `Reg<CHANNEL_CHN2_CFG_SPEC>`"]
pub type CHANNEL_CHN2_CFG = crate::Reg<channel_chn2_cfg::CHANNEL_CHN2_CFG_SPEC>;
#[doc = "Configure Register"]
pub mod channel_chn2_cfg;
#[doc = "CHANNEL_CHN2_DACCFG register accessor: an alias for `Reg<CHANNEL_CHN2_DACCFG_SPEC>`"]
pub type CHANNEL_CHN2_DACCFG = crate::Reg<channel_chn2_daccfg::CHANNEL_CHN2_DACCFG_SPEC>;
#[doc = "DAC configure register"]
pub mod channel_chn2_daccfg;
#[doc = "CHANNEL_CHN2_SR register accessor: an alias for `Reg<CHANNEL_CHN2_SR_SPEC>`"]
pub type CHANNEL_CHN2_SR = crate::Reg<channel_chn2_sr::CHANNEL_CHN2_SR_SPEC>;
#[doc = "Status register"]
pub mod channel_chn2_sr;
#[doc = "CHANNEL_CHN2_IRQEN register accessor: an alias for `Reg<CHANNEL_CHN2_IRQEN_SPEC>`"]
pub type CHANNEL_CHN2_IRQEN = crate::Reg<channel_chn2_irqen::CHANNEL_CHN2_IRQEN_SPEC>;
#[doc = "Interrupt request enable register"]
pub mod channel_chn2_irqen;
#[doc = "CHANNEL_CHN2_DMAEN register accessor: an alias for `Reg<CHANNEL_CHN2_DMAEN_SPEC>`"]
pub type CHANNEL_CHN2_DMAEN = crate::Reg<channel_chn2_dmaen::CHANNEL_CHN2_DMAEN_SPEC>;
#[doc = "DMA request enable register"]
pub mod channel_chn2_dmaen;
#[doc = "CHANNEL_CHN3_CFG register accessor: an alias for `Reg<CHANNEL_CHN3_CFG_SPEC>`"]
pub type CHANNEL_CHN3_CFG = crate::Reg<channel_chn3_cfg::CHANNEL_CHN3_CFG_SPEC>;
#[doc = "Configure Register"]
pub mod channel_chn3_cfg;
#[doc = "CHANNEL_CHN3_DACCFG register accessor: an alias for `Reg<CHANNEL_CHN3_DACCFG_SPEC>`"]
pub type CHANNEL_CHN3_DACCFG = crate::Reg<channel_chn3_daccfg::CHANNEL_CHN3_DACCFG_SPEC>;
#[doc = "DAC configure register"]
pub mod channel_chn3_daccfg;
#[doc = "CHANNEL_CHN3_SR register accessor: an alias for `Reg<CHANNEL_CHN3_SR_SPEC>`"]
pub type CHANNEL_CHN3_SR = crate::Reg<channel_chn3_sr::CHANNEL_CHN3_SR_SPEC>;
#[doc = "Status register"]
pub mod channel_chn3_sr;
#[doc = "CHANNEL_CHN3_IRQEN register accessor: an alias for `Reg<CHANNEL_CHN3_IRQEN_SPEC>`"]
pub type CHANNEL_CHN3_IRQEN = crate::Reg<channel_chn3_irqen::CHANNEL_CHN3_IRQEN_SPEC>;
#[doc = "Interrupt request enable register"]
pub mod channel_chn3_irqen;
#[doc = "CHANNEL_CHN3_DMAEN register accessor: an alias for `Reg<CHANNEL_CHN3_DMAEN_SPEC>`"]
pub type CHANNEL_CHN3_DMAEN = crate::Reg<channel_chn3_dmaen::CHANNEL_CHN3_DMAEN_SPEC>;
#[doc = "DMA request enable register"]
pub mod channel_chn3_dmaen;
