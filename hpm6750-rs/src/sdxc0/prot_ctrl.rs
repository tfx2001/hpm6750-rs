#[doc = "Register `PROT_CTRL` reader"]
pub struct R(crate::R<PROT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROT_CTRL` writer"]
pub struct W(crate::W<PROT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROT_CTRL_SPEC>;
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
impl From<crate::W<PROT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT_XFER_WIDTH` reader - Data Transfer Width For SD/eMMC mode,this bit selects the data transfer width of the Host Controller. The Host Driver sets it to match the data width of the SD/eMMC card. In UHS-II mode, this bit is irrelevant. Values: 0x1 (FOUR_BIT): 4-bit mode 0x0 (ONE_BIT): 1-bit mode"]
pub type DAT_XFER_WIDTH_R = crate::BitReader<bool>;
#[doc = "Field `DAT_XFER_WIDTH` writer - Data Transfer Width For SD/eMMC mode,this bit selects the data transfer width of the Host Controller. The Host Driver sets it to match the data width of the SD/eMMC card. In UHS-II mode, this bit is irrelevant. Values: 0x1 (FOUR_BIT): 4-bit mode 0x0 (ONE_BIT): 1-bit mode"]
pub type DAT_XFER_WIDTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, bool, O>;
#[doc = "Field `HIGH_SPEED_EN` reader - High Speed Enable this bit is used to determine the selection of preset value for High Speed mode. Before setting this bit, the Host Driver checks the High Speed Support in the Capabilities register. Note: SDXC always outputs the sd_cmd_out and sd_dat_out lines at the rising edge of cclk_tx clock irrespective of this bit. Values: 0x1 (HIGH_SPEED): High Speed mode 0x0 (NORMAL_SPEED): Normal Speed mode"]
pub type HIGH_SPEED_EN_R = crate::BitReader<bool>;
#[doc = "Field `HIGH_SPEED_EN` writer - High Speed Enable this bit is used to determine the selection of preset value for High Speed mode. Before setting this bit, the Host Driver checks the High Speed Support in the Capabilities register. Note: SDXC always outputs the sd_cmd_out and sd_dat_out lines at the rising edge of cclk_tx clock irrespective of this bit. Values: 0x1 (HIGH_SPEED): High Speed mode 0x0 (NORMAL_SPEED): Normal Speed mode"]
pub type HIGH_SPEED_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, bool, O>;
#[doc = "Field `DMA_SEL` reader - DMA Select This field is used to select the DMA type. When Host Version 4 Enable is 1 in Host Control 2 register: 0x0 : SDMA is selected 0x1 : Reserved 0x2 : ADMA2 is selected 0x3 : ADMA2 or ADMA3 is selected When Host Version 4 Enable is 0 in Host Control 2 register: 0x0 : SDMA is selected 0x1 : Reserved 0x2 : 32-bit Address ADMA2 is selected 0x3 : 64-bit Address ADMA2 is selected Values: 0x0 (SDMA): SDMA is selected 0x1 (RSVD_BIT): Reserved 0x2 (ADMA2): ADMA2 is selected 0x3 (ADMA2_3): ADMA2 or ADMA3 is selected"]
pub type DMA_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_SEL` writer - DMA Select This field is used to select the DMA type. When Host Version 4 Enable is 1 in Host Control 2 register: 0x0 : SDMA is selected 0x1 : Reserved 0x2 : ADMA2 is selected 0x3 : ADMA2 or ADMA3 is selected When Host Version 4 Enable is 0 in Host Control 2 register: 0x0 : SDMA is selected 0x1 : Reserved 0x2 : 32-bit Address ADMA2 is selected 0x3 : 64-bit Address ADMA2 is selected Values: 0x0 (SDMA): SDMA is selected 0x1 (RSVD_BIT): Reserved 0x2 (ADMA2): ADMA2 is selected 0x3 (ADMA2_3): ADMA2 or ADMA3 is selected"]
pub type DMA_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PROT_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EXT_DAT_XFER` reader - Extended Data Transfer Width This bit controls 8-bit bus width mode of embedded device. Values: 0x1 (EIGHT_BIT): 8-bit Bus Width 0x0 (DEFAULT): Bus Width is selected by the Data Transfer Width"]
pub type EXT_DAT_XFER_R = crate::BitReader<bool>;
#[doc = "Field `EXT_DAT_XFER` writer - Extended Data Transfer Width This bit controls 8-bit bus width mode of embedded device. Values: 0x1 (EIGHT_BIT): 8-bit Bus Width 0x0 (DEFAULT): Bus Width is selected by the Data Transfer Width"]
pub type EXT_DAT_XFER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, bool, O>;
#[doc = "Field `SD_BUS_VOL_VDD1` reader - SD Bus Voltage Select for VDD1/eMMC Bus Voltage Select for VDD These bits enable the Host Driver to select the voltage level for an SD/eMMC card. Before setting this register, the Host Driver checks the Voltage Support bits in the Capabilities register. If an unsupported voltage is selected, the Host System does not supply the SD Bus voltage. The value set in this field is available on the SDXC output signal (sd_vdd1_sel), which is used by the voltage switching circuitry. SD Bus Voltage Select options: 0x7 : 3.3V(Typical) 0x6 : 3.0V(Typical) 0x5 : 1.8V(Typical) for Embedded 0x4 : 0x0 - Reserved eMMC Bus Voltage Select options: 0x7 : 3.3V(Typical) 0x6 : 1.8V(Typical) 0x5 : 1.2V(Typical) 0x4 : 0x0 - Reserved Values: 0x7 (V_3_3): 3.3V (Typ.) 0x6 (V_3_0): 3.0V (Typ.) 0x5 (V_1_8): 1.8V (Typ.) for Embedded 0x4 (RSVD4): Reserved 0x3 (RSVD3): Reserved 0x2 (RSVD2): Reserved 0x1 (RSVD1): Reserved 0x0 (RSVD0): Reserved"]
pub type SD_BUS_VOL_VDD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SD_BUS_VOL_VDD1` writer - SD Bus Voltage Select for VDD1/eMMC Bus Voltage Select for VDD These bits enable the Host Driver to select the voltage level for an SD/eMMC card. Before setting this register, the Host Driver checks the Voltage Support bits in the Capabilities register. If an unsupported voltage is selected, the Host System does not supply the SD Bus voltage. The value set in this field is available on the SDXC output signal (sd_vdd1_sel), which is used by the voltage switching circuitry. SD Bus Voltage Select options: 0x7 : 3.3V(Typical) 0x6 : 3.0V(Typical) 0x5 : 1.8V(Typical) for Embedded 0x4 : 0x0 - Reserved eMMC Bus Voltage Select options: 0x7 : 3.3V(Typical) 0x6 : 1.8V(Typical) 0x5 : 1.2V(Typical) 0x4 : 0x0 - Reserved Values: 0x7 (V_3_3): 3.3V (Typ.) 0x6 (V_3_0): 3.0V (Typ.) 0x5 (V_1_8): 1.8V (Typ.) for Embedded 0x4 (RSVD4): Reserved 0x3 (RSVD3): Reserved 0x2 (RSVD2): Reserved 0x1 (RSVD1): Reserved 0x0 (RSVD0): Reserved"]
pub type SD_BUS_VOL_VDD1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PROT_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `STOP_BG_REQ` reader - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: 0x0 (XFER): Transfer 0x1 (STOP): Stop"]
pub type STOP_BG_REQ_R = crate::BitReader<bool>;
#[doc = "Field `STOP_BG_REQ` writer - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: 0x0 (XFER): Transfer 0x1 (STOP): Stop"]
pub type STOP_BG_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, bool, O>;
#[doc = "Field `CONTINUE_REQ` reader - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: 0x0 (NO_AFFECT): No Affect 0x1 (RESTART): Restart"]
pub type CONTINUE_REQ_R = crate::BitReader<bool>;
#[doc = "Field `CONTINUE_REQ` writer - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: 0x0 (NO_AFFECT): No Affect 0x1 (RESTART): Restart"]
pub type CONTINUE_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, bool, O>;
#[doc = "Field `RD_WAIT_CTRL` reader - Read Wait Control This bit is used to enable the read wait protocol to stop read data using DAT\\[2\\]
line if the card supports read wait. Otherwise, the Host Controller has to stop the card clock to hold the read data. In UHS-II mode, Read Wait is disabled. Values: 0x0 (DISABLE): Disable Read Wait Control 0x1 (ENABLE): Enable Read Wait Control"]
pub type RD_WAIT_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `RD_WAIT_CTRL` writer - Read Wait Control This bit is used to enable the read wait protocol to stop read data using DAT\\[2\\]
line if the card supports read wait. Otherwise, the Host Controller has to stop the card clock to hold the read data. In UHS-II mode, Read Wait is disabled. Values: 0x0 (DISABLE): Disable Read Wait Control 0x1 (ENABLE): Enable Read Wait Control"]
pub type RD_WAIT_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, bool, O>;
#[doc = "Field `INT_AT_BGAP` reader - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: 0x0 (DISABLE): Disabled 0x1 (ENABLE): Enabled"]
pub type INT_AT_BGAP_R = crate::BitReader<bool>;
#[doc = "Field `INT_AT_BGAP` writer - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: 0x0 (DISABLE): Disabled 0x1 (ENABLE): Enabled"]
pub type INT_AT_BGAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, bool, O>;
#[doc = "Field `CARD_INT` reader - Wakeup Event Enable on Card Interrupt This bit enables wakeup event through a Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
pub type CARD_INT_R = crate::BitReader<bool>;
#[doc = "Field `CARD_INT` writer - Wakeup Event Enable on Card Interrupt This bit enables wakeup event through a Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
pub type CARD_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, bool, O>;
#[doc = "Field `CARD_INSERT` reader - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
pub type CARD_INSERT_R = crate::BitReader<bool>;
#[doc = "Field `CARD_INSERT` writer - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
pub type CARD_INSERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, bool, O>;
#[doc = "Field `CARD_REMOVAL` reader - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
pub type CARD_REMOVAL_R = crate::BitReader<bool>;
#[doc = "Field `CARD_REMOVAL` writer - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
pub type CARD_REMOVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PROT_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Data Transfer Width For SD/eMMC mode,this bit selects the data transfer width of the Host Controller. The Host Driver sets it to match the data width of the SD/eMMC card. In UHS-II mode, this bit is irrelevant. Values: 0x1 (FOUR_BIT): 4-bit mode 0x0 (ONE_BIT): 1-bit mode"]
    #[inline(always)]
    pub fn dat_xfer_width(&self) -> DAT_XFER_WIDTH_R {
        DAT_XFER_WIDTH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable this bit is used to determine the selection of preset value for High Speed mode. Before setting this bit, the Host Driver checks the High Speed Support in the Capabilities register. Note: SDXC always outputs the sd_cmd_out and sd_dat_out lines at the rising edge of cclk_tx clock irrespective of this bit. Values: 0x1 (HIGH_SPEED): High Speed mode 0x0 (NORMAL_SPEED): Normal Speed mode"]
    #[inline(always)]
    pub fn high_speed_en(&self) -> HIGH_SPEED_EN_R {
        HIGH_SPEED_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DMA Select This field is used to select the DMA type. When Host Version 4 Enable is 1 in Host Control 2 register: 0x0 : SDMA is selected 0x1 : Reserved 0x2 : ADMA2 is selected 0x3 : ADMA2 or ADMA3 is selected When Host Version 4 Enable is 0 in Host Control 2 register: 0x0 : SDMA is selected 0x1 : Reserved 0x2 : 32-bit Address ADMA2 is selected 0x3 : 64-bit Address ADMA2 is selected Values: 0x0 (SDMA): SDMA is selected 0x1 (RSVD_BIT): Reserved 0x2 (ADMA2): ADMA2 is selected 0x3 (ADMA2_3): ADMA2 or ADMA3 is selected"]
    #[inline(always)]
    pub fn dma_sel(&self) -> DMA_SEL_R {
        DMA_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width This bit controls 8-bit bus width mode of embedded device. Values: 0x1 (EIGHT_BIT): 8-bit Bus Width 0x0 (DEFAULT): Bus Width is selected by the Data Transfer Width"]
    #[inline(always)]
    pub fn ext_dat_xfer(&self) -> EXT_DAT_XFER_R {
        EXT_DAT_XFER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 9:11 - SD Bus Voltage Select for VDD1/eMMC Bus Voltage Select for VDD These bits enable the Host Driver to select the voltage level for an SD/eMMC card. Before setting this register, the Host Driver checks the Voltage Support bits in the Capabilities register. If an unsupported voltage is selected, the Host System does not supply the SD Bus voltage. The value set in this field is available on the SDXC output signal (sd_vdd1_sel), which is used by the voltage switching circuitry. SD Bus Voltage Select options: 0x7 : 3.3V(Typical) 0x6 : 3.0V(Typical) 0x5 : 1.8V(Typical) for Embedded 0x4 : 0x0 - Reserved eMMC Bus Voltage Select options: 0x7 : 3.3V(Typical) 0x6 : 1.8V(Typical) 0x5 : 1.2V(Typical) 0x4 : 0x0 - Reserved Values: 0x7 (V_3_3): 3.3V (Typ.) 0x6 (V_3_0): 3.0V (Typ.) 0x5 (V_1_8): 1.8V (Typ.) for Embedded 0x4 (RSVD4): Reserved 0x3 (RSVD3): Reserved 0x2 (RSVD2): Reserved 0x1 (RSVD1): Reserved 0x0 (RSVD0): Reserved"]
    #[inline(always)]
    pub fn sd_bus_vol_vdd1(&self) -> SD_BUS_VOL_VDD1_R {
        SD_BUS_VOL_VDD1_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 16 - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: 0x0 (XFER): Transfer 0x1 (STOP): Stop"]
    #[inline(always)]
    pub fn stop_bg_req(&self) -> STOP_BG_REQ_R {
        STOP_BG_REQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: 0x0 (NO_AFFECT): No Affect 0x1 (RESTART): Restart"]
    #[inline(always)]
    pub fn continue_req(&self) -> CONTINUE_REQ_R {
        CONTINUE_REQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Read Wait Control This bit is used to enable the read wait protocol to stop read data using DAT\\[2\\]
line if the card supports read wait. Otherwise, the Host Controller has to stop the card clock to hold the read data. In UHS-II mode, Read Wait is disabled. Values: 0x0 (DISABLE): Disable Read Wait Control 0x1 (ENABLE): Enable Read Wait Control"]
    #[inline(always)]
    pub fn rd_wait_ctrl(&self) -> RD_WAIT_CTRL_R {
        RD_WAIT_CTRL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: 0x0 (DISABLE): Disabled 0x1 (ENABLE): Enabled"]
    #[inline(always)]
    pub fn int_at_bgap(&self) -> INT_AT_BGAP_R {
        INT_AT_BGAP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Wakeup Event Enable on Card Interrupt This bit enables wakeup event through a Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn card_int(&self) -> CARD_INT_R {
        CARD_INT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn card_insert(&self) -> CARD_INSERT_R {
        CARD_INSERT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn card_removal(&self) -> CARD_REMOVAL_R {
        CARD_REMOVAL_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Data Transfer Width For SD/eMMC mode,this bit selects the data transfer width of the Host Controller. The Host Driver sets it to match the data width of the SD/eMMC card. In UHS-II mode, this bit is irrelevant. Values: 0x1 (FOUR_BIT): 4-bit mode 0x0 (ONE_BIT): 1-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn dat_xfer_width(&mut self) -> DAT_XFER_WIDTH_W<1> {
        DAT_XFER_WIDTH_W::new(self)
    }
    #[doc = "Bit 2 - High Speed Enable this bit is used to determine the selection of preset value for High Speed mode. Before setting this bit, the Host Driver checks the High Speed Support in the Capabilities register. Note: SDXC always outputs the sd_cmd_out and sd_dat_out lines at the rising edge of cclk_tx clock irrespective of this bit. Values: 0x1 (HIGH_SPEED): High Speed mode 0x0 (NORMAL_SPEED): Normal Speed mode"]
    #[inline(always)]
    #[must_use]
    pub fn high_speed_en(&mut self) -> HIGH_SPEED_EN_W<2> {
        HIGH_SPEED_EN_W::new(self)
    }
    #[doc = "Bits 3:4 - DMA Select This field is used to select the DMA type. When Host Version 4 Enable is 1 in Host Control 2 register: 0x0 : SDMA is selected 0x1 : Reserved 0x2 : ADMA2 is selected 0x3 : ADMA2 or ADMA3 is selected When Host Version 4 Enable is 0 in Host Control 2 register: 0x0 : SDMA is selected 0x1 : Reserved 0x2 : 32-bit Address ADMA2 is selected 0x3 : 64-bit Address ADMA2 is selected Values: 0x0 (SDMA): SDMA is selected 0x1 (RSVD_BIT): Reserved 0x2 (ADMA2): ADMA2 is selected 0x3 (ADMA2_3): ADMA2 or ADMA3 is selected"]
    #[inline(always)]
    #[must_use]
    pub fn dma_sel(&mut self) -> DMA_SEL_W<3> {
        DMA_SEL_W::new(self)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width This bit controls 8-bit bus width mode of embedded device. Values: 0x1 (EIGHT_BIT): 8-bit Bus Width 0x0 (DEFAULT): Bus Width is selected by the Data Transfer Width"]
    #[inline(always)]
    #[must_use]
    pub fn ext_dat_xfer(&mut self) -> EXT_DAT_XFER_W<5> {
        EXT_DAT_XFER_W::new(self)
    }
    #[doc = "Bits 9:11 - SD Bus Voltage Select for VDD1/eMMC Bus Voltage Select for VDD These bits enable the Host Driver to select the voltage level for an SD/eMMC card. Before setting this register, the Host Driver checks the Voltage Support bits in the Capabilities register. If an unsupported voltage is selected, the Host System does not supply the SD Bus voltage. The value set in this field is available on the SDXC output signal (sd_vdd1_sel), which is used by the voltage switching circuitry. SD Bus Voltage Select options: 0x7 : 3.3V(Typical) 0x6 : 3.0V(Typical) 0x5 : 1.8V(Typical) for Embedded 0x4 : 0x0 - Reserved eMMC Bus Voltage Select options: 0x7 : 3.3V(Typical) 0x6 : 1.8V(Typical) 0x5 : 1.2V(Typical) 0x4 : 0x0 - Reserved Values: 0x7 (V_3_3): 3.3V (Typ.) 0x6 (V_3_0): 3.0V (Typ.) 0x5 (V_1_8): 1.8V (Typ.) for Embedded 0x4 (RSVD4): Reserved 0x3 (RSVD3): Reserved 0x2 (RSVD2): Reserved 0x1 (RSVD1): Reserved 0x0 (RSVD0): Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn sd_bus_vol_vdd1(&mut self) -> SD_BUS_VOL_VDD1_W<9> {
        SD_BUS_VOL_VDD1_W::new(self)
    }
    #[doc = "Bit 16 - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: 0x0 (XFER): Transfer 0x1 (STOP): Stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop_bg_req(&mut self) -> STOP_BG_REQ_W<16> {
        STOP_BG_REQ_W::new(self)
    }
    #[doc = "Bit 17 - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: 0x0 (NO_AFFECT): No Affect 0x1 (RESTART): Restart"]
    #[inline(always)]
    #[must_use]
    pub fn continue_req(&mut self) -> CONTINUE_REQ_W<17> {
        CONTINUE_REQ_W::new(self)
    }
    #[doc = "Bit 18 - Read Wait Control This bit is used to enable the read wait protocol to stop read data using DAT\\[2\\]
line if the card supports read wait. Otherwise, the Host Controller has to stop the card clock to hold the read data. In UHS-II mode, Read Wait is disabled. Values: 0x0 (DISABLE): Disable Read Wait Control 0x1 (ENABLE): Enable Read Wait Control"]
    #[inline(always)]
    #[must_use]
    pub fn rd_wait_ctrl(&mut self) -> RD_WAIT_CTRL_W<18> {
        RD_WAIT_CTRL_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: 0x0 (DISABLE): Disabled 0x1 (ENABLE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn int_at_bgap(&mut self) -> INT_AT_BGAP_W<19> {
        INT_AT_BGAP_W::new(self)
    }
    #[doc = "Bit 24 - Wakeup Event Enable on Card Interrupt This bit enables wakeup event through a Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn card_int(&mut self) -> CARD_INT_W<24> {
        CARD_INT_W::new(self)
    }
    #[doc = "Bit 25 - Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn card_insert(&mut self) -> CARD_INSERT_W<25> {
        CARD_INSERT_W::new(self)
    }
    #[doc = "Bit 26 - Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal(&mut self) -> CARD_REMOVAL_W<26> {
        CARD_REMOVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prot_ctrl](index.html) module"]
pub struct PROT_CTRL_SPEC;
impl crate::RegisterSpec for PROT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prot_ctrl::R](R) reader structure"]
impl crate::Readable for PROT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prot_ctrl::W](W) writer structure"]
impl crate::Writable for PROT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROT_CTRL to value 0"]
impl crate::Resettable for PROT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
