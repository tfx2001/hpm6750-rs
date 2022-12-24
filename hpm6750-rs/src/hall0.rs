#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Phase configure register"]
    pub phcfg: PHCFG,
    #[doc = "0x08 - Watchdog configure register"]
    pub wdgcfg: WDGCFG,
    #[doc = "0x0c - U,V,W configure register"]
    pub uvwcfg: UVWCFG,
    #[doc = "0x10 - Trigger output enable register"]
    pub trgoen: TRGOEN,
    #[doc = "0x14 - Read event enable register"]
    pub readen: READEN,
    _reserved6: [u8; 0x0c],
    #[doc = "0x24 - DMA enable register"]
    pub dmaen: DMAEN,
    #[doc = "0x28 - Status register"]
    pub sr: SR,
    #[doc = "0x2c - Interrupt request enable register"]
    pub irqen: IRQEN,
    #[doc = "0x30 - W counter"]
    pub count_current_w: COUNT_CURRENT_W,
    #[doc = "0x34 - V counter"]
    pub count_current_v: COUNT_CURRENT_V,
    #[doc = "0x38 - U counter"]
    pub count_current_u: COUNT_CURRENT_U,
    #[doc = "0x3c - Timer counter"]
    pub count_current_tmr: COUNT_CURRENT_TMR,
    #[doc = "0x40 - W read register"]
    pub count_read_w: COUNT_READ_W,
    #[doc = "0x44 - V read register"]
    pub count_read_v: COUNT_READ_V,
    #[doc = "0x48 - U read register"]
    pub count_read_u: COUNT_READ_U,
    #[doc = "0x4c - Timer read register"]
    pub count_read_tmr: COUNT_READ_TMR,
    #[doc = "0x50 - W snap register 0"]
    pub count_snap0_w: COUNT_SNAP0_W,
    #[doc = "0x54 - V snap register 0"]
    pub count_snap0_v: COUNT_SNAP0_V,
    #[doc = "0x58 - Usnap register 0"]
    pub count_snap0_u: COUNT_SNAP0_U,
    #[doc = "0x5c - Timer snap register 0"]
    pub count_snap0_tmr: COUNT_SNAP0_TMR,
    #[doc = "0x60 - W snap register 1"]
    pub count_snap1_w: COUNT_SNAP1_W,
    #[doc = "0x64 - V snap register 1"]
    pub count_snap1_v: COUNT_SNAP1_V,
    #[doc = "0x68 - U snap register 1"]
    pub count_snap1_u: COUNT_SNAP1_U,
    #[doc = "0x6c - Timer snap register 1"]
    pub count_snap1_tmr: COUNT_SNAP1_TMR,
    #[doc = "0x70 - history register 0"]
    pub his_u_his0: HIS_U_HIS0,
    #[doc = "0x74 - history register 1"]
    pub his_u_his1: HIS_U_HIS1,
    #[doc = "0x78 - V histroy register 0"]
    pub his_v_his0: HIS_V_HIS0,
    #[doc = "0x7c - V histroy register 1"]
    pub his_v_his1: HIS_V_HIS1,
    #[doc = "0x80 - W histroy register 0"]
    pub his_w_his0: HIS_W_HIS0,
    #[doc = "0x84 - W histroy register 1"]
    pub his_w_his1: HIS_W_HIS1,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "PHCFG (rw) register accessor: an alias for `Reg<PHCFG_SPEC>`"]
