#[doc = "Register `CMD_ARG` reader"]
pub struct R(crate::R<CMD_ARG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_ARG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_ARG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_ARG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD_ARG` writer"]
pub struct W(crate::W<CMD_ARG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_ARG_SPEC>;
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
impl From<crate::W<CMD_ARG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_ARG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARGUMNET` reader - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
pub type ARGUMNET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ARGUMNET` writer - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
pub type ARGUMNET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_ARG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
    #[inline(always)]
    pub fn argumnet(&self) -> ARGUMNET_R {
        ARGUMNET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
    #[inline(always)]
    #[must_use]
    pub fn argumnet(&mut self) -> ARGUMNET_W<0> {
        ARGUMNET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_arg](index.html) module"]
pub struct CMD_ARG_SPEC;
impl crate::RegisterSpec for CMD_ARG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_arg::R](R) reader structure"]
impl crate::Readable for CMD_ARG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd_arg::W](W) writer structure"]
impl crate::Writable for CMD_ARG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD_ARG to value 0"]
impl crate::Resettable for CMD_ARG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
