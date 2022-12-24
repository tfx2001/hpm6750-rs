#[doc = "Register `RXFIFOOVERFLOW` reader"]
pub struct R(crate::R<RXFIFOOVERFLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIFOOVERFLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIFOOVERFLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIFOOVERFLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFIFOOVERFLOW` writer"]
pub struct W(crate::W<RXFIFOOVERFLOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFIFOOVERFLOW_SPEC>;
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
impl From<crate::W<RXFIFOOVERFLOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFIFOOVERFLOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRMCNT` reader - Number of missed received frames because of FIFO overflow. This counter is not present in the GMAC-CORE configuration."]
pub type FRMCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRMCNT` writer - Number of missed received frames because of FIFO overflow. This counter is not present in the GMAC-CORE configuration."]
pub type FRMCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RXFIFOOVERFLOW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Number of missed received frames because of FIFO overflow. This counter is not present in the GMAC-CORE configuration."]
    #[inline(always)]
    pub fn frmcnt(&self) -> FRMCNT_R {
        FRMCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of missed received frames because of FIFO overflow. This counter is not present in the GMAC-CORE configuration."]
    #[inline(always)]
    #[must_use]
    pub fn frmcnt(&mut self) -> FRMCNT_W<0> {
        FRMCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of missed received frames because of FIFO overflow. This counter is not present in the GMAC-CORE configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifooverflow](index.html) module"]
pub struct RXFIFOOVERFLOW_SPEC;
impl crate::RegisterSpec for RXFIFOOVERFLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfifooverflow::R](R) reader structure"]
impl crate::Readable for RXFIFOOVERFLOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxfifooverflow::W](W) writer structure"]
impl crate::Writable for RXFIFOOVERFLOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXFIFOOVERFLOW to value 0"]
impl crate::Resettable for RXFIFOOVERFLOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
