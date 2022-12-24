#[doc = "Register `SPDCMP` reader"]
pub struct R(crate::R<SPDCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDCMP` writer"]
pub struct W(crate::W<SPDCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDCMP_SPEC>;
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
impl From<crate::W<SPDCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDCMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPDCMP` reader - spdcnt position compare value"]
pub type SPDCMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPDCMP` writer - spdcnt position compare value"]
pub type SPDCMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPDCMP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - spdcnt position compare value"]
    #[inline(always)]
    pub fn spdcmp(&self) -> SPDCMP_R {
        SPDCMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - spdcnt position compare value"]
    #[inline(always)]
    #[must_use]
    pub fn spdcmp(&mut self) -> SPDCMP_W<0> {
        SPDCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Speed comparator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdcmp](index.html) module"]
pub struct SPDCMP_SPEC;
impl crate::RegisterSpec for SPDCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdcmp::R](R) reader structure"]
impl crate::Readable for SPDCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdcmp::W](W) writer structure"]
impl crate::Writable for SPDCMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPDCMP to value 0"]
impl crate::Resettable for SPDCMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
