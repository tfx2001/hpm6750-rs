#[doc = "Register `SECURE_STATE_CONFIG` reader"]
pub struct R(crate::R<SECURE_STATE_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECURE_STATE_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECURE_STATE_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECURE_STATE_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECURE_STATE_CONFIG` writer"]
pub struct W(crate::W<SECURE_STATE_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECURE_STATE_CONFIG_SPEC>;
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
impl From<crate::W<SECURE_STATE_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECURE_STATE_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALLOW_RESTART` reader - allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state"]
pub type ALLOW_RESTART_R = crate::BitReader<bool>;
#[doc = "Field `ALLOW_RESTART` writer - allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state"]
pub type ALLOW_RESTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SECURE_STATE_CONFIG_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECURE_STATE_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state"]
    #[inline(always)]
    pub fn allow_restart(&self) -> ALLOW_RESTART_R {
        ALLOW_RESTART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state"]
    #[inline(always)]
    #[must_use]
    pub fn allow_restart(&mut self) -> ALLOW_RESTART_W<0> {
        ALLOW_RESTART_W::new(self)
    }
    #[doc = "Bit 3 - Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<3> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "secure state configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secure_state_config](index.html) module"]
pub struct SECURE_STATE_CONFIG_SPEC;
impl crate::RegisterSpec for SECURE_STATE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secure_state_config::R](R) reader structure"]
impl crate::Readable for SECURE_STATE_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secure_state_config::W](W) writer structure"]
impl crate::Writable for SECURE_STATE_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECURE_STATE_CONFIG to value 0"]
impl crate::Resettable for SECURE_STATE_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
