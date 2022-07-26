#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - receive buffer registers and reception time stamp"]
    pub rbuf_buf0: crate::Reg<rbuf_buf0::RBUF_BUF0_SPEC>,
    #[doc = "0x04 - receive buffer registers and reception time stamp"]
    pub rbuf_buf1: crate::Reg<rbuf_buf1::RBUF_BUF1_SPEC>,
    #[doc = "0x08 - receive buffer registers and reception time stamp"]
    pub rbuf_buf2: crate::Reg<rbuf_buf2::RBUF_BUF2_SPEC>,
    #[doc = "0x0c - receive buffer registers and reception time stamp"]
    pub rbuf_buf3: crate::Reg<rbuf_buf3::RBUF_BUF3_SPEC>,
    #[doc = "0x10 - receive buffer registers and reception time stamp"]
    pub rbuf_buf4: crate::Reg<rbuf_buf4::RBUF_BUF4_SPEC>,
    #[doc = "0x14 - receive buffer registers and reception time stamp"]
    pub rbuf_buf5: crate::Reg<rbuf_buf5::RBUF_BUF5_SPEC>,
    #[doc = "0x18 - receive buffer registers and reception time stamp"]
    pub rbuf_buf6: crate::Reg<rbuf_buf6::RBUF_BUF6_SPEC>,
    #[doc = "0x1c - receive buffer registers and reception time stamp"]
    pub rbuf_buf7: crate::Reg<rbuf_buf7::RBUF_BUF7_SPEC>,
    #[doc = "0x20 - receive buffer registers and reception time stamp"]
    pub rbuf_buf8: crate::Reg<rbuf_buf8::RBUF_BUF8_SPEC>,
    #[doc = "0x24 - receive buffer registers and reception time stamp"]
    pub rbuf_buf9: crate::Reg<rbuf_buf9::RBUF_BUF9_SPEC>,
    #[doc = "0x28 - receive buffer registers and reception time stamp"]
    pub rbuf_buf10: crate::Reg<rbuf_buf10::RBUF_BUF10_SPEC>,
    #[doc = "0x2c - receive buffer registers and reception time stamp"]
    pub rbuf_buf11: crate::Reg<rbuf_buf11::RBUF_BUF11_SPEC>,
    #[doc = "0x30 - receive buffer registers and reception time stamp"]
    pub rbuf_buf12: crate::Reg<rbuf_buf12::RBUF_BUF12_SPEC>,
    #[doc = "0x34 - receive buffer registers and reception time stamp"]
    pub rbuf_buf13: crate::Reg<rbuf_buf13::RBUF_BUF13_SPEC>,
    #[doc = "0x38 - receive buffer registers and reception time stamp"]
    pub rbuf_buf14: crate::Reg<rbuf_buf14::RBUF_BUF14_SPEC>,
    #[doc = "0x3c - receive buffer registers and reception time stamp"]
    pub rbuf_buf15: crate::Reg<rbuf_buf15::RBUF_BUF15_SPEC>,
    #[doc = "0x40 - receive buffer registers and reception time stamp"]
    pub rbuf_buf16: crate::Reg<rbuf_buf16::RBUF_BUF16_SPEC>,
    #[doc = "0x44 - receive buffer registers and reception time stamp"]
    pub rbuf_buf17: crate::Reg<rbuf_buf17::RBUF_BUF17_SPEC>,
    #[doc = "0x48 - receive buffer registers and reception time stamp"]
    pub rbuf_buf18: crate::Reg<rbuf_buf18::RBUF_BUF18_SPEC>,
    #[doc = "0x4c - receive buffer registers and reception time stamp"]
    pub rbuf_buf19: crate::Reg<rbuf_buf19::RBUF_BUF19_SPEC>,
    #[doc = "0x50 - transmit buffer register"]
    pub tbuf_buf0: crate::Reg<tbuf_buf0::TBUF_BUF0_SPEC>,
    #[doc = "0x54 - transmit buffer register"]
    pub tbuf_buf1: crate::Reg<tbuf_buf1::TBUF_BUF1_SPEC>,
    #[doc = "0x58 - transmit buffer register"]
    pub tbuf_buf2: crate::Reg<tbuf_buf2::TBUF_BUF2_SPEC>,
    #[doc = "0x5c - transmit buffer register"]
    pub tbuf_buf3: crate::Reg<tbuf_buf3::TBUF_BUF3_SPEC>,
    #[doc = "0x60 - transmit buffer register"]
    pub tbuf_buf4: crate::Reg<tbuf_buf4::TBUF_BUF4_SPEC>,
    #[doc = "0x64 - transmit buffer register"]
    pub tbuf_buf5: crate::Reg<tbuf_buf5::TBUF_BUF5_SPEC>,
    #[doc = "0x68 - transmit buffer register"]
    pub tbuf_buf6: crate::Reg<tbuf_buf6::TBUF_BUF6_SPEC>,
    #[doc = "0x6c - transmit buffer register"]
    pub tbuf_buf7: crate::Reg<tbuf_buf7::TBUF_BUF7_SPEC>,
    #[doc = "0x70 - transmit buffer register"]
    pub tbuf_buf8: crate::Reg<tbuf_buf8::TBUF_BUF8_SPEC>,
    #[doc = "0x74 - transmit buffer register"]
    pub tbuf_buf9: crate::Reg<tbuf_buf9::TBUF_BUF9_SPEC>,
    #[doc = "0x78 - transmit buffer register"]
    pub tbuf_buf10: crate::Reg<tbuf_buf10::TBUF_BUF10_SPEC>,
    #[doc = "0x7c - transmit buffer register"]
    pub tbuf_buf11: crate::Reg<tbuf_buf11::TBUF_BUF11_SPEC>,
    #[doc = "0x80 - transmit buffer register"]
    pub tbuf_buf12: crate::Reg<tbuf_buf12::TBUF_BUF12_SPEC>,
    #[doc = "0x84 - transmit buffer register"]
    pub tbuf_buf13: crate::Reg<tbuf_buf13::TBUF_BUF13_SPEC>,
    #[doc = "0x88 - transmit buffer register"]
    pub tbuf_buf14: crate::Reg<tbuf_buf14::TBUF_BUF14_SPEC>,
    #[doc = "0x8c - transmit buffer register"]
    pub tbuf_buf15: crate::Reg<tbuf_buf15::TBUF_BUF15_SPEC>,
    #[doc = "0x90 - transmit buffer register"]
    pub tbuf_buf16: crate::Reg<tbuf_buf16::TBUF_BUF16_SPEC>,
    #[doc = "0x94 - transmit buffer register"]
    pub tbuf_buf17: crate::Reg<tbuf_buf17::TBUF_BUF17_SPEC>,
    #[doc = "0x98 - transmission time stamp, LSB 32bit"]
    pub tts_wrd0: crate::Reg<tts_wrd0::TTS_WRD0_SPEC>,
    #[doc = "0x9c - transmission time stamp, MSB 32bit"]
    pub tts_wrd1: crate::Reg<tts_wrd1::TTS_WRD1_SPEC>,
    #[doc = "0xa0 - config, status, command and control bits"]
    pub cmd_sta_cmd_ctrl: crate::Reg<cmd_sta_cmd_ctrl::CMD_STA_CMD_CTRL_SPEC>,
    #[doc = "0xa4 - Receive and Transmit Interrupt Enable Register RTIE"]
    pub rtie: crate::Reg<rtie::RTIE_SPEC>,
    #[doc = "0xa5 - Receive and Transmit Interrupt Flag Register RTIF (0xa5)"]
    pub rtif: crate::Reg<rtif::RTIF_SPEC>,
    #[doc = "0xa6 - ERRor INTerrupt Enable and Flag Register ERRINT"]
    pub errint: crate::Reg<errint::ERRINT_SPEC>,
    #[doc = "0xa7 - Warning Limits Register LIMIT"]
    pub limit: crate::Reg<limit::LIMIT_SPEC>,
    #[doc = "0xa8 - Bit Timing Register(Slow Speed)"]
    pub s_presc: crate::Reg<s_presc::S_PRESC_SPEC>,
    #[doc = "0xac - Bit Timing Register(Fast Speed)"]
    pub f_presc: crate::Reg<f_presc::F_PRESC_SPEC>,
    #[doc = "0xb0 - Error and Arbitration Lost Capture Register EALCAP"]
    pub ealcap: crate::Reg<ealcap::EALCAP_SPEC>,
    #[doc = "0xb1 - Transmitter Delay Compensation Register TDC"]
    pub tdc: crate::Reg<tdc::TDC_SPEC>,
    #[doc = "0xb2 - Error Counter Registers RECNT"]
    pub recnt: crate::Reg<recnt::RECNT_SPEC>,
    #[doc = "0xb3 - Error Counter Registers TECNT"]
    pub tecnt: crate::Reg<tecnt::TECNT_SPEC>,
    #[doc = "0xb4 - Acceptance Filter Control Register ACFCTRL"]
    pub acfctrl: crate::Reg<acfctrl::ACFCTRL_SPEC>,
    #[doc = "0xb5 - CiA 603 Time-Stamping TIMECFG"]
    pub timecfg: crate::Reg<timecfg::TIMECFG_SPEC>,
    #[doc = "0xb6 - Acceptance Filter Enable ACF_EN"]
    pub acf_en: crate::Reg<acf_en::ACF_EN_SPEC>,
    #[doc = "0xb8 - Acceptance CODE ACODE or ACMASK"]
    pub acf: crate::Reg<acf::ACF_SPEC>,
    #[doc = "0xbc - Version Information VER"]
    pub ver: crate::Reg<ver::VER_SPEC>,
    #[doc = "0xbe - TTCAN: TB Slot Pointer TBSLOT"]
    pub tbslot: crate::Reg<tbslot::TBSLOT_SPEC>,
    #[doc = "0xbf - TTCAN: Time Trigger Configuration TTCFG"]
    pub ttcfg: crate::Reg<ttcfg::TTCFG_SPEC>,
    #[doc = "0xc0 - TTCAN: Reference Message REF_MSG"]
    pub ref_msg: crate::Reg<ref_msg::REF_MSG_SPEC>,
    #[doc = "0xc4 - TTCAN: Trigger Configuration TRIG_CFG"]
    pub trig_cfg: crate::Reg<trig_cfg::TRIG_CFG_SPEC>,
    #[doc = "0xc6 - TTCAN: Trigger Time TT_TRIG"]
    pub tt_trig: crate::Reg<tt_trig::TT_TRIG_SPEC>,
    #[doc = "0xc8 - TTCAN: Watch Trigger Time TT_WTRIG"]
    pub tt_wtrig: crate::Reg<tt_wtrig::TT_WTRIG_SPEC>,
}
#[doc = "RBUF_BUF0 register accessor: an alias for `Reg<RBUF_BUF0_SPEC>`"]
pub type RBUF_BUF0 = crate::Reg<rbuf_buf0::RBUF_BUF0_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf0;
#[doc = "RBUF_BUF1 register accessor: an alias for `Reg<RBUF_BUF1_SPEC>`"]
pub type RBUF_BUF1 = crate::Reg<rbuf_buf1::RBUF_BUF1_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf1;
#[doc = "RBUF_BUF2 register accessor: an alias for `Reg<RBUF_BUF2_SPEC>`"]
pub type RBUF_BUF2 = crate::Reg<rbuf_buf2::RBUF_BUF2_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf2;
#[doc = "RBUF_BUF3 register accessor: an alias for `Reg<RBUF_BUF3_SPEC>`"]
pub type RBUF_BUF3 = crate::Reg<rbuf_buf3::RBUF_BUF3_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf3;
#[doc = "RBUF_BUF4 register accessor: an alias for `Reg<RBUF_BUF4_SPEC>`"]
pub type RBUF_BUF4 = crate::Reg<rbuf_buf4::RBUF_BUF4_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf4;
#[doc = "RBUF_BUF5 register accessor: an alias for `Reg<RBUF_BUF5_SPEC>`"]
pub type RBUF_BUF5 = crate::Reg<rbuf_buf5::RBUF_BUF5_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf5;
#[doc = "RBUF_BUF6 register accessor: an alias for `Reg<RBUF_BUF6_SPEC>`"]
pub type RBUF_BUF6 = crate::Reg<rbuf_buf6::RBUF_BUF6_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf6;
#[doc = "RBUF_BUF7 register accessor: an alias for `Reg<RBUF_BUF7_SPEC>`"]
pub type RBUF_BUF7 = crate::Reg<rbuf_buf7::RBUF_BUF7_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf7;
#[doc = "RBUF_BUF8 register accessor: an alias for `Reg<RBUF_BUF8_SPEC>`"]
pub type RBUF_BUF8 = crate::Reg<rbuf_buf8::RBUF_BUF8_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf8;
#[doc = "RBUF_BUF9 register accessor: an alias for `Reg<RBUF_BUF9_SPEC>`"]
pub type RBUF_BUF9 = crate::Reg<rbuf_buf9::RBUF_BUF9_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf9;
#[doc = "RBUF_BUF10 register accessor: an alias for `Reg<RBUF_BUF10_SPEC>`"]
pub type RBUF_BUF10 = crate::Reg<rbuf_buf10::RBUF_BUF10_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf10;
#[doc = "RBUF_BUF11 register accessor: an alias for `Reg<RBUF_BUF11_SPEC>`"]
pub type RBUF_BUF11 = crate::Reg<rbuf_buf11::RBUF_BUF11_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf11;
#[doc = "RBUF_BUF12 register accessor: an alias for `Reg<RBUF_BUF12_SPEC>`"]
pub type RBUF_BUF12 = crate::Reg<rbuf_buf12::RBUF_BUF12_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf12;
#[doc = "RBUF_BUF13 register accessor: an alias for `Reg<RBUF_BUF13_SPEC>`"]
pub type RBUF_BUF13 = crate::Reg<rbuf_buf13::RBUF_BUF13_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf13;
#[doc = "RBUF_BUF14 register accessor: an alias for `Reg<RBUF_BUF14_SPEC>`"]
pub type RBUF_BUF14 = crate::Reg<rbuf_buf14::RBUF_BUF14_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf14;
#[doc = "RBUF_BUF15 register accessor: an alias for `Reg<RBUF_BUF15_SPEC>`"]
pub type RBUF_BUF15 = crate::Reg<rbuf_buf15::RBUF_BUF15_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf15;
#[doc = "RBUF_BUF16 register accessor: an alias for `Reg<RBUF_BUF16_SPEC>`"]
pub type RBUF_BUF16 = crate::Reg<rbuf_buf16::RBUF_BUF16_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf16;
#[doc = "RBUF_BUF17 register accessor: an alias for `Reg<RBUF_BUF17_SPEC>`"]
pub type RBUF_BUF17 = crate::Reg<rbuf_buf17::RBUF_BUF17_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf17;
#[doc = "RBUF_BUF18 register accessor: an alias for `Reg<RBUF_BUF18_SPEC>`"]
pub type RBUF_BUF18 = crate::Reg<rbuf_buf18::RBUF_BUF18_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf18;
#[doc = "RBUF_BUF19 register accessor: an alias for `Reg<RBUF_BUF19_SPEC>`"]
pub type RBUF_BUF19 = crate::Reg<rbuf_buf19::RBUF_BUF19_SPEC>;
#[doc = "receive buffer registers and reception time stamp"]
pub mod rbuf_buf19;
#[doc = "TBUF_BUF0 register accessor: an alias for `Reg<TBUF_BUF0_SPEC>`"]
pub type TBUF_BUF0 = crate::Reg<tbuf_buf0::TBUF_BUF0_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf0;
#[doc = "TBUF_BUF1 register accessor: an alias for `Reg<TBUF_BUF1_SPEC>`"]
pub type TBUF_BUF1 = crate::Reg<tbuf_buf1::TBUF_BUF1_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf1;
#[doc = "TBUF_BUF2 register accessor: an alias for `Reg<TBUF_BUF2_SPEC>`"]
pub type TBUF_BUF2 = crate::Reg<tbuf_buf2::TBUF_BUF2_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf2;
#[doc = "TBUF_BUF3 register accessor: an alias for `Reg<TBUF_BUF3_SPEC>`"]
pub type TBUF_BUF3 = crate::Reg<tbuf_buf3::TBUF_BUF3_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf3;
#[doc = "TBUF_BUF4 register accessor: an alias for `Reg<TBUF_BUF4_SPEC>`"]
pub type TBUF_BUF4 = crate::Reg<tbuf_buf4::TBUF_BUF4_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf4;
#[doc = "TBUF_BUF5 register accessor: an alias for `Reg<TBUF_BUF5_SPEC>`"]
pub type TBUF_BUF5 = crate::Reg<tbuf_buf5::TBUF_BUF5_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf5;
#[doc = "TBUF_BUF6 register accessor: an alias for `Reg<TBUF_BUF6_SPEC>`"]
pub type TBUF_BUF6 = crate::Reg<tbuf_buf6::TBUF_BUF6_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf6;
#[doc = "TBUF_BUF7 register accessor: an alias for `Reg<TBUF_BUF7_SPEC>`"]
pub type TBUF_BUF7 = crate::Reg<tbuf_buf7::TBUF_BUF7_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf7;
#[doc = "TBUF_BUF8 register accessor: an alias for `Reg<TBUF_BUF8_SPEC>`"]
pub type TBUF_BUF8 = crate::Reg<tbuf_buf8::TBUF_BUF8_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf8;
#[doc = "TBUF_BUF9 register accessor: an alias for `Reg<TBUF_BUF9_SPEC>`"]
pub type TBUF_BUF9 = crate::Reg<tbuf_buf9::TBUF_BUF9_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf9;
#[doc = "TBUF_BUF10 register accessor: an alias for `Reg<TBUF_BUF10_SPEC>`"]
pub type TBUF_BUF10 = crate::Reg<tbuf_buf10::TBUF_BUF10_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf10;
#[doc = "TBUF_BUF11 register accessor: an alias for `Reg<TBUF_BUF11_SPEC>`"]
pub type TBUF_BUF11 = crate::Reg<tbuf_buf11::TBUF_BUF11_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf11;
#[doc = "TBUF_BUF12 register accessor: an alias for `Reg<TBUF_BUF12_SPEC>`"]
pub type TBUF_BUF12 = crate::Reg<tbuf_buf12::TBUF_BUF12_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf12;
#[doc = "TBUF_BUF13 register accessor: an alias for `Reg<TBUF_BUF13_SPEC>`"]
pub type TBUF_BUF13 = crate::Reg<tbuf_buf13::TBUF_BUF13_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf13;
#[doc = "TBUF_BUF14 register accessor: an alias for `Reg<TBUF_BUF14_SPEC>`"]
pub type TBUF_BUF14 = crate::Reg<tbuf_buf14::TBUF_BUF14_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf14;
#[doc = "TBUF_BUF15 register accessor: an alias for `Reg<TBUF_BUF15_SPEC>`"]
pub type TBUF_BUF15 = crate::Reg<tbuf_buf15::TBUF_BUF15_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf15;
#[doc = "TBUF_BUF16 register accessor: an alias for `Reg<TBUF_BUF16_SPEC>`"]
pub type TBUF_BUF16 = crate::Reg<tbuf_buf16::TBUF_BUF16_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf16;
#[doc = "TBUF_BUF17 register accessor: an alias for `Reg<TBUF_BUF17_SPEC>`"]
pub type TBUF_BUF17 = crate::Reg<tbuf_buf17::TBUF_BUF17_SPEC>;
#[doc = "transmit buffer register"]
pub mod tbuf_buf17;
#[doc = "TTS_WRD0 register accessor: an alias for `Reg<TTS_WRD0_SPEC>`"]
pub type TTS_WRD0 = crate::Reg<tts_wrd0::TTS_WRD0_SPEC>;
#[doc = "transmission time stamp, LSB 32bit"]
pub mod tts_wrd0;
#[doc = "TTS_WRD1 register accessor: an alias for `Reg<TTS_WRD1_SPEC>`"]
pub type TTS_WRD1 = crate::Reg<tts_wrd1::TTS_WRD1_SPEC>;
#[doc = "transmission time stamp, MSB 32bit"]
pub mod tts_wrd1;
#[doc = "CMD_STA_CMD_CTRL register accessor: an alias for `Reg<CMD_STA_CMD_CTRL_SPEC>`"]
pub type CMD_STA_CMD_CTRL = crate::Reg<cmd_sta_cmd_ctrl::CMD_STA_CMD_CTRL_SPEC>;
#[doc = "config, status, command and control bits"]
pub mod cmd_sta_cmd_ctrl;
#[doc = "RTIE register accessor: an alias for `Reg<RTIE_SPEC>`"]
pub type RTIE = crate::Reg<rtie::RTIE_SPEC>;
#[doc = "Receive and Transmit Interrupt Enable Register RTIE"]
pub mod rtie;
#[doc = "RTIF register accessor: an alias for `Reg<RTIF_SPEC>`"]
pub type RTIF = crate::Reg<rtif::RTIF_SPEC>;
#[doc = "Receive and Transmit Interrupt Flag Register RTIF (0xa5)"]
pub mod rtif;
#[doc = "ERRINT register accessor: an alias for `Reg<ERRINT_SPEC>`"]
pub type ERRINT = crate::Reg<errint::ERRINT_SPEC>;
#[doc = "ERRor INTerrupt Enable and Flag Register ERRINT"]
pub mod errint;
#[doc = "LIMIT register accessor: an alias for `Reg<LIMIT_SPEC>`"]
pub type LIMIT = crate::Reg<limit::LIMIT_SPEC>;
#[doc = "Warning Limits Register LIMIT"]
pub mod limit;
#[doc = "S_PRESC register accessor: an alias for `Reg<S_PRESC_SPEC>`"]
pub type S_PRESC = crate::Reg<s_presc::S_PRESC_SPEC>;
#[doc = "Bit Timing Register(Slow Speed)"]
pub mod s_presc;
#[doc = "F_PRESC register accessor: an alias for `Reg<F_PRESC_SPEC>`"]
pub type F_PRESC = crate::Reg<f_presc::F_PRESC_SPEC>;
#[doc = "Bit Timing Register(Fast Speed)"]
pub mod f_presc;
#[doc = "EALCAP register accessor: an alias for `Reg<EALCAP_SPEC>`"]
pub type EALCAP = crate::Reg<ealcap::EALCAP_SPEC>;
#[doc = "Error and Arbitration Lost Capture Register EALCAP"]
pub mod ealcap;
#[doc = "TDC register accessor: an alias for `Reg<TDC_SPEC>`"]
pub type TDC = crate::Reg<tdc::TDC_SPEC>;
#[doc = "Transmitter Delay Compensation Register TDC"]
pub mod tdc;
#[doc = "RECNT register accessor: an alias for `Reg<RECNT_SPEC>`"]
pub type RECNT = crate::Reg<recnt::RECNT_SPEC>;
#[doc = "Error Counter Registers RECNT"]
pub mod recnt;
#[doc = "TECNT register accessor: an alias for `Reg<TECNT_SPEC>`"]
pub type TECNT = crate::Reg<tecnt::TECNT_SPEC>;
#[doc = "Error Counter Registers TECNT"]
pub mod tecnt;
#[doc = "ACFCTRL register accessor: an alias for `Reg<ACFCTRL_SPEC>`"]
pub type ACFCTRL = crate::Reg<acfctrl::ACFCTRL_SPEC>;
#[doc = "Acceptance Filter Control Register ACFCTRL"]
pub mod acfctrl;
#[doc = "TIMECFG register accessor: an alias for `Reg<TIMECFG_SPEC>`"]
pub type TIMECFG = crate::Reg<timecfg::TIMECFG_SPEC>;
#[doc = "CiA 603 Time-Stamping TIMECFG"]
pub mod timecfg;
#[doc = "ACF_EN register accessor: an alias for `Reg<ACF_EN_SPEC>`"]
pub type ACF_EN = crate::Reg<acf_en::ACF_EN_SPEC>;
#[doc = "Acceptance Filter Enable ACF_EN"]
pub mod acf_en;
#[doc = "ACF register accessor: an alias for `Reg<ACF_SPEC>`"]
pub type ACF = crate::Reg<acf::ACF_SPEC>;
#[doc = "Acceptance CODE ACODE or ACMASK"]
pub mod acf;
#[doc = "VER register accessor: an alias for `Reg<VER_SPEC>`"]
pub type VER = crate::Reg<ver::VER_SPEC>;
#[doc = "Version Information VER"]
pub mod ver;
#[doc = "TBSLOT register accessor: an alias for `Reg<TBSLOT_SPEC>`"]
pub type TBSLOT = crate::Reg<tbslot::TBSLOT_SPEC>;
#[doc = "TTCAN: TB Slot Pointer TBSLOT"]
pub mod tbslot;
#[doc = "TTCFG register accessor: an alias for `Reg<TTCFG_SPEC>`"]
pub type TTCFG = crate::Reg<ttcfg::TTCFG_SPEC>;
#[doc = "TTCAN: Time Trigger Configuration TTCFG"]
pub mod ttcfg;
#[doc = "REF_MSG register accessor: an alias for `Reg<REF_MSG_SPEC>`"]
pub type REF_MSG = crate::Reg<ref_msg::REF_MSG_SPEC>;
#[doc = "TTCAN: Reference Message REF_MSG"]
pub mod ref_msg;
#[doc = "TRIG_CFG register accessor: an alias for `Reg<TRIG_CFG_SPEC>`"]
pub type TRIG_CFG = crate::Reg<trig_cfg::TRIG_CFG_SPEC>;
#[doc = "TTCAN: Trigger Configuration TRIG_CFG"]
pub mod trig_cfg;
#[doc = "TT_TRIG register accessor: an alias for `Reg<TT_TRIG_SPEC>`"]
pub type TT_TRIG = crate::Reg<tt_trig::TT_TRIG_SPEC>;
#[doc = "TTCAN: Trigger Time TT_TRIG"]
pub mod tt_trig;
#[doc = "TT_WTRIG register accessor: an alias for `Reg<TT_WTRIG_SPEC>`"]
pub type TT_WTRIG = crate::Reg<tt_wtrig::TT_WTRIG_SPEC>;
#[doc = "TTCAN: Watch Trigger Time TT_WTRIG"]
pub mod tt_wtrig;
