#[doc = "Register `DCDC_ADVPARAM` reader"]
pub struct R(crate::R<DCDC_ADVPARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC_ADVPARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC_ADVPARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC_ADVPARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC_ADVPARAM` writer"]
pub struct W(crate::W<DCDC_ADVPARAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC_ADVPARAM_SPEC>;
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
impl From<crate::W<DCDC_ADVPARAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC_ADVPARAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIN_DUT` reader - minimum duty cycle"]
pub type MIN_DUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN_DUT` writer - minimum duty cycle"]
pub type MIN_DUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDC_ADVPARAM_SPEC, u8, u8, 7, O>;
#[doc = "Field `MAX_DUT` reader - maximum duty cycle"]
pub type MAX_DUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_DUT` writer - maximum duty cycle"]
pub type MAX_DUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDC_ADVPARAM_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 8:14 - minimum duty cycle"]
    #[inline(always)]
    pub fn min_dut(&self) -> MIN_DUT_R {
        MIN_DUT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6 - maximum duty cycle"]
    #[inline(always)]
    pub fn max_dut(&self) -> MAX_DUT_R {
        MAX_DUT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - minimum duty cycle"]
    #[inline(always)]
    pub fn min_dut(&mut self) -> MIN_DUT_W<8> {
        MIN_DUT_W::new(self)
    }
    #[doc = "Bits 0:6 - maximum duty cycle"]
    #[inline(always)]
    pub fn max_dut(&mut self) -> MAX_DUT_W<0> {
        MAX_DUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC advance parameter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_advparam](index.html) module"]
pub struct DCDC_ADVPARAM_SPEC;
impl crate::RegisterSpec for DCDC_ADVPARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc_advparam::R](R) reader structure"]
impl crate::Readable for DCDC_ADVPARAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc_advparam::W](W) writer structure"]
impl crate::Writable for DCDC_ADVPARAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDC_ADVPARAM to value 0x00ef_1c6e"]
impl crate::Resettable for DCDC_ADVPARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ef_1c6e
    }
}
