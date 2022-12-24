#[doc = "Register `DMA_RX_POLL_DEMAND` reader"]
pub struct R(crate::R<DMA_RX_POLL_DEMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RX_POLL_DEMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RX_POLL_DEMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RX_POLL_DEMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RX_POLL_DEMAND` writer"]
pub struct W(crate::W<DMA_RX_POLL_DEMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RX_POLL_DEMAND_SPEC>;
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
impl From<crate::W<DMA_RX_POLL_DEMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RX_POLL_DEMAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPD` reader - Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 19 (Current Host Receive Descriptor Register) is pointing. If that descriptor is not available (owned by the Host), the reception returns to the Suspended state and Bit 7 (RU) of Register 5 (Status Register) is asserted. If the descriptor is available, the Rx DMA returns to the active state."]
pub type RPD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RPD` writer - Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 19 (Current Host Receive Descriptor Register) is pointing. If that descriptor is not available (owned by the Host), the reception returns to the Suspended state and Bit 7 (RU) of Register 5 (Status Register) is asserted. If the descriptor is available, the Rx DMA returns to the active state."]
pub type RPD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_RX_POLL_DEMAND_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 19 (Current Host Receive Descriptor Register) is pointing. If that descriptor is not available (owned by the Host), the reception returns to the Suspended state and Bit 7 (RU) of Register 5 (Status Register) is asserted. If the descriptor is available, the Rx DMA returns to the active state."]
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 19 (Current Host Receive Descriptor Register) is pointing. If that descriptor is not available (owned by the Host), the reception returns to the Suspended state and Bit 7 (RU) of Register 5 (Status Register) is asserted. If the descriptor is available, the Rx DMA returns to the active state."]
    #[inline(always)]
    #[must_use]
    pub fn rpd(&mut self) -> RPD_W<0> {
        RPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Poll Demand Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rx_poll_demand](index.html) module"]
pub struct DMA_RX_POLL_DEMAND_SPEC;
impl crate::RegisterSpec for DMA_RX_POLL_DEMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rx_poll_demand::R](R) reader structure"]
impl crate::Readable for DMA_RX_POLL_DEMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_rx_poll_demand::W](W) writer structure"]
impl crate::Writable for DMA_RX_POLL_DEMAND_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_RX_POLL_DEMAND to value 0"]
impl crate::Resettable for DMA_RX_POLL_DEMAND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
