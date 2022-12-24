#[doc = "Register `RESTART` writer"]
pub struct W(crate::W<RESTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESTART_SPEC>;
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
impl From<crate::W<RESTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESTART` writer - Write the magic number ATCWDT200_RESTART_NUM to restart the watchdog timer."]
pub type RESTART_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESTART_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - Write the magic number ATCWDT200_RESTART_NUM to restart the watchdog timer."]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<0> {
        RESTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Restart Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [restart](index.html) module"]
pub struct RESTART_SPEC;
impl crate::RegisterSpec for RESTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [restart::W](W) writer structure"]
impl crate::Writable for RESTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESTART to value 0"]
impl crate::Resettable for RESTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
