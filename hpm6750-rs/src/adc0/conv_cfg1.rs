#[doc = "Register `CONV_CFG1` reader"]
pub struct R(crate::R<CONV_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONV_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONV_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONV_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONV_CFG1` writer"]
pub struct W(crate::W<CONV_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONV_CFG1_SPEC>;
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
impl From<crate::W<CONV_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONV_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLOCK_DIVIDER` reader - clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3. set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk"]
pub type CLOCK_DIVIDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_DIVIDER` writer - clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3. set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk"]
pub type CLOCK_DIVIDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONV_CFG1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CONVERT_CLOCK_NUMBER` reader - convert clock numbers, set to 13 (0xD) for 12bit mode, which means convert need 14 adc clock cycles(based on clock after divider); set to 11 for 10bit mode; set to 9 for 8bit mode; set to 7 or 6bit mode; Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 13 for 12bit mode, clock_divder to 2, then each ADC convertion(plus sample) need 18(14 convert, 4 sample) cycles(66MHz)."]
pub type CONVERT_CLOCK_NUMBER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONVERT_CLOCK_NUMBER` writer - convert clock numbers, set to 13 (0xD) for 12bit mode, which means convert need 14 adc clock cycles(based on clock after divider); set to 11 for 10bit mode; set to 9 for 8bit mode; set to 7 or 6bit mode; Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 13 for 12bit mode, clock_divder to 2, then each ADC convertion(plus sample) need 18(14 convert, 4 sample) cycles(66MHz)."]
pub type CONVERT_CLOCK_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONV_CFG1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:3 - clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3. set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk"]
    #[inline(always)]
    pub fn clock_divider(&self) -> CLOCK_DIVIDER_R {
        CLOCK_DIVIDER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - convert clock numbers, set to 13 (0xD) for 12bit mode, which means convert need 14 adc clock cycles(based on clock after divider); set to 11 for 10bit mode; set to 9 for 8bit mode; set to 7 or 6bit mode; Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 13 for 12bit mode, clock_divder to 2, then each ADC convertion(plus sample) need 18(14 convert, 4 sample) cycles(66MHz)."]
    #[inline(always)]
    pub fn convert_clock_number(&self) -> CONVERT_CLOCK_NUMBER_R {
        CONVERT_CLOCK_NUMBER_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3. set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk"]
    #[inline(always)]
    #[must_use]
    pub fn clock_divider(&mut self) -> CLOCK_DIVIDER_W<0> {
        CLOCK_DIVIDER_W::new(self)
    }
    #[doc = "Bits 4:8 - convert clock numbers, set to 13 (0xD) for 12bit mode, which means convert need 14 adc clock cycles(based on clock after divider); set to 11 for 10bit mode; set to 9 for 8bit mode; set to 7 or 6bit mode; Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 13 for 12bit mode, clock_divder to 2, then each ADC convertion(plus sample) need 18(14 convert, 4 sample) cycles(66MHz)."]
    #[inline(always)]
    #[must_use]
    pub fn convert_clock_number(&mut self) -> CONVERT_CLOCK_NUMBER_W<4> {
        CONVERT_CLOCK_NUMBER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conv_cfg1](index.html) module"]
pub struct CONV_CFG1_SPEC;
impl crate::RegisterSpec for CONV_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conv_cfg1::R](R) reader structure"]
impl crate::Readable for CONV_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conv_cfg1::W](W) writer structure"]
impl crate::Writable for CONV_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONV_CFG1 to value 0"]
impl crate::Resettable for CONV_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
