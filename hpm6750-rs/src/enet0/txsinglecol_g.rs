#[doc = "Register `TXSINGLECOL_G` reader"]
pub struct R(crate::R<TXSINGLECOL_G_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXSINGLECOL_G_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXSINGLECOL_G_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXSINGLECOL_G_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXSINGLECOL_G` writer"]
pub struct W(crate::W<TXSINGLECOL_G_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXSINGLECOL_G_SPEC>;
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
impl From<crate::W<TXSINGLECOL_G_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXSINGLECOL_G_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRMCNT` reader - Number of successfully transmitted frames after a single collision in the half-duplex mode."]
pub type FRMCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRMCNT` writer - Number of successfully transmitted frames after a single collision in the half-duplex mode."]
pub type FRMCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TXSINGLECOL_G_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Number of successfully transmitted frames after a single collision in the half-duplex mode."]
    #[inline(always)]
    pub fn frmcnt(&self) -> FRMCNT_R {
        FRMCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of successfully transmitted frames after a single collision in the half-duplex mode."]
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
#[doc = "Number of successfully transmitted frames after a single collision in the half-duplex mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txsinglecol_g](index.html) module"]
pub struct TXSINGLECOL_G_SPEC;
impl crate::RegisterSpec for TXSINGLECOL_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txsinglecol_g::R](R) reader structure"]
impl crate::Readable for TXSINGLECOL_G_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txsinglecol_g::W](W) writer structure"]
impl crate::Writable for TXSINGLECOL_G_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXSINGLECOL_G to value 0"]
impl crate::Resettable for TXSINGLECOL_G_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
