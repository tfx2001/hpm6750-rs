#[doc = "Register `SEQ_DMA_CFG` reader"]
pub struct R(crate::R<SEQ_DMA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_DMA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_DMA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_DMA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ_DMA_CFG` writer"]
pub struct W(crate::W<SEQ_DMA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ_DMA_CFG_SPEC>;
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
impl From<crate::W<SEQ_DMA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ_DMA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP_POS` reader - if stop_en is set, SW is responsible to udpate this field to the next read point, HW should not write data to this point since it's not read out by SW yet"]
pub type STOP_POS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STOP_POS` writer - if stop_en is set, SW is responsible to udpate this field to the next read point, HW should not write data to this point since it's not read out by SW yet"]
pub type STOP_POS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEQ_DMA_CFG_SPEC, u16, u16, 12, O>;
#[doc = "Field `DMA_RST` reader - set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst"]
pub type DMA_RST_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RST` writer - set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst"]
pub type DMA_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_DMA_CFG_SPEC, bool, O>;
#[doc = "Field `STOP_EN` reader - set to stop dma if reach the stop_pos"]
pub type STOP_EN_R = crate::BitReader<bool>;
#[doc = "Field `STOP_EN` writer - set to stop dma if reach the stop_pos"]
pub type STOP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQ_DMA_CFG_SPEC, bool, O>;
#[doc = "Field `BUF_LEN` reader - dma buffer length, after write to (tar_addr\\[31:2\\]+buf_len)*4, the next dma address will be tar_addr\\[31:2\\]*4 0 for 4byte; 0xFFF for 16kbyte."]
pub type BUF_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BUF_LEN` writer - dma buffer length, after write to (tar_addr\\[31:2\\]+buf_len)*4, the next dma address will be tar_addr\\[31:2\\]*4 0 for 4byte; 0xFFF for 16kbyte."]
pub type BUF_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEQ_DMA_CFG_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 16:27 - if stop_en is set, SW is responsible to udpate this field to the next read point, HW should not write data to this point since it's not read out by SW yet"]
    #[inline(always)]
    pub fn stop_pos(&self) -> STOP_POS_R {
        STOP_POS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 13 - set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst"]
    #[inline(always)]
    pub fn dma_rst(&self) -> DMA_RST_R {
        DMA_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - set to stop dma if reach the stop_pos"]
    #[inline(always)]
    pub fn stop_en(&self) -> STOP_EN_R {
        STOP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 0:11 - dma buffer length, after write to (tar_addr\\[31:2\\]+buf_len)*4, the next dma address will be tar_addr\\[31:2\\]*4 0 for 4byte; 0xFFF for 16kbyte."]
    #[inline(always)]
    pub fn buf_len(&self) -> BUF_LEN_R {
        BUF_LEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - if stop_en is set, SW is responsible to udpate this field to the next read point, HW should not write data to this point since it's not read out by SW yet"]
    #[inline(always)]
    pub fn stop_pos(&mut self) -> STOP_POS_W<16> {
        STOP_POS_W::new(self)
    }
    #[doc = "Bit 13 - set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst"]
    #[inline(always)]
    pub fn dma_rst(&mut self) -> DMA_RST_W<13> {
        DMA_RST_W::new(self)
    }
    #[doc = "Bit 12 - set to stop dma if reach the stop_pos"]
    #[inline(always)]
    pub fn stop_en(&mut self) -> STOP_EN_W<12> {
        STOP_EN_W::new(self)
    }
    #[doc = "Bits 0:11 - dma buffer length, after write to (tar_addr\\[31:2\\]+buf_len)*4, the next dma address will be tar_addr\\[31:2\\]*4 0 for 4byte; 0xFFF for 16kbyte."]
    #[inline(always)]
    pub fn buf_len(&mut self) -> BUF_LEN_W<0> {
        BUF_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_dma_cfg](index.html) module"]
pub struct SEQ_DMA_CFG_SPEC;
impl crate::RegisterSpec for SEQ_DMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_dma_cfg::R](R) reader structure"]
impl crate::Readable for SEQ_DMA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq_dma_cfg::W](W) writer structure"]
impl crate::Writable for SEQ_DMA_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQ_DMA_CFG to value 0"]
impl crate::Resettable for SEQ_DMA_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
