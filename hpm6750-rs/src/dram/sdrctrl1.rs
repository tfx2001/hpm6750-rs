#[doc = "Register `SDRCTRL1` reader"]
pub struct R(crate::R<SDRCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRCTRL1` writer"]
pub struct W(crate::W<SDRCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRCTRL1_SPEC>;
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
impl From<crate::W<SDRCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT2PRE` reader - ACT to Precharge minimum time It is promised ACT2PRE+1 clock cycles delay between ACTIVE command to PRECHARGE/PRECHARGE_ALL command."]
pub type ACT2PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT2PRE` writer - ACT to Precharge minimum time It is promised ACT2PRE+1 clock cycles delay between ACTIVE command to PRECHARGE/PRECHARGE_ALL command."]
pub type ACT2PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CKEOFF` reader - CKE OFF minimum time It is promised clock suspend last at leat CKEOFF+1 clock cycles."]
pub type CKEOFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKEOFF` writer - CKE OFF minimum time It is promised clock suspend last at leat CKEOFF+1 clock cycles."]
pub type CKEOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `WRC` reader - Write recovery time It is promised WRC+1 clock cycles delay between WRITE command to PRECHARGE/PRECHARGE_ALL command. This could help to meet tWR timing requirement by SDRAM device."]
pub type WRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRC` writer - Write recovery time It is promised WRC+1 clock cycles delay between WRITE command to PRECHARGE/PRECHARGE_ALL command. This could help to meet tWR timing requirement by SDRAM device."]
pub type WRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `RFRC` reader - Refresh recovery time It is promised RFRC+1 clock cycles delay between REFRESH command to ACTIVE command. Thiscould help to meet tRFC timing requirement by SDRAM device."]
pub type RFRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFRC` writer - Refresh recovery time It is promised RFRC+1 clock cycles delay between REFRESH command to ACTIVE command. Thiscould help to meet tRFC timing requirement by SDRAM device."]
pub type RFRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL1_SPEC, u8, u8, 5, O>;
#[doc = "Field `ACT2RW` reader - ACT to Read/Write wait time It is promised ACT2RW+1 clock cycles delay between ACTIVE command to READ/WRITE command.This could help to meet tRCD timing requirement by SDRAM device."]
pub type ACT2RW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT2RW` writer - ACT to Read/Write wait time It is promised ACT2RW+1 clock cycles delay between ACTIVE command to READ/WRITE command.This could help to meet tRCD timing requirement by SDRAM device."]
pub type ACT2RW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRE2ACT` reader - PRECHARGE to ACT/Refresh wait time It is promised PRE2ACT+1 clock cycles delay between PRECHARGE/PRECHARGE_ALL commandto ACTIVE/REFRESH command. This could help to meet tRP timing requirement by SDRAM device."]
pub type PRE2ACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE2ACT` writer - PRECHARGE to ACT/Refresh wait time It is promised PRE2ACT+1 clock cycles delay between PRECHARGE/PRECHARGE_ALL commandto ACTIVE/REFRESH command. This could help to meet tRP timing requirement by SDRAM device."]
pub type PRE2ACT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 20:23 - ACT to Precharge minimum time It is promised ACT2PRE+1 clock cycles delay between ACTIVE command to PRECHARGE/PRECHARGE_ALL command."]
    #[inline(always)]
    pub fn act2pre(&self) -> ACT2PRE_R {
        ACT2PRE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CKE OFF minimum time It is promised clock suspend last at leat CKEOFF+1 clock cycles."]
    #[inline(always)]
    pub fn ckeoff(&self) -> CKEOFF_R {
        CKEOFF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Write recovery time It is promised WRC+1 clock cycles delay between WRITE command to PRECHARGE/PRECHARGE_ALL command. This could help to meet tWR timing requirement by SDRAM device."]
    #[inline(always)]
    pub fn wrc(&self) -> WRC_R {
        WRC_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 8:12 - Refresh recovery time It is promised RFRC+1 clock cycles delay between REFRESH command to ACTIVE command. Thiscould help to meet tRFC timing requirement by SDRAM device."]
    #[inline(always)]
    pub fn rfrc(&self) -> RFRC_R {
        RFRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 4:7 - ACT to Read/Write wait time It is promised ACT2RW+1 clock cycles delay between ACTIVE command to READ/WRITE command.This could help to meet tRCD timing requirement by SDRAM device."]
    #[inline(always)]
    pub fn act2rw(&self) -> ACT2RW_R {
        ACT2RW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - PRECHARGE to ACT/Refresh wait time It is promised PRE2ACT+1 clock cycles delay between PRECHARGE/PRECHARGE_ALL commandto ACTIVE/REFRESH command. This could help to meet tRP timing requirement by SDRAM device."]
    #[inline(always)]
    pub fn pre2act(&self) -> PRE2ACT_R {
        PRE2ACT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - ACT to Precharge minimum time It is promised ACT2PRE+1 clock cycles delay between ACTIVE command to PRECHARGE/PRECHARGE_ALL command."]
    #[inline(always)]
    pub fn act2pre(&mut self) -> ACT2PRE_W<20> {
        ACT2PRE_W::new(self)
    }
    #[doc = "Bits 16:19 - CKE OFF minimum time It is promised clock suspend last at leat CKEOFF+1 clock cycles."]
    #[inline(always)]
    pub fn ckeoff(&mut self) -> CKEOFF_W<16> {
        CKEOFF_W::new(self)
    }
    #[doc = "Bits 13:15 - Write recovery time It is promised WRC+1 clock cycles delay between WRITE command to PRECHARGE/PRECHARGE_ALL command. This could help to meet tWR timing requirement by SDRAM device."]
    #[inline(always)]
    pub fn wrc(&mut self) -> WRC_W<13> {
        WRC_W::new(self)
    }
    #[doc = "Bits 8:12 - Refresh recovery time It is promised RFRC+1 clock cycles delay between REFRESH command to ACTIVE command. Thiscould help to meet tRFC timing requirement by SDRAM device."]
    #[inline(always)]
    pub fn rfrc(&mut self) -> RFRC_W<8> {
        RFRC_W::new(self)
    }
    #[doc = "Bits 4:7 - ACT to Read/Write wait time It is promised ACT2RW+1 clock cycles delay between ACTIVE command to READ/WRITE command.This could help to meet tRCD timing requirement by SDRAM device."]
    #[inline(always)]
    pub fn act2rw(&mut self) -> ACT2RW_W<4> {
        ACT2RW_W::new(self)
    }
    #[doc = "Bits 0:3 - PRECHARGE to ACT/Refresh wait time It is promised PRE2ACT+1 clock cycles delay between PRECHARGE/PRECHARGE_ALL commandto ACTIVE/REFRESH command. This could help to meet tRP timing requirement by SDRAM device."]
    #[inline(always)]
    pub fn pre2act(&mut self) -> PRE2ACT_W<0> {
        PRE2ACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrctrl1](index.html) module"]
pub struct SDRCTRL1_SPEC;
impl crate::RegisterSpec for SDRCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdrctrl1::R](R) reader structure"]
impl crate::Readable for SDRCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdrctrl1::W](W) writer structure"]
impl crate::Writable for SDRCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDRCTRL1 to value 0"]
impl crate::Resettable for SDRCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
