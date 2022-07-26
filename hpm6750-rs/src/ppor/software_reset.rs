#[doc = "Register `SOFTWARE_RESET` reader"]
pub struct R(crate::R<SOFTWARE_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFTWARE_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFTWARE_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFTWARE_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOFTWARE_RESET` writer"]
pub struct W(crate::W<SOFTWARE_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTWARE_RESET_SPEC>;
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
impl From<crate::W<SOFTWARE_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTWARE_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER` reader - counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset"]
pub type COUNTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNTER` writer - counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset"]
pub type COUNTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOFTWARE_RESET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset"]
    #[inline(always)]
    pub fn counter(&mut self) -> COUNTER_W<0> {
        COUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software reset counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [software_reset](index.html) module"]
pub struct SOFTWARE_RESET_SPEC;
impl crate::RegisterSpec for SOFTWARE_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [software_reset::R](R) reader structure"]
impl crate::Readable for SOFTWARE_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [software_reset::W](W) writer structure"]
impl crate::Writable for SOFTWARE_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOFTWARE_RESET to value 0"]
impl crate::Resettable for SOFTWARE_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
