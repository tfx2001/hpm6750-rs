#[doc = "Register `CHCTRL_CH3_DSTADDR` reader"]
pub struct R(crate::R<CHCTRL_CH3_DSTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTRL_CH3_DSTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTRL_CH3_DSTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTRL_CH3_DSTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTRL_CH3_DSTADDR` writer"]
pub struct W(crate::W<CHCTRL_CH3_DSTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTRL_CH3_DSTADDR_SPEC>;
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
impl From<crate::W<CHCTRL_CH3_DSTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTRL_CH3_DSTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTADDRL` reader - Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
pub type DSTADDRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSTADDRL` writer - Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
pub type DSTADDRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHCTRL_CH3_DSTADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
    #[inline(always)]
    pub fn dstaddrl(&self) -> DSTADDRL_R {
        DSTADDRL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
    #[inline(always)]
    #[must_use]
    pub fn dstaddrl(&mut self) -> DSTADDRL_W<0> {
        DSTADDRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Destination Address Low Part Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrl_ch3_dstaddr](index.html) module"]
pub struct CHCTRL_CH3_DSTADDR_SPEC;
impl crate::RegisterSpec for CHCTRL_CH3_DSTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctrl_ch3_dstaddr::R](R) reader structure"]
impl crate::Readable for CHCTRL_CH3_DSTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctrl_ch3_dstaddr::W](W) writer structure"]
impl crate::Writable for CHCTRL_CH3_DSTADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTRL_CH3_DSTADDR to value 0x01"]
impl crate::Resettable for CHCTRL_CH3_DSTADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
