#[doc = "Register `CHCTRL_CH4_DSTADDRH` reader"]
pub struct R(crate::R<CHCTRL_CH4_DSTADDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTRL_CH4_DSTADDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTRL_CH4_DSTADDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTRL_CH4_DSTADDRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTRL_CH4_DSTADDRH` writer"]
pub struct W(crate::W<CHCTRL_CH4_DSTADDRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTRL_CH4_DSTADDRH_SPEC>;
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
impl From<crate::W<CHCTRL_CH4_DSTADDRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTRL_CH4_DSTADDRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTADDRH` reader - High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
pub type DSTADDRH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSTADDRH` writer - High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
pub type DSTADDRH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHCTRL_CH4_DSTADDRH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
    #[inline(always)]
    pub fn dstaddrh(&self) -> DSTADDRH_R {
        DSTADDRH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn dstaddrh(&mut self) -> DSTADDRH_W<0> {
        DSTADDRH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Destination Address High Part Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrl_ch4_dstaddrh](index.html) module"]
pub struct CHCTRL_CH4_DSTADDRH_SPEC;
impl crate::RegisterSpec for CHCTRL_CH4_DSTADDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctrl_ch4_dstaddrh::R](R) reader structure"]
impl crate::Readable for CHCTRL_CH4_DSTADDRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctrl_ch4_dstaddrh::W](W) writer structure"]
impl crate::Writable for CHCTRL_CH4_DSTADDRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTRL_CH4_DSTADDRH to value 0x01"]
impl crate::Resettable for CHCTRL_CH4_DSTADDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