pub type PHCFG = crate::Reg<phcfg::PHCFG_SPEC>;
#[doc = "Phase configure register"]
pub mod phcfg;
#[doc = "WDGCFG (rw) register accessor: an alias for `Reg<WDGCFG_SPEC>`"]
pub type WDGCFG = crate::Reg<wdgcfg::WDGCFG_SPEC>;
#[doc = "Watchdog configure register"]
pub mod wdgcfg;
#[doc = "UVWCFG (rw) register accessor: an alias for `Reg<UVWCFG_SPEC>`"]
pub type UVWCFG = crate::Reg<uvwcfg::UVWCFG_SPEC>;
#[doc = "U,V,W configure register"]
pub mod uvwcfg;
#[doc = "TRGOEN (rw) register accessor: an alias for `Reg<TRGOEN_SPEC>`"]
pub type TRGOEN = crate::Reg<trgoen::TRGOEN_SPEC>;
#[doc = "Trigger output enable register"]
pub mod trgoen;
#[doc = "READEN (rw) register accessor: an alias for `Reg<READEN_SPEC>`"]
pub type READEN = crate::Reg<readen::READEN_SPEC>;
#[doc = "Read event enable register"]
pub mod readen;
#[doc = "DMAEN (rw) register accessor: an alias for `Reg<DMAEN_SPEC>`"]
pub type DMAEN = crate::Reg<dmaen::DMAEN_SPEC>;
#[doc = "DMA enable register"]
pub mod dmaen;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IRQEN (rw) register accessor: an alias for `Reg<IRQEN_SPEC>`"]
pub type IRQEN = crate::Reg<irqen::IRQEN_SPEC>;
#[doc = "Interrupt request enable register"]
pub mod irqen;
#[doc = "COUNT_CURRENT_W (r) register accessor: an alias for `Reg<COUNT_CURRENT_W_SPEC>`"]
pub type COUNT_CURRENT_W = crate::Reg<count_current_w::COUNT_CURRENT_W_SPEC>;
#[doc = "W counter"]
pub mod count_current_w;
#[doc = "COUNT_CURRENT_V (r) register accessor: an alias for `Reg<COUNT_CURRENT_V_SPEC>`"]
pub type COUNT_CURRENT_V = crate::Reg<count_current_v::COUNT_CURRENT_V_SPEC>;
#[doc = "V counter"]
pub mod count_current_v;
#[doc = "COUNT_CURRENT_U (r) register accessor: an alias for `Reg<COUNT_CURRENT_U_SPEC>`"]
pub type COUNT_CURRENT_U = crate::Reg<count_current_u::COUNT_CURRENT_U_SPEC>;
#[doc = "U counter"]
pub mod count_current_u;
#[doc = "COUNT_CURRENT_TMR (r) register accessor: an alias for `Reg<COUNT_CURRENT_TMR_SPEC>`"]
pub type COUNT_CURRENT_TMR = crate::Reg<count_current_tmr::COUNT_CURRENT_TMR_SPEC>;
#[doc = "Timer counter"]
pub mod count_current_tmr;
#[doc = "COUNT_READ_W (r) register accessor: an alias for `Reg<COUNT_READ_W_SPEC>`"]
pub type COUNT_READ_W = crate::Reg<count_read_w::COUNT_READ_W_SPEC>;
#[doc = "W read register"]
pub mod count_read_w;
#[doc = "COUNT_READ_V (r) register accessor: an alias for `Reg<COUNT_READ_V_SPEC>`"]
pub type COUNT_READ_V = crate::Reg<count_read_v::COUNT_READ_V_SPEC>;
#[doc = "V read register"]
pub mod count_read_v;
#[doc = "COUNT_READ_U (r) register accessor: an alias for `Reg<COUNT_READ_U_SPEC>`"]
pub type COUNT_READ_U = crate::Reg<count_read_u::COUNT_READ_U_SPEC>;
#[doc = "U read register"]
pub mod count_read_u;
#[doc = "COUNT_READ_TMR (r) register accessor: an alias for `Reg<COUNT_READ_TMR_SPEC>`"]
pub type COUNT_READ_TMR = crate::Reg<count_read_tmr::COUNT_READ_TMR_SPEC>;
#[doc = "Timer read register"]
pub mod count_read_tmr;
#[doc = "COUNT_SNAP0_W (r) register accessor: an alias for `Reg<COUNT_SNAP0_W_SPEC>`"]
pub type COUNT_SNAP0_W = crate::Reg<count_snap0_w::COUNT_SNAP0_W_SPEC>;
#[doc = "W snap register 0"]
pub mod count_snap0_w;
#[doc = "COUNT_SNAP0_V (r) register accessor: an alias for `Reg<COUNT_SNAP0_V_SPEC>`"]
pub type COUNT_SNAP0_V = crate::Reg<count_snap0_v::COUNT_SNAP0_V_SPEC>;
#[doc = "V snap register 0"]
pub mod count_snap0_v;
#[doc = "COUNT_SNAP0_U (r) register accessor: an alias for `Reg<COUNT_SNAP0_U_SPEC>`"]
pub type COUNT_SNAP0_U = crate::Reg<count_snap0_u::COUNT_SNAP0_U_SPEC>;
#[doc = "Usnap register 0"]
pub mod count_snap0_u;
#[doc = "COUNT_SNAP0_TMR (r) register accessor: an alias for `Reg<COUNT_SNAP0_TMR_SPEC>`"]
pub type COUNT_SNAP0_TMR = crate::Reg<count_snap0_tmr::COUNT_SNAP0_TMR_SPEC>;
#[doc = "Timer snap register 0"]
pub mod count_snap0_tmr;
#[doc = "COUNT_SNAP1_W (r) register accessor: an alias for `Reg<COUNT_SNAP1_W_SPEC>`"]
pub type COUNT_SNAP1_W = crate::Reg<count_snap1_w::COUNT_SNAP1_W_SPEC>;
#[doc = "W snap register 1"]
pub mod count_snap1_w;
#[doc = "COUNT_SNAP1_V (r) register accessor: an alias for `Reg<COUNT_SNAP1_V_SPEC>`"]
pub type COUNT_SNAP1_V = crate::Reg<count_snap1_v::COUNT_SNAP1_V_SPEC>;
#[doc = "V snap register 1"]
pub mod count_snap1_v;
#[doc = "COUNT_SNAP1_U (r) register accessor: an alias for `Reg<COUNT_SNAP1_U_SPEC>`"]
pub type COUNT_SNAP1_U = crate::Reg<count_snap1_u::COUNT_SNAP1_U_SPEC>;
#[doc = "U snap register 1"]
pub mod count_snap1_u;
#[doc = "COUNT_SNAP1_TMR (r) register accessor: an alias for `Reg<COUNT_SNAP1_TMR_SPEC>`"]
pub type COUNT_SNAP1_TMR = crate::Reg<count_snap1_tmr::COUNT_SNAP1_TMR_SPEC>;
#[doc = "Timer snap register 1"]
pub mod count_snap1_tmr;
#[doc = "HIS_U_HIS0 (r) register accessor: an alias for `Reg<HIS_U_HIS0_SPEC>`"]
pub type HIS_U_HIS0 = crate::Reg<his_u_his0::HIS_U_HIS0_SPEC>;
#[doc = "history register 0"]
pub mod his_u_his0;
#[doc = "HIS_U_HIS1 (r) register accessor: an alias for `Reg<HIS_U_HIS1_SPEC>`"]
pub type HIS_U_HIS1 = crate::Reg<his_u_his1::HIS_U_HIS1_SPEC>;
#[doc = "history register 1"]
pub mod his_u_his1;
#[doc = "HIS_V_HIS0 (r) register accessor: an alias for `Reg<HIS_V_HIS0_SPEC>`"]
pub type HIS_V_HIS0 = crate::Reg<his_v_his0::HIS_V_HIS0_SPEC>;
#[doc = "V histroy register 0"]
pub mod his_v_his0;
#[doc = "HIS_V_HIS1 (r) register accessor: an alias for `Reg<HIS_V_HIS1_SPEC>`"]
pub type HIS_V_HIS1 = crate::Reg<his_v_his1::HIS_V_HIS1_SPEC>;
#[doc = "V histroy register 1"]
pub mod his_v_his1;
#[doc = "HIS_W_HIS0 (r) register accessor: an alias for `Reg<HIS_W_HIS0_SPEC>`"]
pub type HIS_W_HIS0 = crate::Reg<his_w_his0::HIS_W_HIS0_SPEC>;
#[doc = "W histroy register 0"]
pub mod his_w_his0;
#[doc = "HIS_W_HIS1 (r) register accessor: an alias for `Reg<HIS_W_HIS1_SPEC>`"]
pub type HIS_W_HIS1 = crate::Reg<his_w_his1::HIS_W_HIS1_SPEC>;
#[doc = "W histroy register 1"]
pub mod his_w_his1;
