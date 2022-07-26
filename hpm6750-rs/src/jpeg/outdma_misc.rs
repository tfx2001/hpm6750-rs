#[doc = "Register `OUTDMA_MISC` reader"]
pub struct R(crate::R<OUTDMA_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTDMA_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTDMA_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTDMA_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTDMA_MISC` writer"]
pub struct W(crate::W<OUTDMA_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTDMA_MISC_SPEC>;
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
impl From<crate::W<OUTDMA_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTDMA_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWQOS` reader - No description avaiable"]
pub type AWQOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWQOS` writer - No description avaiable"]
pub type AWQOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTDMA_MISC_SPEC, u8, u8, 4, O>;
#[doc = "Field `PACK_DIR` reader - Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. All outdma data are impacted. 2'b00: no change {A3, A2, A1, A0} (This is used for ecs stream) 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
pub type PACK_DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PACK_DIR` writer - Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. All outdma data are impacted. 2'b00: no change {A3, A2, A1, A0} (This is used for ecs stream) 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
pub type PACK_DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTDMA_MISC_SPEC, u8, u8, 2, O>;
#[doc = "Field `EN_OUTCNT` reader - Enable output counter (unit as bytes)"]
pub type EN_OUTCNT_R = crate::BitReader<bool>;
#[doc = "Field `EN_OUTCNT` writer - Enable output counter (unit as bytes)"]
pub type EN_OUTCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTDMA_MISC_SPEC, bool, O>;
#[doc = "Field `INI_OUTCNT` reader - Asserted to ini output counter"]
pub type INI_OUTCNT_R = crate::BitReader<bool>;
#[doc = "Field `INI_OUTCNT` writer - Asserted to ini output counter"]
pub type INI_OUTCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTDMA_MISC_SPEC, bool, O>;
#[doc = "Field `ADD_ODMA_ENDINGS` reader - Add 0xFFD9 to the ending of the odma stream when all original image pixels are processed by the encoder module."]
pub type ADD_ODMA_ENDINGS_R = crate::BitReader<bool>;
#[doc = "Field `ADD_ODMA_ENDINGS` writer - Add 0xFFD9 to the ending of the odma stream when all original image pixels are processed by the encoder module."]
pub type ADD_ODMA_ENDINGS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTDMA_MISC_SPEC, bool, O>;
#[doc = "Field `NXT_IRQ_EN` reader - Out DMA Next Interrupt Enable"]
pub type NXT_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `NXT_IRQ_EN` writer - Out DMA Next Interrupt Enable"]
pub type NXT_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTDMA_MISC_SPEC, bool, O>;
#[doc = "Field `OUT_DMA_DONE_IRQ_EN` reader - Out DMA Done interrupt Enable"]
pub type OUT_DMA_DONE_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DMA_DONE_IRQ_EN` writer - Out DMA Done interrupt Enable"]
pub type OUT_DMA_DONE_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUTDMA_MISC_SPEC, bool, O>;
#[doc = "Field `AXI_ERR_IRQ_EN` reader - Out DMA axi bus error inetrrupt enable"]
pub type AXI_ERR_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `AXI_ERR_IRQ_EN` writer - Out DMA axi bus error inetrrupt enable"]
pub type AXI_ERR_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTDMA_MISC_SPEC, bool, O>;
#[doc = "Field `IRQ_EN` reader - interrupt enable for all interrupt sources of Out DMA module"]
pub type IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `IRQ_EN` writer - interrupt enable for all interrupt sources of Out DMA module"]
pub type IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTDMA_MISC_SPEC, bool, O>;
#[doc = "Field `OUT_DMA_ID` reader - 0: Pixel (Out) 1: ECS (Out)"]
pub type OUT_DMA_ID_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DMA_ID` writer - 0: Pixel (Out) 1: ECS (Out)"]
pub type OUT_DMA_ID_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTDMA_MISC_SPEC, bool, O>;
#[doc = "Field `OUT_DMA_REQ` reader - Asserted to enable Out DMA request"]
pub type OUT_DMA_REQ_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DMA_REQ` writer - Asserted to enable Out DMA request"]
pub type OUT_DMA_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTDMA_MISC_SPEC, bool, O>;
#[doc = "Field `OUTDMA2D` reader - Asserted if Out_DMA_ID==Pixel"]
pub type OUTDMA2D_R = crate::BitReader<bool>;
#[doc = "Field `OUTDMA2D` writer - Asserted if Out_DMA_ID==Pixel"]
pub type OUTDMA2D_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTDMA_MISC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 14:17 - No description avaiable"]
    #[inline(always)]
    pub fn awqos(&self) -> AWQOS_R {
        AWQOS_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. All outdma data are impacted. 2'b00: no change {A3, A2, A1, A0} (This is used for ecs stream) 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    #[inline(always)]
    pub fn pack_dir(&self) -> PACK_DIR_R {
        PACK_DIR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 11 - Enable output counter (unit as bytes)"]
    #[inline(always)]
    pub fn en_outcnt(&self) -> EN_OUTCNT_R {
        EN_OUTCNT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Asserted to ini output counter"]
    #[inline(always)]
    pub fn ini_outcnt(&self) -> INI_OUTCNT_R {
        INI_OUTCNT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Add 0xFFD9 to the ending of the odma stream when all original image pixels are processed by the encoder module."]
    #[inline(always)]
    pub fn add_odma_endings(&self) -> ADD_ODMA_ENDINGS_R {
        ADD_ODMA_ENDINGS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Out DMA Next Interrupt Enable"]
    #[inline(always)]
    pub fn nxt_irq_en(&self) -> NXT_IRQ_EN_R {
        NXT_IRQ_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Out DMA Done interrupt Enable"]
    #[inline(always)]
    pub fn out_dma_done_irq_en(&self) -> OUT_DMA_DONE_IRQ_EN_R {
        OUT_DMA_DONE_IRQ_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Out DMA axi bus error inetrrupt enable"]
    #[inline(always)]
    pub fn axi_err_irq_en(&self) -> AXI_ERR_IRQ_EN_R {
        AXI_ERR_IRQ_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - interrupt enable for all interrupt sources of Out DMA module"]
    #[inline(always)]
    pub fn irq_en(&self) -> IRQ_EN_R {
        IRQ_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: Pixel (Out) 1: ECS (Out)"]
    #[inline(always)]
    pub fn out_dma_id(&self) -> OUT_DMA_ID_R {
        OUT_DMA_ID_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Asserted to enable Out DMA request"]
    #[inline(always)]
    pub fn out_dma_req(&self) -> OUT_DMA_REQ_R {
        OUT_DMA_REQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Asserted if Out_DMA_ID==Pixel"]
    #[inline(always)]
    pub fn outdma2d(&self) -> OUTDMA2D_R {
        OUTDMA2D_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 14:17 - No description avaiable"]
    #[inline(always)]
    pub fn awqos(&mut self) -> AWQOS_W<14> {
        AWQOS_W::new(self)
    }
    #[doc = "Bits 12:13 - Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. All outdma data are impacted. 2'b00: no change {A3, A2, A1, A0} (This is used for ecs stream) 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    #[inline(always)]
    pub fn pack_dir(&mut self) -> PACK_DIR_W<12> {
        PACK_DIR_W::new(self)
    }
    #[doc = "Bit 11 - Enable output counter (unit as bytes)"]
    #[inline(always)]
    pub fn en_outcnt(&mut self) -> EN_OUTCNT_W<11> {
        EN_OUTCNT_W::new(self)
    }
    #[doc = "Bit 10 - Asserted to ini output counter"]
    #[inline(always)]
    pub fn ini_outcnt(&mut self) -> INI_OUTCNT_W<10> {
        INI_OUTCNT_W::new(self)
    }
    #[doc = "Bit 9 - Add 0xFFD9 to the ending of the odma stream when all original image pixels are processed by the encoder module."]
    #[inline(always)]
    pub fn add_odma_endings(&mut self) -> ADD_ODMA_ENDINGS_W<9> {
        ADD_ODMA_ENDINGS_W::new(self)
    }
    #[doc = "Bit 8 - Out DMA Next Interrupt Enable"]
    #[inline(always)]
    pub fn nxt_irq_en(&mut self) -> NXT_IRQ_EN_W<8> {
        NXT_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 7 - Out DMA Done interrupt Enable"]
    #[inline(always)]
    pub fn out_dma_done_irq_en(&mut self) -> OUT_DMA_DONE_IRQ_EN_W<7> {
        OUT_DMA_DONE_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 6 - Out DMA axi bus error inetrrupt enable"]
    #[inline(always)]
    pub fn axi_err_irq_en(&mut self) -> AXI_ERR_IRQ_EN_W<6> {
        AXI_ERR_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 5 - interrupt enable for all interrupt sources of Out DMA module"]
    #[inline(always)]
    pub fn irq_en(&mut self) -> IRQ_EN_W<5> {
        IRQ_EN_W::new(self)
    }
    #[doc = "Bit 4 - 0: Pixel (Out) 1: ECS (Out)"]
    #[inline(always)]
    pub fn out_dma_id(&mut self) -> OUT_DMA_ID_W<4> {
        OUT_DMA_ID_W::new(self)
    }
    #[doc = "Bit 3 - Asserted to enable Out DMA request"]
    #[inline(always)]
    pub fn out_dma_req(&mut self) -> OUT_DMA_REQ_W<3> {
        OUT_DMA_REQ_W::new(self)
    }
    #[doc = "Bit 2 - Asserted if Out_DMA_ID==Pixel"]
    #[inline(always)]
    pub fn outdma2d(&mut self) -> OUTDMA2D_W<2> {
        OUTDMA2D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Out DMA Misc Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outdma_misc](index.html) module"]
pub struct OUTDMA_MISC_SPEC;
impl crate::RegisterSpec for OUTDMA_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outdma_misc::R](R) reader structure"]
impl crate::Readable for OUTDMA_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outdma_misc::W](W) writer structure"]
impl crate::Writable for OUTDMA_MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTDMA_MISC to value 0"]
impl crate::Resettable for OUTDMA_MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
