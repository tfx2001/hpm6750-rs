#[doc = "Register `ALARM_FLAG` reader"]
pub struct R(crate::R<ALARM_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARM_FLAG` writer"]
pub struct W(crate::W<ALARM_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM_FLAG_SPEC>;
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
impl From<crate::W<ALARM_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARM0` reader - alarm0 happen"]
pub type ALARM0_R = crate::BitReader<bool>;
#[doc = "Field `ALARM0` writer - alarm0 happen"]
pub type ALARM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALARM_FLAG_SPEC, bool, O>;
#[doc = "Field `ALARM1` reader - alarm1 happen"]
pub type ALARM1_R = crate::BitReader<bool>;
#[doc = "Field `ALARM1` writer - alarm1 happen"]
pub type ALARM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALARM_FLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - alarm0 happen"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - alarm1 happen"]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - alarm0 happen"]
    #[inline(always)]
    #[must_use]
    pub fn alarm0(&mut self) -> ALARM0_W<0> {
        ALARM0_W::new(self)
    }
    #[doc = "Bit 1 - alarm1 happen"]
    #[inline(always)]
    #[must_use]
    pub fn alarm1(&mut self) -> ALARM1_W<1> {
        ALARM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC alarm flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm_flag](index.html) module"]
pub struct ALARM_FLAG_SPEC;
impl crate::RegisterSpec for ALARM_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm_flag::R](R) reader structure"]
impl crate::Readable for ALARM_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm_flag::W](W) writer structure"]
impl crate::Writable for ALARM_FLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALARM_FLAG to value 0"]
impl crate::Resettable for ALARM_FLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
