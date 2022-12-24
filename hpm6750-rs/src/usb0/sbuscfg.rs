#[doc = "Register `SBUSCFG` reader"]
pub struct R(crate::R<SBUSCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBUSCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBUSCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBUSCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBUSCFG` writer"]
pub struct W(crate::W<SBUSCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBUSCFG_SPEC>;
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
impl From<crate::W<SBUSCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBUSCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHBBRST` reader - AHBBRST AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority). NOTE: This register overrides n_BURSTSIZE register when its value is not zero. 000 - Incremental burst of unspecified length only 001 - INCR4 burst, then single transfer 010 - INCR8 burst, INCR4 burst, then single transfer 011 - INCR16 burst, INCR8 burst, INCR4 burst, then single transfer 100 - Reserved, don't use 101 - INCR4 burst, then incremental burst of unspecified length 110 - INCR8 burst, INCR4 burst, then incremental burst of unspecified length 111 - INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
pub type AHBBRST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHBBRST` writer - AHBBRST AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority). NOTE: This register overrides n_BURSTSIZE register when its value is not zero. 000 - Incremental burst of unspecified length only 001 - INCR4 burst, then single transfer 010 - INCR8 burst, INCR4 burst, then single transfer 011 - INCR16 burst, INCR8 burst, INCR4 burst, then single transfer 100 - Reserved, don't use 101 - INCR4 burst, then incremental burst of unspecified length 110 - INCR8 burst, INCR4 burst, then incremental burst of unspecified length 111 - INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
pub type AHBBRST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SBUSCFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - AHBBRST AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority). NOTE: This register overrides n_BURSTSIZE register when its value is not zero. 000 - Incremental burst of unspecified length only 001 - INCR4 burst, then single transfer 010 - INCR8 burst, INCR4 burst, then single transfer 011 - INCR16 burst, INCR8 burst, INCR4 burst, then single transfer 100 - Reserved, don't use 101 - INCR4 burst, then incremental burst of unspecified length 110 - INCR8 burst, INCR4 burst, then incremental burst of unspecified length 111 - INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    #[inline(always)]
    pub fn ahbbrst(&self) -> AHBBRST_R {
        AHBBRST_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - AHBBRST AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority). NOTE: This register overrides n_BURSTSIZE register when its value is not zero. 000 - Incremental burst of unspecified length only 001 - INCR4 burst, then single transfer 010 - INCR8 burst, INCR4 burst, then single transfer 011 - INCR16 burst, INCR8 burst, INCR4 burst, then single transfer 100 - Reserved, don't use 101 - INCR4 burst, then incremental burst of unspecified length 110 - INCR8 burst, INCR4 burst, then incremental burst of unspecified length 111 - INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    #[inline(always)]
    #[must_use]
    pub fn ahbbrst(&mut self) -> AHBBRST_W<0> {
        AHBBRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Bus Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbuscfg](index.html) module"]
pub struct SBUSCFG_SPEC;
impl crate::RegisterSpec for SBUSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbuscfg::R](R) reader structure"]
impl crate::Readable for SBUSCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbuscfg::W](W) writer structure"]
impl crate::Writable for SBUSCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBUSCFG to value 0"]
impl crate::Resettable for SBUSCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
