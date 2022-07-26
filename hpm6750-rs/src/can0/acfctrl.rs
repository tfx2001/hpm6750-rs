#[doc = "Register `ACFCTRL` reader"]
pub struct R(crate::R<ACFCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACFCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACFCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACFCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACFCTRL` writer"]
pub struct W(crate::W<ACFCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACFCTRL_SPEC>;
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
impl From<crate::W<ACFCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACFCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELMASK` reader - SELect acceptance MASK 0 - Registers ACF_x point to acceptance code 1 - Registers ACF_x point to acceptance mask. ACFADR selects one specific acceptance filter."]
pub type SELMASK_R = crate::BitReader<bool>;
#[doc = "Field `SELMASK` writer - SELect acceptance MASK 0 - Registers ACF_x point to acceptance code 1 - Registers ACF_x point to acceptance mask. ACFADR selects one specific acceptance filter."]
pub type SELMASK_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACFCTRL_SPEC, bool, O>;
#[doc = "Field `ACFADR` reader - acceptance filter address ACFADR points to a specific acceptance filter. The selected filter is accessible using theregisters ACF_x. Bit SELMASK selects between acceptance code and mask for theselected acceptance filter. A value of ACFADR>ACF_NUMBER-1 is meaningless and automatically treated as value ACF_NUMBER-1. ACF_NUMBER = 16."]
pub type ACFADR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACFADR` writer - acceptance filter address ACFADR points to a specific acceptance filter. The selected filter is accessible using theregisters ACF_x. Bit SELMASK selects between acceptance code and mask for theselected acceptance filter. A value of ACFADR>ACF_NUMBER-1 is meaningless and automatically treated as value ACF_NUMBER-1. ACF_NUMBER = 16."]
pub type ACFADR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, ACFCTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 5 - SELect acceptance MASK 0 - Registers ACF_x point to acceptance code 1 - Registers ACF_x point to acceptance mask. ACFADR selects one specific acceptance filter."]
    #[inline(always)]
    pub fn selmask(&self) -> SELMASK_R {
        SELMASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 0:3 - acceptance filter address ACFADR points to a specific acceptance filter. The selected filter is accessible using theregisters ACF_x. Bit SELMASK selects between acceptance code and mask for theselected acceptance filter. A value of ACFADR>ACF_NUMBER-1 is meaningless and automatically treated as value ACF_NUMBER-1. ACF_NUMBER = 16."]
    #[inline(always)]
    pub fn acfadr(&self) -> ACFADR_R {
        ACFADR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - SELect acceptance MASK 0 - Registers ACF_x point to acceptance code 1 - Registers ACF_x point to acceptance mask. ACFADR selects one specific acceptance filter."]
    #[inline(always)]
    pub fn selmask(&mut self) -> SELMASK_W<5> {
        SELMASK_W::new(self)
    }
    #[doc = "Bits 0:3 - acceptance filter address ACFADR points to a specific acceptance filter. The selected filter is accessible using theregisters ACF_x. Bit SELMASK selects between acceptance code and mask for theselected acceptance filter. A value of ACFADR>ACF_NUMBER-1 is meaningless and automatically treated as value ACF_NUMBER-1. ACF_NUMBER = 16."]
    #[inline(always)]
    pub fn acfadr(&mut self) -> ACFADR_W<0> {
        ACFADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Acceptance Filter Control Register ACFCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acfctrl](index.html) module"]
pub struct ACFCTRL_SPEC;
impl crate::RegisterSpec for ACFCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [acfctrl::R](R) reader structure"]
impl crate::Readable for ACFCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acfctrl::W](W) writer structure"]
impl crate::Writable for ACFCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACFCTRL to value 0"]
impl crate::Resettable for ACFCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
