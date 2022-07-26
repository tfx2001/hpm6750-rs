#[doc = "Register `CTRL5` reader"]
pub struct R(crate::R<CTRL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL5` writer"]
pub struct W(crate::W<CTRL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL5_SPEC>;
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
impl From<crate::W<CTRL5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDXC1_SYS_IRQ_EN` reader - system irq enable"]
pub type SDXC1_SYS_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `SDXC1_SYS_IRQ_EN` writer - system irq enable"]
pub type SDXC1_SYS_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL5_SPEC, bool, O>;
#[doc = "Field `SDXC1_WKP_IRQ_EN` reader - wakeup irq enable"]
pub type SDXC1_WKP_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `SDXC1_WKP_IRQ_EN` writer - wakeup irq enable"]
pub type SDXC1_WKP_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL5_SPEC, bool, O>;
#[doc = "Field `SDXC1_CARDCLK_INV_EN` reader - card clock inverter enable"]
pub type SDXC1_CARDCLK_INV_EN_R = crate::BitReader<bool>;
#[doc = "Field `SDXC1_CARDCLK_INV_EN` writer - card clock inverter enable"]
pub type SDXC1_CARDCLK_INV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL5_SPEC, bool, O>;
#[doc = "Field `SDXC1_GPR_TUNING_CARD_CLK_SEL` reader - No description avaiable"]
pub type SDXC1_GPR_TUNING_CARD_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDXC1_GPR_TUNING_CARD_CLK_SEL` writer - No description avaiable"]
pub type SDXC1_GPR_TUNING_CARD_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL5_SPEC, u8, u8, 5, O>;
#[doc = "Field `SDXC1_GPR_TUNING_STROBE_SEL` reader - No description avaiable"]
pub type SDXC1_GPR_TUNING_STROBE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDXC1_GPR_TUNING_STROBE_SEL` writer - No description avaiable"]
pub type SDXC1_GPR_TUNING_STROBE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL5_SPEC, u8, u8, 5, O>;
#[doc = "Field `SDXC1_GPR_STROBE_IN_ENABLE` reader - No description avaiable"]
pub type SDXC1_GPR_STROBE_IN_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SDXC1_GPR_STROBE_IN_ENABLE` writer - No description avaiable"]
pub type SDXC1_GPR_STROBE_IN_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL5_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - system irq enable"]
    #[inline(always)]
    pub fn sdxc1_sys_irq_en(&self) -> SDXC1_SYS_IRQ_EN_R {
        SDXC1_SYS_IRQ_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - wakeup irq enable"]
    #[inline(always)]
    pub fn sdxc1_wkp_irq_en(&self) -> SDXC1_WKP_IRQ_EN_R {
        SDXC1_WKP_IRQ_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 28 - card clock inverter enable"]
    #[inline(always)]
    pub fn sdxc1_cardclk_inv_en(&self) -> SDXC1_CARDCLK_INV_EN_R {
        SDXC1_CARDCLK_INV_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 23:27 - No description avaiable"]
    #[inline(always)]
    pub fn sdxc1_gpr_tuning_card_clk_sel(&self) -> SDXC1_GPR_TUNING_CARD_CLK_SEL_R {
        SDXC1_GPR_TUNING_CARD_CLK_SEL_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - No description avaiable"]
    #[inline(always)]
    pub fn sdxc1_gpr_tuning_strobe_sel(&self) -> SDXC1_GPR_TUNING_STROBE_SEL_R {
        SDXC1_GPR_TUNING_STROBE_SEL_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - No description avaiable"]
    #[inline(always)]
    pub fn sdxc1_gpr_strobe_in_enable(&self) -> SDXC1_GPR_STROBE_IN_ENABLE_R {
        SDXC1_GPR_STROBE_IN_ENABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - system irq enable"]
    #[inline(always)]
    pub fn sdxc1_sys_irq_en(&mut self) -> SDXC1_SYS_IRQ_EN_W<31> {
        SDXC1_SYS_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 30 - wakeup irq enable"]
    #[inline(always)]
    pub fn sdxc1_wkp_irq_en(&mut self) -> SDXC1_WKP_IRQ_EN_W<30> {
        SDXC1_WKP_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 28 - card clock inverter enable"]
    #[inline(always)]
    pub fn sdxc1_cardclk_inv_en(&mut self) -> SDXC1_CARDCLK_INV_EN_W<28> {
        SDXC1_CARDCLK_INV_EN_W::new(self)
    }
    #[doc = "Bits 23:27 - No description avaiable"]
    #[inline(always)]
    pub fn sdxc1_gpr_tuning_card_clk_sel(&mut self) -> SDXC1_GPR_TUNING_CARD_CLK_SEL_W<23> {
        SDXC1_GPR_TUNING_CARD_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 18:22 - No description avaiable"]
    #[inline(always)]
    pub fn sdxc1_gpr_tuning_strobe_sel(&mut self) -> SDXC1_GPR_TUNING_STROBE_SEL_W<18> {
        SDXC1_GPR_TUNING_STROBE_SEL_W::new(self)
    }
    #[doc = "Bit 17 - No description avaiable"]
    #[inline(always)]
    pub fn sdxc1_gpr_strobe_in_enable(&mut self) -> SDXC1_GPR_STROBE_IN_ENABLE_W<17> {
        SDXC1_GPR_STROBE_IN_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl5](index.html) module"]
pub struct CTRL5_SPEC;
impl crate::RegisterSpec for CTRL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl5::R](R) reader structure"]
impl crate::Readable for CTRL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl5::W](W) writer structure"]
impl crate::Writable for CTRL5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL5 to value 0"]
impl crate::Resettable for CTRL5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
