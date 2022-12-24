#[doc = "Register `CTRL4` reader"]
pub struct R(crate::R<CTRL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL4` writer"]
pub struct W(crate::W<CTRL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL4_SPEC>;
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
impl From<crate::W<CTRL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDXC0_GPR_STROBE_IN_ENABLE` reader - enable strobe clock, maybe used when update strobe DLL"]
pub type SDXC0_GPR_STROBE_IN_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SDXC0_GPR_STROBE_IN_ENABLE` writer - enable strobe clock, maybe used when update strobe DLL"]
pub type SDXC0_GPR_STROBE_IN_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL4_SPEC, bool, O>;
#[doc = "Field `SDXC0_GPR_TUNING_STROBE_SEL` reader - for strobe DLL, default 7taps(1ns)"]
pub type SDXC0_GPR_TUNING_STROBE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDXC0_GPR_TUNING_STROBE_SEL` writer - for strobe DLL, default 7taps(1ns)"]
pub type SDXC0_GPR_TUNING_STROBE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL4_SPEC, u8, u8, 5, O>;
#[doc = "Field `SDXC0_GPR_TUNING_CARD_CLK_SEL` reader - for card clock DLL, default 0"]
pub type SDXC0_GPR_TUNING_CARD_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDXC0_GPR_TUNING_CARD_CLK_SEL` writer - for card clock DLL, default 0"]
pub type SDXC0_GPR_TUNING_CARD_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL4_SPEC, u8, u8, 5, O>;
#[doc = "Field `SDXC0_CARDCLK_INV_EN` reader - card clock inverter enable"]
pub type SDXC0_CARDCLK_INV_EN_R = crate::BitReader<bool>;
#[doc = "Field `SDXC0_CARDCLK_INV_EN` writer - card clock inverter enable"]
pub type SDXC0_CARDCLK_INV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL4_SPEC, bool, O>;
#[doc = "Field `SDXC0_WKP_IRQ_EN` reader - wakeup irq enable"]
pub type SDXC0_WKP_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `SDXC0_WKP_IRQ_EN` writer - wakeup irq enable"]
pub type SDXC0_WKP_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL4_SPEC, bool, O>;
#[doc = "Field `SDXC0_SYS_IRQ_EN` reader - system irq enable"]
pub type SDXC0_SYS_IRQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `SDXC0_SYS_IRQ_EN` writer - system irq enable"]
pub type SDXC0_SYS_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 17 - enable strobe clock, maybe used when update strobe DLL"]
    #[inline(always)]
    pub fn sdxc0_gpr_strobe_in_enable(&self) -> SDXC0_GPR_STROBE_IN_ENABLE_R {
        SDXC0_GPR_STROBE_IN_ENABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:22 - for strobe DLL, default 7taps(1ns)"]
    #[inline(always)]
    pub fn sdxc0_gpr_tuning_strobe_sel(&self) -> SDXC0_GPR_TUNING_STROBE_SEL_R {
        SDXC0_GPR_TUNING_STROBE_SEL_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - for card clock DLL, default 0"]
    #[inline(always)]
    pub fn sdxc0_gpr_tuning_card_clk_sel(&self) -> SDXC0_GPR_TUNING_CARD_CLK_SEL_R {
        SDXC0_GPR_TUNING_CARD_CLK_SEL_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - card clock inverter enable"]
    #[inline(always)]
    pub fn sdxc0_cardclk_inv_en(&self) -> SDXC0_CARDCLK_INV_EN_R {
        SDXC0_CARDCLK_INV_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - wakeup irq enable"]
    #[inline(always)]
    pub fn sdxc0_wkp_irq_en(&self) -> SDXC0_WKP_IRQ_EN_R {
        SDXC0_WKP_IRQ_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - system irq enable"]
    #[inline(always)]
    pub fn sdxc0_sys_irq_en(&self) -> SDXC0_SYS_IRQ_EN_R {
        SDXC0_SYS_IRQ_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - enable strobe clock, maybe used when update strobe DLL"]
    #[inline(always)]
    #[must_use]
    pub fn sdxc0_gpr_strobe_in_enable(&mut self) -> SDXC0_GPR_STROBE_IN_ENABLE_W<17> {
        SDXC0_GPR_STROBE_IN_ENABLE_W::new(self)
    }
    #[doc = "Bits 18:22 - for strobe DLL, default 7taps(1ns)"]
    #[inline(always)]
    #[must_use]
    pub fn sdxc0_gpr_tuning_strobe_sel(&mut self) -> SDXC0_GPR_TUNING_STROBE_SEL_W<18> {
        SDXC0_GPR_TUNING_STROBE_SEL_W::new(self)
    }
    #[doc = "Bits 23:27 - for card clock DLL, default 0"]
    #[inline(always)]
    #[must_use]
    pub fn sdxc0_gpr_tuning_card_clk_sel(&mut self) -> SDXC0_GPR_TUNING_CARD_CLK_SEL_W<23> {
        SDXC0_GPR_TUNING_CARD_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 28 - card clock inverter enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdxc0_cardclk_inv_en(&mut self) -> SDXC0_CARDCLK_INV_EN_W<28> {
        SDXC0_CARDCLK_INV_EN_W::new(self)
    }
    #[doc = "Bit 30 - wakeup irq enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdxc0_wkp_irq_en(&mut self) -> SDXC0_WKP_IRQ_EN_W<30> {
        SDXC0_WKP_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 31 - system irq enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdxc0_sys_irq_en(&mut self) -> SDXC0_SYS_IRQ_EN_W<31> {
        SDXC0_SYS_IRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl4](index.html) module"]
pub struct CTRL4_SPEC;
impl crate::RegisterSpec for CTRL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl4::R](R) reader structure"]
impl crate::Readable for CTRL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl4::W](W) writer structure"]
impl crate::Writable for CTRL4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL4 to value 0"]
impl crate::Resettable for CTRL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
