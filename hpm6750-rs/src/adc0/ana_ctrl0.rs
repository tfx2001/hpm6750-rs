#[doc = "Register `ANA_CTRL0` reader"]
pub struct R(crate::R<ANA_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_CTRL0` writer"]
pub struct W(crate::W<ANA_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_CTRL0_SPEC>;
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
impl From<crate::W<ANA_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOADCAL` reader - Signal that loads the offset calibration word into the internal registers (Active H)"]
pub type LOADCAL_R = crate::BitReader<bool>;
#[doc = "Field `LOADCAL` writer - Signal that loads the offset calibration word into the internal registers (Active H)"]
pub type LOADCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTRL0_SPEC, bool, O>;
#[doc = "Field `STARTCAL` reader - set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
pub type STARTCAL_R = crate::BitReader<bool>;
#[doc = "Field `STARTCAL` writer - set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
pub type STARTCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTRL0_SPEC, bool, O>;
#[doc = "Field `RESETCAL` reader - set to 1 to reset calibration logic; default high."]
pub type RESETCAL_R = crate::BitReader<bool>;
#[doc = "Field `RESETCAL` writer - set to 1 to reset calibration logic; default high."]
pub type RESETCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTRL0_SPEC, bool, O>;
#[doc = "Field `RESETADC` reader - set to 1 to reset adc analog; default high."]
pub type RESETADC_R = crate::BitReader<bool>;
#[doc = "Field `RESETADC` writer - set to 1 to reset adc analog; default high."]
pub type RESETADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTRL0_SPEC, bool, O>;
#[doc = "Field `ENADC` reader - set to enable adc analog function. user need set it after LDO stable, or wait at least 20us after setting enldo, then set this bit."]
pub type ENADC_R = crate::BitReader<bool>;
#[doc = "Field `ENADC` writer - set to enable adc analog function. user need set it after LDO stable, or wait at least 20us after setting enldo, then set this bit."]
pub type ENADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTRL0_SPEC, bool, O>;
#[doc = "Field `ENLDO` reader - set to enable adc LDO, need at least 20us for LDO to be stable."]
pub type ENLDO_R = crate::BitReader<bool>;
#[doc = "Field `ENLDO` writer - set to enable adc LDO, need at least 20us for LDO to be stable."]
pub type ENLDO_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTRL0_SPEC, bool, O>;
#[doc = "Field `SELRANGE_LDO` reader - Defines the range for the LDO reference (vdd_soc) selrange_ldo = 0: LDO reference dvdd or vref_ldo in range \\[0.81;0.99\\]
selrange_ldo = 1: LDO reference dvdd or vref_ldo in range \\[0.99;1.21\\]"]
pub type SELRANGE_LDO_R = crate::BitReader<bool>;
#[doc = "Field `SELRANGE_LDO` writer - Defines the range for the LDO reference (vdd_soc) selrange_ldo = 0: LDO reference dvdd or vref_ldo in range \\[0.81;0.99\\]
selrange_ldo = 1: LDO reference dvdd or vref_ldo in range \\[0.99;1.21\\]"]
pub type SELRANGE_LDO_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTRL0_SPEC, bool, O>;
#[doc = "Field `REARM_EN` reader - set will insert one adc cycle rearm before sample, user need to increase one to sample_clock_number"]
pub type REARM_EN_R = crate::BitReader<bool>;
#[doc = "Field `REARM_EN` writer - set will insert one adc cycle rearm before sample, user need to increase one to sample_clock_number"]
pub type REARM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTRL0_SPEC, bool, O>;
#[doc = "Field `CAL_VAL_SE` reader - calibration value for single-end mode"]
pub type CAL_VAL_SE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAL_VAL_SE` writer - calibration value for single-end mode"]
pub type CAL_VAL_SE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANA_CTRL0_SPEC, u8, u8, 7, O>;
#[doc = "Field `CAL_VAL_DIFF` reader - calibration value for differential mode"]
pub type CAL_VAL_DIFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAL_VAL_DIFF` writer - calibration value for differential mode"]
pub type CAL_VAL_DIFF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANA_CTRL0_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 1 - Signal that loads the offset calibration word into the internal registers (Active H)"]
    #[inline(always)]
    pub fn loadcal(&self) -> LOADCAL_R {
        LOADCAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
    #[inline(always)]
    pub fn startcal(&self) -> STARTCAL_R {
        STARTCAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - set to 1 to reset calibration logic; default high."]
    #[inline(always)]
    pub fn resetcal(&self) -> RESETCAL_R {
        RESETCAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set to 1 to reset adc analog; default high."]
    #[inline(always)]
    pub fn resetadc(&self) -> RESETADC_R {
        RESETADC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - set to enable adc analog function. user need set it after LDO stable, or wait at least 20us after setting enldo, then set this bit."]
    #[inline(always)]
    pub fn enadc(&self) -> ENADC_R {
        ENADC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - set to enable adc LDO, need at least 20us for LDO to be stable."]
    #[inline(always)]
    pub fn enldo(&self) -> ENLDO_R {
        ENLDO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - Defines the range for the LDO reference (vdd_soc) selrange_ldo = 0: LDO reference dvdd or vref_ldo in range \\[0.81;0.99\\]
selrange_ldo = 1: LDO reference dvdd or vref_ldo in range \\[0.99;1.21\\]"]
    #[inline(always)]
    pub fn selrange_ldo(&self) -> SELRANGE_LDO_R {
        SELRANGE_LDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - set will insert one adc cycle rearm before sample, user need to increase one to sample_clock_number"]
    #[inline(always)]
    pub fn rearm_en(&self) -> REARM_EN_R {
        REARM_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - calibration value for single-end mode"]
    #[inline(always)]
    pub fn cal_val_se(&self) -> CAL_VAL_SE_R {
        CAL_VAL_SE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - calibration value for differential mode"]
    #[inline(always)]
    pub fn cal_val_diff(&self) -> CAL_VAL_DIFF_R {
        CAL_VAL_DIFF_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Signal that loads the offset calibration word into the internal registers (Active H)"]
    #[inline(always)]
    #[must_use]
    pub fn loadcal(&mut self) -> LOADCAL_W<1> {
        LOADCAL_W::new(self)
    }
    #[doc = "Bit 2 - set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
    #[inline(always)]
    #[must_use]
    pub fn startcal(&mut self) -> STARTCAL_W<2> {
        STARTCAL_W::new(self)
    }
    #[doc = "Bit 3 - set to 1 to reset calibration logic; default high."]
    #[inline(always)]
    #[must_use]
    pub fn resetcal(&mut self) -> RESETCAL_W<3> {
        RESETCAL_W::new(self)
    }
    #[doc = "Bit 4 - set to 1 to reset adc analog; default high."]
    #[inline(always)]
    #[must_use]
    pub fn resetadc(&mut self) -> RESETADC_W<4> {
        RESETADC_W::new(self)
    }
    #[doc = "Bit 5 - set to enable adc analog function. user need set it after LDO stable, or wait at least 20us after setting enldo, then set this bit."]
    #[inline(always)]
    #[must_use]
    pub fn enadc(&mut self) -> ENADC_W<5> {
        ENADC_W::new(self)
    }
    #[doc = "Bit 6 - set to enable adc LDO, need at least 20us for LDO to be stable."]
    #[inline(always)]
    #[must_use]
    pub fn enldo(&mut self) -> ENLDO_W<6> {
        ENLDO_W::new(self)
    }
    #[doc = "Bit 11 - Defines the range for the LDO reference (vdd_soc) selrange_ldo = 0: LDO reference dvdd or vref_ldo in range \\[0.81;0.99\\]
selrange_ldo = 1: LDO reference dvdd or vref_ldo in range \\[0.99;1.21\\]"]
    #[inline(always)]
    #[must_use]
    pub fn selrange_ldo(&mut self) -> SELRANGE_LDO_W<11> {
        SELRANGE_LDO_W::new(self)
    }
    #[doc = "Bit 14 - set will insert one adc cycle rearm before sample, user need to increase one to sample_clock_number"]
    #[inline(always)]
    #[must_use]
    pub fn rearm_en(&mut self) -> REARM_EN_W<14> {
        REARM_EN_W::new(self)
    }
    #[doc = "Bits 16:22 - calibration value for single-end mode"]
    #[inline(always)]
    #[must_use]
    pub fn cal_val_se(&mut self) -> CAL_VAL_SE_W<16> {
        CAL_VAL_SE_W::new(self)
    }
    #[doc = "Bits 24:30 - calibration value for differential mode"]
    #[inline(always)]
    #[must_use]
    pub fn cal_val_diff(&mut self) -> CAL_VAL_DIFF_W<24> {
        CAL_VAL_DIFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_ctrl0](index.html) module"]
pub struct ANA_CTRL0_SPEC;
impl crate::RegisterSpec for ANA_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_ctrl0::R](R) reader structure"]
impl crate::Readable for ANA_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_ctrl0::W](W) writer structure"]
impl crate::Writable for ANA_CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANA_CTRL0 to value 0"]
impl crate::Resettable for ANA_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
