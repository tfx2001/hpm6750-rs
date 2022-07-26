#[doc = "Register `RNG` reader"]
pub struct R(crate::R<RNG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNG` writer"]
pub struct W(crate::W<RNG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNG_SPEC>;
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
impl From<crate::W<RNG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCK_RNG_XOR` reader - block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software"]
pub type BLOCK_RNG_XOR_R = crate::BitReader<bool>;
#[doc = "Field `BLOCK_RNG_XOR` writer - block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software"]
pub type BLOCK_RNG_XOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RNG_SPEC, bool, O>;
#[doc = "Field `RNG_XOR` reader - control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG"]
pub type RNG_XOR_R = crate::BitReader<bool>;
#[doc = "Field `RNG_XOR` writer - control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG"]
pub type RNG_XOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RNG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software"]
    #[inline(always)]
    pub fn block_rng_xor(&self) -> BLOCK_RNG_XOR_R {
        BLOCK_RNG_XOR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 0 - control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG"]
    #[inline(always)]
    pub fn rng_xor(&self) -> RNG_XOR_R {
        RNG_XOR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software"]
    #[inline(always)]
    pub fn block_rng_xor(&mut self) -> BLOCK_RNG_XOR_W<16> {
        BLOCK_RNG_XOR_W::new(self)
    }
    #[doc = "Bit 0 - control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG"]
    #[inline(always)]
    pub fn rng_xor(&mut self) -> RNG_XOR_W<0> {
        RNG_XOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Random number interface behavior\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng](index.html) module"]
pub struct RNG_SPEC;
impl crate::RegisterSpec for RNG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng::R](R) reader structure"]
impl crate::Readable for RNG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rng::W](W) writer structure"]
impl crate::Writable for RNG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNG to value 0"]
impl crate::Resettable for RNG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
