#[doc = "Register `POR_CONTROL` reader"]
pub struct R(crate::R<POR_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POR_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POR_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POR_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POR_CONTROL` writer"]
pub struct W(crate::W<POR_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POR_CONTROL_SPEC>;
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
impl From<crate::W<POR_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POR_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER` reader - Chip power down counter, counter decreasing if value is not 0, power down of chip happens on counter value is 1"]
pub type COUNTER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNTER` writer - Chip power down counter, counter decreasing if value is not 0, power down of chip happens on counter value is 1"]
pub type COUNTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, POR_CONTROL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Chip power down counter, counter decreasing if value is not 0, power down of chip happens on counter value is 1"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Chip power down counter, counter decreasing if value is not 0, power down of chip happens on counter value is 1"]
    #[inline(always)]
    #[must_use]
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
#[doc = "Power down control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [por_control](index.html) module"]
pub struct POR_CONTROL_SPEC;
impl crate::RegisterSpec for POR_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [por_control::R](R) reader structure"]
impl crate::Readable for POR_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [por_control::W](W) writer structure"]
impl crate::Writable for POR_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POR_CONTROL to value 0"]
impl crate::Resettable for POR_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
