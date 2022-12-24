#[doc = "Register `INT_STAT_EN` reader"]
pub struct R(crate::R<INT_STAT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STAT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STAT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STAT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_STAT_EN` writer"]
pub struct W(crate::W<INT_STAT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_STAT_EN_SPEC>;
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
impl From<crate::W<INT_STAT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_STAT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_COMPLETE_STAT_EN` reader - Command Complete Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CMD_COMPLETE_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMD_COMPLETE_STAT_EN` writer - Command Complete Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CMD_COMPLETE_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `XFER_COMPLETE_STAT_EN` reader - Transfer Complete Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type XFER_COMPLETE_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `XFER_COMPLETE_STAT_EN` writer - Transfer Complete Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type XFER_COMPLETE_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `BGAP_EVENT_STAT_EN` reader - Block Gap Event Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type BGAP_EVENT_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `BGAP_EVENT_STAT_EN` writer - Block Gap Event Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type BGAP_EVENT_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `DMA_INTERRUPT_STAT_EN` reader - DMA Interrupt Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type DMA_INTERRUPT_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_INTERRUPT_STAT_EN` writer - DMA Interrupt Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type DMA_INTERRUPT_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `BUF_WR_READY_STAT_EN` reader - Buffer Write Ready Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type BUF_WR_READY_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `BUF_WR_READY_STAT_EN` writer - Buffer Write Ready Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type BUF_WR_READY_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `BUF_RD_READY_STAT_EN` reader - Buffer Read Ready Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type BUF_RD_READY_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `BUF_RD_READY_STAT_EN` writer - Buffer Read Ready Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type BUF_RD_READY_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `CARD_INSERTION_STAT_EN` reader - Card Insertion Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CARD_INSERTION_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CARD_INSERTION_STAT_EN` writer - Card Insertion Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CARD_INSERTION_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `CARD_REMOVAL_STAT_EN` reader - Card Removal Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CARD_REMOVAL_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CARD_REMOVAL_STAT_EN` writer - Card Removal Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CARD_REMOVAL_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `CARD_INTERRUPT_STAT_EN` reader - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. By setting this bit to 0, interrupt input must be masked by implementation so that the interrupt input is not affected by external signal in any state (for example, floating). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CARD_INTERRUPT_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CARD_INTERRUPT_STAT_EN` writer - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. By setting this bit to 0, interrupt input must be masked by implementation so that the interrupt input is not affected by external signal in any state (for example, floating). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CARD_INTERRUPT_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `RE_TUNE_EVENT_STAT_EN` reader - Re-Tuning Event (UHS-I only) Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type RE_TUNE_EVENT_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RE_TUNE_EVENT_STAT_EN` writer - Re-Tuning Event (UHS-I only) Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type RE_TUNE_EVENT_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `FX_EVENT_STAT_EN` reader - FX Event Status Enable This bit is added from Version 4.10. Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type FX_EVENT_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `FX_EVENT_STAT_EN` writer - FX Event Status Enable This bit is added from Version 4.10. Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type FX_EVENT_STAT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `CQE_EVENT_STAT_EN` reader - CQE Event Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CQE_EVENT_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CQE_EVENT_STAT_EN` writer - CQE Event Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CQE_EVENT_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `CMD_TOUT_ERR_STAT_EN` reader - Command Timeout Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CMD_TOUT_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMD_TOUT_ERR_STAT_EN` writer - Command Timeout Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CMD_TOUT_ERR_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `CMD_CRC_ERR_STAT_EN` reader - Command CRC Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CMD_CRC_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMD_CRC_ERR_STAT_EN` writer - Command CRC Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CMD_CRC_ERR_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `CMD_END_BIT_ERR_STAT_EN` reader - Command End Bit Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CMD_END_BIT_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMD_END_BIT_ERR_STAT_EN` writer - Command End Bit Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CMD_END_BIT_ERR_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `CMD_IDX_ERR_STAT_EN` reader - Command Index Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CMD_IDX_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMD_IDX_ERR_STAT_EN` writer - Command Index Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CMD_IDX_ERR_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `DATA_TOUT_ERR_STAT_EN` reader - Data Timeout Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type DATA_TOUT_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DATA_TOUT_ERR_STAT_EN` writer - Data Timeout Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type DATA_TOUT_ERR_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `DATA_CRC_ERR_STAT_EN` reader - Data CRC Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type DATA_CRC_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DATA_CRC_ERR_STAT_EN` writer - Data CRC Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type DATA_CRC_ERR_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `DATA_END_BIT_ERR_STAT_EN` reader - Data End Bit Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type DATA_END_BIT_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `DATA_END_BIT_ERR_STAT_EN` writer - Data End Bit Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type DATA_END_BIT_ERR_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `CUR_LMT_ERR_STAT_EN` reader - Current Limit Error Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CUR_LMT_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `CUR_LMT_ERR_STAT_EN` writer - Current Limit Error Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type CUR_LMT_ERR_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `AUTO_CMD_ERR_STAT_EN` reader - Auto CMD Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type AUTO_CMD_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_CMD_ERR_STAT_EN` writer - Auto CMD Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type AUTO_CMD_ERR_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `ADMA_ERR_STAT_EN` reader - ADMA Error Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type ADMA_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADMA_ERR_STAT_EN` writer - ADMA Error Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type ADMA_ERR_STAT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `TUNING_ERR_STAT_EN` reader - Tuning Error Status Enable (UHS-I Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type TUNING_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `TUNING_ERR_STAT_EN` writer - Tuning Error Status Enable (UHS-I Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type TUNING_ERR_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `RESP_ERR_STAT_EN` reader - Response Error Status Enable (SD Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type RESP_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RESP_ERR_STAT_EN` writer - Response Error Status Enable (SD Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type RESP_ERR_STAT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
#[doc = "Field `BOOT_ACK_ERR_STAT_EN` reader - Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type BOOT_ACK_ERR_STAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_ACK_ERR_STAT_EN` writer - Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
pub type BOOT_ACK_ERR_STAT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_STAT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Command Complete Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_complete_stat_en(&self) -> CMD_COMPLETE_STAT_EN_R {
        CMD_COMPLETE_STAT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn xfer_complete_stat_en(&self) -> XFER_COMPLETE_STAT_EN_R {
        XFER_COMPLETE_STAT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn bgap_event_stat_en(&self) -> BGAP_EVENT_STAT_EN_R {
        BGAP_EVENT_STAT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn dma_interrupt_stat_en(&self) -> DMA_INTERRUPT_STAT_EN_R {
        DMA_INTERRUPT_STAT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_wr_ready_stat_en(&self) -> BUF_WR_READY_STAT_EN_R {
        BUF_WR_READY_STAT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_rd_ready_stat_en(&self) -> BUF_RD_READY_STAT_EN_R {
        BUF_RD_READY_STAT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_insertion_stat_en(&self) -> CARD_INSERTION_STAT_EN_R {
        CARD_INSERTION_STAT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_removal_stat_en(&self) -> CARD_REMOVAL_STAT_EN_R {
        CARD_REMOVAL_STAT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. By setting this bit to 0, interrupt input must be masked by implementation so that the interrupt input is not affected by external signal in any state (for example, floating). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_interrupt_stat_en(&self) -> CARD_INTERRUPT_STAT_EN_R {
        CARD_INTERRUPT_STAT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event (UHS-I only) Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn re_tune_event_stat_en(&self) -> RE_TUNE_EVENT_STAT_EN_R {
        RE_TUNE_EVENT_STAT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FX Event Status Enable This bit is added from Version 4.10. Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn fx_event_stat_en(&self) -> FX_EVENT_STAT_EN_R {
        FX_EVENT_STAT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CQE Event Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cqe_event_stat_en(&self) -> CQE_EVENT_STAT_EN_R {
        CQE_EVENT_STAT_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_tout_err_stat_en(&self) -> CMD_TOUT_ERR_STAT_EN_R {
        CMD_TOUT_ERR_STAT_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_crc_err_stat_en(&self) -> CMD_CRC_ERR_STAT_EN_R {
        CMD_CRC_ERR_STAT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_end_bit_err_stat_en(&self) -> CMD_END_BIT_ERR_STAT_EN_R {
        CMD_END_BIT_ERR_STAT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_idx_err_stat_en(&self) -> CMD_IDX_ERR_STAT_EN_R {
        CMD_IDX_ERR_STAT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_tout_err_stat_en(&self) -> DATA_TOUT_ERR_STAT_EN_R {
        DATA_TOUT_ERR_STAT_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_crc_err_stat_en(&self) -> DATA_CRC_ERR_STAT_EN_R {
        DATA_CRC_ERR_STAT_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_end_bit_err_stat_en(&self) -> DATA_END_BIT_ERR_STAT_EN_R {
        DATA_END_BIT_ERR_STAT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Current Limit Error Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cur_lmt_err_stat_en(&self) -> CUR_LMT_ERR_STAT_EN_R {
        CUR_LMT_ERR_STAT_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn auto_cmd_err_stat_en(&self) -> AUTO_CMD_ERR_STAT_EN_R {
        AUTO_CMD_ERR_STAT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADMA Error Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn adma_err_stat_en(&self) -> ADMA_ERR_STAT_EN_R {
        ADMA_ERR_STAT_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning Error Status Enable (UHS-I Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn tuning_err_stat_en(&self) -> TUNING_ERR_STAT_EN_R {
        TUNING_ERR_STAT_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Response Error Status Enable (SD Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn resp_err_stat_en(&self) -> RESP_ERR_STAT_EN_R {
        RESP_ERR_STAT_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn boot_ack_err_stat_en(&self) -> BOOT_ACK_ERR_STAT_EN_R {
        BOOT_ACK_ERR_STAT_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete_stat_en(&mut self) -> CMD_COMPLETE_STAT_EN_W<0> {
        CMD_COMPLETE_STAT_EN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_complete_stat_en(&mut self) -> XFER_COMPLETE_STAT_EN_W<1> {
        XFER_COMPLETE_STAT_EN_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_event_stat_en(&mut self) -> BGAP_EVENT_STAT_EN_W<2> {
        BGAP_EVENT_STAT_EN_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dma_interrupt_stat_en(&mut self) -> DMA_INTERRUPT_STAT_EN_W<3> {
        DMA_INTERRUPT_STAT_EN_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn buf_wr_ready_stat_en(&mut self) -> BUF_WR_READY_STAT_EN_W<4> {
        BUF_WR_READY_STAT_EN_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn buf_rd_ready_stat_en(&mut self) -> BUF_RD_READY_STAT_EN_W<5> {
        BUF_RD_READY_STAT_EN_W::new(self)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_insertion_stat_en(&mut self) -> CARD_INSERTION_STAT_EN_W<6> {
        CARD_INSERTION_STAT_EN_W::new(self)
    }
    #[doc = "Bit 7 - Card Removal Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal_stat_en(&mut self) -> CARD_REMOVAL_STAT_EN_W<7> {
        CARD_REMOVAL_STAT_EN_W::new(self)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. By setting this bit to 0, interrupt input must be masked by implementation so that the interrupt input is not affected by external signal in any state (for example, floating). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_interrupt_stat_en(&mut self) -> CARD_INTERRUPT_STAT_EN_W<8> {
        CARD_INTERRUPT_STAT_EN_W::new(self)
    }
    #[doc = "Bit 12 - Re-Tuning Event (UHS-I only) Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn re_tune_event_stat_en(&mut self) -> RE_TUNE_EVENT_STAT_EN_W<12> {
        RE_TUNE_EVENT_STAT_EN_W::new(self)
    }
    #[doc = "Bit 13 - FX Event Status Enable This bit is added from Version 4.10. Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fx_event_stat_en(&mut self) -> FX_EVENT_STAT_EN_W<13> {
        FX_EVENT_STAT_EN_W::new(self)
    }
    #[doc = "Bit 14 - CQE Event Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cqe_event_stat_en(&mut self) -> CQE_EVENT_STAT_EN_W<14> {
        CQE_EVENT_STAT_EN_W::new(self)
    }
    #[doc = "Bit 16 - Command Timeout Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_tout_err_stat_en(&mut self) -> CMD_TOUT_ERR_STAT_EN_W<16> {
        CMD_TOUT_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_err_stat_en(&mut self) -> CMD_CRC_ERR_STAT_EN_W<17> {
        CMD_CRC_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_end_bit_err_stat_en(&mut self) -> CMD_END_BIT_ERR_STAT_EN_W<18> {
        CMD_END_BIT_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_idx_err_stat_en(&mut self) -> CMD_IDX_ERR_STAT_EN_W<19> {
        CMD_IDX_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_tout_err_stat_en(&mut self) -> DATA_TOUT_ERR_STAT_EN_W<20> {
        DATA_TOUT_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_crc_err_stat_en(&mut self) -> DATA_CRC_ERR_STAT_EN_W<21> {
        DATA_CRC_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_end_bit_err_stat_en(&mut self) -> DATA_END_BIT_ERR_STAT_EN_W<22> {
        DATA_END_BIT_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Bit 23 - Current Limit Error Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cur_lmt_err_stat_en(&mut self) -> CUR_LMT_ERR_STAT_EN_W<23> {
        CUR_LMT_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Bit 24 - Auto CMD Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_err_stat_en(&mut self) -> AUTO_CMD_ERR_STAT_EN_W<24> {
        AUTO_CMD_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Bit 25 - ADMA Error Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn adma_err_stat_en(&mut self) -> ADMA_ERR_STAT_EN_W<25> {
        ADMA_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Bit 26 - Tuning Error Status Enable (UHS-I Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_err_stat_en(&mut self) -> TUNING_ERR_STAT_EN_W<26> {
        TUNING_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Bit 27 - Response Error Status Enable (SD Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn resp_err_stat_en(&mut self) -> RESP_ERR_STAT_EN_W<27> {
        RESP_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Bit 28 - Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack_err_stat_en(&mut self) -> BOOT_ACK_ERR_STAT_EN_W<28> {
        BOOT_ACK_ERR_STAT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_stat_en](index.html) module"]
pub struct INT_STAT_EN_SPEC;
impl crate::RegisterSpec for INT_STAT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_stat_en::R](R) reader structure"]
impl crate::Readable for INT_STAT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_stat_en::W](W) writer structure"]
impl crate::Writable for INT_STAT_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_STAT_EN to value 0"]
impl crate::Resettable for INT_STAT_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
