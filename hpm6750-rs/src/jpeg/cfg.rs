#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JD_UVSWAP` reader - Normally the default CbCr sequence is that Cb macro block coming before Cr macro blk. If Cr macro block is first, set this bit to 1'b1. This bit only impact the color space conversion from/to RGB."]
pub type JD_UVSWAP_R = crate::BitReader<bool>;
#[doc = "Field `JD_UVSWAP` writer - Normally the default CbCr sequence is that Cb macro block coming before Cr macro blk. If Cr macro block is first, set this bit to 1'b1. This bit only impact the color space conversion from/to RGB."]
pub type JD_UVSWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CFG_IPATH_SEL` reader - 2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as B,R 2'b11: YUV422H, byte sequence as Y0,U0,Y1,V0"]
pub type CFG_IPATH_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG_IPATH_SEL` writer - 2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as B,R 2'b11: YUV422H, byte sequence as Y0,U0,Y1,V0"]
pub type CFG_IPATH_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CODEC_OVER_IRQ_EN` reader - The jpg endec process done interrupt enable"]
pub type CODEC_OVER_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `CODEC_OVER_IRQ_EN` writer - The jpg endec process done interrupt enable"]
pub type CODEC_OVER_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CODEC_RESTART_ERR_IRQ_EN` reader - The jpg endec restart error interrupt enable"]
pub type CODEC_RESTART_ERR_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `CODEC_RESTART_ERR_IRQ_EN` writer - The jpg endec restart error interrupt enable"]
pub type CODEC_RESTART_ERR_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `MEM_DEBUG_CLK_SEL` reader - asserted to use APB clock, so that the memory contents could be read out through APB interface"]
pub type MEM_DEBUG_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `MEM_DEBUG_CLK_SEL` writer - asserted to use APB clock, so that the memory contents could be read out through APB interface"]
pub type MEM_DEBUG_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CLKGATE` reader - Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
pub type CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `CLKGATE` writer - Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
pub type CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CFG_OPATH_SEL` reader - 2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as R,B 2'b11: YUV422H1P, byte sequence as Y0,U0,Y1,V0"]
pub type CFG_OPATH_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG_OPATH_SEL` writer - 2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as R,B 2'b11: YUV422H1P, byte sequence as Y0,U0,Y1,V0"]
pub type CFG_OPATH_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `JDATA_FORMAT` reader - 3'b000: for 420, hy=2, vy=2, hc=1, vc=1 // 6 sub-blocks per MCU 3'b001: for 422h, hy=2, vy=1, hc=1, vc=1 // 4 sub-blocks per MCU 3'b010: for 422v, hy=1, vy=2, hc=1, vc=1 // 4 sub-blocks per MCU 3'b011: for 444, hy=1, vy=1, hc=1, vc=1 // 3 sub-blocks per MCU 3'b100: for 400, hy=2, vy=2, hc=0, vc=0 // 4 sub-blocks per MCU Others: Undefined"]
pub type JDATA_FORMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JDATA_FORMAT` writer - 3'b000: for 420, hy=2, vy=2, hc=1, vc=1 // 6 sub-blocks per MCU 3'b001: for 422h, hy=2, vy=1, hc=1, vc=1 // 4 sub-blocks per MCU 3'b010: for 422v, hy=1, vy=2, hc=1, vc=1 // 4 sub-blocks per MCU 3'b011: for 444, hy=1, vy=1, hc=1, vc=1 // 3 sub-blocks per MCU 3'b100: for 400, hy=2, vy=2, hc=0, vc=0 // 4 sub-blocks per MCU Others: Undefined"]
pub type JDATA_FORMAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `JPEG_SFTRST` reader - Software Reset"]
pub type JPEG_SFTRST_R = crate::BitReader<bool>;
#[doc = "Field `JPEG_SFTRST` writer - Software Reset"]
pub type JPEG_SFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `START` reader - Asserted if to start a new encoder/decoder conversion. It will at first stop the inner JPEG module, then reset it, and then re-run it. It is a different mode from DMA phase mode. It cannot be configured in the DMA chain descriptor. It should be configured by the core processor. Auto clear."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Asserted if to start a new encoder/decoder conversion. It will at first stop the inner JPEG module, then reset it, and then re-run it. It is a different mode from DMA phase mode. It cannot be configured in the DMA chain descriptor. It should be configured by the core processor. Auto clear."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `MODE` reader - 1: decoder, 0:encoder"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - 1: decoder, 0:encoder"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `JPEG_EN` reader - 1b - Enabled"]
pub type JPEG_EN_R = crate::BitReader<bool>;
#[doc = "Field `JPEG_EN` writer - 1b - Enabled"]
pub type JPEG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 22 - Normally the default CbCr sequence is that Cb macro block coming before Cr macro blk. If Cr macro block is first, set this bit to 1'b1. This bit only impact the color space conversion from/to RGB."]
    #[inline(always)]
    pub fn jd_uvswap(&self) -> JD_UVSWAP_R {
        JD_UVSWAP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 20:21 - 2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as B,R 2'b11: YUV422H, byte sequence as Y0,U0,Y1,V0"]
    #[inline(always)]
    pub fn cfg_ipath_sel(&self) -> CFG_IPATH_SEL_R {
        CFG_IPATH_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 19 - The jpg endec process done interrupt enable"]
    #[inline(always)]
    pub fn codec_over_irq_en(&self) -> CODEC_OVER_IRQ_EN_R {
        CODEC_OVER_IRQ_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - The jpg endec restart error interrupt enable"]
    #[inline(always)]
    pub fn codec_restart_err_irq_en(&self) -> CODEC_RESTART_ERR_IRQ_EN_R {
        CODEC_RESTART_ERR_IRQ_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - asserted to use APB clock, so that the memory contents could be read out through APB interface"]
    #[inline(always)]
    pub fn mem_debug_clk_sel(&self) -> MEM_DEBUG_CLK_SEL_R {
        MEM_DEBUG_CLK_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 9 - Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 7:8 - 2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as R,B 2'b11: YUV422H1P, byte sequence as Y0,U0,Y1,V0"]
    #[inline(always)]
    pub fn cfg_opath_sel(&self) -> CFG_OPATH_SEL_R {
        CFG_OPATH_SEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 4:6 - 3'b000: for 420, hy=2, vy=2, hc=1, vc=1 // 6 sub-blocks per MCU 3'b001: for 422h, hy=2, vy=1, hc=1, vc=1 // 4 sub-blocks per MCU 3'b010: for 422v, hy=1, vy=2, hc=1, vc=1 // 4 sub-blocks per MCU 3'b011: for 444, hy=1, vy=1, hc=1, vc=1 // 3 sub-blocks per MCU 3'b100: for 400, hy=2, vy=2, hc=0, vc=0 // 4 sub-blocks per MCU Others: Undefined"]
    #[inline(always)]
    pub fn jdata_format(&self) -> JDATA_FORMAT_R {
        JDATA_FORMAT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - Software Reset"]
    #[inline(always)]
    pub fn jpeg_sftrst(&self) -> JPEG_SFTRST_R {
        JPEG_SFTRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Asserted if to start a new encoder/decoder conversion. It will at first stop the inner JPEG module, then reset it, and then re-run it. It is a different mode from DMA phase mode. It cannot be configured in the DMA chain descriptor. It should be configured by the core processor. Auto clear."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - 1: decoder, 0:encoder"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - 1b - Enabled"]
    #[inline(always)]
    pub fn jpeg_en(&self) -> JPEG_EN_R {
        JPEG_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - Normally the default CbCr sequence is that Cb macro block coming before Cr macro blk. If Cr macro block is first, set this bit to 1'b1. This bit only impact the color space conversion from/to RGB."]
    #[inline(always)]
    pub fn jd_uvswap(&mut self) -> JD_UVSWAP_W<22> {
        JD_UVSWAP_W::new(self)
    }
    #[doc = "Bits 20:21 - 2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as B,R 2'b11: YUV422H, byte sequence as Y0,U0,Y1,V0"]
    #[inline(always)]
    pub fn cfg_ipath_sel(&mut self) -> CFG_IPATH_SEL_W<20> {
        CFG_IPATH_SEL_W::new(self)
    }
    #[doc = "Bit 19 - The jpg endec process done interrupt enable"]
    #[inline(always)]
    pub fn codec_over_irq_en(&mut self) -> CODEC_OVER_IRQ_EN_W<19> {
        CODEC_OVER_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 18 - The jpg endec restart error interrupt enable"]
    #[inline(always)]
    pub fn codec_restart_err_irq_en(&mut self) -> CODEC_RESTART_ERR_IRQ_EN_W<18> {
        CODEC_RESTART_ERR_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 17 - asserted to use APB clock, so that the memory contents could be read out through APB interface"]
    #[inline(always)]
    pub fn mem_debug_clk_sel(&mut self) -> MEM_DEBUG_CLK_SEL_W<17> {
        MEM_DEBUG_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 9 - Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
    #[inline(always)]
    pub fn clkgate(&mut self) -> CLKGATE_W<9> {
        CLKGATE_W::new(self)
    }
    #[doc = "Bits 7:8 - 2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as R,B 2'b11: YUV422H1P, byte sequence as Y0,U0,Y1,V0"]
    #[inline(always)]
    pub fn cfg_opath_sel(&mut self) -> CFG_OPATH_SEL_W<7> {
        CFG_OPATH_SEL_W::new(self)
    }
    #[doc = "Bits 4:6 - 3'b000: for 420, hy=2, vy=2, hc=1, vc=1 // 6 sub-blocks per MCU 3'b001: for 422h, hy=2, vy=1, hc=1, vc=1 // 4 sub-blocks per MCU 3'b010: for 422v, hy=1, vy=2, hc=1, vc=1 // 4 sub-blocks per MCU 3'b011: for 444, hy=1, vy=1, hc=1, vc=1 // 3 sub-blocks per MCU 3'b100: for 400, hy=2, vy=2, hc=0, vc=0 // 4 sub-blocks per MCU Others: Undefined"]
    #[inline(always)]
    pub fn jdata_format(&mut self) -> JDATA_FORMAT_W<4> {
        JDATA_FORMAT_W::new(self)
    }
    #[doc = "Bit 3 - Software Reset"]
    #[inline(always)]
    pub fn jpeg_sftrst(&mut self) -> JPEG_SFTRST_W<3> {
        JPEG_SFTRST_W::new(self)
    }
    #[doc = "Bit 2 - Asserted if to start a new encoder/decoder conversion. It will at first stop the inner JPEG module, then reset it, and then re-run it. It is a different mode from DMA phase mode. It cannot be configured in the DMA chain descriptor. It should be configured by the core processor. Auto clear."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<2> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - 1: decoder, 0:encoder"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<1> {
        MODE_W::new(self)
    }
    #[doc = "Bit 0 - 1b - Enabled"]
    #[inline(always)]
    pub fn jpeg_en(&mut self) -> JPEG_EN_W<0> {
        JPEG_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
