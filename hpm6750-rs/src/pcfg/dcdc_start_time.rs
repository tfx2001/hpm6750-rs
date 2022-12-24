#[doc = "Register `DCDC_START_TIME` reader"]
pub struct R(crate::R<DCDC_START_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC_START_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC_START_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC_START_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC_START_TIME` writer"]
pub struct W(crate::W<DCDC_START_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC_START_TIME_SPEC>;
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
impl From<crate::W<DCDC_START_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC_START_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_TIME` reader - Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS"]
pub type START_TIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `START_TIME` writer - Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS"]
pub type START_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC_START_TIME_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS"]
    #[inline(always)]
    pub fn start_time(&self) -> START_TIME_R {
        START_TIME_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS"]
    #[inline(always)]
    #[must_use]
    pub fn start_time(&mut self) -> START_TIME_W<0> {
        START_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC ramp time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_start_time](index.html) module"]
pub struct DCDC_START_TIME_SPEC;
impl crate::RegisterSpec for DCDC_START_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc_start_time::R](R) reader structure"]
impl crate::Readable for DCDC_START_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc_start_time::W](W) writer structure"]
impl crate::Writable for DCDC_START_TIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDC_START_TIME to value 0x0001_193f"]
impl crate::Resettable for DCDC_START_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_193f;
}
