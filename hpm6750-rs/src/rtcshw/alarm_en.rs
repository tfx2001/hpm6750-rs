#[doc = "Register `ALARM_EN` reader"]
pub struct R(crate::R<ALARM_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARM_EN` writer"]
pub struct W(crate::W<ALARM_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM_EN_SPEC>;
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
impl From<crate::W<ALARM_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE1` reader - alarm1 mask 0: alarm1 disabled 1: alarm1 enabled"]
pub type ENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE1` writer - alarm1 mask 0: alarm1 disabled 1: alarm1 enabled"]
pub type ENABLE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALARM_EN_SPEC, bool, O>;
#[doc = "Field `ENABLE0` reader - alarm0 mask 0: alarm0 disabled 1: alarm0 enabled"]
pub type ENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE0` writer - alarm0 mask 0: alarm0 disabled 1: alarm0 enabled"]
pub type ENABLE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALARM_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - alarm1 mask 0: alarm1 disabled 1: alarm1 enabled"]
    #[inline(always)]
    pub fn enable1(&self) -> ENABLE1_R {
        ENABLE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - alarm0 mask 0: alarm0 disabled 1: alarm0 enabled"]
    #[inline(always)]
    pub fn enable0(&self) -> ENABLE0_R {
        ENABLE0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - alarm1 mask 0: alarm1 disabled 1: alarm1 enabled"]
    #[inline(always)]
    pub fn enable1(&mut self) -> ENABLE1_W<1> {
        ENABLE1_W::new(self)
    }
    #[doc = "Bit 0 - alarm0 mask 0: alarm0 disabled 1: alarm0 enabled"]
    #[inline(always)]
    pub fn enable0(&mut self) -> ENABLE0_W<0> {
        ENABLE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC alarm enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm_en](index.html) module"]
pub struct ALARM_EN_SPEC;
impl crate::RegisterSpec for ALARM_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm_en::R](R) reader structure"]
impl crate::Readable for ALARM_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm_en::W](W) writer structure"]
impl crate::Writable for ALARM_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALARM_EN to value 0"]
impl crate::Resettable for ALARM_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
