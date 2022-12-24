#[doc = "Register `DMA_BUS_STATUS` reader"]
pub struct R(crate::R<DMA_BUS_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_BUS_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_BUS_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_BUS_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_BUS_STATUS` writer"]
pub struct W(crate::W<DMA_BUS_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_BUS_STATUS_SPEC>;
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
impl From<crate::W<DMA_BUS_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_BUS_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AXWHSTS` reader - AXI Master Write Channel or AHB Master Status When high, it indicates that AXI master's write channel is active and transferring data in the GMAC-AXI configuration. In the GMAC-AHB configuration, it indicates that the AHB master interface FSMs are in the non-idle state."]
pub type AXWHSTS_R = crate::BitReader<bool>;
#[doc = "Field `AXWHSTS` writer - AXI Master Write Channel or AHB Master Status When high, it indicates that AXI master's write channel is active and transferring data in the GMAC-AXI configuration. In the GMAC-AHB configuration, it indicates that the AHB master interface FSMs are in the non-idle state."]
pub type AXWHSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_BUS_STATUS_SPEC, bool, O>;
#[doc = "Field `AXIRDSTS` reader - AXI Master Read Channel Status When high, it indicates that AXI master's read channel is active and transferring data."]
pub type AXIRDSTS_R = crate::BitReader<bool>;
#[doc = "Field `AXIRDSTS` writer - AXI Master Read Channel Status When high, it indicates that AXI master's read channel is active and transferring data."]
pub type AXIRDSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_BUS_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - AXI Master Write Channel or AHB Master Status When high, it indicates that AXI master's write channel is active and transferring data in the GMAC-AXI configuration. In the GMAC-AHB configuration, it indicates that the AHB master interface FSMs are in the non-idle state."]
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AXI Master Read Channel Status When high, it indicates that AXI master's read channel is active and transferring data."]
    #[inline(always)]
    pub fn axirdsts(&self) -> AXIRDSTS_R {
        AXIRDSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AXI Master Write Channel or AHB Master Status When high, it indicates that AXI master's write channel is active and transferring data in the GMAC-AXI configuration. In the GMAC-AHB configuration, it indicates that the AHB master interface FSMs are in the non-idle state."]
    #[inline(always)]
    #[must_use]
    pub fn axwhsts(&mut self) -> AXWHSTS_W<0> {
        AXWHSTS_W::new(self)
    }
    #[doc = "Bit 1 - AXI Master Read Channel Status When high, it indicates that AXI master's read channel is active and transferring data."]
    #[inline(always)]
    #[must_use]
    pub fn axirdsts(&mut self) -> AXIRDSTS_W<1> {
        AXIRDSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB or AXI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_bus_status](index.html) module"]
pub struct DMA_BUS_STATUS_SPEC;
impl crate::RegisterSpec for DMA_BUS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_bus_status::R](R) reader structure"]
impl crate::Readable for DMA_BUS_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_bus_status::W](W) writer structure"]
impl crate::Writable for DMA_BUS_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_BUS_STATUS to value 0"]
impl crate::Resettable for DMA_BUS_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
