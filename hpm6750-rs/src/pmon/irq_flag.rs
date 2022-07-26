#[doc = "Register `IRQ_FLAG` reader"]
pub struct R(crate::R<IRQ_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_FLAG` writer"]
pub struct W(crate::W<IRQ_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_FLAG_SPEC>;
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
impl From<crate::W<IRQ_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLAG` reader - interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened"]
pub type FLAG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLAG` writer - interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened"]
pub type FLAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQ_FLAG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened"]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened"]
    #[inline(always)]
    pub fn flag(&mut self) -> FLAG_W<0> {
        FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_flag](index.html) module"]
pub struct IRQ_FLAG_SPEC;
impl crate::RegisterSpec for IRQ_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_flag::R](R) reader structure"]
impl crate::Readable for IRQ_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_flag::W](W) writer structure"]
impl crate::Writable for IRQ_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ_FLAG to value 0"]
impl crate::Resettable for IRQ_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
