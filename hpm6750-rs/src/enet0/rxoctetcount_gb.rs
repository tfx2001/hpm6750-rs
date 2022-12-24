#[doc = "Register `RXOCTETCOUNT_GB` reader"]
pub struct R(crate::R<RXOCTETCOUNT_GB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXOCTETCOUNT_GB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXOCTETCOUNT_GB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXOCTETCOUNT_GB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXOCTETCOUNT_GB` writer"]
pub struct W(crate::W<RXOCTETCOUNT_GB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXOCTETCOUNT_GB_SPEC>;
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
impl From<crate::W<RXOCTETCOUNT_GB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXOCTETCOUNT_GB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTECNT` reader - Number of bytes received, exclusive of preamble, only in good frames."]
pub type BYTECNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BYTECNT` writer - Number of bytes received, exclusive of preamble, only in good frames."]
pub type BYTECNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RXOCTETCOUNT_GB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received, exclusive of preamble, only in good frames."]
    #[inline(always)]
    pub fn bytecnt(&self) -> BYTECNT_R {
        BYTECNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes received, exclusive of preamble, only in good frames."]
    #[inline(always)]
    #[must_use]
    pub fn bytecnt(&mut self) -> BYTECNT_W<0> {
        BYTECNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of bytes received, exclusive of preamble, in good and bad frames.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxoctetcount_gb](index.html) module"]
pub struct RXOCTETCOUNT_GB_SPEC;
impl crate::RegisterSpec for RXOCTETCOUNT_GB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxoctetcount_gb::R](R) reader structure"]
impl crate::Readable for RXOCTETCOUNT_GB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxoctetcount_gb::W](W) writer structure"]
impl crate::Writable for RXOCTETCOUNT_GB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXOCTETCOUNT_GB to value 0"]
impl crate::Resettable for RXOCTETCOUNT_GB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
