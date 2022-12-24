#[doc = "Register `TRG_DMA_ADDR` reader"]
pub struct R(crate::R<TRG_DMA_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRG_DMA_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRG_DMA_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRG_DMA_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRG_DMA_ADDR` writer"]
pub struct W(crate::W<TRG_DMA_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRG_DMA_ADDR_SPEC>;
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
impl From<crate::W<TRG_DMA_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRG_DMA_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRG_DMA_ADDR` reader - buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)"]
pub type TRG_DMA_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TRG_DMA_ADDR` writer - buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)"]
pub type TRG_DMA_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRG_DMA_ADDR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)"]
    #[inline(always)]
    pub fn trg_dma_addr(&self) -> TRG_DMA_ADDR_R {
        TRG_DMA_ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)"]
    #[inline(always)]
    #[must_use]
    pub fn trg_dma_addr(&mut self) -> TRG_DMA_ADDR_W<2> {
        TRG_DMA_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trg_dma_addr](index.html) module"]
pub struct TRG_DMA_ADDR_SPEC;
impl crate::RegisterSpec for TRG_DMA_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trg_dma_addr::R](R) reader structure"]
impl crate::Readable for TRG_DMA_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trg_dma_addr::W](W) writer structure"]
impl crate::Writable for TRG_DMA_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRG_DMA_ADDR to value 0"]
impl crate::Resettable for TRG_DMA_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
