#[doc = "Register `AS_GPIOC_VALUE` reader"]
pub struct R(crate::R<AS_GPIOC_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AS_GPIOC_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AS_GPIOC_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AS_GPIOC_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AS_GPIOC_VALUE` writer"]
pub struct W(crate::W<AS_GPIOC_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AS_GPIOC_VALUE_SPEC>;
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
impl From<crate::W<AS_GPIOC_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AS_GPIOC_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ_ASYNC` reader - GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
pub type IRQ_ASYNC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IRQ_ASYNC` writer - GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
pub type IRQ_ASYNC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AS_GPIOC_VALUE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    #[inline(always)]
    pub fn irq_async(&self) -> IRQ_ASYNC_R {
        IRQ_ASYNC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    #[inline(always)]
    #[must_use]
    pub fn irq_async(&mut self) -> IRQ_ASYNC_W<0> {
        IRQ_ASYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOC interrupt asynchronous\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [as_gpioc_value](index.html) module"]
pub struct AS_GPIOC_VALUE_SPEC;
impl crate::RegisterSpec for AS_GPIOC_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [as_gpioc_value::R](R) reader structure"]
impl crate::Readable for AS_GPIOC_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [as_gpioc_value::W](W) writer structure"]
impl crate::Writable for AS_GPIOC_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AS_GPIOC_VALUE to value 0"]
impl crate::Resettable for AS_GPIOC_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
