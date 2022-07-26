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
#[doc = "Field `DMA_ERR` reader - Interrupt enable for DMA error"]
pub type DMA_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_ERR` writer - Interrupt enable for DMA error"]
pub type DMA_ERR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT_EN_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_DONE` reader - Interrupt enable for DMA done"]
pub type DMA_DONE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_DONE` writer - Interrupt enable for DMA done"]
pub type DMA_DONE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT_EN_SPEC, u8, u8, 8, O>;
#[doc = "Field `URGENT_UNDERRUN` reader - Asserted when the output buffer urgent underrun condition encountered"]
pub type URGENT_UNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `VS_BLANK` reader - Interrupt enable for start of sof"]
pub type VS_BLANK_R = crate::BitReader<bool>;
#[doc = "Field `VS_BLANK` writer - Interrupt enable for start of sof"]
pub type VS_BLANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `UNDERRUN` reader - Interrupt enable for underrun"]
pub type UNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `UNDERRUN` writer - Interrupt enable for underrun"]
pub type UNDERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
#[doc = "Field `VSYNC` reader - Interrupt enable for end of sof"]
pub type VSYNC_R = crate::BitReader<bool>;
#[doc = "Field `VSYNC` writer - Interrupt enable for end of sof"]
pub type VSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 24:31 - Interrupt enable for DMA error"]
    #[inline(always)]
    pub fn dma_err(&self) -> DMA_ERR_R {
        DMA_ERR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt enable for DMA done"]
    #[inline(always)]
    pub fn dma_done(&self) -> DMA_DONE_R {
        DMA_DONE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 3 - Asserted when the output buffer urgent underrun condition encountered"]
    #[inline(always)]
    pub fn urgent_underrun(&self) -> URGENT_UNDERRUN_R {
        URGENT_UNDERRUN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for start of sof"]
    #[inline(always)]
    pub fn vs_blank(&self) -> VS_BLANK_R {
        VS_BLANK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable for underrun"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Interrupt enable for end of sof"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - Interrupt enable for DMA error"]
    #[inline(always)]
    pub fn dma_err(&mut self) -> DMA_ERR_W<24> {
        DMA_ERR_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt enable for DMA done"]
    #[inline(always)]
    pub fn dma_done(&mut self) -> DMA_DONE_W<16> {
        DMA_DONE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable for start of sof"]
    #[inline(always)]
    pub fn vs_blank(&mut self) -> VS_BLANK_W<2> {
        VS_BLANK_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable for underrun"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UNDERRUN_W<1> {
        UNDERRUN_W::new(self)
    }
    #[doc = "Bit 0 - Interrupt enable for end of sof"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VSYNC_W<0> {
        VSYNC_W::new(self)
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
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
