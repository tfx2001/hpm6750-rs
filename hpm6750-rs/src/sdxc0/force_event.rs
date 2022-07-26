#[doc = "Register `FORCE_EVENT` writer"]
pub struct W(crate::W<FORCE_EVENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FORCE_EVENT_SPEC>;
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
impl From<crate::W<FORCE_EVENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FORCE_EVENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCE_BOOT_ACK_ERR` writer - Force Event for Boot Ack error Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Boot ack Error Status is set"]
pub type FORCE_BOOT_ACK_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_RESP_ERR` writer - Force Event for Response Error (SD Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Response Error Status is set"]
pub type FORCE_RESP_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_TUNING_ERR` writer - Force Event for Tuning Error (UHS-I Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Tuning Error Status is set"]
pub type FORCE_TUNING_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_ADMA_ERR` writer - Force Event for ADMA Error Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): ADMA Error Status is set"]
pub type FORCE_ADMA_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_AUTO_CMD_ERR` writer - Force Event for Auto CMD Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Auto CMD Error Status is set"]
pub type FORCE_AUTO_CMD_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_CUR_LMT_ERR` writer - Force Event for Current Limit Error Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Current Limit Error Status is set"]
pub type FORCE_CUR_LMT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_DATA_END_BIT_ERR` writer - Force Event for Data End Bit Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Data End Bit Error Status is set"]
pub type FORCE_DATA_END_BIT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_DATA_CRC_ERR` writer - Force Event for Data CRC Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Data CRC Error Status is set"]
pub type FORCE_DATA_CRC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_DATA_TOUT_ERR` writer - Force Event for Data Timeout Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Data Timeout Error Status is set"]
pub type FORCE_DATA_TOUT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_CMD_IDX_ERR` writer - Force Event for Command Index Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Command Index Error Status is set"]
pub type FORCE_CMD_IDX_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_CMD_END_BIT_ERR` writer - Force Event for Command End Bit Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Command End Bit Error Status is set"]
pub type FORCE_CMD_END_BIT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_CMD_CRC_ERR` writer - Force Event for Command CRC Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Command CRC Error Status is set"]
pub type FORCE_CMD_CRC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_CMD_TOUT_ERR` writer - Force Event for Command Timeout Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Command Timeout Error Status is set"]
pub type FORCE_CMD_TOUT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_CMD_NOT_ISSUED_AUTO_CMD12` writer - Force Event for Command Not Issued By Auto CMD12 Error Values: 0x1 (TRUE): Command Not Issued By Auto CMD12 Error Status is set 0x0 (FALSE): Not Affected"]
pub type FORCE_CMD_NOT_ISSUED_AUTO_CMD12_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_AUTO_CMD_RESP_ERR` writer - Force Event for Auto CMD Response Error Values: 0x1 (TRUE): Auto CMD Response Error Status is set 0x0 (FALSE): Not Affected"]
pub type FORCE_AUTO_CMD_RESP_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_AUTO_CMD_IDX_ERR` writer - Force Event for Auto CMD Index Error Values: 0x1 (TRUE): Auto CMD Index Error Status is set 0x0 (FALSE): Not Affected"]
pub type FORCE_AUTO_CMD_IDX_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_AUTO_CMD_EBIT_ERR` writer - Force Event for Auto CMD End Bit Error Values: 0x1 (TRUE): Auto CMD End Bit Error Status is set 0x0 (FALSE): Not Affected"]
pub type FORCE_AUTO_CMD_EBIT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_AUTO_CMD_CRC_ERR` writer - Force Event for Auto CMD CRC Error Values: 0x1 (TRUE): Auto CMD CRC Error Status is set 0x0 (FALSE): Not Affected"]
pub type FORCE_AUTO_CMD_CRC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_AUTO_CMD_TOUT_ERR` writer - Force Event for Auto CMD Timeout Error Values: 0x1 (TRUE): Auto CMD Timeout Error Status is set 0x0 (FALSE): Not Affected"]
pub type FORCE_AUTO_CMD_TOUT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
#[doc = "Field `FORCE_AUTO_CMD12_NOT_EXEC` writer - Force Event for Auto CMD12 Not Executed Values: 0x1 (TRUE): Auto CMD12 Not Executed Status is set 0x0 (FALSE): Not Affected"]
pub type FORCE_AUTO_CMD12_NOT_EXEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FORCE_EVENT_SPEC, bool, O>;
impl W {
    #[doc = "Bit 28 - Force Event for Boot Ack error Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Boot ack Error Status is set"]
    #[inline(always)]
    pub fn force_boot_ack_err(&mut self) -> FORCE_BOOT_ACK_ERR_W<28> {
        FORCE_BOOT_ACK_ERR_W::new(self)
    }
    #[doc = "Bit 27 - Force Event for Response Error (SD Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Response Error Status is set"]
    #[inline(always)]
    pub fn force_resp_err(&mut self) -> FORCE_RESP_ERR_W<27> {
        FORCE_RESP_ERR_W::new(self)
    }
    #[doc = "Bit 26 - Force Event for Tuning Error (UHS-I Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Tuning Error Status is set"]
    #[inline(always)]
    pub fn force_tuning_err(&mut self) -> FORCE_TUNING_ERR_W<26> {
        FORCE_TUNING_ERR_W::new(self)
    }
    #[doc = "Bit 25 - Force Event for ADMA Error Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): ADMA Error Status is set"]
    #[inline(always)]
    pub fn force_adma_err(&mut self) -> FORCE_ADMA_ERR_W<25> {
        FORCE_ADMA_ERR_W::new(self)
    }
    #[doc = "Bit 24 - Force Event for Auto CMD Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Auto CMD Error Status is set"]
    #[inline(always)]
    pub fn force_auto_cmd_err(&mut self) -> FORCE_AUTO_CMD_ERR_W<24> {
        FORCE_AUTO_CMD_ERR_W::new(self)
    }
    #[doc = "Bit 23 - Force Event for Current Limit Error Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Current Limit Error Status is set"]
    #[inline(always)]
    pub fn force_cur_lmt_err(&mut self) -> FORCE_CUR_LMT_ERR_W<23> {
        FORCE_CUR_LMT_ERR_W::new(self)
    }
    #[doc = "Bit 22 - Force Event for Data End Bit Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Data End Bit Error Status is set"]
    #[inline(always)]
    pub fn force_data_end_bit_err(&mut self) -> FORCE_DATA_END_BIT_ERR_W<22> {
        FORCE_DATA_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 21 - Force Event for Data CRC Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Data CRC Error Status is set"]
    #[inline(always)]
    pub fn force_data_crc_err(&mut self) -> FORCE_DATA_CRC_ERR_W<21> {
        FORCE_DATA_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 20 - Force Event for Data Timeout Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Data Timeout Error Status is set"]
    #[inline(always)]
    pub fn force_data_tout_err(&mut self) -> FORCE_DATA_TOUT_ERR_W<20> {
        FORCE_DATA_TOUT_ERR_W::new(self)
    }
    #[doc = "Bit 19 - Force Event for Command Index Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Command Index Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_idx_err(&mut self) -> FORCE_CMD_IDX_ERR_W<19> {
        FORCE_CMD_IDX_ERR_W::new(self)
    }
    #[doc = "Bit 18 - Force Event for Command End Bit Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Command End Bit Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_end_bit_err(&mut self) -> FORCE_CMD_END_BIT_ERR_W<18> {
        FORCE_CMD_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 17 - Force Event for Command CRC Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Command CRC Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_crc_err(&mut self) -> FORCE_CMD_CRC_ERR_W<17> {
        FORCE_CMD_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 16 - Force Event for Command Timeout Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Command Timeout Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_tout_err(&mut self) -> FORCE_CMD_TOUT_ERR_W<16> {
        FORCE_CMD_TOUT_ERR_W::new(self)
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error Values: 0x1 (TRUE): Command Not Issued By Auto CMD12 Error Status is set 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_cmd_not_issued_auto_cmd12(&mut self) -> FORCE_CMD_NOT_ISSUED_AUTO_CMD12_W<7> {
        FORCE_CMD_NOT_ISSUED_AUTO_CMD12_W::new(self)
    }
    #[doc = "Bit 5 - Force Event for Auto CMD Response Error Values: 0x1 (TRUE): Auto CMD Response Error Status is set 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_resp_err(&mut self) -> FORCE_AUTO_CMD_RESP_ERR_W<5> {
        FORCE_AUTO_CMD_RESP_ERR_W::new(self)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error Values: 0x1 (TRUE): Auto CMD Index Error Status is set 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_idx_err(&mut self) -> FORCE_AUTO_CMD_IDX_ERR_W<4> {
        FORCE_AUTO_CMD_IDX_ERR_W::new(self)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error Values: 0x1 (TRUE): Auto CMD End Bit Error Status is set 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_ebit_err(&mut self) -> FORCE_AUTO_CMD_EBIT_ERR_W<3> {
        FORCE_AUTO_CMD_EBIT_ERR_W::new(self)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error Values: 0x1 (TRUE): Auto CMD CRC Error Status is set 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_crc_err(&mut self) -> FORCE_AUTO_CMD_CRC_ERR_W<2> {
        FORCE_AUTO_CMD_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error Values: 0x1 (TRUE): Auto CMD Timeout Error Status is set 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd_tout_err(&mut self) -> FORCE_AUTO_CMD_TOUT_ERR_W<1> {
        FORCE_AUTO_CMD_TOUT_ERR_W::new(self)
    }
    #[doc = "Bit 0 - Force Event for Auto CMD12 Not Executed Values: 0x1 (TRUE): Auto CMD12 Not Executed Status is set 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    pub fn force_auto_cmd12_not_exec(&mut self) -> FORCE_AUTO_CMD12_NOT_EXEC_W<0> {
        FORCE_AUTO_CMD12_NOT_EXEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [force_event](index.html) module"]
pub struct FORCE_EVENT_SPEC;
impl crate::RegisterSpec for FORCE_EVENT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [force_event::W](W) writer structure"]
impl crate::Writable for FORCE_EVENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FORCE_EVENT to value 0"]
impl crate::Resettable for FORCE_EVENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
