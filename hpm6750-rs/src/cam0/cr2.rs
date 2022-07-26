#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRMCNT_15_0` reader - Frame Counter. This is a 16-bit Frame Counter (Wraps around automatically after reaching the maximum)"]
pub type FRMCNT_15_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRMCNT_RST` reader - Frame Count Reset. Resets the Frame Counter. 0 Do not reset 1 Reset frame counter immediately"]
pub type FRMCNT_RST_R = crate::BitReader<bool>;
#[doc = "Field `FRMCNT_RST` writer - Frame Count Reset. Resets the Frame Counter. 0 Do not reset 1 Reset frame counter immediately"]
pub type FRMCNT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RXFF_LEVEL` reader - RxFIFO Full Level. When the number of data in RxFIFO reaches this level, a RxFIFO full interrupt is generated, or an RXFIFO DMA request is sent. 000 4 Double words 001 8 Double words 010 16 Double words 011 24 Double words 100 32 Double words 101 48 Double words 110 64 Double words 111 96 Double words"]
pub type RXFF_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFF_LEVEL` writer - RxFIFO Full Level. When the number of data in RxFIFO reaches this level, a RxFIFO full interrupt is generated, or an RXFIFO DMA request is sent. 000 4 Double words 001 8 Double words 010 16 Double words 011 24 Double words 100 32 Double words 101 48 Double words 110 64 Double words 111 96 Double words"]
pub type RXFF_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `DMA_REQ_EN_RFF` reader - DMA Request Enable for RxFIFO. This bit enables the dma request from RxFIFO to the embedded DMA controller. 0 Disable the dma request 1 Enable the dma request. The UV Rx FIFO is only enabled to filling data in 2 plane mode."]
pub type DMA_REQ_EN_RFF_R = crate::BitReader<bool>;
#[doc = "Field `DMA_REQ_EN_RFF` writer - DMA Request Enable for RxFIFO. This bit enables the dma request from RxFIFO to the embedded DMA controller. 0 Disable the dma request 1 Enable the dma request. The UV Rx FIFO is only enabled to filling data in 2 plane mode."]
pub type DMA_REQ_EN_RFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CLRBITFORMAT` reader - Input Byte & bit sequence same as OV5640, except for Raw mode. Used only for internal ARGB conversion."]
pub type CLRBITFORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLRBITFORMAT` writer - Input Byte & bit sequence same as OV5640, except for Raw mode. Used only for internal ARGB conversion."]
pub type CLRBITFORMAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 16:31 - Frame Counter. This is a 16-bit Frame Counter (Wraps around automatically after reaching the maximum)"]
    #[inline(always)]
    pub fn frmcnt_15_0(&self) -> FRMCNT_15_0_R {
        FRMCNT_15_0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15 - Frame Count Reset. Resets the Frame Counter. 0 Do not reset 1 Reset frame counter immediately"]
    #[inline(always)]
    pub fn frmcnt_rst(&self) -> FRMCNT_RST_R {
        FRMCNT_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 9:11 - RxFIFO Full Level. When the number of data in RxFIFO reaches this level, a RxFIFO full interrupt is generated, or an RXFIFO DMA request is sent. 000 4 Double words 001 8 Double words 010 16 Double words 011 24 Double words 100 32 Double words 101 48 Double words 110 64 Double words 111 96 Double words"]
    #[inline(always)]
    pub fn rxff_level(&self) -> RXFF_LEVEL_R {
        RXFF_LEVEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 5 - DMA Request Enable for RxFIFO. This bit enables the dma request from RxFIFO to the embedded DMA controller. 0 Disable the dma request 1 Enable the dma request. The UV Rx FIFO is only enabled to filling data in 2 plane mode."]
    #[inline(always)]
    pub fn dma_req_en_rff(&self) -> DMA_REQ_EN_RFF_R {
        DMA_REQ_EN_RFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 0:3 - Input Byte & bit sequence same as OV5640, except for Raw mode. Used only for internal ARGB conversion."]
    #[inline(always)]
    pub fn clrbitformat(&self) -> CLRBITFORMAT_R {
        CLRBITFORMAT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Frame Count Reset. Resets the Frame Counter. 0 Do not reset 1 Reset frame counter immediately"]
    #[inline(always)]
    pub fn frmcnt_rst(&mut self) -> FRMCNT_RST_W<15> {
        FRMCNT_RST_W::new(self)
    }
    #[doc = "Bits 9:11 - RxFIFO Full Level. When the number of data in RxFIFO reaches this level, a RxFIFO full interrupt is generated, or an RXFIFO DMA request is sent. 000 4 Double words 001 8 Double words 010 16 Double words 011 24 Double words 100 32 Double words 101 48 Double words 110 64 Double words 111 96 Double words"]
    #[inline(always)]
    pub fn rxff_level(&mut self) -> RXFF_LEVEL_W<9> {
        RXFF_LEVEL_W::new(self)
    }
    #[doc = "Bit 5 - DMA Request Enable for RxFIFO. This bit enables the dma request from RxFIFO to the embedded DMA controller. 0 Disable the dma request 1 Enable the dma request. The UV Rx FIFO is only enabled to filling data in 2 plane mode."]
    #[inline(always)]
    pub fn dma_req_en_rff(&mut self) -> DMA_REQ_EN_RFF_W<5> {
        DMA_REQ_EN_RFF_W::new(self)
    }
    #[doc = "Bits 0:3 - Input Byte & bit sequence same as OV5640, except for Raw mode. Used only for internal ARGB conversion."]
    #[inline(always)]
    pub fn clrbitformat(&mut self) -> CLRBITFORMAT_W<0> {
        CLRBITFORMAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
