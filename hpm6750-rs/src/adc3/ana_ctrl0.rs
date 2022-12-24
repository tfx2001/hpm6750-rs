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
#[doc = "Field `STARTCAL` reader - set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
pub type STARTCAL_R = crate::BitReader<bool>;
#[doc = "Field `STARTCAL` writer - set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
pub type STARTCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTRL0_SPEC, bool, O>;
#[doc = "Field `ADC_CLK_ON` reader - set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access"]
pub type ADC_CLK_ON_R = crate::BitReader<bool>;
#[doc = "Field `ADC_CLK_ON` writer - set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access"]
pub type ADC_CLK_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
    #[inline(always)]
    pub fn startcal(&self) -> STARTCAL_R {
        STARTCAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access"]
    #[inline(always)]
    pub fn adc_clk_on(&self) -> ADC_CLK_ON_R {
        ADC_CLK_ON_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
    #[inline(always)]
    #[must_use]
    pub fn startcal(&mut self) -> STARTCAL_W<2> {
        STARTCAL_W::new(self)
    }
    #[doc = "Bit 12 - set to enable adc clock to analog, Software should set this bit before access to any adc16_* register. MUST set clock_period to 0 or 1 for adc16 reg access"]
    #[inline(always)]
    #[must_use]
    pub fn adc_clk_on(&mut self) -> ADC_CLK_ON_W<12> {
        ADC_CLK_ON_W::new(self)
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
