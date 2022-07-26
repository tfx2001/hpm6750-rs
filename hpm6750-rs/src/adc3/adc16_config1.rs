#[doc = "Register `ADC16_CONFIG1` reader"]
pub struct R(crate::R<ADC16_CONFIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC16_CONFIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC16_CONFIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC16_CONFIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC16_CONFIG1` writer"]
pub struct W(crate::W<ADC16_CONFIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC16_CONFIG1_SPEC>;
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
impl From<crate::W<ADC16_CONFIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC16_CONFIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COV_END_CNT` reader - used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number)."]
pub type COV_END_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COV_END_CNT` writer - used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number)."]
pub type COV_END_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC16_CONFIG1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 8:12 - used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number)."]
    #[inline(always)]
    pub fn cov_end_cnt(&self) -> COV_END_CNT_R {
        COV_END_CNT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - used for faster conversion, user can change it to get higher convert speed(but less accuracy). should set to (21-convert_clock_number)."]
    #[inline(always)]
    pub fn cov_end_cnt(&mut self) -> COV_END_CNT_W<8> {
        COV_END_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc16_config1](index.html) module"]
pub struct ADC16_CONFIG1_SPEC;
impl crate::RegisterSpec for ADC16_CONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc16_config1::R](R) reader structure"]
impl crate::Readable for ADC16_CONFIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc16_config1::W](W) writer structure"]
impl crate::Writable for ADC16_CONFIG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC16_CONFIG1 to value 0"]
impl crate::Resettable for ADC16_CONFIG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
