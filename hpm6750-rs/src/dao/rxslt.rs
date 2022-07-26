#[doc = "Register `RXSLT` reader"]
pub struct R(crate::R<RXSLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXSLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXSLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXSLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXSLT` writer"]
pub struct W(crate::W<RXSLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXSLT_SPEC>;
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
impl From<crate::W<RXSLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXSLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Slot enable for the channels."]
pub type EN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EN` writer - Slot enable for the channels."]
pub type EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXSLT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Slot enable for the channels."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Slot enable for the channels."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Slot Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxslt](index.html) module"]
pub struct RXSLT_SPEC;
impl crate::RegisterSpec for RXSLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxslt::R](R) reader structure"]
impl crate::Readable for RXSLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxslt::W](W) writer structure"]
impl crate::Writable for RXSLT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXSLT to value 0"]
impl crate::Resettable for RXSLT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
