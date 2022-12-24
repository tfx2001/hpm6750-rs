#[doc = "Register `TXUNDERFLOWERROR` reader"]
pub struct R(crate::R<TXUNDERFLOWERROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXUNDERFLOWERROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXUNDERFLOWERROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXUNDERFLOWERROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXUNDERFLOWERROR` writer"]
pub struct W(crate::W<TXUNDERFLOWERROR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXUNDERFLOWERROR_SPEC>;
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
impl From<crate::W<TXUNDERFLOWERROR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXUNDERFLOWERROR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRMCNT` reader - Number of frames aborted because of frame underflow error."]
pub type FRMCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRMCNT` writer - Number of frames aborted because of frame underflow error."]
pub type FRMCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TXUNDERFLOWERROR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Number of frames aborted because of frame underflow error."]
    #[inline(always)]
    pub fn frmcnt(&self) -> FRMCNT_R {
        FRMCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of frames aborted because of frame underflow error."]
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
#[doc = "Number of frames aborted because of frame underflow error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txunderflowerror](index.html) module"]
pub struct TXUNDERFLOWERROR_SPEC;
impl crate::RegisterSpec for TXUNDERFLOWERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txunderflowerror::R](R) reader structure"]
impl crate::Readable for TXUNDERFLOWERROR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txunderflowerror::W](W) writer structure"]
impl crate::Writable for TXUNDERFLOWERROR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXUNDERFLOWERROR to value 0"]
impl crate::Resettable for TXUNDERFLOWERROR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
