#[doc = "Register `RXIPV6_GD_FRMS` reader"]
pub struct R(crate::R<RXIPV6_GD_FRMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIPV6_GD_FRMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIPV6_GD_FRMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIPV6_GD_FRMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXIPV6_GD_FRMS` writer"]
pub struct W(crate::W<RXIPV6_GD_FRMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXIPV6_GD_FRMS_SPEC>;
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
impl From<crate::W<RXIPV6_GD_FRMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXIPV6_GD_FRMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRMCNT` reader - Number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads"]
pub type FRMCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRMCNT` writer - Number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads"]
pub type FRMCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RXIPV6_GD_FRMS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads"]
    #[inline(always)]
    pub fn frmcnt(&self) -> FRMCNT_R {
        FRMCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads"]
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
#[doc = "Number of good IPv6 datagrams received with TCP, UDP, or ICMP payloads\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxipv6_gd_frms](index.html) module"]
pub struct RXIPV6_GD_FRMS_SPEC;
impl crate::RegisterSpec for RXIPV6_GD_FRMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxipv6_gd_frms::R](R) reader structure"]
impl crate::Readable for RXIPV6_GD_FRMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxipv6_gd_frms::W](W) writer structure"]
impl crate::Writable for RXIPV6_GD_FRMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXIPV6_GD_FRMS to value 0"]
impl crate::Resettable for RXIPV6_GD_FRMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}