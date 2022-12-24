#[doc = "Register `DCDC_MODE` reader"]
pub struct R(crate::R<DCDC_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC_MODE` writer"]
pub struct W(crate::W<DCDC_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC_MODE_SPEC>;
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
impl From<crate::W<DCDC_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOLT` reader - DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
pub type VOLT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VOLT` writer - DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
pub type VOLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDC_MODE_SPEC, u16, u16, 12, O>;
#[doc = "Field `MODE` reader - DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDC_MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `READY` reader - Ready flag 0: DCDC is applying new change 1: DCDC is ready"]
pub type READY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:11 - DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
    #[inline(always)]
    pub fn volt(&self) -> VOLT_R {
        VOLT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 28 - Ready flag 0: DCDC is applying new change 1: DCDC is ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
    #[inline(always)]
    #[must_use]
    pub fn volt(&mut self) -> VOLT_W<0> {
        VOLT_W::new(self)
    }
    #[doc = "Bits 16:18 - DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<16> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC mode select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_mode](index.html) module"]
pub struct DCDC_MODE_SPEC;
impl crate::RegisterSpec for DCDC_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc_mode::R](R) reader structure"]
impl crate::Readable for DCDC_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc_mode::W](W) writer structure"]
impl crate::Writable for DCDC_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDC_MODE to value 0x00b0_10b0"]
impl crate::Resettable for DCDC_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x00b0_10b0;
}
