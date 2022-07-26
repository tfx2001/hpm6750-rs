#[doc = "Register `PKTBUF` reader"]
pub struct R(crate::R<PKTBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKTBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKTBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKTBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKTBUF` writer"]
pub struct W(crate::W<PKTBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKTBUF_SPEC>;
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
impl From<crate::W<PKTBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKTBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKTBUF` reader - No description avaiable"]
pub type PKTBUF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PKTBUF` writer - No description avaiable"]
pub type PKTBUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PKTBUF_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    pub fn pktbuf(&self) -> PKTBUF_R {
        PKTBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    pub fn pktbuf(&mut self) -> PKTBUF_W<0> {
        PKTBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet buffer size.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pktbuf](index.html) module"]
pub struct PKTBUF_SPEC;
impl crate::RegisterSpec for PKTBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pktbuf::R](R) reader structure"]
impl crate::Readable for PKTBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pktbuf::W](W) writer structure"]
impl crate::Writable for PKTBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKTBUF to value 0"]
impl crate::Resettable for PKTBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
