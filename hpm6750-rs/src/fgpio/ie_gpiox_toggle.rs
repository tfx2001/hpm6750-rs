#[doc = "Register `IE_GPIOX_TOGGLE` reader"]
pub struct R(crate::R<IE_GPIOX_TOGGLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_GPIOX_TOGGLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_GPIOX_TOGGLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_GPIOX_TOGGLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE_GPIOX_TOGGLE` writer"]
pub struct W(crate::W<IE_GPIOX_TOGGLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_GPIOX_TOGGLE_SPEC>;
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
impl From<crate::W<IE_GPIOX_TOGGLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_GPIOX_TOGGLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ_EN` reader - GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
pub type IRQ_EN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IRQ_EN` writer - GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
pub type IRQ_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IE_GPIOX_TOGGLE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    #[inline(always)]
    pub fn irq_en(&self) -> IRQ_EN_R {
        IRQ_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    #[inline(always)]
    pub fn irq_en(&mut self) -> IRQ_EN_W<0> {
        IRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt enable toggle\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie_gpiox_toggle](index.html) module"]
pub struct IE_GPIOX_TOGGLE_SPEC;
impl crate::RegisterSpec for IE_GPIOX_TOGGLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie_gpiox_toggle::R](R) reader structure"]
impl crate::Readable for IE_GPIOX_TOGGLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie_gpiox_toggle::W](W) writer structure"]
impl crate::Writable for IE_GPIOX_TOGGLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE_GPIOX_TOGGLE to value 0"]
impl crate::Resettable for IE_GPIOX_TOGGLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
