#[doc = "Register `CQDQS` reader"]
pub struct R(crate::R<CQDQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQDQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQDQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQDQS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQDQS` writer"]
pub struct W(crate::W<CQDQS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQDQS_SPEC>;
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
impl From<crate::W<CQDQS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQDQS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DQS` reader - Device Queue Status Each of the 32 bits are bit mapped to the 32 tasks. Bit-N(1): Device has marked task N as ready for execution Bit-N(0): Task-N is not ready for execution. This task could be pending in device or not submitted. Host controller updates this register with response of the Device Queue Status command."]
pub type DQS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DQS` writer - Device Queue Status Each of the 32 bits are bit mapped to the 32 tasks. Bit-N(1): Device has marked task N as ready for execution Bit-N(0): Task-N is not ready for execution. This task could be pending in device or not submitted. Host controller updates this register with response of the Device Queue Status command."]
pub type DQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CQDQS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Device Queue Status Each of the 32 bits are bit mapped to the 32 tasks. Bit-N(1): Device has marked task N as ready for execution Bit-N(0): Task-N is not ready for execution. This task could be pending in device or not submitted. Host controller updates this register with response of the Device Queue Status command."]
    #[inline(always)]
    pub fn dqs(&self) -> DQS_R {
        DQS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device Queue Status Each of the 32 bits are bit mapped to the 32 tasks. Bit-N(1): Device has marked task N as ready for execution Bit-N(0): Task-N is not ready for execution. This task could be pending in device or not submitted. Host controller updates this register with response of the Device Queue Status command."]
    #[inline(always)]
    pub fn dqs(&mut self) -> DQS_W<0> {
        DQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqdqs](index.html) module"]
pub struct CQDQS_SPEC;
impl crate::RegisterSpec for CQDQS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqdqs::R](R) reader structure"]
impl crate::Readable for CQDQS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqdqs::W](W) writer structure"]
impl crate::Writable for CQDQS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQDQS to value 0"]
impl crate::Resettable for CQDQS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
