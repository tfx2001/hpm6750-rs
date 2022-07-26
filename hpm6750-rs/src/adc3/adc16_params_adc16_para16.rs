#[doc = "Register `ADC16_PARAMS_ADC16_PARA16` reader"]
pub struct R(crate::R<ADC16_PARAMS_ADC16_PARA16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC16_PARAMS_ADC16_PARA16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC16_PARAMS_ADC16_PARA16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC16_PARAMS_ADC16_PARA16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC16_PARAMS_ADC16_PARA16` writer"]
pub struct W(crate::W<ADC16_PARAMS_ADC16_PARA16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC16_PARAMS_ADC16_PARA16_SPEC>;
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
impl From<crate::W<ADC16_PARAMS_ADC16_PARA16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC16_PARAMS_ADC16_PARA16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARAM_VAL` reader - No description avaiable"]
pub type PARAM_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PARAM_VAL` writer - No description avaiable"]
pub type PARAM_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, ADC16_PARAMS_ADC16_PARA16_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - No description avaiable"]
    #[inline(always)]
    pub fn param_val(&self) -> PARAM_VAL_R {
        PARAM_VAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - No description avaiable"]
    #[inline(always)]
    pub fn param_val(&mut self) -> PARAM_VAL_W<0> {
        PARAM_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc16_params_adc16_para16](index.html) module"]
pub struct ADC16_PARAMS_ADC16_PARA16_SPEC;
impl crate::RegisterSpec for ADC16_PARAMS_ADC16_PARA16_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adc16_params_adc16_para16::R](R) reader structure"]
impl crate::Readable for ADC16_PARAMS_ADC16_PARA16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc16_params_adc16_para16::W](W) writer structure"]
impl crate::Writable for ADC16_PARAMS_ADC16_PARA16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC16_PARAMS_ADC16_PARA16 to value 0"]
impl crate::Resettable for ADC16_PARAMS_ADC16_PARA16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
