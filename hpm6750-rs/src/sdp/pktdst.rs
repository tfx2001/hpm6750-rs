#[doc = "Register `PKTDST` reader"]
pub struct R(crate::R<PKTDST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKTDST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKTDST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKTDST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKTDST` writer"]
pub struct W(crate::W<PKTDST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKTDST_SPEC>;
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
impl From<crate::W<PKTDST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKTDST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKTDST` reader - Packet Memory Destination Address"]
pub type PKTDST_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PKTDST` writer - Packet Memory Destination Address"]
pub type PKTDST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PKTDST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Packet Memory Destination Address"]
    #[inline(always)]
    pub fn pktdst(&self) -> PKTDST_R {
        PKTDST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Packet Memory Destination Address"]
    #[inline(always)]
    pub fn pktdst(&mut self) -> PKTDST_W<0> {
        PKTDST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet Memory Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pktdst](index.html) module"]
pub struct PKTDST_SPEC;
impl crate::RegisterSpec for PKTDST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pktdst::R](R) reader structure"]
impl crate::Readable for PKTDST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pktdst::W](W) writer structure"]
impl crate::Writable for PKTDST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKTDST to value 0"]
impl crate::Resettable for PKTDST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
