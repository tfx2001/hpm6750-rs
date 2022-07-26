#[doc = "Register `DMA_ST` reader"]
pub struct R(crate::R<DMA_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_ST` writer"]
pub struct W(crate::W<DMA_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ST_SPEC>;
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
impl From<crate::W<DMA_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_ERR` reader - plane n axi error. W1C."]
pub type DMA_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_ERR` writer - plane n axi error. W1C."]
pub type DMA_ERR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_ST_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA1_DONE` reader - Plane n frame 1 dma done. W1C."]
pub type DMA1_DONE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA1_DONE` writer - Plane n frame 1 dma done. W1C."]
pub type DMA1_DONE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_ST_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA0_DONE` reader - Plane n frame 0 dma done. W1C."]
pub type DMA0_DONE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA0_DONE` writer - Plane n frame 0 dma done. W1C."]
pub type DMA0_DONE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_ST_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 24:31 - plane n axi error. W1C."]
    #[inline(always)]
    pub fn dma_err(&self) -> DMA_ERR_R {
        DMA_ERR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Plane n frame 1 dma done. W1C."]
    #[inline(always)]
    pub fn dma1_done(&self) -> DMA1_DONE_R {
        DMA1_DONE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Plane n frame 0 dma done. W1C."]
    #[inline(always)]
    pub fn dma0_done(&self) -> DMA0_DONE_R {
        DMA0_DONE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - plane n axi error. W1C."]
    #[inline(always)]
    pub fn dma_err(&mut self) -> DMA_ERR_W<24> {
        DMA_ERR_W::new(self)
    }
    #[doc = "Bits 16:23 - Plane n frame 1 dma done. W1C."]
    #[inline(always)]
    pub fn dma1_done(&mut self) -> DMA1_DONE_W<16> {
        DMA1_DONE_W::new(self)
    }
    #[doc = "Bits 8:15 - Plane n frame 0 dma done. W1C."]
    #[inline(always)]
    pub fn dma0_done(&mut self) -> DMA0_DONE_W<8> {
        DMA0_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_st](index.html) module"]
pub struct DMA_ST_SPEC;
impl crate::RegisterSpec for DMA_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_st::R](R) reader structure"]
impl crate::Readable for DMA_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_st::W](W) writer structure"]
impl crate::Writable for DMA_ST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_ST to value 0"]
impl crate::Resettable for DMA_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
