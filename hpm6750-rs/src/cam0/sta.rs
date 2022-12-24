#[doc = "Register `STA` reader"]
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STA` writer"]
pub struct W(crate::W<STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STA_SPEC>;
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
impl From<crate::W<STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HRESP_ERR_INT` reader - Hresponse Error Interrupt Status. Indicates that a hresponse error has been detected. (Cleared by writing 1) 0 No hresponse error. 1 Hresponse error is detected."]
pub type HRESP_ERR_INT_R = crate::BitReader<bool>;
#[doc = "Field `HRESP_ERR_INT` writer - Hresponse Error Interrupt Status. Indicates that a hresponse error has been detected. (Cleared by writing 1) 0 No hresponse error. 1 Hresponse error is detected."]
pub type HRESP_ERR_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `SOF_INT` reader - Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1) 0 SOF is not detected. 1 SOF is detected."]
pub type SOF_INT_R = crate::BitReader<bool>;
#[doc = "Field `SOF_INT` writer - Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1) 0 SOF is not detected. 1 SOF is detected."]
pub type SOF_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `EOF_INT` reader - End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1) 0 EOF is not detected. 1 EOF is detected."]
pub type EOF_INT_R = crate::BitReader<bool>;
#[doc = "Field `EOF_INT` writer - End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1) 0 EOF is not detected. 1 EOF is detected."]
pub type EOF_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `DMA_TSF_DONE_FB1` reader - DMA Transfer Done in Frame Buffer1. Indicates that the DMA transfer from RxFIFO to Frame Buffer1 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writting 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
pub type DMA_TSF_DONE_FB1_R = crate::BitReader<bool>;
#[doc = "Field `DMA_TSF_DONE_FB1` writer - DMA Transfer Done in Frame Buffer1. Indicates that the DMA transfer from RxFIFO to Frame Buffer1 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writting 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
pub type DMA_TSF_DONE_FB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `DMA_TSF_DONE_FB2` reader - DMA Transfer Done in Frame Buffer2. Indicates that the DMA transfer from RxFIFO to Frame Buffer2 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writting 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
pub type DMA_TSF_DONE_FB2_R = crate::BitReader<bool>;
#[doc = "Field `DMA_TSF_DONE_FB2` writer - DMA Transfer Done in Frame Buffer2. Indicates that the DMA transfer from RxFIFO to Frame Buffer2 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writting 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
pub type DMA_TSF_DONE_FB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `RF_OR_INT` reader - RxFIFO Overrun Interrupt Status. Indicates the overflow status of the RxFIFO register. (Cleared by writing 1) 0 RXFIFO has not overflowed. 1 RXFIFO has overflowed."]
pub type RF_OR_INT_R = crate::BitReader<bool>;
#[doc = "Field `RF_OR_INT` writer - RxFIFO Overrun Interrupt Status. Indicates the overflow status of the RxFIFO register. (Cleared by writing 1) 0 RXFIFO has not overflowed. 1 RXFIFO has overflowed."]
pub type RF_OR_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `HIST_DONE` reader - hist cal done"]
pub type HIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `HIST_DONE` writer - hist cal done"]
pub type HIST_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `ERR_CL_BWID_CFG` reader - The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation found"]
pub type ERR_CL_BWID_CFG_R = crate::BitReader<bool>;
#[doc = "Field `ERR_CL_BWID_CFG` writer - The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation found"]
pub type ERR_CL_BWID_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Hresponse Error Interrupt Status. Indicates that a hresponse error has been detected. (Cleared by writing 1) 0 No hresponse error. 1 Hresponse error is detected."]
    #[inline(always)]
    pub fn hresp_err_int(&self) -> HRESP_ERR_INT_R {
        HRESP_ERR_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1) 0 SOF is not detected. 1 SOF is detected."]
    #[inline(always)]
    pub fn sof_int(&self) -> SOF_INT_R {
        SOF_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1) 0 EOF is not detected. 1 EOF is detected."]
    #[inline(always)]
    pub fn eof_int(&self) -> EOF_INT_R {
        EOF_INT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA Transfer Done in Frame Buffer1. Indicates that the DMA transfer from RxFIFO to Frame Buffer1 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writting 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
    #[inline(always)]
    pub fn dma_tsf_done_fb1(&self) -> DMA_TSF_DONE_FB1_R {
        DMA_TSF_DONE_FB1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA Transfer Done in Frame Buffer2. Indicates that the DMA transfer from RxFIFO to Frame Buffer2 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writting 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
    #[inline(always)]
    pub fn dma_tsf_done_fb2(&self) -> DMA_TSF_DONE_FB2_R {
        DMA_TSF_DONE_FB2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - RxFIFO Overrun Interrupt Status. Indicates the overflow status of the RxFIFO register. (Cleared by writing 1) 0 RXFIFO has not overflowed. 1 RXFIFO has overflowed."]
    #[inline(always)]
    pub fn rf_or_int(&self) -> RF_OR_INT_R {
        RF_OR_INT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 18 - hist cal done"]
    #[inline(always)]
    pub fn hist_done(&self) -> HIST_DONE_R {
        HIST_DONE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation found"]
    #[inline(always)]
    pub fn err_cl_bwid_cfg(&self) -> ERR_CL_BWID_CFG_R {
        ERR_CL_BWID_CFG_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Hresponse Error Interrupt Status. Indicates that a hresponse error has been detected. (Cleared by writing 1) 0 No hresponse error. 1 Hresponse error is detected."]
    #[inline(always)]
    #[must_use]
    pub fn hresp_err_int(&mut self) -> HRESP_ERR_INT_W<2> {
        HRESP_ERR_INT_W::new(self)
    }
    #[doc = "Bit 6 - Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1) 0 SOF is not detected. 1 SOF is detected."]
    #[inline(always)]
    #[must_use]
    pub fn sof_int(&mut self) -> SOF_INT_W<6> {
        SOF_INT_W::new(self)
    }
    #[doc = "Bit 7 - End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1) 0 EOF is not detected. 1 EOF is detected."]
    #[inline(always)]
    #[must_use]
    pub fn eof_int(&mut self) -> EOF_INT_W<7> {
        EOF_INT_W::new(self)
    }
    #[doc = "Bit 9 - DMA Transfer Done in Frame Buffer1. Indicates that the DMA transfer from RxFIFO to Frame Buffer1 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writting 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tsf_done_fb1(&mut self) -> DMA_TSF_DONE_FB1_W<9> {
        DMA_TSF_DONE_FB1_W::new(self)
    }
    #[doc = "Bit 10 - DMA Transfer Done in Frame Buffer2. Indicates that the DMA transfer from RxFIFO to Frame Buffer2 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writting 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tsf_done_fb2(&mut self) -> DMA_TSF_DONE_FB2_W<10> {
        DMA_TSF_DONE_FB2_W::new(self)
    }
    #[doc = "Bit 13 - RxFIFO Overrun Interrupt Status. Indicates the overflow status of the RxFIFO register. (Cleared by writing 1) 0 RXFIFO has not overflowed. 1 RXFIFO has overflowed."]
    #[inline(always)]
    #[must_use]
    pub fn rf_or_int(&mut self) -> RF_OR_INT_W<13> {
        RF_OR_INT_W::new(self)
    }
    #[doc = "Bit 18 - hist cal done"]
    #[inline(always)]
    #[must_use]
    pub fn hist_done(&mut self) -> HIST_DONE_W<18> {
        HIST_DONE_W::new(self)
    }
    #[doc = "Bit 19 - The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation found"]
    #[inline(always)]
    #[must_use]
    pub fn err_cl_bwid_cfg(&mut self) -> ERR_CL_BWID_CFG_W<19> {
        ERR_CL_BWID_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](index.html) module"]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sta::R](R) reader structure"]
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sta::W](W) writer structure"]
impl crate::Writable for STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
