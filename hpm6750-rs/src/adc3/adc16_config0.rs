#[doc = "Register `ADC16_CONFIG0` reader"]
pub struct R(crate::R<ADC16_CONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC16_CONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC16_CONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC16_CONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC16_CONFIG0` writer"]
pub struct W(crate::W<ADC16_CONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC16_CONFIG0_SPEC>;
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
impl From<crate::W<ADC16_CONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC16_CONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONV_PARAM` reader - convertion parameter"]
pub type CONV_PARAM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONV_PARAM` writer - convertion parameter"]
pub type CONV_PARAM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC16_CONFIG0_SPEC, u16, u16, 14, O>;
#[doc = "Field `PREEMPT_EN` reader - set to enable preemption feature"]
pub type PREEMPT_EN_R = crate::BitReader<bool>;
#[doc = "Field `PREEMPT_EN` writer - set to enable preemption feature"]
pub type PREEMPT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC16_CONFIG0_SPEC, bool, O>;
#[doc = "Field `CAL_AVG_CFG` reader - for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved"]
pub type CAL_AVG_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAL_AVG_CFG` writer - for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved"]
pub type CAL_AVG_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC16_CONFIG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `BANDGAP_EN` reader - set to enable bandgap. user should set reg_en and bandgap_en before use adc16."]
pub type BANDGAP_EN_R = crate::BitReader<bool>;
#[doc = "Field `BANDGAP_EN` writer - set to enable bandgap. user should set reg_en and bandgap_en before use adc16."]
pub type BANDGAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC16_CONFIG0_SPEC, bool, O>;
#[doc = "Field `REG_EN` reader - set to enable regulator"]
pub type REG_EN_R = crate::BitReader<bool>;
#[doc = "Field `REG_EN` writer - set to enable regulator"]
pub type REG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC16_CONFIG0_SPEC, bool, O>;
#[doc = "Field `TEMPSNS_EN` reader - set to enable temp senser"]
pub type TEMPSNS_EN_R = crate::BitReader<bool>;
#[doc = "Field `TEMPSNS_EN` writer - set to enable temp senser"]
pub type TEMPSNS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC16_CONFIG0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - convertion parameter"]
    #[inline(always)]
    pub fn conv_param(&self) -> CONV_PARAM_R {
        CONV_PARAM_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - set to enable preemption feature"]
    #[inline(always)]
    pub fn preempt_en(&self) -> PREEMPT_EN_R {
        PREEMPT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 20:22 - for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved"]
    #[inline(always)]
    pub fn cal_avg_cfg(&self) -> CAL_AVG_CFG_R {
        CAL_AVG_CFG_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - set to enable bandgap. user should set reg_en and bandgap_en before use adc16."]
    #[inline(always)]
    pub fn bandgap_en(&self) -> BANDGAP_EN_R {
        BANDGAP_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - set to enable regulator"]
    #[inline(always)]
    pub fn reg_en(&self) -> REG_EN_R {
        REG_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - set to enable temp senser"]
    #[inline(always)]
    pub fn tempsns_en(&self) -> TEMPSNS_EN_R {
        TEMPSNS_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - convertion parameter"]
    #[inline(always)]
    #[must_use]
    pub fn conv_param(&mut self) -> CONV_PARAM_W<0> {
        CONV_PARAM_W::new(self)
    }
    #[doc = "Bit 14 - set to enable preemption feature"]
    #[inline(always)]
    #[must_use]
    pub fn preempt_en(&mut self) -> PREEMPT_EN_W<14> {
        PREEMPT_EN_W::new(self)
    }
    #[doc = "Bits 20:22 - for average the calibration result. 0- 1 loop; 1- 2 loops; 2- 4 loops; 3- 8 loops; 4- 16 loops; 5-32 loops; others reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cal_avg_cfg(&mut self) -> CAL_AVG_CFG_W<20> {
        CAL_AVG_CFG_W::new(self)
    }
    #[doc = "Bit 23 - set to enable bandgap. user should set reg_en and bandgap_en before use adc16."]
    #[inline(always)]
    #[must_use]
    pub fn bandgap_en(&mut self) -> BANDGAP_EN_W<23> {
        BANDGAP_EN_W::new(self)
    }
    #[doc = "Bit 24 - set to enable regulator"]
    #[inline(always)]
    #[must_use]
    pub fn reg_en(&mut self) -> REG_EN_W<24> {
        REG_EN_W::new(self)
    }
    #[doc = "Bit 25 - set to enable temp senser"]
    #[inline(always)]
    #[must_use]
    pub fn tempsns_en(&mut self) -> TEMPSNS_EN_W<25> {
        TEMPSNS_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc16_config0](index.html) module"]
pub struct ADC16_CONFIG0_SPEC;
impl crate::RegisterSpec for ADC16_CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc16_config0::R](R) reader structure"]
impl crate::Readable for ADC16_CONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc16_config0::W](W) writer structure"]
impl crate::Writable for ADC16_CONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC16_CONFIG0 to value 0"]
impl crate::Resettable for ADC16_CONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
