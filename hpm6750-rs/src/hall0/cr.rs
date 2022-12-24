#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTCNT` reader - set to reset all counter and related snapshots"]
pub type RSTCNT_R = crate::BitReader<bool>;
#[doc = "Field `RSTCNT` writer - set to reset all counter and related snapshots"]
pub type RSTCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SNAPEN` reader - 1- load ucnt, vcnt, wcnt and tmrcnt into their snap registers when snapi input assert"]
pub type SNAPEN_R = crate::BitReader<bool>;
#[doc = "Field `SNAPEN` writer - 1- load ucnt, vcnt, wcnt and tmrcnt into their snap registers when snapi input assert"]
pub type SNAPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `READ` writer - 1- load ucnt, vcnt, wcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0"]
pub type READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - set to reset all counter and related snapshots"]
    #[inline(always)]
    pub fn rstcnt(&self) -> RSTCNT_R {
        RSTCNT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 11 - 1- load ucnt, vcnt, wcnt and tmrcnt into their snap registers when snapi input assert"]
    #[inline(always)]
    pub fn snapen(&self) -> SNAPEN_R {
        SNAPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - set to reset all counter and related snapshots"]
    #[inline(always)]
    #[must_use]
    pub fn rstcnt(&mut self) -> RSTCNT_W<4> {
        RSTCNT_W::new(self)
    }
    #[doc = "Bit 11 - 1- load ucnt, vcnt, wcnt and tmrcnt into their snap registers when snapi input assert"]
    #[inline(always)]
    #[must_use]
    pub fn snapen(&mut self) -> SNAPEN_W<11> {
        SNAPEN_W::new(self)
    }
    #[doc = "Bit 31 - 1- load ucnt, vcnt, wcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<31> {
        READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
