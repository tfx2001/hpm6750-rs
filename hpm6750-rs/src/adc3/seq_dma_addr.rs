#[doc = "Register `SEQ_DMA_ADDR` reader"]
pub struct R(crate::R<SEQ_DMA_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_DMA_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_DMA_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_DMA_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ_DMA_ADDR` writer"]
pub struct W(crate::W<SEQ_DMA_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ_DMA_ADDR_SPEC>;
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
impl From<crate::W<SEQ_DMA_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ_DMA_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAR_ADDR` reader - dma target address, should be 4-byte aligned"]
pub type TAR_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TAR_ADDR` writer - dma target address, should be 4-byte aligned"]
pub type TAR_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEQ_DMA_ADDR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - dma target address, should be 4-byte aligned"]
    #[inline(always)]
    pub fn tar_addr(&self) -> TAR_ADDR_R {
        TAR_ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - dma target address, should be 4-byte aligned"]
    #[inline(always)]
    #[must_use]
    pub fn tar_addr(&mut self) -> TAR_ADDR_W<2> {
        TAR_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_dma_addr](index.html) module"]
pub struct SEQ_DMA_ADDR_SPEC;
impl crate::RegisterSpec for SEQ_DMA_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_dma_addr::R](R) reader structure"]
impl crate::Readable for SEQ_DMA_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq_dma_addr::W](W) writer structure"]
impl crate::Writable for SEQ_DMA_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ_DMA_ADDR to value 0"]
impl crate::Resettable for SEQ_DMA_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
