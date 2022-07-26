#[doc = "Register `DCDC_RESUME_TIME` reader"]
pub struct R(crate::R<DCDC_RESUME_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC_RESUME_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC_RESUME_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC_RESUME_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC_RESUME_TIME` writer"]
pub struct W(crate::W<DCDC_RESUME_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC_RESUME_TIME_SPEC>;
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
impl From<crate::W<DCDC_RESUME_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC_RESUME_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESUME_TIME` reader - Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS"]
pub type RESUME_TIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESUME_TIME` writer - Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS"]
pub type RESUME_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDC_RESUME_TIME_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS"]
    #[inline(always)]
    pub fn resume_time(&self) -> RESUME_TIME_R {
        RESUME_TIME_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS"]
    #[inline(always)]
    pub fn resume_time(&mut self) -> RESUME_TIME_W<0> {
        RESUME_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC resume time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_resume_time](index.html) module"]
pub struct DCDC_RESUME_TIME_SPEC;
impl crate::RegisterSpec for DCDC_RESUME_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc_resume_time::R](R) reader structure"]
impl crate::Readable for DCDC_RESUME_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc_resume_time::W](W) writer structure"]
impl crate::Writable for DCDC_RESUME_TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDC_RESUME_TIME to value 0x8c9f"]
impl crate::Resettable for DCDC_RESUME_TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8c9f
    }
}
