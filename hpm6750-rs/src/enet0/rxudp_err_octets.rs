#[doc = "Register `RXUDP_ERR_OCTETS` reader"]
pub struct R(crate::R<RXUDP_ERR_OCTETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXUDP_ERR_OCTETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXUDP_ERR_OCTETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXUDP_ERR_OCTETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXUDP_ERR_OCTETS` writer"]
pub struct W(crate::W<RXUDP_ERR_OCTETS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXUDP_ERR_OCTETS_SPEC>;
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
impl From<crate::W<RXUDP_ERR_OCTETS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXUDP_ERR_OCTETS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTECNT` reader - Number of bytes received in a UDP segment that had checksum errors"]
pub type BYTECNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BYTECNT` writer - Number of bytes received in a UDP segment that had checksum errors"]
pub type BYTECNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RXUDP_ERR_OCTETS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received in a UDP segment that had checksum errors"]
    #[inline(always)]
    pub fn bytecnt(&self) -> BYTECNT_R {
        BYTECNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes received in a UDP segment that had checksum errors"]
    #[inline(always)]
    pub fn bytecnt(&mut self) -> BYTECNT_W<0> {
        BYTECNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of bytes received in a UDP segment that had checksum errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxudp_err_octets](index.html) module"]
pub struct RXUDP_ERR_OCTETS_SPEC;
impl crate::RegisterSpec for RXUDP_ERR_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxudp_err_octets::R](R) reader structure"]
impl crate::Readable for RXUDP_ERR_OCTETS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxudp_err_octets::W](W) writer structure"]
impl crate::Writable for RXUDP_ERR_OCTETS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXUDP_ERR_OCTETS to value 0"]
impl crate::Resettable for RXUDP_ERR_OCTETS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
