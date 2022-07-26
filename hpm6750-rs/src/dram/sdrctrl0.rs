#[doc = "Register `SDRCTRL0` reader"]
pub struct R(crate::R<SDRCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRCTRL0` writer"]
pub struct W(crate::W<SDRCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRCTRL0_SPEC>;
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
impl From<crate::W<SDRCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BANK2` reader - 2 Bank selection bit 0b - SDRAM device has 4 banks. 1b - SDRAM device has 2 banks."]
pub type BANK2_R = crate::BitReader<bool>;
#[doc = "Field `BANK2` writer - 2 Bank selection bit 0b - SDRAM device has 4 banks. 1b - SDRAM device has 2 banks."]
pub type BANK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRCTRL0_SPEC, bool, O>;
#[doc = "Field `CAS` reader - CAS Latency 00b - 1 01b - 1 10b - 2 11b - 3"]
pub type CAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAS` writer - CAS Latency 00b - 1 01b - 1 10b - 2 11b - 3"]
pub type CAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `COL` reader - Column address bit number 00b - 12 bit 01b - 11 bit 10b - 10 bit 11b - 9 bit"]
pub type COL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COL` writer - Column address bit number 00b - 12 bit 01b - 11 bit 10b - 10 bit 11b - 9 bit"]
pub type COL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `COL8` reader - Column 8 selection bit 0b - Column address bit number is decided by COL field. 1b - Column address bit number is 8. COL field is ignored."]
pub type COL8_R = crate::BitReader<bool>;
#[doc = "Field `COL8` writer - Column 8 selection bit 0b - Column address bit number is decided by COL field. 1b - Column address bit number is 8. COL field is ignored."]
pub type COL8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRCTRL0_SPEC, bool, O>;
#[doc = "Field `BURSTLEN` reader - Burst Length 000b - 1 001b - 2 010b - 4 011b - 8 100b - 8 101b - 8 110b - 8 111b - 8"]
pub type BURSTLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BURSTLEN` writer - Burst Length 000b - 1 001b - 2 010b - 4 011b - 8 100b - 8 101b - 8 110b - 8 111b - 8"]
pub type BURSTLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL0_SPEC, u8, u8, 3, O>;
#[doc = "Field `HIGHBAND` reader - high band select 0: use data\\[15:0\\]
for 16bit SDRAM; 1: use data\\[31:16\\]
for 16bit SDRAM; only used when Port Size is 16bit(PORTSZ=01b)"]
pub type HIGHBAND_R = crate::BitReader<bool>;
#[doc = "Field `HIGHBAND` writer - high band select 0: use data\\[15:0\\]
for 16bit SDRAM; 1: use data\\[31:16\\]
for 16bit SDRAM; only used when Port Size is 16bit(PORTSZ=01b)"]
pub type HIGHBAND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRCTRL0_SPEC, bool, O>;
#[doc = "Field `PORTSZ` reader - Port Size 00b - 8bit 01b - 16bit 10b - 32bit"]
pub type PORTSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORTSZ` writer - Port Size 00b - 8bit 01b - 16bit 10b - 32bit"]
pub type PORTSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDRCTRL0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 14 - 2 Bank selection bit 0b - SDRAM device has 4 banks. 1b - SDRAM device has 2 banks."]
    #[inline(always)]
    pub fn bank2(&self) -> BANK2_R {
        BANK2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 10:11 - CAS Latency 00b - 1 01b - 1 10b - 2 11b - 3"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Column address bit number 00b - 12 bit 01b - 11 bit 10b - 10 bit 11b - 9 bit"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - Column 8 selection bit 0b - Column address bit number is decided by COL field. 1b - Column address bit number is 8. COL field is ignored."]
    #[inline(always)]
    pub fn col8(&self) -> COL8_R {
        COL8_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Burst Length 000b - 1 001b - 2 010b - 4 011b - 8 100b - 8 101b - 8 110b - 8 111b - 8"]
    #[inline(always)]
    pub fn burstlen(&self) -> BURSTLEN_R {
        BURSTLEN_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - high band select 0: use data\\[15:0\\]
for 16bit SDRAM; 1: use data\\[31:16\\]
for 16bit SDRAM; only used when Port Size is 16bit(PORTSZ=01b)"]
    #[inline(always)]
    pub fn highband(&self) -> HIGHBAND_R {
        HIGHBAND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Port Size 00b - 8bit 01b - 16bit 10b - 32bit"]
    #[inline(always)]
    pub fn portsz(&self) -> PORTSZ_R {
        PORTSZ_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - 2 Bank selection bit 0b - SDRAM device has 4 banks. 1b - SDRAM device has 2 banks."]
    #[inline(always)]
    pub fn bank2(&mut self) -> BANK2_W<14> {
        BANK2_W::new(self)
    }
    #[doc = "Bits 10:11 - CAS Latency 00b - 1 01b - 1 10b - 2 11b - 3"]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W<10> {
        CAS_W::new(self)
    }
    #[doc = "Bits 8:9 - Column address bit number 00b - 12 bit 01b - 11 bit 10b - 10 bit 11b - 9 bit"]
    #[inline(always)]
    pub fn col(&mut self) -> COL_W<8> {
        COL_W::new(self)
    }
    #[doc = "Bit 7 - Column 8 selection bit 0b - Column address bit number is decided by COL field. 1b - Column address bit number is 8. COL field is ignored."]
    #[inline(always)]
    pub fn col8(&mut self) -> COL8_W<7> {
        COL8_W::new(self)
    }
    #[doc = "Bits 4:6 - Burst Length 000b - 1 001b - 2 010b - 4 011b - 8 100b - 8 101b - 8 110b - 8 111b - 8"]
    #[inline(always)]
    pub fn burstlen(&mut self) -> BURSTLEN_W<4> {
        BURSTLEN_W::new(self)
    }
    #[doc = "Bit 3 - high band select 0: use data\\[15:0\\]
for 16bit SDRAM; 1: use data\\[31:16\\]
for 16bit SDRAM; only used when Port Size is 16bit(PORTSZ=01b)"]
    #[inline(always)]
    pub fn highband(&mut self) -> HIGHBAND_W<3> {
        HIGHBAND_W::new(self)
    }
    #[doc = "Bits 0:1 - Port Size 00b - 8bit 01b - 16bit 10b - 32bit"]
    #[inline(always)]
    pub fn portsz(&mut self) -> PORTSZ_W<0> {
        PORTSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdrctrl0](index.html) module"]
pub struct SDRCTRL0_SPEC;
impl crate::RegisterSpec for SDRCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdrctrl0::R](R) reader structure"]
impl crate::Readable for SDRCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdrctrl0::W](W) writer structure"]
impl crate::Writable for SDRCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDRCTRL0 to value 0"]
impl crate::Resettable for SDRCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
