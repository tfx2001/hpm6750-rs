#[doc = "Register `DMA_MISS_OVF_CNT` reader"]
pub struct R(crate::R<DMA_MISS_OVF_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_MISS_OVF_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_MISS_OVF_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_MISS_OVF_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_MISS_OVF_CNT` writer"]
pub struct W(crate::W<DMA_MISS_OVF_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_MISS_OVF_CNT_SPEC>;
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
impl From<crate::W<DMA_MISS_OVF_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_MISS_OVF_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ONFCNTOVF` reader - Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\]) overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value. In such a scenario, the overflow frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
pub type ONFCNTOVF_R = crate::BitReader<bool>;
#[doc = "Field `ONFCNTOVF` writer - Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\]) overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value. In such a scenario, the overflow frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
pub type ONFCNTOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_MISS_OVF_CNT_SPEC, bool, O>;
#[doc = "Field `OVFFRMCNT` reader - Overflow Frame Counter This field indicates the number of frames missed by the application. This counter is incremented each time the MTL FIFO overflows. The counter is cleared when this register is read with mci_be_i\\[2\\]
at 1’b1."]
pub type OVFFRMCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OVFFRMCNT` writer - Overflow Frame Counter This field indicates the number of frames missed by the application. This counter is incremented each time the MTL FIFO overflows. The counter is cleared when this register is read with mci_be_i\\[2\\]
at 1’b1."]
pub type OVFFRMCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_MISS_OVF_CNT_SPEC, u16, u16, 11, O>;
#[doc = "Field `MISCNTOVF` reader - Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value. In such a scenario, the Missed frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
pub type MISCNTOVF_R = crate::BitReader<bool>;
#[doc = "Field `MISCNTOVF` writer - Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value. In such a scenario, the Missed frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
pub type MISCNTOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_MISS_OVF_CNT_SPEC, bool, O>;
#[doc = "Field `MISFRMCNT` reader - Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read with mci_be_i\\[0\\]
at 1’b1."]
pub type MISFRMCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MISFRMCNT` writer - Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read with mci_be_i\\[0\\]
at 1’b1."]
pub type MISFRMCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_MISS_OVF_CNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 28 - Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\]) overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value. In such a scenario, the overflow frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
    #[inline(always)]
    pub fn onfcntovf(&self) -> ONFCNTOVF_R {
        ONFCNTOVF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Overflow Frame Counter This field indicates the number of frames missed by the application. This counter is incremented each time the MTL FIFO overflows. The counter is cleared when this register is read with mci_be_i\\[2\\]
at 1’b1."]
    #[inline(always)]
    pub fn ovffrmcnt(&self) -> OVFFRMCNT_R {
        OVFFRMCNT_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value. In such a scenario, the Missed frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 0:15 - Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read with mci_be_i\\[0\\]
at 1’b1."]
    #[inline(always)]
    pub fn misfrmcnt(&self) -> MISFRMCNT_R {
        MISFRMCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 28 - Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter (Bits\\[27:17\\]) overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value. In such a scenario, the overflow frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
    #[inline(always)]
    pub fn onfcntovf(&mut self) -> ONFCNTOVF_W<28> {
        ONFCNTOVF_W::new(self)
    }
    #[doc = "Bits 17:27 - Overflow Frame Counter This field indicates the number of frames missed by the application. This counter is incremented each time the MTL FIFO overflows. The counter is cleared when this register is read with mci_be_i\\[2\\]
at 1’b1."]
    #[inline(always)]
    pub fn ovffrmcnt(&mut self) -> OVFFRMCNT_W<17> {
        OVFFRMCNT_W::new(self)
    }
    #[doc = "Bit 16 - Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter (Bits\\[15:0\\]) overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value. In such a scenario, the Missed frame counter is reset to all-zeros and this bit indicates that the rollover happened."]
    #[inline(always)]
    pub fn miscntovf(&mut self) -> MISCNTOVF_W<16> {
        MISCNTOVF_W::new(self)
    }
    #[doc = "Bits 0:15 - Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable. This counter is incremented each time the DMA discards an incoming frame. The counter is cleared when this register is read with mci_be_i\\[0\\]
at 1’b1."]
    #[inline(always)]
    pub fn misfrmcnt(&mut self) -> MISFRMCNT_W<0> {
        MISFRMCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Missed Frame And Buffer Overflow Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_miss_ovf_cnt](index.html) module"]
pub struct DMA_MISS_OVF_CNT_SPEC;
impl crate::RegisterSpec for DMA_MISS_OVF_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_miss_ovf_cnt::R](R) reader structure"]
impl crate::Readable for DMA_MISS_OVF_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_miss_ovf_cnt::W](W) writer structure"]
impl crate::Writable for DMA_MISS_OVF_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_MISS_OVF_CNT to value 0"]
impl crate::Resettable for DMA_MISS_OVF_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
