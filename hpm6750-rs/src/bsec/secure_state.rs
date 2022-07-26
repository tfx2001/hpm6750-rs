#[doc = "Register `SECURE_STATE` reader"]
pub struct R(crate::R<SECURE_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECURE_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECURE_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECURE_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECURE_STATE` writer"]
pub struct W(crate::W<SECURE_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECURE_STATE_SPEC>;
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
impl From<crate::W<SECURE_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECURE_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALLOW_NSC` reader - Non-secure state allow 0: system is not healthy to enter non-secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter non-secure state"]
pub type ALLOW_NSC_R = crate::BitReader<bool>;
#[doc = "Field `ALLOW_SEC` reader - Secure state allow 0: system is not healthy to enter secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter secure state"]
pub type ALLOW_SEC_R = crate::BitReader<bool>;
#[doc = "Field `BATT_FAIL` reader - BATT secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state"]
pub type BATT_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `BATT_FAIL` writer - BATT secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state"]
pub type BATT_FAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECURE_STATE_SPEC, bool, O>;
#[doc = "Field `BATT_NSC` reader - BATT secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state"]
pub type BATT_NSC_R = crate::BitReader<bool>;
#[doc = "Field `BATT_NSC` writer - BATT secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state"]
pub type BATT_NSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECURE_STATE_SPEC, bool, O>;
#[doc = "Field `BATT_SEC` reader - BATT secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state"]
pub type BATT_SEC_R = crate::BitReader<bool>;
#[doc = "Field `BATT_SEC` writer - BATT secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state"]
pub type BATT_SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECURE_STATE_SPEC, bool, O>;
#[doc = "Field `BATT_INS` reader - BATT secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state"]
pub type BATT_INS_R = crate::BitReader<bool>;
#[doc = "Field `BATT_INS` writer - BATT secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state"]
pub type BATT_INS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECURE_STATE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 17 - Non-secure state allow 0: system is not healthy to enter non-secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter non-secure state"]
    #[inline(always)]
    pub fn allow_nsc(&self) -> ALLOW_NSC_R {
        ALLOW_NSC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Secure state allow 0: system is not healthy to enter secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter secure state"]
    #[inline(always)]
    pub fn allow_sec(&self) -> ALLOW_SEC_R {
        ALLOW_SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 3 - BATT secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state"]
    #[inline(always)]
    pub fn batt_fail(&self) -> BATT_FAIL_R {
        BATT_FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - BATT secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state"]
    #[inline(always)]
    pub fn batt_nsc(&self) -> BATT_NSC_R {
        BATT_NSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - BATT secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state"]
    #[inline(always)]
    pub fn batt_sec(&self) -> BATT_SEC_R {
        BATT_SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - BATT secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state"]
    #[inline(always)]
    pub fn batt_ins(&self) -> BATT_INS_R {
        BATT_INS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - BATT secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state"]
    #[inline(always)]
    pub fn batt_fail(&mut self) -> BATT_FAIL_W<3> {
        BATT_FAIL_W::new(self)
    }
    #[doc = "Bit 2 - BATT secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state"]
    #[inline(always)]
    pub fn batt_nsc(&mut self) -> BATT_NSC_W<2> {
        BATT_NSC_W::new(self)
    }
    #[doc = "Bit 1 - BATT secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state"]
    #[inline(always)]
    pub fn batt_sec(&mut self) -> BATT_SEC_W<1> {
        BATT_SEC_W::new(self)
    }
    #[doc = "Bit 0 - BATT secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state"]
    #[inline(always)]
    pub fn batt_ins(&mut self) -> BATT_INS_W<0> {
        BATT_INS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure state\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secure_state](index.html) module"]
pub struct SECURE_STATE_SPEC;
impl crate::RegisterSpec for SECURE_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secure_state::R](R) reader structure"]
impl crate::Readable for SECURE_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secure_state::W](W) writer structure"]
impl crate::Writable for SECURE_STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECURE_STATE to value 0"]
impl crate::Resettable for SECURE_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
