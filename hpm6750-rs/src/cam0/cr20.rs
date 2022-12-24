#[doc = "Register `CR20` reader"]
pub struct R(crate::R<CR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR20` writer"]
pub struct W(crate::W<CR20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR20_SPEC>;
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
impl From<crate::W<CR20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRESHOLD` reader - Threshold to generate binary color. Bin 1 is output if the pixel is greater than the threshold."]
pub type THRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRESHOLD` writer - Threshold to generate binary color. Bin 1 is output if the pixel is greater than the threshold."]
pub type THRESHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR20_SPEC, u8, u8, 8, O>;
#[doc = "Field `BIG_END` reader - Asserted when binary output is in big-endian type, which mean the right most data is at the LSBs. Take function only inside the 32-bit word."]
pub type BIG_END_R = crate::BitReader<bool>;
#[doc = "Field `BIG_END` writer - Asserted when binary output is in big-endian type, which mean the right most data is at the LSBs. Take function only inside the 32-bit word."]
pub type BIG_END_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR20_SPEC, bool, O>;
#[doc = "Field `HISTOGRAM_EN` reader - histogarm enable"]
pub type HISTOGRAM_EN_R = crate::BitReader<bool>;
#[doc = "Field `HISTOGRAM_EN` writer - histogarm enable"]
pub type HISTOGRAM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR20_SPEC, bool, O>;
#[doc = "Field `BINARY_EN` reader - binary picture output enable"]
pub type BINARY_EN_R = crate::BitReader<bool>;
#[doc = "Field `BINARY_EN` writer - binary picture output enable"]
pub type BINARY_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR20_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Threshold to generate binary color. Bin 1 is output if the pixel is greater than the threshold."]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Asserted when binary output is in big-endian type, which mean the right most data is at the LSBs. Take function only inside the 32-bit word."]
    #[inline(always)]
    pub fn big_end(&self) -> BIG_END_R {
        BIG_END_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 30 - histogarm enable"]
    #[inline(always)]
    pub fn histogram_en(&self) -> HISTOGRAM_EN_R {
        HISTOGRAM_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - binary picture output enable"]
    #[inline(always)]
    pub fn binary_en(&self) -> BINARY_EN_R {
        BINARY_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold to generate binary color. Bin 1 is output if the pixel is greater than the threshold."]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> THRESHOLD_W<0> {
        THRESHOLD_W::new(self)
    }
    #[doc = "Bit 8 - Asserted when binary output is in big-endian type, which mean the right most data is at the LSBs. Take function only inside the 32-bit word."]
    #[inline(always)]
    #[must_use]
    pub fn big_end(&mut self) -> BIG_END_W<8> {
        BIG_END_W::new(self)
    }
    #[doc = "Bit 30 - histogarm enable"]
    #[inline(always)]
    #[must_use]
    pub fn histogram_en(&mut self) -> HISTOGRAM_EN_W<30> {
        HISTOGRAM_EN_W::new(self)
    }
    #[doc = "Bit 31 - binary picture output enable"]
    #[inline(always)]
    #[must_use]
    pub fn binary_en(&mut self) -> BINARY_EN_W<31> {
        BINARY_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control CR20 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr20](index.html) module"]
pub struct CR20_SPEC;
impl crate::RegisterSpec for CR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr20::R](R) reader structure"]
impl crate::Readable for CR20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr20::W](W) writer structure"]
impl crate::Writable for CR20_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR20 to value 0"]
impl crate::Resettable for CR20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
