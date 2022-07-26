#[doc = "Register `CMD_XFER` reader"]
pub struct R(crate::R<CMD_XFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_XFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_XFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_XFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD_XFER` writer"]
pub struct W(crate::W<CMD_XFER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_XFER_SPEC>;
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
impl From<crate::W<CMD_XFER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_XFER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_INDEX` reader - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
pub type CMD_INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD_INDEX` writer - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
pub type CMD_INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_XFER_SPEC, u8, u8, 6, O>;
#[doc = "Field `CMD_TYPE` reader - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: 0x3 (ABORT_CMD): Abort 0x2 (RESUME_CMD): Resume 0x1 (SUSPEND_CMD): Suspend 0x0 (NORMAL_CMD): Normal"]
pub type CMD_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD_TYPE` writer - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: 0x3 (ABORT_CMD): Abort 0x2 (RESUME_CMD): Resume 0x1 (SUSPEND_CMD): Suspend 0x0 (NORMAL_CMD): Normal"]
pub type CMD_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_XFER_SPEC, u8, u8, 2, O>;
#[doc = "Field `DATA_PRESENT_SEL` reader - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: Command using the CMD line Command with no data transfer but using busy signal on the DAT\\[0\\]
line Resume Command Values: 0x0 (NO_DATA): No Data Present 0x1 (DATA): Data Present"]
pub type DATA_PRESENT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `DATA_PRESENT_SEL` writer - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: Command using the CMD line Command with no data transfer but using busy signal on the DAT\\[0\\]
line Resume Command Values: 0x0 (NO_DATA): No Data Present 0x1 (DATA): Data Present"]
pub type DATA_PRESENT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFER_SPEC, bool, O>;
#[doc = "Field `CMD_IDX_CHK_ENABLE` reader - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. For the tuning command, this bit must always be set to enable the index check. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
pub type CMD_IDX_CHK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CMD_IDX_CHK_ENABLE` writer - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. For the tuning command, this bit must always be set to enable the index check. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
pub type CMD_IDX_CHK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFER_SPEC, bool, O>;
#[doc = "Field `CMD_CRC_CHK_ENABLE` reader - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. For the tuning command, this bit must always be set to 1 to enable the CRC check. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
pub type CMD_CRC_CHK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CMD_CRC_CHK_ENABLE` writer - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. For the tuning command, this bit must always be set to 1 to enable the CRC check. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
pub type CMD_CRC_CHK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFER_SPEC, bool, O>;
#[doc = "Field `SUB_CMD_FLAG` reader - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: 0x0 (MAIN): Main Command 0x1 (SUB): Sub Command"]
pub type SUB_CMD_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `SUB_CMD_FLAG` writer - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: 0x0 (MAIN): Main Command 0x1 (SUB): Sub Command"]
pub type SUB_CMD_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFER_SPEC, bool, O>;
#[doc = "Field `RESP_TYPE_SELECT` reader - Response Type Select This bit indicates the type of response expected from the card. Values: 0x0 (NO_RESP): No Response 0x1 (RESP_LEN_136): Response Length 136 0x2 (RESP_LEN_48): Response Length 48 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
pub type RESP_TYPE_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESP_TYPE_SELECT` writer - Response Type Select This bit indicates the type of response expected from the card. Values: 0x0 (NO_RESP): No Response 0x1 (RESP_LEN_136): Response Length 136 0x2 (RESP_LEN_48): Response Length 48 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
pub type RESP_TYPE_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMD_XFER_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESP_INT_DISABLE` reader - Response Interrupt Disable The Host Controller supports response check function to avoid overhead of response error check by the Host driver. Response types of only R1 and R5 can be checked by the Controller. If Host Driver checks the response error, set this bit to 0 and wait for Command Complete Interrupt and then check the response register. If the Host Controller checks the response error, set this bit to 1 and set the Response Error Check Enable bit to 1. The Command Complete Interrupt is disabled by this bit regardless of the Command Complete Signal Enable. Note: During tuning (when the Execute Tuning bit in the Host Control2 register is set), the Command Complete Interrupt is not generated irrespective of the Response Interrupt Disable setting. Values: - 0x0 (ENABLED): Response Interrupt is enabled - 0x1 (DISABLED): Response Interrupt is disabled"]
pub type RESP_INT_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `RESP_INT_DISABLE` writer - Response Interrupt Disable The Host Controller supports response check function to avoid overhead of response error check by the Host driver. Response types of only R1 and R5 can be checked by the Controller. If Host Driver checks the response error, set this bit to 0 and wait for Command Complete Interrupt and then check the response register. If the Host Controller checks the response error, set this bit to 1 and set the Response Error Check Enable bit to 1. The Command Complete Interrupt is disabled by this bit regardless of the Command Complete Signal Enable. Note: During tuning (when the Execute Tuning bit in the Host Control2 register is set), the Command Complete Interrupt is not generated irrespective of the Response Interrupt Disable setting. Values: - 0x0 (ENABLED): Response Interrupt is enabled - 0x1 (DISABLED): Response Interrupt is disabled"]
pub type RESP_INT_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFER_SPEC, bool, O>;
#[doc = "Field `RESP_ERR_CHK_ENABLE` reader - Response Error Check Enable The Host Controller supports response check function to avoid overhead of response error check by Host driver. Response types of only R1 and R5 can be checked by the Controller. If the Host Controller checks the response error, set this bit to 1 and set Response Interrupt Disable to 1. If an error is detected, the Response Error interrupt is generated in the Error Interrupt Status register. Note: - Response error check must not be enabled for any response type other than R1 and R5. - Response check must not be enabled for the tuning command. Values: - 0x0 (DISABLED): Response Error Check is disabled - 0x1 (ENABLED): Response Error Check is enabled"]
pub type RESP_ERR_CHK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RESP_ERR_CHK_ENABLE` writer - Response Error Check Enable The Host Controller supports response check function to avoid overhead of response error check by Host driver. Response types of only R1 and R5 can be checked by the Controller. If the Host Controller checks the response error, set this bit to 1 and set Response Interrupt Disable to 1. If an error is detected, the Response Error interrupt is generated in the Error Interrupt Status register. Note: - Response error check must not be enabled for any response type other than R1 and R5. - Response check must not be enabled for the tuning command. Values: - 0x0 (DISABLED): Response Error Check is disabled - 0x1 (ENABLED): Response Error Check is enabled"]
pub type RESP_ERR_CHK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFER_SPEC, bool, O>;
#[doc = "Field `RESP_TYPE` reader - Response Type R1/R5 This bit selects either R1 or R5 as a response type when the Response Error Check is selected. Error statuses checked in R1: OUT_OF_RANGE ADDRESS_ERROR BLOCK_LEN_ERROR WP_VIOLATION CARD_IS_LOCKED COM_CRC_ERROR CARD_ECC_FAILED CC_ERROR ERROR Response Flags checked in R5: COM_CRC_ERROR ERROR FUNCTION_NUMBER OUT_OF_RANGE Values: 0x0 (RESP_R1): R1 (Memory) 0x1 (RESP_R5): R5 (SDIO)"]
pub type RESP_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `RESP_TYPE` writer - Response Type R1/R5 This bit selects either R1 or R5 as a response type when the Response Error Check is selected. Error statuses checked in R1: OUT_OF_RANGE ADDRESS_ERROR BLOCK_LEN_ERROR WP_VIOLATION CARD_IS_LOCKED COM_CRC_ERROR CARD_ECC_FAILED CC_ERROR ERROR Response Flags checked in R5: COM_CRC_ERROR ERROR FUNCTION_NUMBER OUT_OF_RANGE Values: 0x0 (RESP_R1): R1 (Memory) 0x1 (RESP_R5): R5 (SDIO)"]
pub type RESP_TYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFER_SPEC, bool, O>;
#[doc = "Field `MULTI_BLK_SEL` reader - Multi/Single Block Select This bit is set when issuing multiple-block transfer commands using the DAT line. If this bit is set to 0, it is not necessary to set the Block Count register. Values: 0x0 (SINGLE): Single Block 0x1 (MULTI): Multiple Block"]
pub type MULTI_BLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `MULTI_BLK_SEL` writer - Multi/Single Block Select This bit is set when issuing multiple-block transfer commands using the DAT line. If this bit is set to 0, it is not necessary to set the Block Count register. Values: 0x0 (SINGLE): Single Block 0x1 (MULTI): Multiple Block"]
pub type MULTI_BLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFER_SPEC, bool, O>;
#[doc = "Field `DATA_XFER_DIR` reader - Data Transfer Direction Select This bit defines the direction of DAT line data transfers. This bit is set to 1 by the Host Driver to transfer data from the SD/eMMC card to the Host Controller and it is set to 0 for all other commands. Values: 0x1 (READ): Read (Card to Host) 0x0 (WRITE): Write (Host to Card)"]
pub type DATA_XFER_DIR_R = crate::BitReader<bool>;
#[doc = "Field `DATA_XFER_DIR` writer - Data Transfer Direction Select This bit defines the direction of DAT line data transfers. This bit is set to 1 by the Host Driver to transfer data from the SD/eMMC card to the Host Controller and it is set to 0 for all other commands. Values: 0x1 (READ): Read (Card to Host) 0x0 (WRITE): Write (Host to Card)"]
pub type DATA_XFER_DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFER_SPEC, bool, O>;
#[doc = "Field `AUTO_CMD_ENABLE` reader - Auto Command Enable This field determines use of Auto Command functions. Note: In SDIO, this field must be set as 00b (Auto Command Disabled). Values: 0x0 (AUTO_CMD_DISABLED): Auto Command Disabled 0x1 (AUTO_CMD12_ENABLED): Auto CMD12 Enable 0x2 (AUTO_CMD23_ENABLED): Auto CMD23 Enable 0x3 (AUTO_CMD_AUTO_SEL): Auto CMD Auto Sel"]
pub type AUTO_CMD_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUTO_CMD_ENABLE` writer - Auto Command Enable This field determines use of Auto Command functions. Note: In SDIO, this field must be set as 00b (Auto Command Disabled). Values: 0x0 (AUTO_CMD_DISABLED): Auto Command Disabled 0x1 (AUTO_CMD12_ENABLED): Auto CMD12 Enable 0x2 (AUTO_CMD23_ENABLED): Auto CMD23 Enable 0x3 (AUTO_CMD_AUTO_SEL): Auto CMD Auto Sel"]
pub type AUTO_CMD_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMD_XFER_SPEC, u8, u8, 2, O>;
#[doc = "Field `BLOCK_COUNT_ENABLE` reader - Block Count Enable This bit is used to enable the Block Count register, which is relevant for multiple block transfers. If this bit is set to 0, the Block Count register is disabled, which is useful in executing an infinite transfer. The Host Driver must set this bit to 0 when ADMA is used. Values: 0x1 (ENABLED): Enable 0x0 (DISABLED): Disable"]
pub type BLOCK_COUNT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `BLOCK_COUNT_ENABLE` writer - Block Count Enable This bit is used to enable the Block Count register, which is relevant for multiple block transfers. If this bit is set to 0, the Block Count register is disabled, which is useful in executing an infinite transfer. The Host Driver must set this bit to 0 when ADMA is used. Values: 0x1 (ENABLED): Enable 0x0 (DISABLED): Disable"]
pub type BLOCK_COUNT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFER_SPEC, bool, O>;
#[doc = "Field `DMA_ENABLE` reader - DMA Enable This bit enables the DMA functionality. If this bit is set to 1, a DMA operation begins when the Host Driver writes to the Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register. Values: 0x1 (ENABLED): DMA Data transfer 0x0 (DISABLED): No data transfer or Non-DMA data transfer"]
pub type DMA_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DMA_ENABLE` writer - DMA Enable This bit enables the DMA functionality. If this bit is set to 1, a DMA operation begins when the Host Driver writes to the Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register. Values: 0x1 (ENABLED): DMA Data transfer 0x0 (DISABLED): No data transfer or Non-DMA data transfer"]
pub type DMA_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_XFER_SPEC, bool, O>;
impl R {
    #[doc = "Bits 24:29 - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CMD_INDEX_R {
        CMD_INDEX_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: 0x3 (ABORT_CMD): Abort 0x2 (RESUME_CMD): Resume 0x1 (SUSPEND_CMD): Suspend 0x0 (NORMAL_CMD): Normal"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CMD_TYPE_R {
        CMD_TYPE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 21 - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: Command using the CMD line Command with no data transfer but using busy signal on the DAT\\[0\\]
line Resume Command Values: 0x0 (NO_DATA): No Data Present 0x1 (DATA): Data Present"]
    #[inline(always)]
    pub fn data_present_sel(&self) -> DATA_PRESENT_SEL_R {
        DATA_PRESENT_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. For the tuning command, this bit must always be set to enable the index check. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_idx_chk_enable(&self) -> CMD_IDX_CHK_ENABLE_R {
        CMD_IDX_CHK_ENABLE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. For the tuning command, this bit must always be set to 1 to enable the CRC check. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_crc_chk_enable(&self) -> CMD_CRC_CHK_ENABLE_R {
        CMD_CRC_CHK_ENABLE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: 0x0 (MAIN): Main Command 0x1 (SUB): Sub Command"]
    #[inline(always)]
    pub fn sub_cmd_flag(&self) -> SUB_CMD_FLAG_R {
        SUB_CMD_FLAG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Response Type Select This bit indicates the type of response expected from the card. Values: 0x0 (NO_RESP): No Response 0x1 (RESP_LEN_136): Response Length 136 0x2 (RESP_LEN_48): Response Length 48 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
    #[inline(always)]
    pub fn resp_type_select(&self) -> RESP_TYPE_SELECT_R {
        RESP_TYPE_SELECT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 8 - Response Interrupt Disable The Host Controller supports response check function to avoid overhead of response error check by the Host driver. Response types of only R1 and R5 can be checked by the Controller. If Host Driver checks the response error, set this bit to 0 and wait for Command Complete Interrupt and then check the response register. If the Host Controller checks the response error, set this bit to 1 and set the Response Error Check Enable bit to 1. The Command Complete Interrupt is disabled by this bit regardless of the Command Complete Signal Enable. Note: During tuning (when the Execute Tuning bit in the Host Control2 register is set), the Command Complete Interrupt is not generated irrespective of the Response Interrupt Disable setting. Values: - 0x0 (ENABLED): Response Interrupt is enabled - 0x1 (DISABLED): Response Interrupt is disabled"]
    #[inline(always)]
    pub fn resp_int_disable(&self) -> RESP_INT_DISABLE_R {
        RESP_INT_DISABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Response Error Check Enable The Host Controller supports response check function to avoid overhead of response error check by Host driver. Response types of only R1 and R5 can be checked by the Controller. If the Host Controller checks the response error, set this bit to 1 and set Response Interrupt Disable to 1. If an error is detected, the Response Error interrupt is generated in the Error Interrupt Status register. Note: - Response error check must not be enabled for any response type other than R1 and R5. - Response check must not be enabled for the tuning command. Values: - 0x0 (DISABLED): Response Error Check is disabled - 0x1 (ENABLED): Response Error Check is enabled"]
    #[inline(always)]
    pub fn resp_err_chk_enable(&self) -> RESP_ERR_CHK_ENABLE_R {
        RESP_ERR_CHK_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Response Type R1/R5 This bit selects either R1 or R5 as a response type when the Response Error Check is selected. Error statuses checked in R1: OUT_OF_RANGE ADDRESS_ERROR BLOCK_LEN_ERROR WP_VIOLATION CARD_IS_LOCKED COM_CRC_ERROR CARD_ECC_FAILED CC_ERROR ERROR Response Flags checked in R5: COM_CRC_ERROR ERROR FUNCTION_NUMBER OUT_OF_RANGE Values: 0x0 (RESP_R1): R1 (Memory) 0x1 (RESP_R5): R5 (SDIO)"]
    #[inline(always)]
    pub fn resp_type(&self) -> RESP_TYPE_R {
        RESP_TYPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi/Single Block Select This bit is set when issuing multiple-block transfer commands using the DAT line. If this bit is set to 0, it is not necessary to set the Block Count register. Values: 0x0 (SINGLE): Single Block 0x1 (MULTI): Multiple Block"]
    #[inline(always)]
    pub fn multi_blk_sel(&self) -> MULTI_BLK_SEL_R {
        MULTI_BLK_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select This bit defines the direction of DAT line data transfers. This bit is set to 1 by the Host Driver to transfer data from the SD/eMMC card to the Host Controller and it is set to 0 for all other commands. Values: 0x1 (READ): Read (Card to Host) 0x0 (WRITE): Write (Host to Card)"]
    #[inline(always)]
    pub fn data_xfer_dir(&self) -> DATA_XFER_DIR_R {
        DATA_XFER_DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Auto Command Enable This field determines use of Auto Command functions. Note: In SDIO, this field must be set as 00b (Auto Command Disabled). Values: 0x0 (AUTO_CMD_DISABLED): Auto Command Disabled 0x1 (AUTO_CMD12_ENABLED): Auto CMD12 Enable 0x2 (AUTO_CMD23_ENABLED): Auto CMD23 Enable 0x3 (AUTO_CMD_AUTO_SEL): Auto CMD Auto Sel"]
    #[inline(always)]
    pub fn auto_cmd_enable(&self) -> AUTO_CMD_ENABLE_R {
        AUTO_CMD_ENABLE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 1 - Block Count Enable This bit is used to enable the Block Count register, which is relevant for multiple block transfers. If this bit is set to 0, the Block Count register is disabled, which is useful in executing an infinite transfer. The Host Driver must set this bit to 0 when ADMA is used. Values: 0x1 (ENABLED): Enable 0x0 (DISABLED): Disable"]
    #[inline(always)]
    pub fn block_count_enable(&self) -> BLOCK_COUNT_ENABLE_R {
        BLOCK_COUNT_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - DMA Enable This bit enables the DMA functionality. If this bit is set to 1, a DMA operation begins when the Host Driver writes to the Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register. Values: 0x1 (ENABLED): DMA Data transfer 0x0 (DISABLED): No data transfer or Non-DMA data transfer"]
    #[inline(always)]
    pub fn dma_enable(&self) -> DMA_ENABLE_R {
        DMA_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:29 - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
    #[inline(always)]
    pub fn cmd_index(&mut self) -> CMD_INDEX_W<24> {
        CMD_INDEX_W::new(self)
    }
    #[doc = "Bits 22:23 - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: 0x3 (ABORT_CMD): Abort 0x2 (RESUME_CMD): Resume 0x1 (SUSPEND_CMD): Suspend 0x0 (NORMAL_CMD): Normal"]
    #[inline(always)]
    pub fn cmd_type(&mut self) -> CMD_TYPE_W<22> {
        CMD_TYPE_W::new(self)
    }
    #[doc = "Bit 21 - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: Command using the CMD line Command with no data transfer but using busy signal on the DAT\\[0\\]
line Resume Command Values: 0x0 (NO_DATA): No Data Present 0x1 (DATA): Data Present"]
    #[inline(always)]
    pub fn data_present_sel(&mut self) -> DATA_PRESENT_SEL_W<21> {
        DATA_PRESENT_SEL_W::new(self)
    }
    #[doc = "Bit 20 - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. For the tuning command, this bit must always be set to enable the index check. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_idx_chk_enable(&mut self) -> CMD_IDX_CHK_ENABLE_W<20> {
        CMD_IDX_CHK_ENABLE_W::new(self)
    }
    #[doc = "Bit 19 - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. For the tuning command, this bit must always be set to 1 to enable the CRC check. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_crc_chk_enable(&mut self) -> CMD_CRC_CHK_ENABLE_W<19> {
        CMD_CRC_CHK_ENABLE_W::new(self)
    }
    #[doc = "Bit 18 - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: 0x0 (MAIN): Main Command 0x1 (SUB): Sub Command"]
    #[inline(always)]
    pub fn sub_cmd_flag(&mut self) -> SUB_CMD_FLAG_W<18> {
        SUB_CMD_FLAG_W::new(self)
    }
    #[doc = "Bits 16:17 - Response Type Select This bit indicates the type of response expected from the card. Values: 0x0 (NO_RESP): No Response 0x1 (RESP_LEN_136): Response Length 136 0x2 (RESP_LEN_48): Response Length 48 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
    #[inline(always)]
    pub fn resp_type_select(&mut self) -> RESP_TYPE_SELECT_W<16> {
        RESP_TYPE_SELECT_W::new(self)
    }
    #[doc = "Bit 8 - Response Interrupt Disable The Host Controller supports response check function to avoid overhead of response error check by the Host driver. Response types of only R1 and R5 can be checked by the Controller. If Host Driver checks the response error, set this bit to 0 and wait for Command Complete Interrupt and then check the response register. If the Host Controller checks the response error, set this bit to 1 and set the Response Error Check Enable bit to 1. The Command Complete Interrupt is disabled by this bit regardless of the Command Complete Signal Enable. Note: During tuning (when the Execute Tuning bit in the Host Control2 register is set), the Command Complete Interrupt is not generated irrespective of the Response Interrupt Disable setting. Values: - 0x0 (ENABLED): Response Interrupt is enabled - 0x1 (DISABLED): Response Interrupt is disabled"]
    #[inline(always)]
    pub fn resp_int_disable(&mut self) -> RESP_INT_DISABLE_W<8> {
        RESP_INT_DISABLE_W::new(self)
    }
    #[doc = "Bit 7 - Response Error Check Enable The Host Controller supports response check function to avoid overhead of response error check by Host driver. Response types of only R1 and R5 can be checked by the Controller. If the Host Controller checks the response error, set this bit to 1 and set Response Interrupt Disable to 1. If an error is detected, the Response Error interrupt is generated in the Error Interrupt Status register. Note: - Response error check must not be enabled for any response type other than R1 and R5. - Response check must not be enabled for the tuning command. Values: - 0x0 (DISABLED): Response Error Check is disabled - 0x1 (ENABLED): Response Error Check is enabled"]
    #[inline(always)]
    pub fn resp_err_chk_enable(&mut self) -> RESP_ERR_CHK_ENABLE_W<7> {
        RESP_ERR_CHK_ENABLE_W::new(self)
    }
    #[doc = "Bit 6 - Response Type R1/R5 This bit selects either R1 or R5 as a response type when the Response Error Check is selected. Error statuses checked in R1: OUT_OF_RANGE ADDRESS_ERROR BLOCK_LEN_ERROR WP_VIOLATION CARD_IS_LOCKED COM_CRC_ERROR CARD_ECC_FAILED CC_ERROR ERROR Response Flags checked in R5: COM_CRC_ERROR ERROR FUNCTION_NUMBER OUT_OF_RANGE Values: 0x0 (RESP_R1): R1 (Memory) 0x1 (RESP_R5): R5 (SDIO)"]
    #[inline(always)]
    pub fn resp_type(&mut self) -> RESP_TYPE_W<6> {
        RESP_TYPE_W::new(self)
    }
    #[doc = "Bit 5 - Multi/Single Block Select This bit is set when issuing multiple-block transfer commands using the DAT line. If this bit is set to 0, it is not necessary to set the Block Count register. Values: 0x0 (SINGLE): Single Block 0x1 (MULTI): Multiple Block"]
    #[inline(always)]
    pub fn multi_blk_sel(&mut self) -> MULTI_BLK_SEL_W<5> {
        MULTI_BLK_SEL_W::new(self)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select This bit defines the direction of DAT line data transfers. This bit is set to 1 by the Host Driver to transfer data from the SD/eMMC card to the Host Controller and it is set to 0 for all other commands. Values: 0x1 (READ): Read (Card to Host) 0x0 (WRITE): Write (Host to Card)"]
    #[inline(always)]
    pub fn data_xfer_dir(&mut self) -> DATA_XFER_DIR_W<4> {
        DATA_XFER_DIR_W::new(self)
    }
    #[doc = "Bits 2:3 - Auto Command Enable This field determines use of Auto Command functions. Note: In SDIO, this field must be set as 00b (Auto Command Disabled). Values: 0x0 (AUTO_CMD_DISABLED): Auto Command Disabled 0x1 (AUTO_CMD12_ENABLED): Auto CMD12 Enable 0x2 (AUTO_CMD23_ENABLED): Auto CMD23 Enable 0x3 (AUTO_CMD_AUTO_SEL): Auto CMD Auto Sel"]
    #[inline(always)]
    pub fn auto_cmd_enable(&mut self) -> AUTO_CMD_ENABLE_W<2> {
        AUTO_CMD_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Block Count Enable This bit is used to enable the Block Count register, which is relevant for multiple block transfers. If this bit is set to 0, the Block Count register is disabled, which is useful in executing an infinite transfer. The Host Driver must set this bit to 0 when ADMA is used. Values: 0x1 (ENABLED): Enable 0x0 (DISABLED): Disable"]
    #[inline(always)]
    pub fn block_count_enable(&mut self) -> BLOCK_COUNT_ENABLE_W<1> {
        BLOCK_COUNT_ENABLE_W::new(self)
    }
    #[doc = "Bit 0 - DMA Enable This bit enables the DMA functionality. If this bit is set to 1, a DMA operation begins when the Host Driver writes to the Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register. Values: 0x1 (ENABLED): DMA Data transfer 0x0 (DISABLED): No data transfer or Non-DMA data transfer"]
    #[inline(always)]
    pub fn dma_enable(&mut self) -> DMA_ENABLE_W<0> {
        DMA_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_xfer](index.html) module"]
pub struct CMD_XFER_SPEC;
impl crate::RegisterSpec for CMD_XFER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_xfer::R](R) reader structure"]
impl crate::Readable for CMD_XFER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd_xfer::W](W) writer structure"]
impl crate::Writable for CMD_XFER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD_XFER to value 0"]
impl crate::Resettable for CMD_XFER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
