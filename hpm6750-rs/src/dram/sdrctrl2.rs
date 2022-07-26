#[doc = "Register `SDRCTRL2` reader"]
pub struct R(crate::R<SDRCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRCTRL2` writer"]
pub struct W(crate::W<SDRCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRCTRL2_SPEC>;
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
impl From<crate::W<SDRCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITO` reader - SDRAM Idle timeout It closes all opened pages if the SDRAM idle time lasts more than idle timeout period. SDRAM is considered idle when there is no AXI Bus transfer and no SDRAM command pending. 00000000b - IDLE timeout period is 256*Prescale period. 00000001-11111111b - IDLE timeout period is ITO*Prescale period."]
pub type ITO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ITO` writer - SDRAM Idle timeout It closes all opened pages if the SDRAM idle time lasts more than idle timeout period. SDRAM is considered idle when there is no AXI Bus transfer and no SDRAM command pending. 00000000b - IDLE timeout period is 256*Prescale period. 00000001-11111111b - IDLE timeout period is ITO*Prescale period."]
pub type ITO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `ACT2ACT` reader - ACT to ACT wait time It is promised ACT2ACT+1 clock cycles delay between ACTIVE command to ACTIVE command. This could help to meet tRRD timing requirement by SDRAM device."]
pub type ACT2ACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT2ACT` writer - ACT to ACT wait time It is promised ACT2ACT+1 clock cycles delay between ACTIVE command to ACTIVE command. This could help to meet tRRD timing requirement by SDRAM device."]
pub type ACT2ACT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `REF2REF` reader - Refresh to Refresh wait time It is promised REF2REF+1 clock cycles delay between REFRESH command to REFRESH command. This could help to meet tRFC timing requirement by SDRAM device."]
pub type REF2REF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REF2REF` writer - Refresh to Refresh wait time It is promised REF2REF+1 clock cycles delay between REFRESH command to REFRESH command. This could help to meet tRFC timing requirement by SDRAM device."]
pub type REF2REF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `SRRC` reader - Self Refresh Recovery time It is promised SRRC+1 clock cycles delay between Self-REFRESH command to any command."]
pub type SRRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRRC` writer - Self Refresh Recovery time It is promised SRRC+1 clock cycles delay between Self-REFRESH command to any command."]
pub type SRRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 24:31 - SDRAM Idle timeout It closes all opened pages if the SDRAM idle time lasts more than idle timeout period. SDRAM is considered idle when there is no AXI Bus transfer and no SDRAM command pending. 00000000b - IDLE timeout period is 256*Prescale period. 00000001-11111111b - IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito(&self) -> ITO_R {
        ITO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ACT to ACT wait time It is promised ACT2ACT+1 clock cycles delay between ACTIVE command to ACTIVE command. This could help to meet tRRD timing requirement by SDRAM device."]
    #[inline(always)]
    pub fn act2act(&self) -> ACT2ACT_R {
        ACT2ACT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Refresh to Refresh wait time It is promised REF2REF+1 clock cycles delay between REFRESH command to REFRESH command. This could help to meet tRFC timing requirement by SDRAM device."]
    #[inline(always)]
    pub fn ref2ref(&self) -> REF2REF_R {
        REF2REF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Self Refresh Recovery time It is promised SRRC+1 clock cycles delay between Self-REFRESH command to any command."]
    #[inline(always)]
    pub fn srrc(&self) -> SRRC_R {
        SRRC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - SDRAM Idle timeout It closes all opened pages if the SDRAM idle time lasts more than idle timeout period. SDRAM is considered idle when there is no AXI Bus transfer and no SDRAM command pending. 00000000b - IDLE timeout period is 256*Prescale period. 00000001-11111111b - IDLE timeout period is ITO*Prescale period."]
    #[inline(always)]
    pub fn ito(&mut self) -> ITO_W<24> {
        ITO_W::new(self)
    }
    #[doc = "Bits 16:23 - ACT to ACT wait time It is promised ACT2ACT+1 clock cycles delay between ACTIVE command to ACTIVE command. This could help to meet tRRD timing requirement by SDRAM device."]
    #[inline(always)]
    pub fn act2act(&mut self) -> ACT2ACT_W<16> {
        ACT2ACT_W::new(self)
    }
    #[doc = "Bits 8:15 - Refresh to Refresh wait time It is promised REF2REF+1 clock cycles delay between REFRESH command to REFRESH command. This could help to meet tRFC timing requirement by SDRAM device."]
    #[inline(always)]
    pub fn ref2ref(&mut self) -> REF2REF_W<8> {
        REF2REF_W::new(self)
    }
    #[doc = "Bits 0:7 - Self Refresh Recovery time It is promised SRRC+1 clock cycles delay between Self-REFRESH command to any command."]
    #[inline(always)]
    pub fn srrc(&mut self) -> SRRC_W<0> {
        SRRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrctrl2](index.html) module"]
pub struct SDRCTRL2_SPEC;
impl crate::RegisterSpec for SDRCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdrctrl2::R](R) reader structure"]
impl crate::Readable for SDRCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdrctrl2::W](W) writer structure"]
impl crate::Writable for SDRCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDRCTRL2 to value 0"]
impl crate::Resettable for SDRCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
