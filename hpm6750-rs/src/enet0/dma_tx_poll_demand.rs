#[doc = "Register `DMA_TX_POLL_DEMAND` reader"]
pub struct R(crate::R<DMA_TX_POLL_DEMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_TX_POLL_DEMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_TX_POLL_DEMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_TX_POLL_DEMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_TX_POLL_DEMAND` writer"]
pub struct W(crate::W<DMA_TX_POLL_DEMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_TX_POLL_DEMAND_SPEC>;
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
impl From<crate::W<DMA_TX_POLL_DEMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_TX_POLL_DEMAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPD` reader - Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 18 (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host), the transmission returns to the Suspend state and Bit 2 (TU) of Register 5 (Status Register) is asserted. If the descriptor is available, the transmission resumes."]
pub type TPD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TPD` writer - Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 18 (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host), the transmission returns to the Suspend state and Bit 2 (TU) of Register 5 (Status Register) is asserted. If the descriptor is available, the transmission resumes."]
pub type TPD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_TX_POLL_DEMAND_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 18 (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host), the transmission returns to the Suspend state and Bit 2 (TU) of Register 5 (Status Register) is asserted. If the descriptor is available, the transmission resumes."]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 18 (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host), the transmission returns to the Suspend state and Bit 2 (TU) of Register 5 (Status Register) is asserted. If the descriptor is available, the transmission resumes."]
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W<0> {
        TPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Poll Demand Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_tx_poll_demand](index.html) module"]
pub struct DMA_TX_POLL_DEMAND_SPEC;
impl crate::RegisterSpec for DMA_TX_POLL_DEMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_tx_poll_demand::R](R) reader structure"]
impl crate::Readable for DMA_TX_POLL_DEMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_tx_poll_demand::W](W) writer structure"]
impl crate::Writable for DMA_TX_POLL_DEMAND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_TX_POLL_DEMAND to value 0"]
impl crate::Resettable for DMA_TX_POLL_DEMAND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
