#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLFCHK` reader - Self Test, when both ST and GS triggered, ST first and GS next."]
pub type SLFCHK_R = crate::BitReader<bool>;
#[doc = "Field `SLFCHK` writer - Self Test, when both ST and GS triggered, ST first and GS next."]
pub type SLFCHK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `GENSD` reader - Generate Seed, when both ST and GS triggered, ST first and GS next."]
pub type GENSD_R = crate::BitReader<bool>;
#[doc = "Field `GENSD` writer - Generate Seed, when both ST and GS triggered, ST first and GS next."]
pub type GENSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CLRINT` reader - Clear the Interrupt, clear the RNG interrupt if an error is not present. This bit is self-clearing. 0 Do not clear the interrupt. 1 Clear the interrupt"]
pub type CLRINT_R = crate::BitReader<bool>;
#[doc = "Field `CLRINT` writer - Clear the Interrupt, clear the RNG interrupt if an error is not present. This bit is self-clearing. 0 Do not clear the interrupt. 1 Clear the interrupt"]
pub type CLRINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CLRERR` reader - Clear the Error, clear the errors in the ESR register and the RNG interrupt. This bit is self-clearing. 0 Do not clear the errors and the interrupt. 1 Clear the errors and the interrupt."]
pub type CLRERR_R = crate::BitReader<bool>;
#[doc = "Field `CLRERR` writer - Clear the Error, clear the errors in the ESR register and the RNG interrupt. This bit is self-clearing. 0 Do not clear the errors and the interrupt. 1 Clear the errors and the interrupt."]
pub type CLRERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `SFTRST` reader - Soft Reset, Perform a software reset of the RNG This bit is self-clearing. 0 Do not perform a software reset. 1 Software reset"]
pub type SFTRST_R = crate::BitReader<bool>;
#[doc = "Field `SFTRST` writer - Soft Reset, Perform a software reset of the RNG This bit is self-clearing. 0 Do not perform a software reset. 1 Software reset"]
pub type SFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Self Test, when both ST and GS triggered, ST first and GS next."]
    #[inline(always)]
    pub fn slfchk(&self) -> SLFCHK_R {
        SLFCHK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generate Seed, when both ST and GS triggered, ST first and GS next."]
    #[inline(always)]
    pub fn gensd(&self) -> GENSD_R {
        GENSD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear the Interrupt, clear the RNG interrupt if an error is not present. This bit is self-clearing. 0 Do not clear the interrupt. 1 Clear the interrupt"]
    #[inline(always)]
    pub fn clrint(&self) -> CLRINT_R {
        CLRINT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear the Error, clear the errors in the ESR register and the RNG interrupt. This bit is self-clearing. 0 Do not clear the errors and the interrupt. 1 Clear the errors and the interrupt."]
    #[inline(always)]
    pub fn clrerr(&self) -> CLRERR_R {
        CLRERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Soft Reset, Perform a software reset of the RNG This bit is self-clearing. 0 Do not perform a software reset. 1 Software reset"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Self Test, when both ST and GS triggered, ST first and GS next."]
    #[inline(always)]
    #[must_use]
    pub fn slfchk(&mut self) -> SLFCHK_W<0> {
        SLFCHK_W::new(self)
    }
    #[doc = "Bit 1 - Generate Seed, when both ST and GS triggered, ST first and GS next."]
    #[inline(always)]
    #[must_use]
    pub fn gensd(&mut self) -> GENSD_W<1> {
        GENSD_W::new(self)
    }
    #[doc = "Bit 4 - Clear the Interrupt, clear the RNG interrupt if an error is not present. This bit is self-clearing. 0 Do not clear the interrupt. 1 Clear the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clrint(&mut self) -> CLRINT_W<4> {
        CLRINT_W::new(self)
    }
    #[doc = "Bit 5 - Clear the Error, clear the errors in the ESR register and the RNG interrupt. This bit is self-clearing. 0 Do not clear the errors and the interrupt. 1 Clear the errors and the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn clrerr(&mut self) -> CLRERR_W<5> {
        CLRERR_W::new(self)
    }
    #[doc = "Bit 6 - Soft Reset, Perform a software reset of the RNG This bit is self-clearing. 0 Do not perform a software reset. 1 Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn sftrst(&mut self) -> SFTRST_W<6> {
        SFTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
