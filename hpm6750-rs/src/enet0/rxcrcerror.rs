#[doc = "Register `RXCRCERROR` reader"]
pub struct R(crate::R<RXCRCERROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCRCERROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCRCERROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCRCERROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXCRCERROR` writer"]
pub struct W(crate::W<RXCRCERROR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCRCERROR_SPEC>;
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
impl From<crate::W<RXCRCERROR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCRCERROR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRMCNT` reader - Number of frames received with CRC error."]
pub type FRMCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRMCNT` writer - Number of frames received with CRC error."]
pub type FRMCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXCRCERROR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Number of frames received with CRC error."]
    #[inline(always)]
    pub fn frmcnt(&self) -> FRMCNT_R {
        FRMCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of frames received with CRC error."]
    #[inline(always)]
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
#[doc = "Number of frames received with CRC error\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcrcerror](index.html) module"]
pub struct RXCRCERROR_SPEC;
impl crate::RegisterSpec for RXCRCERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxcrcerror::R](R) reader structure"]
impl crate::Readable for RXCRCERROR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcrcerror::W](W) writer structure"]
impl crate::Writable for RXCRCERROR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXCRCERROR to value 0"]
impl crate::Resettable for RXCRCERROR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
