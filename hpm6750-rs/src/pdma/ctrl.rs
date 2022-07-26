#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARQOS` reader - QoS for AXI read bus"]
pub type ARQOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARQOS` writer - QoS for AXI read bus"]
pub type ARQOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWQOS` reader - QoS for AXI write bus"]
pub type AWQOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWQOS` writer - QoS for AXI write bus"]
pub type AWQOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `PACK_DIR` reader - Decide the byte sequence of the 32-bit output word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
pub type PACK_DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PACK_DIR` writer - Decide the byte sequence of the 32-bit output word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
pub type PACK_DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `AXIERR_IRQ_EN` reader - Enable interrupt of AXI bus error"]
pub type AXIERR_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `AXIERR_IRQ_EN` writer - Enable interrupt of AXI bus error"]
pub type AXIERR_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PDMA_DONE_IRQ_EN` reader - Enable interrupt of PDMA_DONE"]
pub type PDMA_DONE_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `PDMA_DONE_IRQ_EN` writer - Enable interrupt of PDMA_DONE"]
pub type PDMA_DONE_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CLKGATE` reader - Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
pub type CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `CLKGATE` writer - Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
pub type CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `IRQ_EN` reader - Enable normal interrupt"]
pub type IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `IRQ_EN` writer - Enable normal interrupt"]
pub type IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BS16` reader - Asserted when the Block Size is 16x16, else 8x8"]
pub type BS16_R = crate::BitReader<bool>;
#[doc = "Field `BS16` writer - Asserted when the Block Size is 16x16, else 8x8"]
pub type BS16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `P1_EN` reader - Plane 1 Enable"]
pub type P1_EN_R = crate::BitReader<bool>;
#[doc = "Field `P1_EN` writer - Plane 1 Enable"]
pub type P1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `P0_EN` reader - Plane 0 Enable"]
pub type P0_EN_R = crate::BitReader<bool>;
#[doc = "Field `P0_EN` writer - Plane 0 Enable"]
pub type P0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PDMA_SFTRST` reader - Software Reset. Write 1 to clear PDMA internal logic. Write 0 to exit software reset mode."]
pub type PDMA_SFTRST_R = crate::BitReader<bool>;
#[doc = "Field `PDMA_SFTRST` writer - Software Reset. Write 1 to clear PDMA internal logic. Write 0 to exit software reset mode."]
pub type PDMA_SFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PDMA_EN` reader - 1b - Enabled"]
pub type PDMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `PDMA_EN` writer - 1b - Enabled"]
pub type PDMA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 19:22 - QoS for AXI read bus"]
    #[inline(always)]
    pub fn arqos(&self) -> ARQOS_R {
        ARQOS_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - QoS for AXI write bus"]
    #[inline(always)]
    pub fn awqos(&self) -> AWQOS_R {
        AWQOS_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 13:14 - Decide the byte sequence of the 32-bit output word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    #[inline(always)]
    pub fn pack_dir(&self) -> PACK_DIR_R {
        PACK_DIR_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 12 - Enable interrupt of AXI bus error"]
    #[inline(always)]
    pub fn axierr_irq_en(&self) -> AXIERR_IRQ_EN_R {
        AXIERR_IRQ_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable interrupt of PDMA_DONE"]
    #[inline(always)]
    pub fn pdma_done_irq_en(&self) -> PDMA_DONE_IRQ_EN_R {
        PDMA_DONE_IRQ_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 9 - Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable normal interrupt"]
    #[inline(always)]
    pub fn irq_en(&self) -> IRQ_EN_R {
        IRQ_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Asserted when the Block Size is 16x16, else 8x8"]
    #[inline(always)]
    pub fn bs16(&self) -> BS16_R {
        BS16_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Plane 1 Enable"]
    #[inline(always)]
    pub fn p1_en(&self) -> P1_EN_R {
        P1_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Plane 0 Enable"]
    #[inline(always)]
    pub fn p0_en(&self) -> P0_EN_R {
        P0_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset. Write 1 to clear PDMA internal logic. Write 0 to exit software reset mode."]
    #[inline(always)]
    pub fn pdma_sftrst(&self) -> PDMA_SFTRST_R {
        PDMA_SFTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - 1b - Enabled"]
    #[inline(always)]
    pub fn pdma_en(&self) -> PDMA_EN_R {
        PDMA_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 19:22 - QoS for AXI read bus"]
    #[inline(always)]
    pub fn arqos(&mut self) -> ARQOS_W<19> {
        ARQOS_W::new(self)
    }
    #[doc = "Bits 15:18 - QoS for AXI write bus"]
    #[inline(always)]
    pub fn awqos(&mut self) -> AWQOS_W<15> {
        AWQOS_W::new(self)
    }
    #[doc = "Bits 13:14 - Decide the byte sequence of the 32-bit output word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    #[inline(always)]
    pub fn pack_dir(&mut self) -> PACK_DIR_W<13> {
        PACK_DIR_W::new(self)
    }
    #[doc = "Bit 12 - Enable interrupt of AXI bus error"]
    #[inline(always)]
    pub fn axierr_irq_en(&mut self) -> AXIERR_IRQ_EN_W<12> {
        AXIERR_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 11 - Enable interrupt of PDMA_DONE"]
    #[inline(always)]
    pub fn pdma_done_irq_en(&mut self) -> PDMA_DONE_IRQ_EN_W<11> {
        PDMA_DONE_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 9 - Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
    #[inline(always)]
    pub fn clkgate(&mut self) -> CLKGATE_W<9> {
        CLKGATE_W::new(self)
    }
    #[doc = "Bit 6 - Enable normal interrupt"]
    #[inline(always)]
    pub fn irq_en(&mut self) -> IRQ_EN_W<6> {
        IRQ_EN_W::new(self)
    }
    #[doc = "Bit 5 - Asserted when the Block Size is 16x16, else 8x8"]
    #[inline(always)]
    pub fn bs16(&mut self) -> BS16_W<5> {
        BS16_W::new(self)
    }
    #[doc = "Bit 4 - Plane 1 Enable"]
    #[inline(always)]
    pub fn p1_en(&mut self) -> P1_EN_W<4> {
        P1_EN_W::new(self)
    }
    #[doc = "Bit 3 - Plane 0 Enable"]
    #[inline(always)]
    pub fn p0_en(&mut self) -> P0_EN_W<3> {
        P0_EN_W::new(self)
    }
    #[doc = "Bit 1 - Software Reset. Write 1 to clear PDMA internal logic. Write 0 to exit software reset mode."]
    #[inline(always)]
    pub fn pdma_sftrst(&mut self) -> PDMA_SFTRST_W<1> {
        PDMA_SFTRST_W::new(self)
    }
    #[doc = "Bit 0 - 1b - Enabled"]
    #[inline(always)]
    pub fn pdma_en(&mut self) -> PDMA_EN_W<0> {
        PDMA_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
