#[doc = "Register `PPS_CTRL` reader"]
pub struct R(crate::R<PPS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPS_CTRL` writer"]
pub struct W(crate::W<PPS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PPS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPSCTRL0` reader - PPSCTRL0: PPS0 Output Frequency Control This field controls the frequency of the PPS0 output (ptp_pps_o\\[0\\]) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: - 0001: The binary rollover is 2 Hz, and the digital rollover is 1 Hz. - 0010: The binary rollover is 4 Hz, and the digital rollover is 2 Hz. - 0011: The binary rollover is 8 Hz, and the digital rollover is 4 Hz. - 0100: The binary rollover is 16 Hz, and the digital rollover is 8 Hz. - ... - 1111: The binary rollover is 32.768 KHz, and the digital rollover is 16.384 KHz. Note: In the binary rollover mode, the PPS output (ptp_pps_o) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: - When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms - When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of: - One clock of 50 percent duty cycle and 537 ms period - Second clock of 463 ms period (268 ms low and 195 ms high) - When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of: - Three clocks of 50 percent duty cycle and 268 ms period - Fourth clock of 195 ms period (134 ms low and 61 ms high)"]
pub type PPSCTRL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPSCTRL0` writer - PPSCTRL0: PPS0 Output Frequency Control This field controls the frequency of the PPS0 output (ptp_pps_o\\[0\\]) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: - 0001: The binary rollover is 2 Hz, and the digital rollover is 1 Hz. - 0010: The binary rollover is 4 Hz, and the digital rollover is 2 Hz. - 0011: The binary rollover is 8 Hz, and the digital rollover is 4 Hz. - 0100: The binary rollover is 16 Hz, and the digital rollover is 8 Hz. - ... - 1111: The binary rollover is 32.768 KHz, and the digital rollover is 16.384 KHz. Note: In the binary rollover mode, the PPS output (ptp_pps_o) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: - When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms - When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of: - One clock of 50 percent duty cycle and 537 ms period - Second clock of 463 ms period (268 ms low and 195 ms high) - When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of: - Three clocks of 50 percent duty cycle and 268 ms period - Fourth clock of 195 ms period (134 ms low and 61 ms high)"]
pub type PPSCTRL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPS_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `PPSEN0` reader - Flexible PPS Output Mode Enable When set low, Bits \\[3:0\\]
function as PPSCTRL (backward compatible). When set high, Bits\\[3:0\\]
function as PPSCMD."]
pub type PPSEN0_R = crate::BitReader<bool>;
#[doc = "Field `PPSEN0` writer - Flexible PPS Output Mode Enable When set low, Bits \\[3:0\\]
function as PPSCTRL (backward compatible). When set high, Bits\\[3:0\\]
function as PPSCMD."]
pub type PPSEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPS_CTRL_SPEC, bool, O>;
#[doc = "Field `TRGTMODSEL0` reader - Target Time Register Mode for PPS0 Output This field indicates the Target Time registers (register 455 and 456) mode for PPS0 output signal: - 00: Indicates that the Target Time registers are programmed only for generating the interrupt event. - 01: Reserved - 10: Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the PPS0 output signal. - 11: Indicates that the Target Time registers are programmed only for starting or stopping the generation of the PPS0 output signal. No interrupt is asserted."]
pub type TRGTMODSEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGTMODSEL0` writer - Target Time Register Mode for PPS0 Output This field indicates the Target Time registers (register 455 and 456) mode for PPS0 output signal: - 00: Indicates that the Target Time registers are programmed only for generating the interrupt event. - 01: Reserved - 10: Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the PPS0 output signal. - 11: Indicates that the Target Time registers are programmed only for starting or stopping the generation of the PPS0 output signal. No interrupt is asserted."]
pub type TRGTMODSEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPS_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PPSCMD1` reader - Flexible PPS1 Output Control This field controls the flexible PPS1 output (ptp_pps_o\\[1\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
pub type PPSCMD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPSCMD1` writer - Flexible PPS1 Output Control This field controls the flexible PPS1 output (ptp_pps_o\\[1\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
pub type PPSCMD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPS_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `TRGTMODSEL1` reader - Target Time Register Mode for PPS1 Output This field indicates the Target Time registers (register 480 and 481) mode for PPS1 output signal. This field is similar to the TRGTMODSEL0 field."]
pub type TRGTMODSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGTMODSEL1` writer - Target Time Register Mode for PPS1 Output This field indicates the Target Time registers (register 480 and 481) mode for PPS1 output signal. This field is similar to the TRGTMODSEL0 field."]
pub type TRGTMODSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPS_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PPSCMD2` reader - Flexible PPS2 Output Control This field controls the flexible PPS2 output (ptp_pps_o\\[2\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
pub type PPSCMD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPSCMD2` writer - Flexible PPS2 Output Control This field controls the flexible PPS2 output (ptp_pps_o\\[2\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
pub type PPSCMD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPS_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `TRGTMODSEL2` reader - Target Time Register Mode for PPS2 Output This field indicates the Target Time registers (register 488 and 489) mode for PPS2 output signal. This field is similar to the TRGTMODSEL0 field."]
pub type TRGTMODSEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGTMODSEL2` writer - Target Time Register Mode for PPS2 Output This field indicates the Target Time registers (register 488 and 489) mode for PPS2 output signal. This field is similar to the TRGTMODSEL0 field."]
pub type TRGTMODSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPS_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PPSCMD3` reader - Flexible PPS3 Output Control This field controls the flexible PPS3 output (ptp_pps_o\\[3\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
pub type PPSCMD3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPSCMD3` writer - Flexible PPS3 Output Control This field controls the flexible PPS3 output (ptp_pps_o\\[3\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
pub type PPSCMD3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPS_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `TRGTMODSEL3` reader - Target Time Register Mode for PPS3 Output This field indicates the Target Time registers (register 496 and 497) mode for PPS3 output signal. This field is similar to the TRGTMODSEL0 field."]
pub type TRGTMODSEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGTMODSEL3` writer - Target Time Register Mode for PPS3 Output This field indicates the Target Time registers (register 496 and 497) mode for PPS3 output signal. This field is similar to the TRGTMODSEL0 field."]
pub type TRGTMODSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PPS_CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - PPSCTRL0: PPS0 Output Frequency Control This field controls the frequency of the PPS0 output (ptp_pps_o\\[0\\]) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: - 0001: The binary rollover is 2 Hz, and the digital rollover is 1 Hz. - 0010: The binary rollover is 4 Hz, and the digital rollover is 2 Hz. - 0011: The binary rollover is 8 Hz, and the digital rollover is 4 Hz. - 0100: The binary rollover is 16 Hz, and the digital rollover is 8 Hz. - ... - 1111: The binary rollover is 32.768 KHz, and the digital rollover is 16.384 KHz. Note: In the binary rollover mode, the PPS output (ptp_pps_o) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: - When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms - When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of: - One clock of 50 percent duty cycle and 537 ms period - Second clock of 463 ms period (268 ms low and 195 ms high) - When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of: - Three clocks of 50 percent duty cycle and 268 ms period - Fourth clock of 195 ms period (134 ms low and 61 ms high)"]
    #[inline(always)]
    pub fn ppsctrl0(&self) -> PPSCTRL0_R {
        PPSCTRL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable When set low, Bits \\[3:0\\]
function as PPSCTRL (backward compatible). When set high, Bits\\[3:0\\]
function as PPSCMD."]
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output This field indicates the Target Time registers (register 455 and 456) mode for PPS0 output signal: - 00: Indicates that the Target Time registers are programmed only for generating the interrupt event. - 01: Reserved - 10: Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the PPS0 output signal. - 11: Indicates that the Target Time registers are programmed only for starting or stopping the generation of the PPS0 output signal. No interrupt is asserted."]
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Flexible PPS1 Output Control This field controls the flexible PPS1 output (ptp_pps_o\\[1\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
    #[inline(always)]
    pub fn ppscmd1(&self) -> PPSCMD1_R {
        PPSCMD1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 13:14 - Target Time Register Mode for PPS1 Output This field indicates the Target Time registers (register 480 and 481) mode for PPS1 output signal. This field is similar to the TRGTMODSEL0 field."]
    #[inline(always)]
    pub fn trgtmodsel1(&self) -> TRGTMODSEL1_R {
        TRGTMODSEL1_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Flexible PPS2 Output Control This field controls the flexible PPS2 output (ptp_pps_o\\[2\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
    #[inline(always)]
    pub fn ppscmd2(&self) -> PPSCMD2_R {
        PPSCMD2_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 21:22 - Target Time Register Mode for PPS2 Output This field indicates the Target Time registers (register 488 and 489) mode for PPS2 output signal. This field is similar to the TRGTMODSEL0 field."]
    #[inline(always)]
    pub fn trgtmodsel2(&self) -> TRGTMODSEL2_R {
        TRGTMODSEL2_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Flexible PPS3 Output Control This field controls the flexible PPS3 output (ptp_pps_o\\[3\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
    #[inline(always)]
    pub fn ppscmd3(&self) -> PPSCMD3_R {
        PPSCMD3_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 29:30 - Target Time Register Mode for PPS3 Output This field indicates the Target Time registers (register 496 and 497) mode for PPS3 output signal. This field is similar to the TRGTMODSEL0 field."]
    #[inline(always)]
    pub fn trgtmodsel3(&self) -> TRGTMODSEL3_R {
        TRGTMODSEL3_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PPSCTRL0: PPS0 Output Frequency Control This field controls the frequency of the PPS0 output (ptp_pps_o\\[0\\]) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: - 0001: The binary rollover is 2 Hz, and the digital rollover is 1 Hz. - 0010: The binary rollover is 4 Hz, and the digital rollover is 2 Hz. - 0011: The binary rollover is 8 Hz, and the digital rollover is 4 Hz. - 0100: The binary rollover is 16 Hz, and the digital rollover is 8 Hz. - ... - 1111: The binary rollover is 32.768 KHz, and the digital rollover is 16.384 KHz. Note: In the binary rollover mode, the PPS output (ptp_pps_o) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: - When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms - When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of: - One clock of 50 percent duty cycle and 537 ms period - Second clock of 463 ms period (268 ms low and 195 ms high) - When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of: - Three clocks of 50 percent duty cycle and 268 ms period - Fourth clock of 195 ms period (134 ms low and 61 ms high)"]
    #[inline(always)]
    #[must_use]
    pub fn ppsctrl0(&mut self) -> PPSCTRL0_W<0> {
        PPSCTRL0_W::new(self)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable When set low, Bits \\[3:0\\]
function as PPSCTRL (backward compatible). When set high, Bits\\[3:0\\]
function as PPSCMD."]
    #[inline(always)]
    #[must_use]
    pub fn ppsen0(&mut self) -> PPSEN0_W<4> {
        PPSEN0_W::new(self)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output This field indicates the Target Time registers (register 455 and 456) mode for PPS0 output signal: - 00: Indicates that the Target Time registers are programmed only for generating the interrupt event. - 01: Reserved - 10: Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the PPS0 output signal. - 11: Indicates that the Target Time registers are programmed only for starting or stopping the generation of the PPS0 output signal. No interrupt is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W<5> {
        TRGTMODSEL0_W::new(self)
    }
    #[doc = "Bits 8:10 - Flexible PPS1 Output Control This field controls the flexible PPS1 output (ptp_pps_o\\[1\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
    #[inline(always)]
    #[must_use]
    pub fn ppscmd1(&mut self) -> PPSCMD1_W<8> {
        PPSCMD1_W::new(self)
    }
    #[doc = "Bits 13:14 - Target Time Register Mode for PPS1 Output This field indicates the Target Time registers (register 480 and 481) mode for PPS1 output signal. This field is similar to the TRGTMODSEL0 field."]
    #[inline(always)]
    #[must_use]
    pub fn trgtmodsel1(&mut self) -> TRGTMODSEL1_W<13> {
        TRGTMODSEL1_W::new(self)
    }
    #[doc = "Bits 16:18 - Flexible PPS2 Output Control This field controls the flexible PPS2 output (ptp_pps_o\\[2\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
    #[inline(always)]
    #[must_use]
    pub fn ppscmd2(&mut self) -> PPSCMD2_W<16> {
        PPSCMD2_W::new(self)
    }
    #[doc = "Bits 21:22 - Target Time Register Mode for PPS2 Output This field indicates the Target Time registers (register 488 and 489) mode for PPS2 output signal. This field is similar to the TRGTMODSEL0 field."]
    #[inline(always)]
    #[must_use]
    pub fn trgtmodsel2(&mut self) -> TRGTMODSEL2_W<21> {
        TRGTMODSEL2_W::new(self)
    }
    #[doc = "Bits 24:26 - Flexible PPS3 Output Control This field controls the flexible PPS3 output (ptp_pps_o\\[3\\]) signal. This field is similar to PPSCMD0\\[2:0\\]
in functionality."]
    #[inline(always)]
    #[must_use]
    pub fn ppscmd3(&mut self) -> PPSCMD3_W<24> {
        PPSCMD3_W::new(self)
    }
    #[doc = "Bits 29:30 - Target Time Register Mode for PPS3 Output This field indicates the Target Time registers (register 496 and 497) mode for PPS3 output signal. This field is similar to the TRGTMODSEL0 field."]
    #[inline(always)]
    #[must_use]
    pub fn trgtmodsel3(&mut self) -> TRGTMODSEL3_W<29> {
        TRGTMODSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPS Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pps_ctrl](index.html) module"]
pub struct PPS_CTRL_SPEC;
impl crate::RegisterSpec for PPS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pps_ctrl::R](R) reader structure"]
impl crate::Readable for PPS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pps_ctrl::W](W) writer structure"]
impl crate::Writable for PPS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPS_CTRL to value 0"]
impl crate::Resettable for PPS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
