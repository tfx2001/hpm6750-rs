#[doc = "Register `PTPC_0_TARH` reader"]
pub struct R(crate::R<PTPC_0_TARH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPC_0_TARH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPC_0_TARH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPC_0_TARH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPC_0_TARH` writer"]
pub struct W(crate::W<PTPC_0_TARH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPC_0_TARH_SPEC>;
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
impl From<crate::W<PTPC_0_TARH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPC_0_TARH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGET_TIME_HIGH` reader - used for generate compare signal if enabled"]
pub type TARGET_TIME_HIGH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TARGET_TIME_HIGH` writer - used for generate compare signal if enabled"]
pub type TARGET_TIME_HIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PTPC_0_TARH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - used for generate compare signal if enabled"]
    #[inline(always)]
    pub fn target_time_high(&self) -> TARGET_TIME_HIGH_R {
        TARGET_TIME_HIGH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - used for generate compare signal if enabled"]
    #[inline(always)]
    pub fn target_time_high(&mut self) -> TARGET_TIME_HIGH_W<0> {
        TARGET_TIME_HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpc_0_tarh](index.html) module"]
pub struct PTPC_0_TARH_SPEC;
impl crate::RegisterSpec for PTPC_0_TARH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptpc_0_tarh::R](R) reader structure"]
impl crate::Readable for PTPC_0_TARH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptpc_0_tarh::W](W) writer structure"]
impl crate::Writable for PTPC_0_TARH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPC_0_TARH to value 0"]
impl crate::Resettable for PTPC_0_TARH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
