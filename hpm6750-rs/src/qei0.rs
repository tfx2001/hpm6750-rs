#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Phase configure register"]
    pub phcfg: crate::Reg<phcfg::PHCFG_SPEC>,
    #[doc = "0x08 - Watchdog configure register"]
    pub wdgcfg: crate::Reg<wdgcfg::WDGCFG_SPEC>,
    #[doc = "0x0c - Phase index register"]
    pub phidx: crate::Reg<phidx::PHIDX_SPEC>,
    #[doc = "0x10 - Tigger output enable register"]
    pub trgoen: crate::Reg<trgoen::TRGOEN_SPEC>,
    #[doc = "0x14 - Read event enable register"]
    pub readen: crate::Reg<readen::READEN_SPEC>,
    #[doc = "0x18 - Z comparator"]
    pub zcmp: crate::Reg<zcmp::ZCMP_SPEC>,
    #[doc = "0x1c - Phase comparator"]
    pub phcmp: crate::Reg<phcmp::PHCMP_SPEC>,
    #[doc = "0x20 - Speed comparator"]
    pub spdcmp: crate::Reg<spdcmp::SPDCMP_SPEC>,
    #[doc = "0x24 - DMA request enable register"]
    pub dmaen: crate::Reg<dmaen::DMAEN_SPEC>,
    #[doc = "0x28 - Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x2c - Interrupt request register"]
    pub irqen: crate::Reg<irqen::IRQEN_SPEC>,
    #[doc = "0x30 - Z counter"]
    pub count_current_z: crate::Reg<count_current_z::COUNT_CURRENT_Z_SPEC>,
    #[doc = "0x34 - Phase counter"]
    pub count_current_ph: crate::Reg<count_current_ph::COUNT_CURRENT_PH_SPEC>,
    #[doc = "0x38 - Speed counter"]
    pub count_current_spd: crate::Reg<count_current_spd::COUNT_CURRENT_SPD_SPEC>,
    #[doc = "0x3c - Timer counter"]
    pub count_current_tmr: crate::Reg<count_current_tmr::COUNT_CURRENT_TMR_SPEC>,
    #[doc = "0x40 - Z counter"]
    pub count_read_z: crate::Reg<count_read_z::COUNT_READ_Z_SPEC>,
    #[doc = "0x44 - Phase counter"]
    pub count_read_ph: crate::Reg<count_read_ph::COUNT_READ_PH_SPEC>,
    #[doc = "0x48 - Speed counter"]
    pub count_read_spd: crate::Reg<count_read_spd::COUNT_READ_SPD_SPEC>,
    #[doc = "0x4c - Timer counter"]
    pub count_read_tmr: crate::Reg<count_read_tmr::COUNT_READ_TMR_SPEC>,
    #[doc = "0x50 - Z snap register"]
    pub count_snap0_z: crate::Reg<count_snap0_z::COUNT_SNAP0_Z_SPEC>,
    #[doc = "0x54 - Phase snap register"]
    pub count_snap0_ph: crate::Reg<count_snap0_ph::COUNT_SNAP0_PH_SPEC>,
    #[doc = "0x58 - Speed snap register"]
    pub count_snap0_spd: crate::Reg<count_snap0_spd::COUNT_SNAP0_SPD_SPEC>,
    #[doc = "0x5c - Timer snap register"]
    pub count_snap0_tmr: crate::Reg<count_snap0_tmr::COUNT_SNAP0_TMR_SPEC>,
    #[doc = "0x60 - Z snap register 1"]
    pub count_snap1_z: crate::Reg<count_snap1_z::COUNT_SNAP1_Z_SPEC>,
    #[doc = "0x64 - Phase snap register 1"]
    pub count_snap1_ph: crate::Reg<count_snap1_ph::COUNT_SNAP1_PH_SPEC>,
    #[doc = "0x68 - Speed snap register 1"]
    pub count_snap1_spd: crate::Reg<count_snap1_spd::COUNT_SNAP1_SPD_SPEC>,
    #[doc = "0x6c - Timer snap register 1"]
    pub count_snap1_tmr: crate::Reg<count_snap1_tmr::COUNT_SNAP1_TMR_SPEC>,
    #[doc = "0x70 - Speed history"]
    pub spdhis_spdhis0: crate::Reg<spdhis_spdhis0::SPDHIS_SPDHIS0_SPEC>,
    #[doc = "0x74 - Speed history 1"]
    pub spdhis_spdhis1: crate::Reg<spdhis_spdhis1::SPDHIS_SPDHIS1_SPEC>,
    #[doc = "0x78 - Speed history 2"]
    pub spdhis_spdhis2: crate::Reg<spdhis_spdhis2::SPDHIS_SPDHIS2_SPEC>,
    #[doc = "0x7c - Speed history 3"]
    pub spdhis_spdhis3: crate::Reg<spdhis_spdhis3::SPDHIS_SPDHIS3_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
