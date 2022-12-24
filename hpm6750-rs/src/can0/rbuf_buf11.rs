#[doc = "Register `RBUF_BUF11` reader"]
pub struct R(crate::R<RBUF_BUF11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBUF_BUF11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBUF_BUF11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBUF_BUF11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBUF_BUF11` writer"]
pub struct W(crate::W<RBUF_BUF11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBUF_BUF11_SPEC>;
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
impl From<crate::W<RBUF_BUF11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBUF_BUF11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBUF` reader - receive buffer"]
pub type RBUF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RBUF` writer - receive buffer"]
pub type RBUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBUF_BUF11_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - receive buffer"]
    #[inline(always)]
    pub fn rbuf(&self) -> RBUF_R {
        RBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - receive buffer"]
    #[inline(always)]
    #[must_use]
    pub fn rbuf(&mut self) -> RBUF_W<0> {
        RBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "receive buffer registers and reception time stamp\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbuf_buf11](index.html) module"]
pub struct RBUF_BUF11_SPEC;
impl crate::RegisterSpec for RBUF_BUF11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbuf_buf11::R](R) reader structure"]
impl crate::Readable for RBUF_BUF11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbuf_buf11::W](W) writer structure"]
impl crate::Writable for RBUF_BUF11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RBUF_BUF11 to value 0"]
impl crate::Resettable for RBUF_BUF11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
