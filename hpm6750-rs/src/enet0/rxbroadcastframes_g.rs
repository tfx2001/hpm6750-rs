#[doc = "Register `RXBROADCASTFRAMES_G` reader"]
pub struct R(crate::R<RXBROADCASTFRAMES_G_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXBROADCASTFRAMES_G_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXBROADCASTFRAMES_G_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXBROADCASTFRAMES_G_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXBROADCASTFRAMES_G` writer"]
pub struct W(crate::W<RXBROADCASTFRAMES_G_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXBROADCASTFRAMES_G_SPEC>;
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
impl From<crate::W<RXBROADCASTFRAMES_G_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXBROADCASTFRAMES_G_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRMCNT` reader - Number of good broadcast frames received."]
pub type FRMCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRMCNT` writer - Number of good broadcast frames received."]
pub type FRMCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RXBROADCASTFRAMES_G_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Number of good broadcast frames received."]
    #[inline(always)]
    pub fn frmcnt(&self) -> FRMCNT_R {
        FRMCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good broadcast frames received."]
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
#[doc = "Number of good broadcast frames received\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbroadcastframes_g](index.html) module"]
pub struct RXBROADCASTFRAMES_G_SPEC;
impl crate::RegisterSpec for RXBROADCASTFRAMES_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxbroadcastframes_g::R](R) reader structure"]
impl crate::Readable for RXBROADCASTFRAMES_G_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxbroadcastframes_g::W](W) writer structure"]
impl crate::Writable for RXBROADCASTFRAMES_G_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXBROADCASTFRAMES_G to value 0"]
impl crate::Resettable for RXBROADCASTFRAMES_G_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
