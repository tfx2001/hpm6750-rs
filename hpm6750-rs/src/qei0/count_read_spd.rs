#[doc = "Register `COUNT_READ_SPD` reader"]
pub struct R(crate::R<COUNT_READ_SPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT_READ_SPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT_READ_SPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT_READ_SPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT_READ_SPD` writer"]
pub struct W(crate::W<COUNT_READ_SPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT_READ_SPD_SPEC>;
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
impl From<crate::W<COUNT_READ_SPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT_READ_SPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPDCNT` reader - spdcnt value"]
pub type SPDCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BSTAT` reader - 1- b input is high 0- b input is low"]
pub type BSTAT_R = crate::BitReader<bool>;
#[doc = "Field `BSTAT` writer - 1- b input is high 0- b input is low"]
pub type BSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, COUNT_READ_SPD_SPEC, bool, O>;
#[doc = "Field `ASTAT` reader - 1- a input is high 0- a input is low"]
pub type ASTAT_R = crate::BitReader<bool>;
#[doc = "Field `DIR` reader - 1- reverse rotation 0- forward rotation"]
pub type DIR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:27 - spdcnt value"]
    #[inline(always)]
    pub fn spdcnt(&self) -> SPDCNT_R {
        SPDCNT_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 29 - 1- b input is high 0- b input is low"]
    #[inline(always)]
    pub fn bstat(&self) -> BSTAT_R {
        BSTAT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- a input is high 0- a input is low"]
    #[inline(always)]
    pub fn astat(&self) -> ASTAT_R {
        ASTAT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- reverse rotation 0- forward rotation"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - 1- b input is high 0- b input is low"]
    #[inline(always)]
    #[must_use]
    pub fn bstat(&mut self) -> BSTAT_W<29> {
        BSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Speed counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_read_spd](index.html) module"]
pub struct COUNT_READ_SPD_SPEC;
impl crate::RegisterSpec for COUNT_READ_SPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count_read_spd::R](R) reader structure"]
impl crate::Readable for COUNT_READ_SPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count_read_spd::W](W) writer structure"]
impl crate::Writable for COUNT_READ_SPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNT_READ_SPD to value 0"]
impl crate::Resettable for COUNT_READ_SPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
