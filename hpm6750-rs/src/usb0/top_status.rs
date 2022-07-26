#[doc = "Register `TOP_STATUS` reader"]
pub struct R(crate::R<TOP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOP_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOP_STATUS` writer"]
pub struct W(crate::W<TOP_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOP_STATUS_SPEC>;
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
impl From<crate::W<TOP_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOP_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUP_INT_STATUS` reader - No description avaiable"]
pub type WAKEUP_INT_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP_INT_STATUS` writer - No description avaiable"]
pub type WAKEUP_INT_STATUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOP_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - No description avaiable"]
    #[inline(always)]
    pub fn wakeup_int_status(&self) -> WAKEUP_INT_STATUS_R {
        WAKEUP_INT_STATUS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - No description avaiable"]
    #[inline(always)]
    pub fn wakeup_int_status(&mut self) -> WAKEUP_INT_STATUS_W<31> {
        WAKEUP_INT_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [top_status](index.html) module"]
pub struct TOP_STATUS_SPEC;
impl crate::RegisterSpec for TOP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [top_status::R](R) reader structure"]
impl crate::Readable for TOP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [top_status::W](W) writer structure"]
impl crate::Writable for TOP_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOP_STATUS to value 0"]
impl crate::Resettable for TOP_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
