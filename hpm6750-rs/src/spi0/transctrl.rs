#[doc = "Register `TRANSCTRL` reader"]
pub struct R(crate::R<TRANSCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSCTRL` writer"]
pub struct W(crate::W<TRANSCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSCTRL_SPEC>;
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
impl From<crate::W<TRANSCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDTRANCNT` reader - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt."]
pub type RDTRANCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RDTRANCNT` writer - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt."]
pub type RDTRANCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRANSCTRL_SPEC, u16, u16, 9, O>;
#[doc = "Field `DUMMYCNT` reader - Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases."]
pub type DUMMYCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUMMYCNT` writer - Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases."]
pub type DUMMYCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRANSCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TOKENVALUE` reader - Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69"]
pub type TOKENVALUE_R = crate::BitReader<bool>;
#[doc = "Field `TOKENVALUE` writer - Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69"]
pub type TOKENVALUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSCTRL_SPEC, bool, O>;
#[doc = "Field `WRTRANCNT` reader - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt."]
pub type WRTRANCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WRTRANCNT` writer - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt."]
pub type WRTRANCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRANSCTRL_SPEC, u16, u16, 9, O>;
#[doc = "Field `TOKENEN` reader - Token transfer enable (Master mode only) Append an one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token"]
pub type TOKENEN_R = crate::BitReader<bool>;
#[doc = "Field `TOKENEN` writer - Token transfer enable (Master mode only) Append an one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token"]
pub type TOKENEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSCTRL_SPEC, bool, O>;
#[doc = "Field `DUALQUAD` reader - SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved"]
pub type DUALQUAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUALQUAD` writer - SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved"]
pub type DUALQUAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRANSCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRANSMODE` reader - Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved"]
pub type TRANSMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRANSMODE` writer - Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved"]
pub type TRANSMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRANSCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRFMT` reader - SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad)."]
pub type ADDRFMT_R = crate::BitReader<bool>;
#[doc = "Field `ADDRFMT` writer - SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad)."]
pub type ADDRFMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSCTRL_SPEC, bool, O>;
#[doc = "Field `ADDREN` reader - SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase"]
pub type ADDREN_R = crate::BitReader<bool>;
#[doc = "Field `ADDREN` writer - SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase"]
pub type ADDREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSCTRL_SPEC, bool, O>;
#[doc = "Field `CMDEN` reader - SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase"]
pub type CMDEN_R = crate::BitReader<bool>;
#[doc = "Field `CMDEN` writer - SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase"]
pub type CMDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSCTRL_SPEC, bool, O>;
#[doc = "Field `SLVDATAONLY` reader - Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0."]
pub type SLVDATAONLY_R = crate::BitReader<bool>;
#[doc = "Field `SLVDATAONLY` writer - Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0."]
pub type SLVDATAONLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt."]
    #[inline(always)]
    pub fn rdtrancnt(&self) -> RDTRANCNT_R {
        RDTRANCNT_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:10 - Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases."]
    #[inline(always)]
    pub fn dummycnt(&self) -> DUMMYCNT_R {
        DUMMYCNT_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69"]
    #[inline(always)]
    pub fn tokenvalue(&self) -> TOKENVALUE_R {
        TOKENVALUE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:20 - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt."]
    #[inline(always)]
    pub fn wrtrancnt(&self) -> WRTRANCNT_R {
        WRTRANCNT_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bit 21 - Token transfer enable (Master mode only) Append an one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token"]
    #[inline(always)]
    pub fn tokenen(&self) -> TOKENEN_R {
        TOKENEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved"]
    #[inline(always)]
    pub fn dualquad(&self) -> DUALQUAD_R {
        DUALQUAD_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved"]
    #[inline(always)]
    pub fn transmode(&self) -> TRANSMODE_R {
        TRANSMODE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad)."]
    #[inline(always)]
    pub fn addrfmt(&self) -> ADDRFMT_R {
        ADDRFMT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase"]
    #[inline(always)]
    pub fn addren(&self) -> ADDREN_R {
        ADDREN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase"]
    #[inline(always)]
    pub fn cmden(&self) -> CMDEN_R {
        CMDEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0."]
    #[inline(always)]
    pub fn slvdataonly(&self) -> SLVDATAONLY_R {
        SLVDATAONLY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Transfer count for read data RdTranCnt indicates the number of units of data to be received from SPI bus and stored to the Data Register. The actual received count is (RdTranCnt+1). RdTransCnt only takes effect when TransMode is 0, 2, 3, 4, 5, 6 or 9. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must equal RdTranCnt."]
    #[inline(always)]
    #[must_use]
    pub fn rdtrancnt(&mut self) -> RDTRANCNT_W<0> {
        RDTRANCNT_W::new(self)
    }
    #[doc = "Bits 9:10 - Dummy data count. The actual dummy count is (DummyCnt +1). The number of dummy cycles on the SPI interface will be (DummyCnt+1)* ((DataLen+1)/SPI IO width) The Data pins are put into the high impedance during the dummy data phase. DummyCnt is only used for TransMode 5, 6, 8 and 9, which has dummy data phases."]
    #[inline(always)]
    #[must_use]
    pub fn dummycnt(&mut self) -> DUMMYCNT_W<9> {
        DUMMYCNT_W::new(self)
    }
    #[doc = "Bit 11 - Token value (Master mode only) The value of the one-byte special token following the address phase for SPI read transfers. 0x0: token value = 0x00 0x1: token value = 0x69"]
    #[inline(always)]
    #[must_use]
    pub fn tokenvalue(&mut self) -> TOKENVALUE_W<11> {
        TOKENVALUE_W::new(self)
    }
    #[doc = "Bits 12:20 - Transfer count for write data WrTranCnt indicates the number of units of data to be transmitted to the SPI bus from the Data Register. The actual transfer count is (WrTranCnt+1). WrTranCnt only takes effect when TransMode is 0, 1, 3, 4, 5, 6 or 8. The size (bit-width) of a data unit is defined by the DataLen field of the Transfer Format Register. For TransMode 0, WrTranCnt must be equal to RdTranCnt."]
    #[inline(always)]
    #[must_use]
    pub fn wrtrancnt(&mut self) -> WRTRANCNT_W<12> {
        WRTRANCNT_W::new(self)
    }
    #[doc = "Bit 21 - Token transfer enable (Master mode only) Append an one-byte special token following the address phase for SPI read transfers. The value of the special token should be selected in TokenValue. 0x0: Disable the one-byte special token 0x1: Enable the one-byte special token"]
    #[inline(always)]
    #[must_use]
    pub fn tokenen(&mut self) -> TOKENEN_W<21> {
        TOKENEN_W::new(self)
    }
    #[doc = "Bits 22:23 - SPI data phase format 0x0: Regular (Single) mode 0x1: Dual I/O mode 0x2: Quad I/O mode 0x3: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn dualquad(&mut self) -> DUALQUAD_W<22> {
        DUALQUAD_W::new(self)
    }
    #[doc = "Bits 24:27 - Transfer mode The transfer sequence could be 0x0: Write and read at the same time 0x1: Write only 0x2: Read only 0x3: Write, Read 0x4: Read, Write 0x5: Write, Dummy, Read 0x6: Read, Dummy, Write 0x7: None Data (must enable CmdEn or AddrEn in master mode) 0x8: Dummy, Write 0x9: Dummy, Read 0xa~0xf: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn transmode(&mut self) -> TRANSMODE_W<24> {
        TRANSMODE_W::new(self)
    }
    #[doc = "Bit 28 - SPI address phase format (Master mode only) 0x0: Address phase is the regular (single) mode 0x1: The format of the address phase is the same as the data phase (DualQuad)."]
    #[inline(always)]
    #[must_use]
    pub fn addrfmt(&mut self) -> ADDRFMT_W<28> {
        ADDRFMT_W::new(self)
    }
    #[doc = "Bit 29 - SPI address phase enable (Master mode only) 0x0: Disable the address phase 0x1: Enable the address phase"]
    #[inline(always)]
    #[must_use]
    pub fn addren(&mut self) -> ADDREN_W<29> {
        ADDREN_W::new(self)
    }
    #[doc = "Bit 30 - SPI command phase enable (Master mode only) 0x0: Disable the command phase 0x1: Enable the command phase"]
    #[inline(always)]
    #[must_use]
    pub fn cmden(&mut self) -> CMDEN_W<30> {
        CMDEN_W::new(self)
    }
    #[doc = "Bit 31 - Data-only mode (slave mode only) 0x0: Disable the data-only mode 0x1: Enable the data-only mode Note: This mode only works in the uni-directional regular (single) mode so MOSIBiDir, DualQuad and TransMode should be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn slvdataonly(&mut self) -> SLVDATAONLY_W<31> {
        SLVDATAONLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transctrl](index.html) module"]
pub struct TRANSCTRL_SPEC;
impl crate::RegisterSpec for TRANSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transctrl::R](R) reader structure"]
impl crate::Readable for TRANSCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transctrl::W](W) writer structure"]
impl crate::Writable for TRANSCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRANSCTRL to value 0"]
impl crate::Resettable for TRANSCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
