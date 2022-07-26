#[doc = "Register `TBUF_BUF6` reader"]
pub struct R(crate::R<TBUF_BUF6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBUF_BUF6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBUF_BUF6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBUF_BUF6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBUF_BUF6` writer"]
pub struct W(crate::W<TBUF_BUF6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBUF_BUF6_SPEC>;
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
impl From<crate::W<TBUF_BUF6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBUF_BUF6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBUF` reader - transmit buffer"]
pub type TBUF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TBUF` writer - transmit buffer"]
pub type TBUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBUF_BUF6_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - transmit buffer"]
    #[inline(always)]
    pub fn tbuf(&self) -> TBUF_R {
        TBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - transmit buffer"]
    #[inline(always)]
    pub fn tbuf(&mut self) -> TBUF_W<0> {
        TBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "transmit buffer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbuf_buf6](index.html) module"]
pub struct TBUF_BUF6_SPEC;
impl crate::RegisterSpec for TBUF_BUF6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbuf_buf6::R](R) reader structure"]
impl crate::Readable for TBUF_BUF6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbuf_buf6::W](W) writer structure"]
impl crate::Writable for TBUF_BUF6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBUF_BUF6 to value 0"]
impl crate::Resettable for TBUF_BUF6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
