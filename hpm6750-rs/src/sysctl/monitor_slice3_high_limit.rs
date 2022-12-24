#[doc = "Register `MONITOR_SLICE3_HIGH_LIMIT` reader"]
pub struct R(crate::R<MONITOR_SLICE3_HIGH_LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MONITOR_SLICE3_HIGH_LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MONITOR_SLICE3_HIGH_LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MONITOR_SLICE3_HIGH_LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MONITOR_SLICE3_HIGH_LIMIT` writer"]
pub struct W(crate::W<MONITOR_SLICE3_HIGH_LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MONITOR_SLICE3_HIGH_LIMIT_SPEC>;
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
impl From<crate::W<MONITOR_SLICE3_HIGH_LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MONITOR_SLICE3_HIGH_LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQUENCY` reader - upper frequency"]
pub type FREQUENCY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FREQUENCY` writer - upper frequency"]
pub type FREQUENCY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MONITOR_SLICE3_HIGH_LIMIT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - upper frequency"]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - upper frequency"]
    #[inline(always)]
    #[must_use]
    pub fn frequency(&mut self) -> FREQUENCY_W<0> {
        FREQUENCY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock upper limit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monitor_slice3_high_limit](index.html) module"]
pub struct MONITOR_SLICE3_HIGH_LIMIT_SPEC;
impl crate::RegisterSpec for MONITOR_SLICE3_HIGH_LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [monitor_slice3_high_limit::R](R) reader structure"]
impl crate::Readable for MONITOR_SLICE3_HIGH_LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [monitor_slice3_high_limit::W](W) writer structure"]
impl crate::Writable for MONITOR_SLICE3_HIGH_LIMIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MONITOR_SLICE3_HIGH_LIMIT to value 0"]
impl crate::Resettable for MONITOR_SLICE3_HIGH_LIMIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
