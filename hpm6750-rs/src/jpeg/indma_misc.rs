#[doc = "Register `INDMA_MISC` reader"]
pub struct R(crate::R<INDMA_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INDMA_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INDMA_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INDMA_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INDMA_MISC` writer"]
pub struct W(crate::W<INDMA_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDMA_MISC_SPEC>;
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
impl From<crate::W<INDMA_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDMA_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARQOS` reader - QoS for AXI read channel"]
pub type ARQOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARQOS` writer - QoS for AXI read channel"]
pub type ARQOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INDMA_MISC_SPEC, u8, u8, 4, O>;
#[doc = "Field `MAX_OT` reader - max_ot when input are RGB pixels. For 16 bits per pixel, it can be set as 4. For 32 bits per pixel, it will be set as 2."]
pub type MAX_OT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_OT` writer - max_ot when input are RGB pixels. For 16 bits per pixel, it can be set as 4. For 32 bits per pixel, it will be set as 2."]
pub type MAX_OT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INDMA_MISC_SPEC, u8, u8, 4, O>;
#[doc = "Field `INB13_SWAP` reader - Swap bit\\[31:24\\]
and bit \\[15:8\\]
before pack dir operation. Only work for pixel data."]
pub type INB13_SWAP_R = crate::BitReader<bool>;
#[doc = "Field `INB13_SWAP` writer - Swap bit\\[31:24\\]
and bit \\[15:8\\]
before pack dir operation. Only work for pixel data."]
pub type INB13_SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INDMA_MISC_SPEC, bool, O>;
#[doc = "Field `PACK_DIR` reader - Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. Only work for pixel data. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
pub type PACK_DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PACK_DIR` writer - Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. Only work for pixel data. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
pub type PACK_DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INDMA_MISC_SPEC, u8, u8, 2, O>;
#[doc = "Field `INDMA_RENEW` reader - Renew In DMA. Default is to continue the write address counter when a new DMA request comes. Asserted to reset the write address counter."]
pub type INDMA_RENEW_R = crate::BitReader<bool>;
#[doc = "Field `INDMA_RENEW` writer - Renew In DMA. Default is to continue the write address counter when a new DMA request comes. Asserted to reset the write address counter."]
pub type INDMA_RENEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INDMA_MISC_SPEC, bool, O>;
#[doc = "Field `NXT_IRQ_EN` reader - In DMA Next Interrupt Enable"]
pub type NXT_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `NXT_IRQ_EN` writer - In DMA Next Interrupt Enable"]
pub type NXT_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INDMA_MISC_SPEC, bool, O>;
#[doc = "Field `IN_DMA_DONE_IRQ_EN` reader - In DMA Done enable"]
pub type IN_DMA_DONE_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `IN_DMA_DONE_IRQ_EN` writer - In DMA Done enable"]
pub type IN_DMA_DONE_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INDMA_MISC_SPEC, bool, O>;
#[doc = "Field `AXI_ERR_IRQ_EN` reader - In DMA axi bus error inetrrupt enable"]
pub type AXI_ERR_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `AXI_ERR_IRQ_EN` writer - In DMA axi bus error inetrrupt enable"]
pub type AXI_ERR_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INDMA_MISC_SPEC, bool, O>;
#[doc = "Field `IRQ_EN` reader - interrupt enable for all interrupt sources of In DMA module"]
pub type IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `IRQ_EN` writer - interrupt enable for all interrupt sources of In DMA module"]
pub type IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INDMA_MISC_SPEC, bool, O>;
#[doc = "Field `IN_DMA_ID` reader - 0: Pixel (In) 1: ECS (In) 2: Qmem 3: HuffEnc 4: HuffMin 5: HuffBase 6: HuffSymb"]
pub type IN_DMA_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN_DMA_ID` writer - 0: Pixel (In) 1: ECS (In) 2: Qmem 3: HuffEnc 4: HuffMin 5: HuffBase 6: HuffSymb"]
pub type IN_DMA_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INDMA_MISC_SPEC, u8, u8, 3, O>;
#[doc = "Field `IN_DMA_REQ` reader - Asserted to request DMA. Automatically clear after DMA is done."]
pub type IN_DMA_REQ_R = crate::BitReader<bool>;
#[doc = "Field `IN_DMA_REQ` writer - Asserted to request DMA. Automatically clear after DMA is done."]
pub type IN_DMA_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, INDMA_MISC_SPEC, bool, O>;
#[doc = "Field `INDMA2D` reader - Asserted if In_DMA_ID=Pixel."]
pub type INDMA2D_R = crate::BitReader<bool>;
#[doc = "Field `INDMA2D` writer - Asserted if In_DMA_ID=Pixel."]
pub type INDMA2D_W<'a, const O: u8> = crate::BitWriter<'a, u32, INDMA_MISC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 19:22 - QoS for AXI read channel"]
    #[inline(always)]
    pub fn arqos(&self) -> ARQOS_R {
        ARQOS_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - max_ot when input are RGB pixels. For 16 bits per pixel, it can be set as 4. For 32 bits per pixel, it will be set as 2."]
    #[inline(always)]
    pub fn max_ot(&self) -> MAX_OT_R {
        MAX_OT_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Swap bit\\[31:24\\]
and bit \\[15:8\\]
before pack dir operation. Only work for pixel data."]
    #[inline(always)]
    pub fn inb13_swap(&self) -> INB13_SWAP_R {
        INB13_SWAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. Only work for pixel data. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    #[inline(always)]
    pub fn pack_dir(&self) -> PACK_DIR_R {
        PACK_DIR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 11 - Renew In DMA. Default is to continue the write address counter when a new DMA request comes. Asserted to reset the write address counter."]
    #[inline(always)]
    pub fn indma_renew(&self) -> INDMA_RENEW_R {
        INDMA_RENEW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - In DMA Next Interrupt Enable"]
    #[inline(always)]
    pub fn nxt_irq_en(&self) -> NXT_IRQ_EN_R {
        NXT_IRQ_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - In DMA Done enable"]
    #[inline(always)]
    pub fn in_dma_done_irq_en(&self) -> IN_DMA_DONE_IRQ_EN_R {
        IN_DMA_DONE_IRQ_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - In DMA axi bus error inetrrupt enable"]
    #[inline(always)]
    pub fn axi_err_irq_en(&self) -> AXI_ERR_IRQ_EN_R {
        AXI_ERR_IRQ_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt enable for all interrupt sources of In DMA module"]
    #[inline(always)]
    pub fn irq_en(&self) -> IRQ_EN_R {
        IRQ_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 0: Pixel (In) 1: ECS (In) 2: Qmem 3: HuffEnc 4: HuffMin 5: HuffBase 6: HuffSymb"]
    #[inline(always)]
    pub fn in_dma_id(&self) -> IN_DMA_ID_R {
        IN_DMA_ID_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - Asserted to request DMA. Automatically clear after DMA is done."]
    #[inline(always)]
    pub fn in_dma_req(&self) -> IN_DMA_REQ_R {
        IN_DMA_REQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Asserted if In_DMA_ID=Pixel."]
    #[inline(always)]
    pub fn indma2d(&self) -> INDMA2D_R {
        INDMA2D_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 19:22 - QoS for AXI read channel"]
    #[inline(always)]
    pub fn arqos(&mut self) -> ARQOS_W<19> {
        ARQOS_W::new(self)
    }
    #[doc = "Bits 15:18 - max_ot when input are RGB pixels. For 16 bits per pixel, it can be set as 4. For 32 bits per pixel, it will be set as 2."]
    #[inline(always)]
    pub fn max_ot(&mut self) -> MAX_OT_W<15> {
        MAX_OT_W::new(self)
    }
    #[doc = "Bit 14 - Swap bit\\[31:24\\]
and bit \\[15:8\\]
before pack dir operation. Only work for pixel data."]
    #[inline(always)]
    pub fn inb13_swap(&mut self) -> INB13_SWAP_W<14> {
        INB13_SWAP_W::new(self)
    }
    #[doc = "Bits 12:13 - Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. Only work for pixel data. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    #[inline(always)]
    pub fn pack_dir(&mut self) -> PACK_DIR_W<12> {
        PACK_DIR_W::new(self)
    }
    #[doc = "Bit 11 - Renew In DMA. Default is to continue the write address counter when a new DMA request comes. Asserted to reset the write address counter."]
    #[inline(always)]
    pub fn indma_renew(&mut self) -> INDMA_RENEW_W<11> {
        INDMA_RENEW_W::new(self)
    }
    #[doc = "Bit 10 - In DMA Next Interrupt Enable"]
    #[inline(always)]
    pub fn nxt_irq_en(&mut self) -> NXT_IRQ_EN_W<10> {
        NXT_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 9 - In DMA Done enable"]
    #[inline(always)]
    pub fn in_dma_done_irq_en(&mut self) -> IN_DMA_DONE_IRQ_EN_W<9> {
        IN_DMA_DONE_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 8 - In DMA axi bus error inetrrupt enable"]
    #[inline(always)]
    pub fn axi_err_irq_en(&mut self) -> AXI_ERR_IRQ_EN_W<8> {
        AXI_ERR_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 7 - interrupt enable for all interrupt sources of In DMA module"]
    #[inline(always)]
    pub fn irq_en(&mut self) -> IRQ_EN_W<7> {
        IRQ_EN_W::new(self)
    }
    #[doc = "Bits 4:6 - 0: Pixel (In) 1: ECS (In) 2: Qmem 3: HuffEnc 4: HuffMin 5: HuffBase 6: HuffSymb"]
    #[inline(always)]
    pub fn in_dma_id(&mut self) -> IN_DMA_ID_W<4> {
        IN_DMA_ID_W::new(self)
    }
    #[doc = "Bit 3 - Asserted to request DMA. Automatically clear after DMA is done."]
    #[inline(always)]
    pub fn in_dma_req(&mut self) -> IN_DMA_REQ_W<3> {
        IN_DMA_REQ_W::new(self)
    }
    #[doc = "Bit 2 - Asserted if In_DMA_ID=Pixel."]
    #[inline(always)]
    pub fn indma2d(&mut self) -> INDMA2D_W<2> {
        INDMA2D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In DMA Misc Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indma_misc](index.html) module"]
pub struct INDMA_MISC_SPEC;
impl crate::RegisterSpec for INDMA_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [indma_misc::R](R) reader structure"]
impl crate::Readable for INDMA_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [indma_misc::W](W) writer structure"]
impl crate::Writable for INDMA_MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INDMA_MISC to value 0"]
impl crate::Resettable for INDMA_MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
