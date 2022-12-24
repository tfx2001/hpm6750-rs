#[doc = "Register `IPCMD` reader"]
pub struct R(crate::R<IPCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCMD` writer"]
pub struct W(crate::W<IPCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCMD_SPEC>;
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
impl From<crate::W<IPCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD` reader - SDRAM Commands: • 0x8: READ • 0x9: WRITE • 0xA: MODESET • 0xB: ACTIVE • 0xC: AUTO REFRESH • 0xD: SELF REFRESH • 0xE: PRECHARGE • 0xF: PRECHARGE ALL • Others: RSVD NOTE: SELF REFRESH is sent to all SDRAM devices because they shared same CLK pin."]
pub type CMD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMD` writer - SDRAM Commands: • 0x8: READ • 0x9: WRITE • 0xA: MODESET • 0xB: ACTIVE • 0xC: AUTO REFRESH • 0xD: SELF REFRESH • 0xE: PRECHARGE • 0xF: PRECHARGE ALL • Others: RSVD NOTE: SELF REFRESH is sent to all SDRAM devices because they shared same CLK pin."]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPCMD_SPEC, u16, u16, 16, O>;
#[doc = "Field `KEY` writer - This field should be written with 0x5AA5 when trigging an IP command for all device types. The memory device is selected by BRx settings and IPCR0 registers."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPCMD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - SDRAM Commands: • 0x8: READ • 0x9: WRITE • 0xA: MODESET • 0xB: ACTIVE • 0xC: AUTO REFRESH • 0xD: SELF REFRESH • 0xE: PRECHARGE • 0xF: PRECHARGE ALL • Others: RSVD NOTE: SELF REFRESH is sent to all SDRAM devices because they shared same CLK pin."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SDRAM Commands: • 0x8: READ • 0x9: WRITE • 0xA: MODESET • 0xB: ACTIVE • 0xC: AUTO REFRESH • 0xD: SELF REFRESH • 0xE: PRECHARGE • 0xF: PRECHARGE ALL • Others: RSVD NOTE: SELF REFRESH is sent to all SDRAM devices because they shared same CLK pin."]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Bits 16:31 - This field should be written with 0x5AA5 when trigging an IP command for all device types. The memory device is selected by BRx settings and IPCR0 registers."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<16> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcmd](index.html) module"]
pub struct IPCMD_SPEC;
impl crate::RegisterSpec for IPCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcmd::R](R) reader structure"]
impl crate::Readable for IPCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipcmd::W](W) writer structure"]
impl crate::Writable for IPCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPCMD to value 0"]
impl crate::Resettable for IPCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
