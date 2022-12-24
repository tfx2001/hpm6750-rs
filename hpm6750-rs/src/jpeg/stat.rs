#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESTART_MARKER_ERROR` reader - codec restart marker error interrupt"]
pub type RESTART_MARKER_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `RESTART_MARKER_ERROR` writer - codec restart marker error interrupt"]
pub type RESTART_MARKER_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `CODEC_OVER` reader - Coding or decoding process is over. DMA is not included. The module is completely not busy only when in_dma_transfer_done and out_dma_transfer_done, and codec_over are all asserted."]
pub type CODEC_OVER_R = crate::BitReader<bool>;
#[doc = "Field `CODEC_OVER` writer - Coding or decoding process is over. DMA is not included. The module is completely not busy only when in_dma_transfer_done and out_dma_transfer_done, and codec_over are all asserted."]
pub type CODEC_OVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `IN_DMA_TRANSFER_DONE` reader - InDMA process done"]
pub type IN_DMA_TRANSFER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `IN_DMA_TRANSFER_DONE` writer - InDMA process done"]
pub type IN_DMA_TRANSFER_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `OUT_DMA_TRANSFER_DONE` reader - OutDMA process done"]
pub type OUT_DMA_TRANSFER_DONE_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DMA_TRANSFER_DONE` writer - OutDMA process done"]
pub type OUT_DMA_TRANSFER_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `INXT_IRQ` reader - InDMA next interrupt"]
pub type INXT_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `INXT_IRQ` writer - InDMA next interrupt"]
pub type INXT_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `ONXT_IRQ` reader - OutDMA next interrupt"]
pub type ONXT_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `ONXT_IRQ` writer - OutDMA next interrupt"]
pub type ONXT_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `AXI_ERR` reader - axi bus error"]
pub type AXI_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AXI_ERR` writer - axi bus error"]
pub type AXI_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `AXI_WRITE_ERR` reader - out-dma axi bus error"]
pub type AXI_WRITE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AXI_READ_ERR` reader - in-dma axi bus error"]
pub type AXI_READ_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AXI_ERR_ID` reader - the axi err id"]
pub type AXI_ERR_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSY` reader - When 1 means that the module is busy doing conversion and data transfer."]
pub type BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - codec restart marker error interrupt"]
    #[inline(always)]
    pub fn restart_marker_error(&self) -> RESTART_MARKER_ERROR_R {
        RESTART_MARKER_ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Coding or decoding process is over. DMA is not included. The module is completely not busy only when in_dma_transfer_done and out_dma_transfer_done, and codec_over are all asserted."]
    #[inline(always)]
    pub fn codec_over(&self) -> CODEC_OVER_R {
        CODEC_OVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - InDMA process done"]
    #[inline(always)]
    pub fn in_dma_transfer_done(&self) -> IN_DMA_TRANSFER_DONE_R {
        IN_DMA_TRANSFER_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OutDMA process done"]
    #[inline(always)]
    pub fn out_dma_transfer_done(&self) -> OUT_DMA_TRANSFER_DONE_R {
        OUT_DMA_TRANSFER_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - InDMA next interrupt"]
    #[inline(always)]
    pub fn inxt_irq(&self) -> INXT_IRQ_R {
        INXT_IRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OutDMA next interrupt"]
    #[inline(always)]
    pub fn onxt_irq(&self) -> ONXT_IRQ_R {
        ONXT_IRQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - axi bus error"]
    #[inline(always)]
    pub fn axi_err(&self) -> AXI_ERR_R {
        AXI_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - out-dma axi bus error"]
    #[inline(always)]
    pub fn axi_write_err(&self) -> AXI_WRITE_ERR_R {
        AXI_WRITE_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - in-dma axi bus error"]
    #[inline(always)]
    pub fn axi_read_err(&self) -> AXI_READ_ERR_R {
        AXI_READ_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - the axi err id"]
    #[inline(always)]
    pub fn axi_err_id(&self) -> AXI_ERR_ID_R {
        AXI_ERR_ID_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - When 1 means that the module is busy doing conversion and data transfer."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - codec restart marker error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn restart_marker_error(&mut self) -> RESTART_MARKER_ERROR_W<1> {
        RESTART_MARKER_ERROR_W::new(self)
    }
    #[doc = "Bit 2 - Coding or decoding process is over. DMA is not included. The module is completely not busy only when in_dma_transfer_done and out_dma_transfer_done, and codec_over are all asserted."]
    #[inline(always)]
    #[must_use]
    pub fn codec_over(&mut self) -> CODEC_OVER_W<2> {
        CODEC_OVER_W::new(self)
    }
    #[doc = "Bit 3 - InDMA process done"]
    #[inline(always)]
    #[must_use]
    pub fn in_dma_transfer_done(&mut self) -> IN_DMA_TRANSFER_DONE_W<3> {
        IN_DMA_TRANSFER_DONE_W::new(self)
    }
    #[doc = "Bit 4 - OutDMA process done"]
    #[inline(always)]
    #[must_use]
    pub fn out_dma_transfer_done(&mut self) -> OUT_DMA_TRANSFER_DONE_W<4> {
        OUT_DMA_TRANSFER_DONE_W::new(self)
    }
    #[doc = "Bit 5 - InDMA next interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn inxt_irq(&mut self) -> INXT_IRQ_W<5> {
        INXT_IRQ_W::new(self)
    }
    #[doc = "Bit 6 - OutDMA next interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn onxt_irq(&mut self) -> ONXT_IRQ_W<6> {
        ONXT_IRQ_W::new(self)
    }
    #[doc = "Bit 7 - axi bus error"]
    #[inline(always)]
    #[must_use]
    pub fn axi_err(&mut self) -> AXI_ERR_W<7> {
        AXI_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
