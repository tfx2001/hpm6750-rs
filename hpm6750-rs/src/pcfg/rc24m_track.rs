#[doc = "Register `RC24M_TRACK` reader"]
pub struct R(crate::R<RC24M_TRACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC24M_TRACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC24M_TRACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC24M_TRACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC24M_TRACK` writer"]
pub struct W(crate::W<RC24M_TRACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC24M_TRACK_SPEC>;
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
impl From<crate::W<RC24M_TRACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC24M_TRACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRACK` reader - track mode 0: RC24M free running 1: track RC24M to external XTAL"]
pub type TRACK_R = crate::BitReader<bool>;
#[doc = "Field `TRACK` writer - track mode 0: RC24M free running 1: track RC24M to external XTAL"]
pub type TRACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC24M_TRACK_SPEC, bool, O>;
#[doc = "Field `RETURN` reader - Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value"]
pub type RETURN_R = crate::BitReader<bool>;
#[doc = "Field `RETURN` writer - Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value"]
pub type RETURN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC24M_TRACK_SPEC, bool, O>;
#[doc = "Field `SEL24M` reader - Select track reference 0: select 32K as reference 1: select 24M XTAL as reference"]
pub type SEL24M_R = crate::BitReader<bool>;
#[doc = "Field `SEL24M` writer - Select track reference 0: select 32K as reference 1: select 24M XTAL as reference"]
pub type SEL24M_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC24M_TRACK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - track mode 0: RC24M free running 1: track RC24M to external XTAL"]
    #[inline(always)]
    pub fn track(&self) -> TRACK_R {
        TRACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value"]
    #[inline(always)]
    pub fn return_(&self) -> RETURN_R {
        RETURN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Select track reference 0: select 32K as reference 1: select 24M XTAL as reference"]
    #[inline(always)]
    pub fn sel24m(&self) -> SEL24M_R {
        SEL24M_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - track mode 0: RC24M free running 1: track RC24M to external XTAL"]
    #[inline(always)]
    #[must_use]
    pub fn track(&mut self) -> TRACK_W<0> {
        TRACK_W::new(self)
    }
    #[doc = "Bit 4 - Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value"]
    #[inline(always)]
    #[must_use]
    pub fn return_(&mut self) -> RETURN_W<4> {
        RETURN_W::new(self)
    }
    #[doc = "Bit 16 - Select track reference 0: select 32K as reference 1: select 24M XTAL as reference"]
    #[inline(always)]
    #[must_use]
    pub fn sel24m(&mut self) -> SEL24M_W<16> {
        SEL24M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RC 24M track mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc24m_track](index.html) module"]
pub struct RC24M_TRACK_SPEC;
impl crate::RegisterSpec for RC24M_TRACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc24m_track::R](R) reader structure"]
impl crate::Readable for RC24M_TRACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc24m_track::W](W) writer structure"]
impl crate::Writable for RC24M_TRACK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC24M_TRACK to value 0"]
impl crate::Resettable for RC24M_TRACK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
