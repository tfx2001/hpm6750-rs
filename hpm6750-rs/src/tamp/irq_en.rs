#[doc = "Register `IRQ_EN` reader"]
pub struct R(crate::R<IRQ_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_EN` writer"]
pub struct W(crate::W<IRQ_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_EN_SPEC>;
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
impl From<crate::W<IRQ_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ_EN` reader - interrupt enable, each bit represents one tamper pin 0: interrupt disabled 1: interrupt enabled"]
pub type IRQ_EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IRQ_EN` writer - interrupt enable, each bit represents one tamper pin 0: interrupt disabled 1: interrupt enabled"]
pub type IRQ_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQ_EN_SPEC, u16, u16, 12, O>;
#[doc = "Field `LOCK` reader - lock bit for IRQ enable 0: enable bits can be changed 1: enable bits hold until next battery domain power cycle"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - lock bit for IRQ enable 0: enable bits can be changed 1: enable bits hold until next battery domain power cycle"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQ_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - interrupt enable, each bit represents one tamper pin 0: interrupt disabled 1: interrupt enabled"]
    #[inline(always)]
    pub fn irq_en(&self) -> IRQ_EN_R {
        IRQ_EN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - lock bit for IRQ enable 0: enable bits can be changed 1: enable bits hold until next battery domain power cycle"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - interrupt enable, each bit represents one tamper pin 0: interrupt disabled 1: interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn irq_en(&mut self) -> IRQ_EN_W<0> {
        IRQ_EN_W::new(self)
    }
    #[doc = "Bit 31 - lock bit for IRQ enable 0: enable bits can be changed 1: enable bits hold until next battery domain power cycle"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_en](index.html) module"]
pub struct IRQ_EN_SPEC;
impl crate::RegisterSpec for IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_en::R](R) reader structure"]
impl crate::Readable for IRQ_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_en::W](W) writer structure"]
impl crate::Writable for IRQ_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQ_EN to value 0"]
impl crate::Resettable for IRQ_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
