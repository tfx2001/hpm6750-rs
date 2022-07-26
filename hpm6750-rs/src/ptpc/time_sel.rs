#[doc = "Register `TIME_SEL` reader"]
pub struct R(crate::R<TIME_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIME_SEL` writer"]
pub struct W(crate::W<TIME_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIME_SEL_SPEC>;
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
impl From<crate::W<TIME_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIME_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN3_TIME_SEL` reader - No description avaiable"]
pub type CAN3_TIME_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CAN3_TIME_SEL` writer - No description avaiable"]
pub type CAN3_TIME_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIME_SEL_SPEC, bool, O>;
#[doc = "Field `CAN2_TIME_SEL` reader - No description avaiable"]
pub type CAN2_TIME_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CAN2_TIME_SEL` writer - No description avaiable"]
pub type CAN2_TIME_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIME_SEL_SPEC, bool, O>;
#[doc = "Field `CAN1_TIME_SEL` reader - No description avaiable"]
pub type CAN1_TIME_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CAN1_TIME_SEL` writer - No description avaiable"]
pub type CAN1_TIME_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIME_SEL_SPEC, bool, O>;
#[doc = "Field `CAN0_TIME_SEL` reader - set to use ptpc1 for canx clr to use ptpc0 for canx"]
pub type CAN0_TIME_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CAN0_TIME_SEL` writer - set to use ptpc1 for canx clr to use ptpc0 for canx"]
pub type CAN0_TIME_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIME_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - No description avaiable"]
    #[inline(always)]
    pub fn can3_time_sel(&self) -> CAN3_TIME_SEL_R {
        CAN3_TIME_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn can2_time_sel(&self) -> CAN2_TIME_SEL_R {
        CAN2_TIME_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn can1_time_sel(&self) -> CAN1_TIME_SEL_R {
        CAN1_TIME_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - set to use ptpc1 for canx clr to use ptpc0 for canx"]
    #[inline(always)]
    pub fn can0_time_sel(&self) -> CAN0_TIME_SEL_R {
        CAN0_TIME_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - No description avaiable"]
    #[inline(always)]
    pub fn can3_time_sel(&mut self) -> CAN3_TIME_SEL_W<3> {
        CAN3_TIME_SEL_W::new(self)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn can2_time_sel(&mut self) -> CAN2_TIME_SEL_W<2> {
        CAN2_TIME_SEL_W::new(self)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn can1_time_sel(&mut self) -> CAN1_TIME_SEL_W<1> {
        CAN1_TIME_SEL_W::new(self)
    }
    #[doc = "Bit 0 - set to use ptpc1 for canx clr to use ptpc0 for canx"]
    #[inline(always)]
    pub fn can0_time_sel(&mut self) -> CAN0_TIME_SEL_W<0> {
        CAN0_TIME_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_sel](index.html) module"]
pub struct TIME_SEL_SPEC;
impl crate::RegisterSpec for TIME_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time_sel::R](R) reader structure"]
impl crate::Readable for TIME_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [time_sel::W](W) writer structure"]
impl crate::Writable for TIME_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIME_SEL to value 0"]
impl crate::Resettable for TIME_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
