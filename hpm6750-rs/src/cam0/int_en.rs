#[doc = "Register `INT_EN` reader"]
pub struct R(crate::R<INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_EN` writer"]
pub struct W(crate::W<INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_EN_SPEC>;
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
impl From<crate::W<INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOF_INT_EN` reader - Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt. 0 SOF interrupt disable 1 SOF interrupt enable"]
pub type SOF_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `SOF_INT_EN` writer - Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt. 0 SOF interrupt disable 1 SOF interrupt enable"]
pub type SOF_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `FB1_DMA_DONE_INTEN` reader - Frame Buffer1 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer1 DMA transfer done. 0 Frame Buffer1 DMA Transfer Done interrupt disable 1 Frame Buffer1 DMA Transfer Done interrupt enable"]
pub type FB1_DMA_DONE_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `FB1_DMA_DONE_INTEN` writer - Frame Buffer1 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer1 DMA transfer done. 0 Frame Buffer1 DMA Transfer Done interrupt disable 1 Frame Buffer1 DMA Transfer Done interrupt enable"]
pub type FB1_DMA_DONE_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `FB2_DMA_DONE_INTEN` reader - Frame Buffer2 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer2 DMA transfer done. 0 Frame Buffer2 DMA Transfer Done interrupt disable 1 Frame Buffer2 DMA Transfer Done interrupt enable"]
pub type FB2_DMA_DONE_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `FB2_DMA_DONE_INTEN` writer - Frame Buffer2 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer2 DMA transfer done. 0 Frame Buffer2 DMA Transfer Done interrupt disable 1 Frame Buffer2 DMA Transfer Done interrupt enable"]
pub type FB2_DMA_DONE_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `RF_OR_INTEN` reader - RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt. 0 RxFIFO overrun interrupt is disabled 1 RxFIFO overrun interrupt is enabled"]
pub type RF_OR_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `RF_OR_INTEN` writer - RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt. 0 RxFIFO overrun interrupt is disabled 1 RxFIFO overrun interrupt is enabled"]
pub type RF_OR_INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `EOF_INT_EN` reader - End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt. 0 EOF interrupt is disabled. 1 EOF interrupt is generated when RX count value is reached."]
pub type EOF_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `EOF_INT_EN` writer - End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt. 0 EOF interrupt is disabled. 1 EOF interrupt is generated when RX count value is reached."]
pub type EOF_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `HRESP_ERR_EN` reader - Hresponse Error Enable. This bit enables the hresponse error interrupt. 0 Disable hresponse error interrupt 1 Enable hresponse error interrupt"]
pub type HRESP_ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `HRESP_ERR_EN` writer - Hresponse Error Enable. This bit enables the hresponse error interrupt. 0 Disable hresponse error interrupt 1 Enable hresponse error interrupt"]
pub type HRESP_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `HIST_DONE_INT_EN` reader - Enable hist done int"]
pub type HIST_DONE_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `HIST_DONE_INT_EN` writer - Enable hist done int"]
pub type HIST_DONE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `ERR_CL_BWID_CFG_INT_EN` reader - The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation interrupt enable"]
pub type ERR_CL_BWID_CFG_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ERR_CL_BWID_CFG_INT_EN` writer - The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation interrupt enable"]
pub type ERR_CL_BWID_CFG_INT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt. 0 SOF interrupt disable 1 SOF interrupt enable"]
    #[inline(always)]
    pub fn sof_int_en(&self) -> SOF_INT_EN_R {
        SOF_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Frame Buffer1 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer1 DMA transfer done. 0 Frame Buffer1 DMA Transfer Done interrupt disable 1 Frame Buffer1 DMA Transfer Done interrupt enable"]
    #[inline(always)]
    pub fn fb1_dma_done_inten(&self) -> FB1_DMA_DONE_INTEN_R {
        FB1_DMA_DONE_INTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame Buffer2 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer2 DMA transfer done. 0 Frame Buffer2 DMA Transfer Done interrupt disable 1 Frame Buffer2 DMA Transfer Done interrupt enable"]
    #[inline(always)]
    pub fn fb2_dma_done_inten(&self) -> FB2_DMA_DONE_INTEN_R {
        FB2_DMA_DONE_INTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt. 0 RxFIFO overrun interrupt is disabled 1 RxFIFO overrun interrupt is enabled"]
    #[inline(always)]
    pub fn rf_or_inten(&self) -> RF_OR_INTEN_R {
        RF_OR_INTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt. 0 EOF interrupt is disabled. 1 EOF interrupt is generated when RX count value is reached."]
    #[inline(always)]
    pub fn eof_int_en(&self) -> EOF_INT_EN_R {
        EOF_INT_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Hresponse Error Enable. This bit enables the hresponse error interrupt. 0 Disable hresponse error interrupt 1 Enable hresponse error interrupt"]
    #[inline(always)]
    pub fn hresp_err_en(&self) -> HRESP_ERR_EN_R {
        HRESP_ERR_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable hist done int"]
    #[inline(always)]
    pub fn hist_done_int_en(&self) -> HIST_DONE_INT_EN_R {
        HIST_DONE_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation interrupt enable"]
    #[inline(always)]
    pub fn err_cl_bwid_cfg_int_en(&self) -> ERR_CL_BWID_CFG_INT_EN_R {
        ERR_CL_BWID_CFG_INT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt. 0 SOF interrupt disable 1 SOF interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sof_int_en(&mut self) -> SOF_INT_EN_W<0> {
        SOF_INT_EN_W::new(self)
    }
    #[doc = "Bit 2 - Frame Buffer1 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer1 DMA transfer done. 0 Frame Buffer1 DMA Transfer Done interrupt disable 1 Frame Buffer1 DMA Transfer Done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fb1_dma_done_inten(&mut self) -> FB1_DMA_DONE_INTEN_W<2> {
        FB1_DMA_DONE_INTEN_W::new(self)
    }
    #[doc = "Bit 3 - Frame Buffer2 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer2 DMA transfer done. 0 Frame Buffer2 DMA Transfer Done interrupt disable 1 Frame Buffer2 DMA Transfer Done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fb2_dma_done_inten(&mut self) -> FB2_DMA_DONE_INTEN_W<3> {
        FB2_DMA_DONE_INTEN_W::new(self)
    }
    #[doc = "Bit 6 - RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt. 0 RxFIFO overrun interrupt is disabled 1 RxFIFO overrun interrupt is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rf_or_inten(&mut self) -> RF_OR_INTEN_W<6> {
        RF_OR_INTEN_W::new(self)
    }
    #[doc = "Bit 9 - End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt. 0 EOF interrupt is disabled. 1 EOF interrupt is generated when RX count value is reached."]
    #[inline(always)]
    #[must_use]
    pub fn eof_int_en(&mut self) -> EOF_INT_EN_W<9> {
        EOF_INT_EN_W::new(self)
    }
    #[doc = "Bit 11 - Hresponse Error Enable. This bit enables the hresponse error interrupt. 0 Disable hresponse error interrupt 1 Enable hresponse error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn hresp_err_en(&mut self) -> HRESP_ERR_EN_W<11> {
        HRESP_ERR_EN_W::new(self)
    }
    #[doc = "Bit 12 - Enable hist done int"]
    #[inline(always)]
    #[must_use]
    pub fn hist_done_int_en(&mut self) -> HIST_DONE_INT_EN_W<12> {
        HIST_DONE_INT_EN_W::new(self)
    }
    #[doc = "Bit 13 - The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_cl_bwid_cfg_int_en(&mut self) -> ERR_CL_BWID_CFG_INT_EN_W<13> {
        ERR_CL_BWID_CFG_INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en](index.html) module"]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_en::R](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_en::W](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
