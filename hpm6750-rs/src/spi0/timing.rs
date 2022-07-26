#[doc = "Register `TIMING` reader"]
pub struct R(crate::R<TIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMING` writer"]
pub struct W(crate::W<TIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMING_SPEC>;
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
impl From<crate::W<TIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS2SCLK` reader - The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2"]
pub type CS2SCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CS2SCLK` writer - The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2"]
pub type CS2SCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `CSHT` reader - The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2"]
pub type CSHT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSHT` writer - The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2"]
pub type CSHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 4, O>;
#[doc = "Field `SCLK_DIV` reader - The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency."]
pub type SCLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLK_DIV` writer - The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency."]
pub type SCLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMING_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 12:13 - The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2"]
    #[inline(always)]
    pub fn cs2sclk(&self) -> CS2SCLK_R {
        CS2SCLK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 8:11 - The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2"]
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency."]
    #[inline(always)]
    pub fn sclk_div(&self) -> SCLK_DIV_R {
        SCLK_DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13 - The minimum time between the edges of SPI CS and the edges of SCLK. SCLK_period * (CS2SCLK + 1) / 2"]
    #[inline(always)]
    pub fn cs2sclk(&mut self) -> CS2SCLK_W<12> {
        CS2SCLK_W::new(self)
    }
    #[doc = "Bits 8:11 - The minimum time that SPI CS should stay HIGH. SCLK_period * (CSHT + 1) / 2"]
    #[inline(always)]
    pub fn csht(&mut self) -> CSHT_W<8> {
        CSHT_W::new(self)
    }
    #[doc = "Bits 0:7 - The clock frequency ratio between the clock source and SPI interface SCLK. SCLK_period = ((SCLK_DIV + 1) * 2) * (Period of the SPI clock source) The SCLK_DIV value 0xff is a special value which indicates that the SCLK frequency should be the same as the spi_clock frequency."]
    #[inline(always)]
    pub fn sclk_div(&mut self) -> SCLK_DIV_W<0> {
        SCLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interface Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timing](index.html) module"]
pub struct TIMING_SPEC;
impl crate::RegisterSpec for TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timing::R](R) reader structure"]
impl crate::Readable for TIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timing::W](W) writer structure"]
impl crate::Writable for TIMING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMING to value 0"]
impl crate::Resettable for TIMING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
