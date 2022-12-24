#[doc = "Register `SAMPLE_CFG_CHN9` reader"]
pub struct R(crate::R<SAMPLE_CFG_CHN9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLE_CFG_CHN9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLE_CFG_CHN9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLE_CFG_CHN9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPLE_CFG_CHN9` writer"]
pub struct W(crate::W<SAMPLE_CFG_CHN9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLE_CFG_CHN9_SPEC>;
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
impl From<crate::W<SAMPLE_CFG_CHN9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLE_CFG_CHN9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPLE_CLOCK_NUMBER` reader - sample clock number, base on clock_period, default one period"]
pub type SAMPLE_CLOCK_NUMBER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAMPLE_CLOCK_NUMBER` writer - sample clock number, base on clock_period, default one period"]
pub type SAMPLE_CLOCK_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAMPLE_CFG_CHN9_SPEC, u16, u16, 9, O>;
#[doc = "Field `SAMPLE_CLOCK_NUMBER_SHIFT` reader - shift for sample_clock_number"]
pub type SAMPLE_CLOCK_NUMBER_SHIFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMPLE_CLOCK_NUMBER_SHIFT` writer - shift for sample_clock_number"]
pub type SAMPLE_CLOCK_NUMBER_SHIFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAMPLE_CFG_CHN9_SPEC, u8, u8, 3, O>;
#[doc = "Field `DIFF_SEL` reader - set to 1 to select differential channel"]
pub type DIFF_SEL_R = crate::BitReader<bool>;
#[doc = "Field `DIFF_SEL` writer - set to 1 to select differential channel"]
pub type DIFF_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAMPLE_CFG_CHN9_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - sample clock number, base on clock_period, default one period"]
    #[inline(always)]
    pub fn sample_clock_number(&self) -> SAMPLE_CLOCK_NUMBER_R {
        SAMPLE_CLOCK_NUMBER_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - shift for sample_clock_number"]
    #[inline(always)]
    pub fn sample_clock_number_shift(&self) -> SAMPLE_CLOCK_NUMBER_SHIFT_R {
        SAMPLE_CLOCK_NUMBER_SHIFT_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - set to 1 to select differential channel"]
    #[inline(always)]
    pub fn diff_sel(&self) -> DIFF_SEL_R {
        DIFF_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - sample clock number, base on clock_period, default one period"]
    #[inline(always)]
    #[must_use]
    pub fn sample_clock_number(&mut self) -> SAMPLE_CLOCK_NUMBER_W<0> {
        SAMPLE_CLOCK_NUMBER_W::new(self)
    }
    #[doc = "Bits 9:11 - shift for sample_clock_number"]
    #[inline(always)]
    #[must_use]
    pub fn sample_clock_number_shift(&mut self) -> SAMPLE_CLOCK_NUMBER_SHIFT_W<9> {
        SAMPLE_CLOCK_NUMBER_SHIFT_W::new(self)
    }
    #[doc = "Bit 12 - set to 1 to select differential channel"]
    #[inline(always)]
    #[must_use]
    pub fn diff_sel(&mut self) -> DIFF_SEL_W<12> {
        DIFF_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample_cfg_chn9](index.html) module"]
pub struct SAMPLE_CFG_CHN9_SPEC;
impl crate::RegisterSpec for SAMPLE_CFG_CHN9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sample_cfg_chn9::R](R) reader structure"]
impl crate::Readable for SAMPLE_CFG_CHN9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sample_cfg_chn9::W](W) writer structure"]
impl crate::Writable for SAMPLE_CFG_CHN9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPLE_CFG_CHN9 to value 0"]
impl crate::Resettable for SAMPLE_CFG_CHN9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
