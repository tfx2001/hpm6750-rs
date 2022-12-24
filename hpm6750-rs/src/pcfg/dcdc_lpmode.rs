#[doc = "Register `DCDC_LPMODE` reader"]
pub struct R(crate::R<DCDC_LPMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC_LPMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC_LPMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC_LPMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC_LPMODE` writer"]
pub struct W(crate::W<DCDC_LPMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC_LPMODE_SPEC>;
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
impl From<crate::W<DCDC_LPMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC_LPMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STBY_VOLT` reader - DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
pub type STBY_VOLT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STBY_VOLT` writer - DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
pub type STBY_VOLT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC_LPMODE_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
    #[inline(always)]
    pub fn stby_volt(&self) -> STBY_VOLT_R {
        STBY_VOLT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
    #[inline(always)]
    #[must_use]
    pub fn stby_volt(&mut self) -> STBY_VOLT_W<0> {
        STBY_VOLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC low power mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_lpmode](index.html) module"]
pub struct DCDC_LPMODE_SPEC;
impl crate::RegisterSpec for DCDC_LPMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc_lpmode::R](R) reader structure"]
impl crate::Readable for DCDC_LPMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc_lpmode::W](W) writer structure"]
impl crate::Writable for DCDC_LPMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDC_LPMODE to value 0x00b0_10b0"]
impl crate::Resettable for DCDC_LPMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x00b0_10b0;
}
