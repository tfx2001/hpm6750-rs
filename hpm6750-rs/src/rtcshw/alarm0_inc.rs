#[doc = "Register `ALARM0_INC` reader"]
pub struct R(crate::R<ALARM0_INC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM0_INC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM0_INC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM0_INC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARM0_INC` writer"]
pub struct W(crate::W<ALARM0_INC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM0_INC_SPEC>;
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
impl From<crate::W<ALARM0_INC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM0_INC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INCREASE` reader - adder when ARLAM0 happen, helps to create periodical alarm"]
pub type INCREASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INCREASE` writer - adder when ARLAM0 happen, helps to create periodical alarm"]
pub type INCREASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALARM0_INC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - adder when ARLAM0 happen, helps to create periodical alarm"]
    #[inline(always)]
    pub fn increase(&self) -> INCREASE_R {
        INCREASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - adder when ARLAM0 happen, helps to create periodical alarm"]
    #[inline(always)]
    pub fn increase(&mut self) -> INCREASE_W<0> {
        INCREASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm0 incremental\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm0_inc](index.html) module"]
pub struct ALARM0_INC_SPEC;
impl crate::RegisterSpec for ALARM0_INC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm0_inc::R](R) reader structure"]
impl crate::Readable for ALARM0_INC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm0_inc::W](W) writer structure"]
impl crate::Writable for ALARM0_INC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALARM0_INC to value 0"]
impl crate::Resettable for ALARM0_INC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
