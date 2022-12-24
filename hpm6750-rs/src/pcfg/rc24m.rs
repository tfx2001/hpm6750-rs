#[doc = "Register `RC24M` reader"]
pub struct R(crate::R<RC24M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC24M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC24M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC24M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC24M` writer"]
pub struct W(crate::W<RC24M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC24M_SPEC>;
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
impl From<crate::W<RC24M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC24M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM_F` reader - Fine trim for RC24M, bigger value means faster"]
pub type TRIM_F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_F` writer - Fine trim for RC24M, bigger value means faster"]
pub type TRIM_F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RC24M_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRIM_C` reader - Coarse trim for RC24M, bigger value means faster"]
pub type TRIM_C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_C` writer - Coarse trim for RC24M, bigger value means faster"]
pub type TRIM_C_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RC24M_SPEC, u8, u8, 3, O>;
#[doc = "Field `RC_TRIMMED` reader - RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed"]
pub type RC_TRIMMED_R = crate::BitReader<bool>;
#[doc = "Field `RC_TRIMMED` writer - RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed"]
pub type RC_TRIMMED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC24M_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Fine trim for RC24M, bigger value means faster"]
    #[inline(always)]
    pub fn trim_f(&self) -> TRIM_F_R {
        TRIM_F_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Coarse trim for RC24M, bigger value means faster"]
    #[inline(always)]
    pub fn trim_c(&self) -> TRIM_C_R {
        TRIM_C_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 31 - RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed"]
    #[inline(always)]
    pub fn rc_trimmed(&self) -> RC_TRIMMED_R {
        RC_TRIMMED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Fine trim for RC24M, bigger value means faster"]
    #[inline(always)]
    #[must_use]
    pub fn trim_f(&mut self) -> TRIM_F_W<0> {
        TRIM_F_W::new(self)
    }
    #[doc = "Bits 8:10 - Coarse trim for RC24M, bigger value means faster"]
    #[inline(always)]
    #[must_use]
    pub fn trim_c(&mut self) -> TRIM_C_W<8> {
        TRIM_C_W::new(self)
    }
    #[doc = "Bit 31 - RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed"]
    #[inline(always)]
    #[must_use]
    pub fn rc_trimmed(&mut self) -> RC_TRIMMED_W<31> {
        RC_TRIMMED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RC 24M config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc24m](index.html) module"]
pub struct RC24M_SPEC;
impl crate::RegisterSpec for RC24M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc24m::R](R) reader structure"]
impl crate::Readable for RC24M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc24m::W](W) writer structure"]
impl crate::Writable for RC24M_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC24M to value 0x0316"]
impl crate::Resettable for RC24M_SPEC {
    const RESET_VALUE: Self::Ux = 0x0316;
}
