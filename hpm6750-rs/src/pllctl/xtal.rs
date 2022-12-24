#[doc = "Register `XTAL` reader"]
pub struct R(crate::R<XTAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL` writer"]
pub struct W(crate::W<XTAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_SPEC>;
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
impl From<crate::W<XTAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMP_TIME` reader - Rampup time of XTAL oscillator in cycles of IRC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles"]
pub type RAMP_TIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RAMP_TIME` writer - Rampup time of XTAL oscillator in cycles of IRC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles"]
pub type RAMP_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTAL_SPEC, u32, u32, 20, O>;
#[doc = "Field `ENABLE` reader - Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RESPONSE` reader - Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
pub type RESPONSE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:19 - Rampup time of XTAL oscillator in cycles of IRC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles"]
    #[inline(always)]
    pub fn ramp_time(&self) -> RAMP_TIME_R {
        RAMP_TIME_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28 - Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Rampup time of XTAL oscillator in cycles of IRC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles"]
    #[inline(always)]
    #[must_use]
    pub fn ramp_time(&mut self) -> RAMP_TIME_W<0> {
        RAMP_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crystal control and status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal](index.html) module"]
pub struct XTAL_SPEC;
impl crate::RegisterSpec for XTAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal::R](R) reader structure"]
impl crate::Readable for XTAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal::W](W) writer structure"]
impl crate::Writable for XTAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTAL to value 0"]
impl crate::Resettable for XTAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
