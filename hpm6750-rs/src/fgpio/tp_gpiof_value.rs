#[doc = "Register `TP_GPIOF_VALUE` reader"]
pub struct R(crate::R<TP_GPIOF_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_GPIOF_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_GPIOF_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_GPIOF_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TP_GPIOF_VALUE` writer"]
pub struct W(crate::W<TP_GPIOF_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_GPIOF_VALUE_SPEC>;
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
impl From<crate::W<TP_GPIOF_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_GPIOF_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ_TYPE` reader - GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
pub type IRQ_TYPE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IRQ_TYPE` writer - GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
pub type IRQ_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TP_GPIOF_VALUE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    #[inline(always)]
    pub fn irq_type(&self) -> IRQ_TYPE_R {
        IRQ_TYPE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    #[inline(always)]
    #[must_use]
    pub fn irq_type(&mut self) -> IRQ_TYPE_W<0> {
        IRQ_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOF interrupt type\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_gpiof_value](index.html) module"]
pub struct TP_GPIOF_VALUE_SPEC;
impl crate::RegisterSpec for TP_GPIOF_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_gpiof_value::R](R) reader structure"]
impl crate::Readable for TP_GPIOF_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_gpiof_value::W](W) writer structure"]
impl crate::Writable for TP_GPIOF_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TP_GPIOF_VALUE to value 0"]
impl crate::Resettable for TP_GPIOF_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
