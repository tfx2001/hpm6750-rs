#[doc = "Register `ESCALATE_CONFIG` reader"]
pub struct R(crate::R<ESCALATE_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESCALATE_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESCALATE_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESCALATE_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESCALATE_CONFIG` writer"]
pub struct W(crate::W<ESCALATE_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESCALATE_CONFIG_SPEC>;
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
impl From<crate::W<ESCALATE_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESCALATE_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_NSC` reader - Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
pub type LOCK_NSC_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_NSC` writer - Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
pub type LOCK_NSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESCALATE_CONFIG_SPEC, bool, O>;
#[doc = "Field `NSC_VIO_CFG` reader - configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
pub type NSC_VIO_CFG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NSC_VIO_CFG` writer - configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
pub type NSC_VIO_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ESCALATE_CONFIG_SPEC, u16, u16, 15, O>;
#[doc = "Field `LOCK_SEC` reader - Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified1: register locked, write access to the configuration is ignored"]
pub type LOCK_SEC_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_SEC` writer - Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified1: register locked, write access to the configuration is ignored"]
pub type LOCK_SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ESCALATE_CONFIG_SPEC, bool, O>;
#[doc = "Field `SEC_VIO_CFG` reader - configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
pub type SEC_VIO_CFG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SEC_VIO_CFG` writer - configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
pub type SEC_VIO_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ESCALATE_CONFIG_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bit 31 - Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
    #[inline(always)]
    pub fn lock_nsc(&self) -> LOCK_NSC_R {
        LOCK_NSC_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 16:30 - configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
    #[inline(always)]
    pub fn nsc_vio_cfg(&self) -> NSC_VIO_CFG_R {
        NSC_VIO_CFG_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified1: register locked, write access to the configuration is ignored"]
    #[inline(always)]
    pub fn lock_sec(&self) -> LOCK_SEC_R {
        LOCK_SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 0:14 - configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
    #[inline(always)]
    pub fn sec_vio_cfg(&self) -> SEC_VIO_CFG_R {
        SEC_VIO_CFG_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
    #[inline(always)]
    pub fn lock_nsc(&mut self) -> LOCK_NSC_W<31> {
        LOCK_NSC_W::new(self)
    }
    #[doc = "Bits 16:30 - configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
    #[inline(always)]
    pub fn nsc_vio_cfg(&mut self) -> NSC_VIO_CFG_W<16> {
        NSC_VIO_CFG_W::new(self)
    }
    #[doc = "Bit 15 - Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified1: register locked, write access to the configuration is ignored"]
    #[inline(always)]
    pub fn lock_sec(&mut self) -> LOCK_SEC_W<15> {
        LOCK_SEC_W::new(self)
    }
    #[doc = "Bits 0:14 - configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
    #[inline(always)]
    pub fn sec_vio_cfg(&mut self) -> SEC_VIO_CFG_W<0> {
        SEC_VIO_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Escalate behavior on security event\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [escalate_config](index.html) module"]
pub struct ESCALATE_CONFIG_SPEC;
impl crate::RegisterSpec for ESCALATE_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [escalate_config::R](R) reader structure"]
impl crate::Readable for ESCALATE_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [escalate_config::W](W) writer structure"]
impl crate::Writable for ESCALATE_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESCALATE_CONFIG to value 0"]
impl crate::Resettable for ESCALATE_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
