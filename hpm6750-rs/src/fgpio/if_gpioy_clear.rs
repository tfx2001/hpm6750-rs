#[doc = "Register `IF_GPIOY_CLEAR` reader"]
pub struct R(crate::R<IF_GPIOY_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_GPIOY_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_GPIOY_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_GPIOY_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF_GPIOY_CLEAR` writer"]
pub struct W(crate::W<IF_GPIOY_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_GPIOY_CLEAR_SPEC>;
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
impl From<crate::W<IF_GPIOY_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_GPIOY_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ_FLAG` reader - GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
pub type IRQ_FLAG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IRQ_FLAG` writer - GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
pub type IRQ_FLAG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IF_GPIOY_CLEAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    #[inline(always)]
    pub fn irq_flag(&self) -> IRQ_FLAG_R {
        IRQ_FLAG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    #[inline(always)]
    #[must_use]
    pub fn irq_flag(&mut self) -> IRQ_FLAG_W<0> {
        IRQ_FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt flag clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_gpioy_clear](index.html) module"]
pub struct IF_GPIOY_CLEAR_SPEC;
impl crate::RegisterSpec for IF_GPIOY_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_gpioy_clear::R](R) reader structure"]
impl crate::Readable for IF_GPIOY_CLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_gpioy_clear::W](W) writer structure"]
impl crate::Writable for IF_GPIOY_CLEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF_GPIOY_CLEAR to value 0"]
impl crate::Resettable for IF_GPIOY_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
