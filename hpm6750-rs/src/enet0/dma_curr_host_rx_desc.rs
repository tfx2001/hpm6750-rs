#[doc = "Register `DMA_CURR_HOST_RX_DESC` reader"]
pub struct R(crate::R<DMA_CURR_HOST_RX_DESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CURR_HOST_RX_DESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CURR_HOST_RX_DESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CURR_HOST_RX_DESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CURR_HOST_RX_DESC` writer"]
pub struct W(crate::W<DMA_CURR_HOST_RX_DESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CURR_HOST_RX_DESC_SPEC>;
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
impl From<crate::W<DMA_CURR_HOST_RX_DESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CURR_HOST_RX_DESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURRDESAPTR` reader - Host Receive Descriptor Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CURRDESAPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CURRDESAPTR` writer - Host Receive Descriptor Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
pub type CURRDESAPTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_CURR_HOST_RX_DESC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer Cleared on Reset. Pointer updated by the DMA during operation."]
    #[inline(always)]
    #[must_use]
    pub fn currdesaptr(&mut self) -> CURRDESAPTR_W<0> {
        CURRDESAPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Host Receive Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_curr_host_rx_desc](index.html) module"]
pub struct DMA_CURR_HOST_RX_DESC_SPEC;
impl crate::RegisterSpec for DMA_CURR_HOST_RX_DESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_curr_host_rx_desc::R](R) reader structure"]
impl crate::Readable for DMA_CURR_HOST_RX_DESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_curr_host_rx_desc::W](W) writer structure"]
impl crate::Writable for DMA_CURR_HOST_RX_DESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_CURR_HOST_RX_DESC to value 0"]
impl crate::Resettable for DMA_CURR_HOST_RX_DESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