pub mod cr;
#[doc = "PHCFG register accessor: an alias for `Reg<PHCFG_SPEC>`"]
pub type PHCFG = crate::Reg<phcfg::PHCFG_SPEC>;
#[doc = "Phase configure register"]
pub mod phcfg;
#[doc = "WDGCFG register accessor: an alias for `Reg<WDGCFG_SPEC>`"]
pub type WDGCFG = crate::Reg<wdgcfg::WDGCFG_SPEC>;
#[doc = "Watchdog configure register"]
pub mod wdgcfg;
#[doc = "PHIDX register accessor: an alias for `Reg<PHIDX_SPEC>`"]
pub type PHIDX = crate::Reg<phidx::PHIDX_SPEC>;
#[doc = "Phase index register"]
pub mod phidx;
#[doc = "TRGOEN register accessor: an alias for `Reg<TRGOEN_SPEC>`"]
pub type TRGOEN = crate::Reg<trgoen::TRGOEN_SPEC>;
#[doc = "Tigger output enable register"]
pub mod trgoen;
#[doc = "READEN register accessor: an alias for `Reg<READEN_SPEC>`"]
pub type READEN = crate::Reg<readen::READEN_SPEC>;
#[doc = "Read event enable register"]
pub mod readen;
#[doc = "ZCMP register accessor: an alias for `Reg<ZCMP_SPEC>`"]
pub type ZCMP = crate::Reg<zcmp::ZCMP_SPEC>;
#[doc = "Z comparator"]
pub mod zcmp;
#[doc = "PHCMP register accessor: an alias for `Reg<PHCMP_SPEC>`"]
pub type PHCMP = crate::Reg<phcmp::PHCMP_SPEC>;
#[doc = "Phase comparator"]
pub mod phcmp;
#[doc = "SPDCMP register accessor: an alias for `Reg<SPDCMP_SPEC>`"]
pub type SPDCMP = crate::Reg<spdcmp::SPDCMP_SPEC>;
#[doc = "Speed comparator"]
pub mod spdcmp;
#[doc = "DMAEN register accessor: an alias for `Reg<DMAEN_SPEC>`"]
pub type DMAEN = crate::Reg<dmaen::DMAEN_SPEC>;
#[doc = "DMA request enable register"]
pub mod dmaen;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IRQEN register accessor: an alias for `Reg<IRQEN_SPEC>`"]
pub type IRQEN = crate::Reg<irqen::IRQEN_SPEC>;
#[doc = "Interrupt request register"]
pub mod irqen;
#[doc = "COUNT_CURRENT_Z register accessor: an alias for `Reg<COUNT_CURRENT_Z_SPEC>`"]
pub type COUNT_CURRENT_Z = crate::Reg<count_current_z::COUNT_CURRENT_Z_SPEC>;
#[doc = "Z counter"]
pub mod count_current_z;
#[doc = "COUNT_CURRENT_PH register accessor: an alias for `Reg<COUNT_CURRENT_PH_SPEC>`"]
pub type COUNT_CURRENT_PH = crate::Reg<count_current_ph::COUNT_CURRENT_PH_SPEC>;
#[doc = "Phase counter"]
pub mod count_current_ph;
#[doc = "COUNT_CURRENT_SPD register accessor: an alias for `Reg<COUNT_CURRENT_SPD_SPEC>`"]
pub type COUNT_CURRENT_SPD = crate::Reg<count_current_spd::COUNT_CURRENT_SPD_SPEC>;
#[doc = "Speed counter"]
pub mod count_current_spd;
#[doc = "COUNT_CURRENT_TMR register accessor: an alias for `Reg<COUNT_CURRENT_TMR_SPEC>`"]
pub type COUNT_CURRENT_TMR = crate::Reg<count_current_tmr::COUNT_CURRENT_TMR_SPEC>;
#[doc = "Timer counter"]
pub mod count_current_tmr;
#[doc = "COUNT_READ_Z register accessor: an alias for `Reg<COUNT_READ_Z_SPEC>`"]
pub type COUNT_READ_Z = crate::Reg<count_read_z::COUNT_READ_Z_SPEC>;
#[doc = "Z counter"]
pub mod count_read_z;
#[doc = "COUNT_READ_PH register accessor: an alias for `Reg<COUNT_READ_PH_SPEC>`"]
pub type COUNT_READ_PH = crate::Reg<count_read_ph::COUNT_READ_PH_SPEC>;
#[doc = "Phase counter"]
pub mod count_read_ph;
#[doc = "COUNT_READ_SPD register accessor: an alias for `Reg<COUNT_READ_SPD_SPEC>`"]
pub type COUNT_READ_SPD = crate::Reg<count_read_spd::COUNT_READ_SPD_SPEC>;
#[doc = "Speed counter"]
pub mod count_read_spd;
#[doc = "COUNT_READ_TMR register accessor: an alias for `Reg<COUNT_READ_TMR_SPEC>`"]
pub type COUNT_READ_TMR = crate::Reg<count_read_tmr::COUNT_READ_TMR_SPEC>;
#[doc = "Timer counter"]
pub mod count_read_tmr;
#[doc = "COUNT_SNAP0_Z register accessor: an alias for `Reg<COUNT_SNAP0_Z_SPEC>`"]
pub type COUNT_SNAP0_Z = crate::Reg<count_snap0_z::COUNT_SNAP0_Z_SPEC>;
#[doc = "Z snap register"]
pub mod count_snap0_z;
#[doc = "COUNT_SNAP0_PH register accessor: an alias for `Reg<COUNT_SNAP0_PH_SPEC>`"]
pub type COUNT_SNAP0_PH = crate::Reg<count_snap0_ph::COUNT_SNAP0_PH_SPEC>;
#[doc = "Phase snap register"]
pub mod count_snap0_ph;
#[doc = "COUNT_SNAP0_SPD register accessor: an alias for `Reg<COUNT_SNAP0_SPD_SPEC>`"]
pub type COUNT_SNAP0_SPD = crate::Reg<count_snap0_spd::COUNT_SNAP0_SPD_SPEC>;
#[doc = "Speed snap register"]
pub mod count_snap0_spd;
#[doc = "COUNT_SNAP0_TMR register accessor: an alias for `Reg<COUNT_SNAP0_TMR_SPEC>`"]
pub type COUNT_SNAP0_TMR = crate::Reg<count_snap0_tmr::COUNT_SNAP0_TMR_SPEC>;
#[doc = "Timer snap register"]
pub mod count_snap0_tmr;
#[doc = "COUNT_SNAP1_Z register accessor: an alias for `Reg<COUNT_SNAP1_Z_SPEC>`"]
pub type COUNT_SNAP1_Z = crate::Reg<count_snap1_z::COUNT_SNAP1_Z_SPEC>;
#[doc = "Z snap register 1"]
pub mod count_snap1_z;
#[doc = "COUNT_SNAP1_PH register accessor: an alias for `Reg<COUNT_SNAP1_PH_SPEC>`"]
pub type COUNT_SNAP1_PH = crate::Reg<count_snap1_ph::COUNT_SNAP1_PH_SPEC>;
#[doc = "Phase snap register 1"]
pub mod count_snap1_ph;
#[doc = "COUNT_SNAP1_SPD register accessor: an alias for `Reg<COUNT_SNAP1_SPD_SPEC>`"]
pub type COUNT_SNAP1_SPD = crate::Reg<count_snap1_spd::COUNT_SNAP1_SPD_SPEC>;
#[doc = "Speed snap register 1"]
pub mod count_snap1_spd;
#[doc = "COUNT_SNAP1_TMR register accessor: an alias for `Reg<COUNT_SNAP1_TMR_SPEC>`"]
pub type COUNT_SNAP1_TMR = crate::Reg<count_snap1_tmr::COUNT_SNAP1_TMR_SPEC>;
#[doc = "Timer snap register 1"]
pub mod count_snap1_tmr;
#[doc = "SPDHIS_SPDHIS0 register accessor: an alias for `Reg<SPDHIS_SPDHIS0_SPEC>`"]
pub type SPDHIS_SPDHIS0 = crate::Reg<spdhis_spdhis0::SPDHIS_SPDHIS0_SPEC>;
#[doc = "Speed history"]
pub mod spdhis_spdhis0;
#[doc = "SPDHIS_SPDHIS1 register accessor: an alias for `Reg<SPDHIS_SPDHIS1_SPEC>`"]
pub type SPDHIS_SPDHIS1 = crate::Reg<spdhis_spdhis1::SPDHIS_SPDHIS1_SPEC>;
#[doc = "Speed history 1"]
pub mod spdhis_spdhis1;
#[doc = "SPDHIS_SPDHIS2 register accessor: an alias for `Reg<SPDHIS_SPDHIS2_SPEC>`"]
pub type SPDHIS_SPDHIS2 = crate::Reg<spdhis_spdhis2::SPDHIS_SPDHIS2_SPEC>;
#[doc = "Speed history 2"]
pub mod spdhis_spdhis2;
#[doc = "SPDHIS_SPDHIS3 register accessor: an alias for `Reg<SPDHIS_SPDHIS3_SPEC>`"]
pub type SPDHIS_SPDHIS3 = crate::Reg<spdhis_spdhis3::SPDHIS_SPDHIS3_SPEC>;
#[doc = "Speed history 3"]
pub mod spdhis_spdhis3;
