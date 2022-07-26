#[doc = "Register `INT_STAT` reader"]
pub struct R(crate::R<INT_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_STAT` writer"]
pub struct W(crate::W<INT_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_STAT_SPEC>;
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
impl From<crate::W<INT_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_ACK_ERR` reader - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD/UHS-II mode, this bit is irrelevant."]
pub type BOOT_ACK_ERR_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_ACK_ERR` writer - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD/UHS-II mode, this bit is irrelevant."]
pub type BOOT_ACK_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `RESP_ERR` reader - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type RESP_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RESP_ERR` writer - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type RESP_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `TUNING_ERR` reader - Tuning Error This bit is set when an unrecoverable error is detected in a tuning circuit except during the tuning procedure (occurrence of an error during tuning procedure is indicated by Sampling Clock Select in the Host Control 2 register). By detecting Tuning Error, Host Driver needs to abort a command executing and perform tuning. To reset tuning circuit, Sampling Clock Select is set to 0 before executing tuning procedure. The Tuning Error is higher priority than the other error interrupts generated during data transfer. By detecting Tuning Error, the Host Driver must discard data transferred by a current read/write command and retry data transfer after the Host Controller retrieved from the tuning circuit error. This is applicable in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type TUNING_ERR_R = crate::BitReader<bool>;
#[doc = "Field `TUNING_ERR` writer - Tuning Error This bit is set when an unrecoverable error is detected in a tuning circuit except during the tuning procedure (occurrence of an error during tuning procedure is indicated by Sampling Clock Select in the Host Control 2 register). By detecting Tuning Error, Host Driver needs to abort a command executing and perform tuning. To reset tuning circuit, Sampling Clock Select is set to 0 before executing tuning procedure. The Tuning Error is higher priority than the other error interrupts generated during data transfer. By detecting Tuning Error, the Host Driver must discard data transferred by a current read/write command and retry data transfer after the Host Controller retrieved from the tuning circuit error. This is applicable in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type TUNING_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `ADMA_ERR` reader - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: Error response received from System bus (Master I/F) ADMA3,ADMA2 Descriptors invalid CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type ADMA_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ADMA_ERR` writer - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: Error response received from System bus (Master I/F) ADMA3,ADMA2 Descriptors invalid CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type ADMA_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `AUTO_CMD_ERR` reader - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type AUTO_CMD_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_CMD_ERR` writer - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type AUTO_CMD_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `CUR_LMT_ERR` reader - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. SDXC Host Controller does not support this function, this bit is always set to 0. Values: 0x0 (FALSE): No error 0x1 (TRUE): Power Fail"]
pub type CUR_LMT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CUR_LMT_ERR` writer - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. SDXC Host Controller does not support this function, this bit is always set to 0. Values: 0x0 (FALSE): No error 0x1 (TRUE): Power Fail"]
pub type CUR_LMT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `DATA_END_BIT_ERR` reader - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type DATA_END_BIT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DATA_END_BIT_ERR` writer - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type DATA_END_BIT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `DATA_CRC_ERR` reader - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type DATA_CRC_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DATA_CRC_ERR` writer - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type DATA_CRC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `DATA_TOUT_ERR` reader - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: Busy timeout for R1b, R5b type Busy timeout after Write CRC status Write CRC Status timeout Read Data timeout Values: 0x0 (FALSE): No error 0x1 (TRUE): Time out"]
pub type DATA_TOUT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DATA_TOUT_ERR` writer - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: Busy timeout for R1b, R5b type Busy timeout after Write CRC status Write CRC Status timeout Read Data timeout Values: 0x0 (FALSE): No error 0x1 (TRUE): Time out"]
pub type DATA_TOUT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `CMD_IDX_ERR` reader - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type CMD_IDX_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CMD_IDX_ERR` writer - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
pub type CMD_IDX_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `CMD_END_BIT_ERR` reader - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): End Bit error generated"]
pub type CMD_END_BIT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CMD_END_BIT_ERR` writer - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): End Bit error generated"]
pub type CMD_END_BIT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `CMD_CRC_ERR` reader - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: 0x0 (FALSE): No error 0x1 (TRUE): CRC error generated"]
pub type CMD_CRC_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CMD_CRC_ERR` writer - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: 0x0 (FALSE): No error 0x1 (TRUE): CRC error generated"]
pub type CMD_CRC_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `CMD_TOUT_ERR` reader - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: 0x0 (FALSE): No error 0x1 (TRUE): Time out"]
pub type CMD_TOUT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CMD_TOUT_ERR` writer - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: 0x0 (FALSE): No error 0x1 (TRUE): Time out"]
pub type CMD_TOUT_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `ERR_INTERRUPT` reader - Error Interrupt If any of the bits in the Error Interrupt Status register are set, then this bit is set. Values: 0x0 (FALSE): No Error 0x1 (TRUE): Error"]
pub type ERR_INTERRUPT_R = crate::BitReader<bool>;
#[doc = "Field `CQE_EVENT` reader - Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. Values: 0x0 (FALSE): No Event 0x1 (TRUE): Command Queuing Event is detected"]
pub type CQE_EVENT_R = crate::BitReader<bool>;
#[doc = "Field `CQE_EVENT` writer - Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. Values: 0x0 (FALSE): No Event 0x1 (TRUE): Command Queuing Event is detected"]
pub type CQE_EVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `FX_EVENT` reader - FX Event This status is set when R\\[14\\]
of response register is set to 1 and Response Type R1/R5 is set to 0 in Transfer Mode register. This interrupt is used with response check function. Values: 0x0 (FALSE): No Event 0x1 (TRUE): FX Event is detected"]
pub type FX_EVENT_R = crate::BitReader<bool>;
#[doc = "Field `RE_TUNE_EVENT` reader - Re-tuning Event This bit is set if the Re-Tuning Request changes from 0 to 1. Re-Tuning request is not supported."]
pub type RE_TUNE_EVENT_R = crate::BitReader<bool>;
#[doc = "Field `CARD_INTERRUPT` reader - Card Interrupt This bit reflects the synchronized value of: DAT\\[1\\]
Interrupt Input for SD Mode DAT\\[2\\]
Interrupt Input for UHS-II Mode Values: 0x0 (FALSE): No Card Interrupt 0x1 (TRUE): Generate Card Interrupt"]
pub type CARD_INTERRUPT_R = crate::BitReader<bool>;
#[doc = "Field `CARD_REMOVAL` reader - Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: 0x0 (FALSE): Card state stable or Debouncing 0x1 (TRUE): Card Removed"]
pub type CARD_REMOVAL_R = crate::BitReader<bool>;
#[doc = "Field `CARD_REMOVAL` writer - Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: 0x0 (FALSE): Card state stable or Debouncing 0x1 (TRUE): Card Removed"]
pub type CARD_REMOVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `CARD_INSERTION` reader - Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: 0x0 (FALSE): Card state stable or Debouncing 0x1 (TRUE): Card Inserted"]
pub type CARD_INSERTION_R = crate::BitReader<bool>;
#[doc = "Field `CARD_INSERTION` writer - Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: 0x0 (FALSE): Card state stable or Debouncing 0x1 (TRUE): Card Inserted"]
pub type CARD_INSERTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `BUF_RD_READY` reader - Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: 0x0 (FALSE): Not ready to read buffer 0x1 (TRUE): Ready to read buffer"]
pub type BUF_RD_READY_R = crate::BitReader<bool>;
#[doc = "Field `BUF_RD_READY` writer - Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: 0x0 (FALSE): Not ready to read buffer 0x1 (TRUE): Ready to read buffer"]
pub type BUF_RD_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `BUF_WR_READY` reader - Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: 0x0 (FALSE): Not ready to write buffer 0x1 (TRUE): Ready to write buffer"]
pub type BUF_WR_READY_R = crate::BitReader<bool>;
#[doc = "Field `BUF_WR_READY` writer - Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: 0x0 (FALSE): Not ready to write buffer 0x1 (TRUE): Ready to write buffer"]
pub type BUF_WR_READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `DMA_INTERRUPT` reader - DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: 0x0 (FALSE): No DMA Interrupt 0x1 (TRUE): DMA Interrupt is generated"]
pub type DMA_INTERRUPT_R = crate::BitReader<bool>;
#[doc = "Field `DMA_INTERRUPT` writer - DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: 0x0 (FALSE): No DMA Interrupt 0x1 (TRUE): DMA Interrupt is generated"]
pub type DMA_INTERRUPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `BGAP_EVENT` reader - Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: 0x0 (FALSE): No Block Gap Event 0x1 (TRUE): Transaction stopped at block gap"]
pub type BGAP_EVENT_R = crate::BitReader<bool>;
#[doc = "Field `BGAP_EVENT` writer - Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: 0x0 (FALSE): No Block Gap Event 0x1 (TRUE): Transaction stopped at block gap"]
pub type BGAP_EVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `XFER_COMPLETE` reader - Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: 0x0 (FALSE): Not complete 0x1 (TRUE): Command execution is completed"]
pub type XFER_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `XFER_COMPLETE` writer - Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: 0x0 (FALSE): Not complete 0x1 (TRUE): Command execution is completed"]
pub type XFER_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
#[doc = "Field `CMD_COMPLETE` reader - Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: 0x0 (FALSE): No command complete 0x1 (TRUE): Command Complete"]
pub type CMD_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `CMD_COMPLETE` writer - Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: 0x0 (FALSE): No command complete 0x1 (TRUE): Command Complete"]
pub type CMD_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 28 - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD/UHS-II mode, this bit is irrelevant."]
    #[inline(always)]
    pub fn boot_ack_err(&self) -> BOOT_ACK_ERR_R {
        BOOT_ACK_ERR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn resp_err(&self) -> RESP_ERR_R {
        RESP_ERR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - Tuning Error This bit is set when an unrecoverable error is detected in a tuning circuit except during the tuning procedure (occurrence of an error during tuning procedure is indicated by Sampling Clock Select in the Host Control 2 register). By detecting Tuning Error, Host Driver needs to abort a command executing and perform tuning. To reset tuning circuit, Sampling Clock Select is set to 0 before executing tuning procedure. The Tuning Error is higher priority than the other error interrupts generated during data transfer. By detecting Tuning Error, the Host Driver must discard data transferred by a current read/write command and retry data transfer after the Host Controller retrieved from the tuning circuit error. This is applicable in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn tuning_err(&self) -> TUNING_ERR_R {
        TUNING_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: Error response received from System bus (Master I/F) ADMA3,ADMA2 Descriptors invalid CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn adma_err(&self) -> ADMA_ERR_R {
        ADMA_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn auto_cmd_err(&self) -> AUTO_CMD_ERR_R {
        AUTO_CMD_ERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. SDXC Host Controller does not support this function, this bit is always set to 0. Values: 0x0 (FALSE): No error 0x1 (TRUE): Power Fail"]
    #[inline(always)]
    pub fn cur_lmt_err(&self) -> CUR_LMT_ERR_R {
        CUR_LMT_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_end_bit_err(&self) -> DATA_END_BIT_ERR_R {
        DATA_END_BIT_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_crc_err(&self) -> DATA_CRC_ERR_R {
        DATA_CRC_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: Busy timeout for R1b, R5b type Busy timeout after Write CRC status Write CRC Status timeout Read Data timeout Values: 0x0 (FALSE): No error 0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn data_tout_err(&self) -> DATA_TOUT_ERR_R {
        DATA_TOUT_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn cmd_idx_err(&self) -> CMD_IDX_ERR_R {
        CMD_IDX_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): End Bit error generated"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&self) -> CMD_END_BIT_ERR_R {
        CMD_END_BIT_ERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: 0x0 (FALSE): No error 0x1 (TRUE): CRC error generated"]
    #[inline(always)]
    pub fn cmd_crc_err(&self) -> CMD_CRC_ERR_R {
        CMD_CRC_ERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: 0x0 (FALSE): No error 0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn cmd_tout_err(&self) -> CMD_TOUT_ERR_R {
        CMD_TOUT_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt If any of the bits in the Error Interrupt Status register are set, then this bit is set. Values: 0x0 (FALSE): No Error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn err_interrupt(&self) -> ERR_INTERRUPT_R {
        ERR_INTERRUPT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. Values: 0x0 (FALSE): No Event 0x1 (TRUE): Command Queuing Event is detected"]
    #[inline(always)]
    pub fn cqe_event(&self) -> CQE_EVENT_R {
        CQE_EVENT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - FX Event This status is set when R\\[14\\]
of response register is set to 1 and Response Type R1/R5 is set to 0 in Transfer Mode register. This interrupt is used with response check function. Values: 0x0 (FALSE): No Event 0x1 (TRUE): FX Event is detected"]
    #[inline(always)]
    pub fn fx_event(&self) -> FX_EVENT_R {
        FX_EVENT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-tuning Event This bit is set if the Re-Tuning Request changes from 0 to 1. Re-Tuning request is not supported."]
    #[inline(always)]
    pub fn re_tune_event(&self) -> RE_TUNE_EVENT_R {
        RE_TUNE_EVENT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt This bit reflects the synchronized value of: DAT\\[1\\]
Interrupt Input for SD Mode DAT\\[2\\]
Interrupt Input for UHS-II Mode Values: 0x0 (FALSE): No Card Interrupt 0x1 (TRUE): Generate Card Interrupt"]
    #[inline(always)]
    pub fn card_interrupt(&self) -> CARD_INTERRUPT_R {
        CARD_INTERRUPT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: 0x0 (FALSE): Card state stable or Debouncing 0x1 (TRUE): Card Removed"]
    #[inline(always)]
    pub fn card_removal(&self) -> CARD_REMOVAL_R {
        CARD_REMOVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: 0x0 (FALSE): Card state stable or Debouncing 0x1 (TRUE): Card Inserted"]
    #[inline(always)]
    pub fn card_insertion(&self) -> CARD_INSERTION_R {
        CARD_INSERTION_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: 0x0 (FALSE): Not ready to read buffer 0x1 (TRUE): Ready to read buffer"]
    #[inline(always)]
    pub fn buf_rd_ready(&self) -> BUF_RD_READY_R {
        BUF_RD_READY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: 0x0 (FALSE): Not ready to write buffer 0x1 (TRUE): Ready to write buffer"]
    #[inline(always)]
    pub fn buf_wr_ready(&self) -> BUF_WR_READY_R {
        BUF_WR_READY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: 0x0 (FALSE): No DMA Interrupt 0x1 (TRUE): DMA Interrupt is generated"]
    #[inline(always)]
    pub fn dma_interrupt(&self) -> DMA_INTERRUPT_R {
        DMA_INTERRUPT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: 0x0 (FALSE): No Block Gap Event 0x1 (TRUE): Transaction stopped at block gap"]
    #[inline(always)]
    pub fn bgap_event(&self) -> BGAP_EVENT_R {
        BGAP_EVENT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: 0x0 (FALSE): Not complete 0x1 (TRUE): Command execution is completed"]
    #[inline(always)]
    pub fn xfer_complete(&self) -> XFER_COMPLETE_R {
        XFER_COMPLETE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: 0x0 (FALSE): No command complete 0x1 (TRUE): Command Complete"]
    #[inline(always)]
    pub fn cmd_complete(&self) -> CMD_COMPLETE_R {
        CMD_COMPLETE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD/UHS-II mode, this bit is irrelevant."]
    #[inline(always)]
    pub fn boot_ack_err(&mut self) -> BOOT_ACK_ERR_W<28> {
        BOOT_ACK_ERR_W::new(self)
    }
    #[doc = "Bit 27 - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn resp_err(&mut self) -> RESP_ERR_W<27> {
        RESP_ERR_W::new(self)
    }
    #[doc = "Bit 26 - Tuning Error This bit is set when an unrecoverable error is detected in a tuning circuit except during the tuning procedure (occurrence of an error during tuning procedure is indicated by Sampling Clock Select in the Host Control 2 register). By detecting Tuning Error, Host Driver needs to abort a command executing and perform tuning. To reset tuning circuit, Sampling Clock Select is set to 0 before executing tuning procedure. The Tuning Error is higher priority than the other error interrupts generated during data transfer. By detecting Tuning Error, the Host Driver must discard data transferred by a current read/write command and retry data transfer after the Host Controller retrieved from the tuning circuit error. This is applicable in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn tuning_err(&mut self) -> TUNING_ERR_W<26> {
        TUNING_ERR_W::new(self)
    }
    #[doc = "Bit 25 - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: Error response received from System bus (Master I/F) ADMA3,ADMA2 Descriptors invalid CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn adma_err(&mut self) -> ADMA_ERR_W<25> {
        ADMA_ERR_W::new(self)
    }
    #[doc = "Bit 24 - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn auto_cmd_err(&mut self) -> AUTO_CMD_ERR_W<24> {
        AUTO_CMD_ERR_W::new(self)
    }
    #[doc = "Bit 23 - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. SDXC Host Controller does not support this function, this bit is always set to 0. Values: 0x0 (FALSE): No error 0x1 (TRUE): Power Fail"]
    #[inline(always)]
    pub fn cur_lmt_err(&mut self) -> CUR_LMT_ERR_W<23> {
        CUR_LMT_ERR_W::new(self)
    }
    #[doc = "Bit 22 - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_end_bit_err(&mut self) -> DATA_END_BIT_ERR_W<22> {
        DATA_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_crc_err(&mut self) -> DATA_CRC_ERR_W<21> {
        DATA_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 20 - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: Busy timeout for R1b, R5b type Busy timeout after Write CRC status Write CRC Status timeout Read Data timeout Values: 0x0 (FALSE): No error 0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn data_tout_err(&mut self) -> DATA_TOUT_ERR_W<20> {
        DATA_TOUT_ERR_W::new(self)
    }
    #[doc = "Bit 19 - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn cmd_idx_err(&mut self) -> CMD_IDX_ERR_W<19> {
        CMD_IDX_ERR_W::new(self)
    }
    #[doc = "Bit 18 - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): End Bit error generated"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&mut self) -> CMD_END_BIT_ERR_W<18> {
        CMD_END_BIT_ERR_W::new(self)
    }
    #[doc = "Bit 17 - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: 0x0 (FALSE): No error 0x1 (TRUE): CRC error generated"]
    #[inline(always)]
    pub fn cmd_crc_err(&mut self) -> CMD_CRC_ERR_W<17> {
        CMD_CRC_ERR_W::new(self)
    }
    #[doc = "Bit 16 - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: 0x0 (FALSE): No error 0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn cmd_tout_err(&mut self) -> CMD_TOUT_ERR_W<16> {
        CMD_TOUT_ERR_W::new(self)
    }
    #[doc = "Bit 14 - Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. Values: 0x0 (FALSE): No Event 0x1 (TRUE): Command Queuing Event is detected"]
    #[inline(always)]
    pub fn cqe_event(&mut self) -> CQE_EVENT_W<14> {
        CQE_EVENT_W::new(self)
    }
    #[doc = "Bit 7 - Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: 0x0 (FALSE): Card state stable or Debouncing 0x1 (TRUE): Card Removed"]
    #[inline(always)]
    pub fn card_removal(&mut self) -> CARD_REMOVAL_W<7> {
        CARD_REMOVAL_W::new(self)
    }
    #[doc = "Bit 6 - Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: 0x0 (FALSE): Card state stable or Debouncing 0x1 (TRUE): Card Inserted"]
    #[inline(always)]
    pub fn card_insertion(&mut self) -> CARD_INSERTION_W<6> {
        CARD_INSERTION_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: 0x0 (FALSE): Not ready to read buffer 0x1 (TRUE): Ready to read buffer"]
    #[inline(always)]
    pub fn buf_rd_ready(&mut self) -> BUF_RD_READY_W<5> {
        BUF_RD_READY_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: 0x0 (FALSE): Not ready to write buffer 0x1 (TRUE): Ready to write buffer"]
    #[inline(always)]
    pub fn buf_wr_ready(&mut self) -> BUF_WR_READY_W<4> {
        BUF_WR_READY_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: 0x0 (FALSE): No DMA Interrupt 0x1 (TRUE): DMA Interrupt is generated"]
    #[inline(always)]
    pub fn dma_interrupt(&mut self) -> DMA_INTERRUPT_W<3> {
        DMA_INTERRUPT_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: 0x0 (FALSE): No Block Gap Event 0x1 (TRUE): Transaction stopped at block gap"]
    #[inline(always)]
    pub fn bgap_event(&mut self) -> BGAP_EVENT_W<2> {
        BGAP_EVENT_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: 0x0 (FALSE): Not complete 0x1 (TRUE): Command execution is completed"]
    #[inline(always)]
    pub fn xfer_complete(&mut self) -> XFER_COMPLETE_W<1> {
        XFER_COMPLETE_W::new(self)
    }
    #[doc = "Bit 0 - Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: 0x0 (FALSE): No command complete 0x1 (TRUE): Command Complete"]
    #[inline(always)]
    pub fn cmd_complete(&mut self) -> CMD_COMPLETE_W<0> {
        CMD_COMPLETE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_stat](index.html) module"]
pub struct INT_STAT_SPEC;
impl crate::RegisterSpec for INT_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_stat::R](R) reader structure"]
impl crate::Readable for INT_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_stat::W](W) writer structure"]
impl crate::Writable for INT_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_STAT to value 0"]
impl crate::Resettable for INT_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
