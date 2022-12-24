#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Filter configure register"]
    pub filtcfg_pwm_in0: FILTCFG_PWM_IN0,
    #[doc = "0x04 - Filter configure register"]
    pub filtcfg_pwm_in1: FILTCFG_PWM_IN1,
    #[doc = "0x08 - Filter configure register"]
    pub filtcfg_pwm_in2: FILTCFG_PWM_IN2,
    #[doc = "0x0c - Filter configure register"]
    pub filtcfg_pwm_in3: FILTCFG_PWM_IN3,
    #[doc = "0x10 - Filter configure register"]
    pub filtcfg_pwm_in4: FILTCFG_PWM_IN4,
    #[doc = "0x14 - Filter configure register"]
    pub filtcfg_pwm_in5: FILTCFG_PWM_IN5,
    #[doc = "0x18 - Filter configure register"]
    pub filtcfg_pwm_in6: FILTCFG_PWM_IN6,
    #[doc = "0x1c - Filter configure register"]
    pub filtcfg_pwm_in7: FILTCFG_PWM_IN7,
    #[doc = "0x20 - Filter configure register"]
    pub filtcfg_trgm_in0: FILTCFG_TRGM_IN0,
    #[doc = "0x24 - Filter configure register"]
    pub filtcfg_trgm_in1: FILTCFG_TRGM_IN1,
    #[doc = "0x28 - Filter configure register"]
    pub filtcfg_trgm_in2: FILTCFG_TRGM_IN2,
    #[doc = "0x2c - Filter configure register"]
    pub filtcfg_trgm_in3: FILTCFG_TRGM_IN3,
    #[doc = "0x30 - Filter configure register"]
    pub filtcfg_trgm_in4: FILTCFG_TRGM_IN4,
    #[doc = "0x34 - Filter configure register"]
    pub filtcfg_trgm_in5: FILTCFG_TRGM_IN5,
    #[doc = "0x38 - Filter configure register"]
    pub filtcfg_trgm_in6: FILTCFG_TRGM_IN6,
    #[doc = "0x3c - Filter configure register"]
    pub filtcfg_trgm_in7: FILTCFG_TRGM_IN7,
    #[doc = "0x40 - Filter configure register"]
    pub filtcfg_trgm_in8: FILTCFG_TRGM_IN8,
    #[doc = "0x44 - Filter configure register"]
    pub filtcfg_trgm_in9: FILTCFG_TRGM_IN9,
    #[doc = "0x48 - Filter configure register"]
    pub filtcfg_trgm_in10: FILTCFG_TRGM_IN10,
    #[doc = "0x4c - Filter configure register"]
    pub filtcfg_trgm_in11: FILTCFG_TRGM_IN11,
    _reserved20: [u8; 0xb0],
    #[doc = "0x100 - Trigger manager output configure register"]
    pub trgocfg_trgm_out0: TRGOCFG_TRGM_OUT0,
    #[doc = "0x104 - Trigger manager output configure register"]
    pub trgocfg_trgm_out1: TRGOCFG_TRGM_OUT1,
    #[doc = "0x108 - Trigger manager output configure register"]
    pub trgocfg_trgm_out2: TRGOCFG_TRGM_OUT2,
    #[doc = "0x10c - Trigger manager output configure register"]
    pub trgocfg_trgm_out3: TRGOCFG_TRGM_OUT3,
    #[doc = "0x110 - Trigger manager output configure register"]
    pub trgocfg_trgm_out4: TRGOCFG_TRGM_OUT4,
    #[doc = "0x114 - Trigger manager output configure register"]
    pub trgocfg_trgm_out5: TRGOCFG_TRGM_OUT5,
    #[doc = "0x118 - Trigger manager output configure register"]
    pub trgocfg_trgm_out6: TRGOCFG_TRGM_OUT6,
    #[doc = "0x11c - Trigger manager output configure register"]
    pub trgocfg_trgm_out7: TRGOCFG_TRGM_OUT7,
    #[doc = "0x120 - Trigger manager output configure register"]
    pub trgocfg_trgm_out8: TRGOCFG_TRGM_OUT8,
    #[doc = "0x124 - Trigger manager output configure register"]
    pub trgocfg_trgm_out9: TRGOCFG_TRGM_OUT9,
    #[doc = "0x128 - Trigger manager output configure register"]
    pub trgocfg_trgm_out10: TRGOCFG_TRGM_OUT10,
    #[doc = "0x12c - Trigger manager output configure register"]
    pub trgocfg_trgm_out11: TRGOCFG_TRGM_OUT11,
    #[doc = "0x130 - Trigger manager output configure register"]
    pub trgocfg_trgm_outx0: TRGOCFG_TRGM_OUTX0,
    #[doc = "0x134 - Trigger manager output configure register"]
    pub trgocfg_trgm_outx1: TRGOCFG_TRGM_OUTX1,
    #[doc = "0x138 - Trigger manager output configure register"]
    pub trgocfg_pwm_synci: TRGOCFG_PWM_SYNCI,
    #[doc = "0x13c - Trigger manager output configure register"]
    pub trgocfg_pwm_frci: TRGOCFG_PWM_FRCI,
    #[doc = "0x140 - Trigger manager output configure register"]
    pub trgocfg_pwm_frcsynci: TRGOCFG_PWM_FRCSYNCI,
    #[doc = "0x144 - Trigger manager output configure register"]
    pub trgocfg_pwm_shrldsynci: TRGOCFG_PWM_SHRLDSYNCI,
    #[doc = "0x148 - Trigger manager output configure register"]
    pub trgocfg_pwm_faulti0: TRGOCFG_PWM_FAULTI0,
    #[doc = "0x14c - Trigger manager output configure register"]
    pub trgocfg_pwm_faulti1: TRGOCFG_PWM_FAULTI1,
    #[doc = "0x150 - Trigger manager output configure register"]
    pub trgocfg_pwm_faulti2: TRGOCFG_PWM_FAULTI2,
    #[doc = "0x154 - Trigger manager output configure register"]
    pub trgocfg_pwm_faulti3: TRGOCFG_PWM_FAULTI3,
    #[doc = "0x158 - Trigger manager output configure register"]
    pub trgocfg_pwm_in8: TRGOCFG_PWM_IN8,
    #[doc = "0x15c - Trigger manager output configure register"]
    pub trgocfg_pwm_in9: TRGOCFG_PWM_IN9,
    #[doc = "0x160 - Trigger manager output configure register"]
    pub trgocfg_pwm_in10: TRGOCFG_PWM_IN10,
    #[doc = "0x164 - Trigger manager output configure register"]
    pub trgocfg_pwm_in11: TRGOCFG_PWM_IN11,
    #[doc = "0x168 - Trigger manager output configure register"]
    pub trgocfg_pwm_in12: TRGOCFG_PWM_IN12,
    #[doc = "0x16c - Trigger manager output configure register"]
    pub trgocfg_pwm_in13: TRGOCFG_PWM_IN13,
    #[doc = "0x170 - Trigger manager output configure register"]
    pub trgocfg_pwm_in14: TRGOCFG_PWM_IN14,
    #[doc = "0x174 - Trigger manager output configure register"]
    pub trgocfg_pwm_in15: TRGOCFG_PWM_IN15,
    #[doc = "0x178 - Trigger manager output configure register"]
    pub trgocfg_pwm_in16: TRGOCFG_PWM_IN16,
    #[doc = "0x17c - Trigger manager output configure register"]
    pub trgocfg_pwm_in17: TRGOCFG_PWM_IN17,
    #[doc = "0x180 - Trigger manager output configure register"]
    pub trgocfg_pwm_in18: TRGOCFG_PWM_IN18,
    #[doc = "0x184 - Trigger manager output configure register"]
    pub trgocfg_pwm_in19: TRGOCFG_PWM_IN19,
    #[doc = "0x188 - Trigger manager output configure register"]
    pub trgocfg_pwm_in20: TRGOCFG_PWM_IN20,
    #[doc = "0x18c - Trigger manager output configure register"]
    pub trgocfg_pwm_in21: TRGOCFG_PWM_IN21,
    #[doc = "0x190 - Trigger manager output configure register"]
    pub trgocfg_pwm_in22: TRGOCFG_PWM_IN22,
    #[doc = "0x194 - Trigger manager output configure register"]
    pub trgocfg_pwm_in23: TRGOCFG_PWM_IN23,
    #[doc = "0x198 - Trigger manager output configure register"]
    pub trgocfg_qei_a: TRGOCFG_QEI_A,
    #[doc = "0x19c - Trigger manager output configure register"]
    pub trgocfg_qei_b: TRGOCFG_QEI_B,
    #[doc = "0x1a0 - Trigger manager output configure register"]
    pub trgocfg_qei_z: TRGOCFG_QEI_Z,
    #[doc = "0x1a4 - Trigger manager output configure register"]
    pub trgocfg_qei_h: TRGOCFG_QEI_H,
    #[doc = "0x1a8 - Trigger manager output configure register"]
    pub trgocfg_qei_pause: TRGOCFG_QEI_PAUSE,
    #[doc = "0x1ac - Trigger manager output configure register"]
    pub trgocfg_qei_snapi: TRGOCFG_QEI_SNAPI,
    #[doc = "0x1b0 - Trigger manager output configure register"]
    pub trgocfg_hall_u: TRGOCFG_HALL_U,
    #[doc = "0x1b4 - Trigger manager output configure register"]
    pub trgocfg_hall_v: TRGOCFG_HALL_V,
    #[doc = "0x1b8 - Trigger manager output configure register"]
    pub trgocfg_hall_w: TRGOCFG_HALL_W,
    #[doc = "0x1bc - Trigger manager output configure register"]
    pub trgocfg_hall_snapi: TRGOCFG_HALL_SNAPI,
    #[doc = "0x1c0 - Trigger manager output configure register"]
    pub trgocfg_adc0_strgi: TRGOCFG_ADC0_STRGI,
    #[doc = "0x1c4 - Trigger manager output configure register"]
    pub trgocfg_adc1_strgi: TRGOCFG_ADC1_STRGI,
    #[doc = "0x1c8 - Trigger manager output configure register"]
    pub trgocfg_adc2_strgi: TRGOCFG_ADC2_STRGI,
    #[doc = "0x1cc - Trigger manager output configure register"]
    pub trgocfg_adc3_strgi: TRGOCFG_ADC3_STRGI,
    #[doc = "0x1d0 - Trigger manager output configure register"]
    pub trgocfg_adcx_ptrgi0a: TRGOCFG_ADCX_PTRGI0A,
    #[doc = "0x1d4 - Trigger manager output configure register"]
    pub trgocfg_adcx_ptrgi0b: TRGOCFG_ADCX_PTRGI0B,
    #[doc = "0x1d8 - Trigger manager output configure register"]
    pub trgocfg_adcx_ptrgi0c: TRGOCFG_ADCX_PTRGI0C,
    #[doc = "0x1dc - Trigger manager output configure register"]
    pub trgocfg_gptmra_synci: TRGOCFG_GPTMRA_SYNCI,
    #[doc = "0x1e0 - Trigger manager output configure register"]
    pub trgocfg_gptmra_in2: TRGOCFG_GPTMRA_IN2,
    #[doc = "0x1e4 - Trigger manager output configure register"]
    pub trgocfg_gptmra_in3: TRGOCFG_GPTMRA_IN3,
    #[doc = "0x1e8 - Trigger manager output configure register"]
    pub trgocfg_gptmrb_synci: TRGOCFG_GPTMRB_SYNCI,
    #[doc = "0x1ec - Trigger manager output configure register"]
    pub trgocfg_gptmrb_in2: TRGOCFG_GPTMRB_IN2,
    #[doc = "0x1f0 - Trigger manager output configure register"]
    pub trgocfg_gptmrb_in3: TRGOCFG_GPTMRB_IN3,
    #[doc = "0x1f4 - Trigger manager output configure register"]
    pub trgocfg_cmpx_win: TRGOCFG_CMPX_WIN,
    #[doc = "0x1f8 - Trigger manager output configure register"]
    pub trgocfg_can_ptpc0_cap: TRGOCFG_CAN_PTPC0_CAP,
    #[doc = "0x1fc - Trigger manager output configure register"]
    pub trgocfg_can_ptpc1_cap: TRGOCFG_CAN_PTPC1_CAP,
    #[doc = "0x200 - DMA request configure register"]
    pub dmacfg_0: DMACFG_0,
    #[doc = "0x204 - DMA request configure register"]
    pub dmacfg_1: DMACFG_1,
    #[doc = "0x208 - DMA request configure register"]
    pub dmacfg_2: DMACFG_2,
    #[doc = "0x20c - DMA request configure register"]
    pub dmacfg_3: DMACFG_3,
    _reserved88: [u8; 0x01f0],
    #[doc = "0x400 - General Control Register"]
    pub gcr: GCR,
}
#[doc = "FILTCFG_PWM_IN0 (rw) register accessor: an alias for `Reg<FILTCFG_PWM_IN0_SPEC>`"]
pub type FILTCFG_PWM_IN0 = crate::Reg<filtcfg_pwm_in0::FILTCFG_PWM_IN0_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_pwm_in0;
#[doc = "FILTCFG_PWM_IN1 (rw) register accessor: an alias for `Reg<FILTCFG_PWM_IN1_SPEC>`"]
pub type FILTCFG_PWM_IN1 = crate::Reg<filtcfg_pwm_in1::FILTCFG_PWM_IN1_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_pwm_in1;
#[doc = "FILTCFG_PWM_IN2 (rw) register accessor: an alias for `Reg<FILTCFG_PWM_IN2_SPEC>`"]
pub type FILTCFG_PWM_IN2 = crate::Reg<filtcfg_pwm_in2::FILTCFG_PWM_IN2_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_pwm_in2;
#[doc = "FILTCFG_PWM_IN3 (rw) register accessor: an alias for `Reg<FILTCFG_PWM_IN3_SPEC>`"]
pub type FILTCFG_PWM_IN3 = crate::Reg<filtcfg_pwm_in3::FILTCFG_PWM_IN3_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_pwm_in3;
#[doc = "FILTCFG_PWM_IN4 (rw) register accessor: an alias for `Reg<FILTCFG_PWM_IN4_SPEC>`"]
pub type FILTCFG_PWM_IN4 = crate::Reg<filtcfg_pwm_in4::FILTCFG_PWM_IN4_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_pwm_in4;
#[doc = "FILTCFG_PWM_IN5 (rw) register accessor: an alias for `Reg<FILTCFG_PWM_IN5_SPEC>`"]
pub type FILTCFG_PWM_IN5 = crate::Reg<filtcfg_pwm_in5::FILTCFG_PWM_IN5_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_pwm_in5;
#[doc = "FILTCFG_PWM_IN6 (rw) register accessor: an alias for `Reg<FILTCFG_PWM_IN6_SPEC>`"]
pub type FILTCFG_PWM_IN6 = crate::Reg<filtcfg_pwm_in6::FILTCFG_PWM_IN6_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_pwm_in6;
#[doc = "FILTCFG_PWM_IN7 (rw) register accessor: an alias for `Reg<FILTCFG_PWM_IN7_SPEC>`"]
pub type FILTCFG_PWM_IN7 = crate::Reg<filtcfg_pwm_in7::FILTCFG_PWM_IN7_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_pwm_in7;
#[doc = "FILTCFG_TRGM_IN0 (rw) register accessor: an alias for `Reg<FILTCFG_TRGM_IN0_SPEC>`"]
pub type FILTCFG_TRGM_IN0 = crate::Reg<filtcfg_trgm_in0::FILTCFG_TRGM_IN0_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_trgm_in0;
#[doc = "FILTCFG_TRGM_IN1 (rw) register accessor: an alias for `Reg<FILTCFG_TRGM_IN1_SPEC>`"]
pub type FILTCFG_TRGM_IN1 = crate::Reg<filtcfg_trgm_in1::FILTCFG_TRGM_IN1_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_trgm_in1;
#[doc = "FILTCFG_TRGM_IN2 (rw) register accessor: an alias for `Reg<FILTCFG_TRGM_IN2_SPEC>`"]
pub type FILTCFG_TRGM_IN2 = crate::Reg<filtcfg_trgm_in2::FILTCFG_TRGM_IN2_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_trgm_in2;
#[doc = "FILTCFG_TRGM_IN3 (rw) register accessor: an alias for `Reg<FILTCFG_TRGM_IN3_SPEC>`"]
pub type FILTCFG_TRGM_IN3 = crate::Reg<filtcfg_trgm_in3::FILTCFG_TRGM_IN3_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_trgm_in3;
#[doc = "FILTCFG_TRGM_IN4 (rw) register accessor: an alias for `Reg<FILTCFG_TRGM_IN4_SPEC>`"]
pub type FILTCFG_TRGM_IN4 = crate::Reg<filtcfg_trgm_in4::FILTCFG_TRGM_IN4_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_trgm_in4;
#[doc = "FILTCFG_TRGM_IN5 (rw) register accessor: an alias for `Reg<FILTCFG_TRGM_IN5_SPEC>`"]
pub type FILTCFG_TRGM_IN5 = crate::Reg<filtcfg_trgm_in5::FILTCFG_TRGM_IN5_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_trgm_in5;
#[doc = "FILTCFG_TRGM_IN6 (rw) register accessor: an alias for `Reg<FILTCFG_TRGM_IN6_SPEC>`"]
pub type FILTCFG_TRGM_IN6 = crate::Reg<filtcfg_trgm_in6::FILTCFG_TRGM_IN6_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_trgm_in6;
#[doc = "FILTCFG_TRGM_IN7 (rw) register accessor: an alias for `Reg<FILTCFG_TRGM_IN7_SPEC>`"]
pub type FILTCFG_TRGM_IN7 = crate::Reg<filtcfg_trgm_in7::FILTCFG_TRGM_IN7_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_trgm_in7;
#[doc = "FILTCFG_TRGM_IN8 (rw) register accessor: an alias for `Reg<FILTCFG_TRGM_IN8_SPEC>`"]
pub type FILTCFG_TRGM_IN8 = crate::Reg<filtcfg_trgm_in8::FILTCFG_TRGM_IN8_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_trgm_in8;
#[doc = "FILTCFG_TRGM_IN9 (rw) register accessor: an alias for `Reg<FILTCFG_TRGM_IN9_SPEC>`"]
pub type FILTCFG_TRGM_IN9 = crate::Reg<filtcfg_trgm_in9::FILTCFG_TRGM_IN9_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_trgm_in9;
#[doc = "FILTCFG_TRGM_IN10 (rw) register accessor: an alias for `Reg<FILTCFG_TRGM_IN10_SPEC>`"]
pub type FILTCFG_TRGM_IN10 = crate::Reg<filtcfg_trgm_in10::FILTCFG_TRGM_IN10_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_trgm_in10;
#[doc = "FILTCFG_TRGM_IN11 (rw) register accessor: an alias for `Reg<FILTCFG_TRGM_IN11_SPEC>`"]
pub type FILTCFG_TRGM_IN11 = crate::Reg<filtcfg_trgm_in11::FILTCFG_TRGM_IN11_SPEC>;
#[doc = "Filter configure register"]
pub mod filtcfg_trgm_in11;
#[doc = "TRGOCFG_TRGM_OUT0 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUT0_SPEC>`"]
pub type TRGOCFG_TRGM_OUT0 = crate::Reg<trgocfg_trgm_out0::TRGOCFG_TRGM_OUT0_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_out0;
#[doc = "TRGOCFG_TRGM_OUT1 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUT1_SPEC>`"]
pub type TRGOCFG_TRGM_OUT1 = crate::Reg<trgocfg_trgm_out1::TRGOCFG_TRGM_OUT1_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_out1;
#[doc = "TRGOCFG_TRGM_OUT2 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUT2_SPEC>`"]
pub type TRGOCFG_TRGM_OUT2 = crate::Reg<trgocfg_trgm_out2::TRGOCFG_TRGM_OUT2_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_out2;
#[doc = "TRGOCFG_TRGM_OUT3 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUT3_SPEC>`"]
pub type TRGOCFG_TRGM_OUT3 = crate::Reg<trgocfg_trgm_out3::TRGOCFG_TRGM_OUT3_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_out3;
#[doc = "TRGOCFG_TRGM_OUT4 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUT4_SPEC>`"]
pub type TRGOCFG_TRGM_OUT4 = crate::Reg<trgocfg_trgm_out4::TRGOCFG_TRGM_OUT4_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_out4;
#[doc = "TRGOCFG_TRGM_OUT5 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUT5_SPEC>`"]
pub type TRGOCFG_TRGM_OUT5 = crate::Reg<trgocfg_trgm_out5::TRGOCFG_TRGM_OUT5_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_out5;
#[doc = "TRGOCFG_TRGM_OUT6 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUT6_SPEC>`"]
pub type TRGOCFG_TRGM_OUT6 = crate::Reg<trgocfg_trgm_out6::TRGOCFG_TRGM_OUT6_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_out6;
#[doc = "TRGOCFG_TRGM_OUT7 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUT7_SPEC>`"]
pub type TRGOCFG_TRGM_OUT7 = crate::Reg<trgocfg_trgm_out7::TRGOCFG_TRGM_OUT7_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_out7;
#[doc = "TRGOCFG_TRGM_OUT8 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUT8_SPEC>`"]
pub type TRGOCFG_TRGM_OUT8 = crate::Reg<trgocfg_trgm_out8::TRGOCFG_TRGM_OUT8_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_out8;
#[doc = "TRGOCFG_TRGM_OUT9 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUT9_SPEC>`"]
pub type TRGOCFG_TRGM_OUT9 = crate::Reg<trgocfg_trgm_out9::TRGOCFG_TRGM_OUT9_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_out9;
#[doc = "TRGOCFG_TRGM_OUT10 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUT10_SPEC>`"]
pub type TRGOCFG_TRGM_OUT10 = crate::Reg<trgocfg_trgm_out10::TRGOCFG_TRGM_OUT10_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_out10;
#[doc = "TRGOCFG_TRGM_OUT11 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUT11_SPEC>`"]
pub type TRGOCFG_TRGM_OUT11 = crate::Reg<trgocfg_trgm_out11::TRGOCFG_TRGM_OUT11_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_out11;
#[doc = "TRGOCFG_TRGM_OUTX0 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUTX0_SPEC>`"]
pub type TRGOCFG_TRGM_OUTX0 = crate::Reg<trgocfg_trgm_outx0::TRGOCFG_TRGM_OUTX0_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_outx0;
#[doc = "TRGOCFG_TRGM_OUTX1 (rw) register accessor: an alias for `Reg<TRGOCFG_TRGM_OUTX1_SPEC>`"]
pub type TRGOCFG_TRGM_OUTX1 = crate::Reg<trgocfg_trgm_outx1::TRGOCFG_TRGM_OUTX1_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_trgm_outx1;
#[doc = "TRGOCFG_PWM_SYNCI (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_SYNCI_SPEC>`"]
pub type TRGOCFG_PWM_SYNCI = crate::Reg<trgocfg_pwm_synci::TRGOCFG_PWM_SYNCI_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_synci;
#[doc = "TRGOCFG_PWM_FRCI (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_FRCI_SPEC>`"]
pub type TRGOCFG_PWM_FRCI = crate::Reg<trgocfg_pwm_frci::TRGOCFG_PWM_FRCI_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_frci;
#[doc = "TRGOCFG_PWM_FRCSYNCI (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_FRCSYNCI_SPEC>`"]
pub type TRGOCFG_PWM_FRCSYNCI = crate::Reg<trgocfg_pwm_frcsynci::TRGOCFG_PWM_FRCSYNCI_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_frcsynci;
#[doc = "TRGOCFG_PWM_SHRLDSYNCI (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_SHRLDSYNCI_SPEC>`"]
pub type TRGOCFG_PWM_SHRLDSYNCI = crate::Reg<trgocfg_pwm_shrldsynci::TRGOCFG_PWM_SHRLDSYNCI_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_shrldsynci;
#[doc = "TRGOCFG_PWM_FAULTI0 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_FAULTI0_SPEC>`"]
pub type TRGOCFG_PWM_FAULTI0 = crate::Reg<trgocfg_pwm_faulti0::TRGOCFG_PWM_FAULTI0_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_faulti0;
#[doc = "TRGOCFG_PWM_FAULTI1 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_FAULTI1_SPEC>`"]
pub type TRGOCFG_PWM_FAULTI1 = crate::Reg<trgocfg_pwm_faulti1::TRGOCFG_PWM_FAULTI1_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_faulti1;
#[doc = "TRGOCFG_PWM_FAULTI2 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_FAULTI2_SPEC>`"]
pub type TRGOCFG_PWM_FAULTI2 = crate::Reg<trgocfg_pwm_faulti2::TRGOCFG_PWM_FAULTI2_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_faulti2;
#[doc = "TRGOCFG_PWM_FAULTI3 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_FAULTI3_SPEC>`"]
pub type TRGOCFG_PWM_FAULTI3 = crate::Reg<trgocfg_pwm_faulti3::TRGOCFG_PWM_FAULTI3_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_faulti3;
#[doc = "TRGOCFG_PWM_IN8 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN8_SPEC>`"]
pub type TRGOCFG_PWM_IN8 = crate::Reg<trgocfg_pwm_in8::TRGOCFG_PWM_IN8_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in8;
#[doc = "TRGOCFG_PWM_IN9 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN9_SPEC>`"]
pub type TRGOCFG_PWM_IN9 = crate::Reg<trgocfg_pwm_in9::TRGOCFG_PWM_IN9_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in9;
#[doc = "TRGOCFG_PWM_IN10 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN10_SPEC>`"]
pub type TRGOCFG_PWM_IN10 = crate::Reg<trgocfg_pwm_in10::TRGOCFG_PWM_IN10_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in10;
#[doc = "TRGOCFG_PWM_IN11 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN11_SPEC>`"]
pub type TRGOCFG_PWM_IN11 = crate::Reg<trgocfg_pwm_in11::TRGOCFG_PWM_IN11_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in11;
#[doc = "TRGOCFG_PWM_IN12 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN12_SPEC>`"]
pub type TRGOCFG_PWM_IN12 = crate::Reg<trgocfg_pwm_in12::TRGOCFG_PWM_IN12_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in12;
#[doc = "TRGOCFG_PWM_IN13 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN13_SPEC>`"]
pub type TRGOCFG_PWM_IN13 = crate::Reg<trgocfg_pwm_in13::TRGOCFG_PWM_IN13_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in13;
#[doc = "TRGOCFG_PWM_IN14 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN14_SPEC>`"]
pub type TRGOCFG_PWM_IN14 = crate::Reg<trgocfg_pwm_in14::TRGOCFG_PWM_IN14_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in14;
#[doc = "TRGOCFG_PWM_IN15 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN15_SPEC>`"]
pub type TRGOCFG_PWM_IN15 = crate::Reg<trgocfg_pwm_in15::TRGOCFG_PWM_IN15_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in15;
#[doc = "TRGOCFG_PWM_IN16 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN16_SPEC>`"]
pub type TRGOCFG_PWM_IN16 = crate::Reg<trgocfg_pwm_in16::TRGOCFG_PWM_IN16_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in16;
#[doc = "TRGOCFG_PWM_IN17 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN17_SPEC>`"]
pub type TRGOCFG_PWM_IN17 = crate::Reg<trgocfg_pwm_in17::TRGOCFG_PWM_IN17_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in17;
#[doc = "TRGOCFG_PWM_IN18 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN18_SPEC>`"]
pub type TRGOCFG_PWM_IN18 = crate::Reg<trgocfg_pwm_in18::TRGOCFG_PWM_IN18_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in18;
#[doc = "TRGOCFG_PWM_IN19 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN19_SPEC>`"]
pub type TRGOCFG_PWM_IN19 = crate::Reg<trgocfg_pwm_in19::TRGOCFG_PWM_IN19_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in19;
#[doc = "TRGOCFG_PWM_IN20 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN20_SPEC>`"]
pub type TRGOCFG_PWM_IN20 = crate::Reg<trgocfg_pwm_in20::TRGOCFG_PWM_IN20_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in20;
#[doc = "TRGOCFG_PWM_IN21 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN21_SPEC>`"]
pub type TRGOCFG_PWM_IN21 = crate::Reg<trgocfg_pwm_in21::TRGOCFG_PWM_IN21_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in21;
#[doc = "TRGOCFG_PWM_IN22 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN22_SPEC>`"]
pub type TRGOCFG_PWM_IN22 = crate::Reg<trgocfg_pwm_in22::TRGOCFG_PWM_IN22_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in22;
#[doc = "TRGOCFG_PWM_IN23 (rw) register accessor: an alias for `Reg<TRGOCFG_PWM_IN23_SPEC>`"]
pub type TRGOCFG_PWM_IN23 = crate::Reg<trgocfg_pwm_in23::TRGOCFG_PWM_IN23_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_pwm_in23;
#[doc = "TRGOCFG_QEI_A (rw) register accessor: an alias for `Reg<TRGOCFG_QEI_A_SPEC>`"]
pub type TRGOCFG_QEI_A = crate::Reg<trgocfg_qei_a::TRGOCFG_QEI_A_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_qei_a;
#[doc = "TRGOCFG_QEI_B (rw) register accessor: an alias for `Reg<TRGOCFG_QEI_B_SPEC>`"]
pub type TRGOCFG_QEI_B = crate::Reg<trgocfg_qei_b::TRGOCFG_QEI_B_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_qei_b;
#[doc = "TRGOCFG_QEI_Z (rw) register accessor: an alias for `Reg<TRGOCFG_QEI_Z_SPEC>`"]
pub type TRGOCFG_QEI_Z = crate::Reg<trgocfg_qei_z::TRGOCFG_QEI_Z_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_qei_z;
#[doc = "TRGOCFG_QEI_H (rw) register accessor: an alias for `Reg<TRGOCFG_QEI_H_SPEC>`"]
pub type TRGOCFG_QEI_H = crate::Reg<trgocfg_qei_h::TRGOCFG_QEI_H_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_qei_h;
#[doc = "TRGOCFG_QEI_PAUSE (rw) register accessor: an alias for `Reg<TRGOCFG_QEI_PAUSE_SPEC>`"]
pub type TRGOCFG_QEI_PAUSE = crate::Reg<trgocfg_qei_pause::TRGOCFG_QEI_PAUSE_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_qei_pause;
#[doc = "TRGOCFG_QEI_SNAPI (rw) register accessor: an alias for `Reg<TRGOCFG_QEI_SNAPI_SPEC>`"]
pub type TRGOCFG_QEI_SNAPI = crate::Reg<trgocfg_qei_snapi::TRGOCFG_QEI_SNAPI_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_qei_snapi;
#[doc = "TRGOCFG_HALL_U (rw) register accessor: an alias for `Reg<TRGOCFG_HALL_U_SPEC>`"]
pub type TRGOCFG_HALL_U = crate::Reg<trgocfg_hall_u::TRGOCFG_HALL_U_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_hall_u;
#[doc = "TRGOCFG_HALL_V (rw) register accessor: an alias for `Reg<TRGOCFG_HALL_V_SPEC>`"]
pub type TRGOCFG_HALL_V = crate::Reg<trgocfg_hall_v::TRGOCFG_HALL_V_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_hall_v;
#[doc = "TRGOCFG_HALL_W (rw) register accessor: an alias for `Reg<TRGOCFG_HALL_W_SPEC>`"]
pub type TRGOCFG_HALL_W = crate::Reg<trgocfg_hall_w::TRGOCFG_HALL_W_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_hall_w;
#[doc = "TRGOCFG_HALL_SNAPI (rw) register accessor: an alias for `Reg<TRGOCFG_HALL_SNAPI_SPEC>`"]
pub type TRGOCFG_HALL_SNAPI = crate::Reg<trgocfg_hall_snapi::TRGOCFG_HALL_SNAPI_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_hall_snapi;
#[doc = "TRGOCFG_ADC0_STRGI (rw) register accessor: an alias for `Reg<TRGOCFG_ADC0_STRGI_SPEC>`"]
pub type TRGOCFG_ADC0_STRGI = crate::Reg<trgocfg_adc0_strgi::TRGOCFG_ADC0_STRGI_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_adc0_strgi;
#[doc = "TRGOCFG_ADC1_STRGI (rw) register accessor: an alias for `Reg<TRGOCFG_ADC1_STRGI_SPEC>`"]
pub type TRGOCFG_ADC1_STRGI = crate::Reg<trgocfg_adc1_strgi::TRGOCFG_ADC1_STRGI_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_adc1_strgi;
#[doc = "TRGOCFG_ADC2_STRGI (rw) register accessor: an alias for `Reg<TRGOCFG_ADC2_STRGI_SPEC>`"]
pub type TRGOCFG_ADC2_STRGI = crate::Reg<trgocfg_adc2_strgi::TRGOCFG_ADC2_STRGI_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_adc2_strgi;
#[doc = "TRGOCFG_ADC3_STRGI (rw) register accessor: an alias for `Reg<TRGOCFG_ADC3_STRGI_SPEC>`"]
pub type TRGOCFG_ADC3_STRGI = crate::Reg<trgocfg_adc3_strgi::TRGOCFG_ADC3_STRGI_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_adc3_strgi;
#[doc = "TRGOCFG_ADCX_PTRGI0A (rw) register accessor: an alias for `Reg<TRGOCFG_ADCX_PTRGI0A_SPEC>`"]
pub type TRGOCFG_ADCX_PTRGI0A = crate::Reg<trgocfg_adcx_ptrgi0a::TRGOCFG_ADCX_PTRGI0A_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_adcx_ptrgi0a;
#[doc = "TRGOCFG_ADCX_PTRGI0B (rw) register accessor: an alias for `Reg<TRGOCFG_ADCX_PTRGI0B_SPEC>`"]
pub type TRGOCFG_ADCX_PTRGI0B = crate::Reg<trgocfg_adcx_ptrgi0b::TRGOCFG_ADCX_PTRGI0B_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_adcx_ptrgi0b;
#[doc = "TRGOCFG_ADCX_PTRGI0C (rw) register accessor: an alias for `Reg<TRGOCFG_ADCX_PTRGI0C_SPEC>`"]
pub type TRGOCFG_ADCX_PTRGI0C = crate::Reg<trgocfg_adcx_ptrgi0c::TRGOCFG_ADCX_PTRGI0C_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_adcx_ptrgi0c;
#[doc = "TRGOCFG_GPTMRA_SYNCI (rw) register accessor: an alias for `Reg<TRGOCFG_GPTMRA_SYNCI_SPEC>`"]
pub type TRGOCFG_GPTMRA_SYNCI = crate::Reg<trgocfg_gptmra_synci::TRGOCFG_GPTMRA_SYNCI_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_gptmra_synci;
#[doc = "TRGOCFG_GPTMRA_IN2 (rw) register accessor: an alias for `Reg<TRGOCFG_GPTMRA_IN2_SPEC>`"]
pub type TRGOCFG_GPTMRA_IN2 = crate::Reg<trgocfg_gptmra_in2::TRGOCFG_GPTMRA_IN2_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_gptmra_in2;
#[doc = "TRGOCFG_GPTMRA_IN3 (rw) register accessor: an alias for `Reg<TRGOCFG_GPTMRA_IN3_SPEC>`"]
pub type TRGOCFG_GPTMRA_IN3 = crate::Reg<trgocfg_gptmra_in3::TRGOCFG_GPTMRA_IN3_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_gptmra_in3;
#[doc = "TRGOCFG_GPTMRB_SYNCI (rw) register accessor: an alias for `Reg<TRGOCFG_GPTMRB_SYNCI_SPEC>`"]
pub type TRGOCFG_GPTMRB_SYNCI = crate::Reg<trgocfg_gptmrb_synci::TRGOCFG_GPTMRB_SYNCI_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_gptmrb_synci;
#[doc = "TRGOCFG_GPTMRB_IN2 (rw) register accessor: an alias for `Reg<TRGOCFG_GPTMRB_IN2_SPEC>`"]
pub type TRGOCFG_GPTMRB_IN2 = crate::Reg<trgocfg_gptmrb_in2::TRGOCFG_GPTMRB_IN2_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_gptmrb_in2;
#[doc = "TRGOCFG_GPTMRB_IN3 (rw) register accessor: an alias for `Reg<TRGOCFG_GPTMRB_IN3_SPEC>`"]
pub type TRGOCFG_GPTMRB_IN3 = crate::Reg<trgocfg_gptmrb_in3::TRGOCFG_GPTMRB_IN3_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_gptmrb_in3;
#[doc = "TRGOCFG_CMPX_WIN (rw) register accessor: an alias for `Reg<TRGOCFG_CMPX_WIN_SPEC>`"]
pub type TRGOCFG_CMPX_WIN = crate::Reg<trgocfg_cmpx_win::TRGOCFG_CMPX_WIN_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_cmpx_win;
#[doc = "TRGOCFG_CAN_PTPC0_CAP (rw) register accessor: an alias for `Reg<TRGOCFG_CAN_PTPC0_CAP_SPEC>`"]
pub type TRGOCFG_CAN_PTPC0_CAP = crate::Reg<trgocfg_can_ptpc0_cap::TRGOCFG_CAN_PTPC0_CAP_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_can_ptpc0_cap;
#[doc = "TRGOCFG_CAN_PTPC1_CAP (rw) register accessor: an alias for `Reg<TRGOCFG_CAN_PTPC1_CAP_SPEC>`"]
pub type TRGOCFG_CAN_PTPC1_CAP = crate::Reg<trgocfg_can_ptpc1_cap::TRGOCFG_CAN_PTPC1_CAP_SPEC>;
#[doc = "Trigger manager output configure register"]
pub mod trgocfg_can_ptpc1_cap;
#[doc = "DMACFG_0 (rw) register accessor: an alias for `Reg<DMACFG_0_SPEC>`"]
pub type DMACFG_0 = crate::Reg<dmacfg_0::DMACFG_0_SPEC>;
#[doc = "DMA request configure register"]
pub mod dmacfg_0;
#[doc = "DMACFG_1 (rw) register accessor: an alias for `Reg<DMACFG_1_SPEC>`"]
pub type DMACFG_1 = crate::Reg<dmacfg_1::DMACFG_1_SPEC>;
#[doc = "DMA request configure register"]
pub mod dmacfg_1;
#[doc = "DMACFG_2 (rw) register accessor: an alias for `Reg<DMACFG_2_SPEC>`"]
pub type DMACFG_2 = crate::Reg<dmacfg_2::DMACFG_2_SPEC>;
#[doc = "DMA request configure register"]
pub mod dmacfg_2;
#[doc = "DMACFG_3 (rw) register accessor: an alias for `Reg<DMACFG_3_SPEC>`"]
pub type DMACFG_3 = crate::Reg<dmacfg_3::DMACFG_3_SPEC>;
#[doc = "DMA request configure register"]
pub mod dmacfg_3;
#[doc = "GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "General Control Register"]
pub mod gcr;
