#[doc = "Register `PLL_PLL0_FREQ` reader"]
pub struct R(crate::R<PLL_PLL0_FREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_PLL0_FREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_PLL0_FREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_PLL0_FREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_PLL0_FREQ` writer"]
pub struct W(crate::W<PLL_PLL0_FREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_PLL0_FREQ_SPEC>;
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
impl From<crate::W<PLL_PLL0_FREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_PLL0_FREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBDIV_FRAC` reader - fbdiv used in frac mode"]
pub type FBDIV_FRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FBDIV_FRAC` writer - fbdiv used in frac mode"]
pub type FBDIV_FRAC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL_PLL0_FREQ_SPEC, u8, u8, 8, O>;
#[doc = "Field `FRAC` reader - PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)"]
pub type FRAC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRAC` writer - PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)"]
pub type FRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_PLL0_FREQ_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - fbdiv used in frac mode"]
    #[inline(always)]
    pub fn fbdiv_frac(&self) -> FBDIV_FRAC_R {
        FBDIV_FRAC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - fbdiv used in frac mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbdiv_frac(&mut self) -> FBDIV_FRAC_W<0> {
        FBDIV_FRAC_W::new(self)
    }
    #[doc = "Bits 8:31 - PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)"]
    #[inline(always)]
    #[must_use]
    pub fn frac(&mut self) -> FRAC_W<8> {
        FRAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLx frac mode frequency adjust\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_pll0_freq](index.html) module"]
pub struct PLL_PLL0_FREQ_SPEC;
impl crate::RegisterSpec for PLL_PLL0_FREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_pll0_freq::R](R) reader structure"]
impl crate::Readable for PLL_PLL0_FREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_pll0_freq::W](W) writer structure"]
impl crate::Writable for PLL_PLL0_FREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_PLL0_FREQ to value 0"]
impl crate::Resettable for PLL_PLL0_FREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
