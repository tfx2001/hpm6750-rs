#[doc = "Register `EMMC_BOOT_CTRL` reader"]
pub struct R(crate::R<EMMC_BOOT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMMC_BOOT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMMC_BOOT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMMC_BOOT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMMC_BOOT_CTRL` writer"]
pub struct W(crate::W<EMMC_BOOT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMMC_BOOT_CTRL_SPEC>;
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
impl From<crate::W<EMMC_BOOT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMMC_BOOT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_TOUT_CNT` reader - Boot Ack Timeout Counter Value. This value determines the interval by which boot ack timeout (50 ms) is detected when boot ack is expected during boot operation. 0xF : Reserved 0xE : TMCLK x 2^27 ............ 0x1 : TMCLK x 2^14 0x0 : TMCLK x 2^13"]
pub type BOOT_TOUT_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOOT_TOUT_CNT` writer - Boot Ack Timeout Counter Value. This value determines the interval by which boot ack timeout (50 ms) is detected when boot ack is expected during boot operation. 0xF : Reserved 0xE : TMCLK x 2^27 ............ 0x1 : TMCLK x 2^14 0x0 : TMCLK x 2^13"]
pub type BOOT_TOUT_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EMMC_BOOT_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `BOOT_ACK_ENABLE` reader - Boot Acknowledge Enable When this bit set, SDXC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: 0x1 (TRUE): Boot Ack enable 0x0 (FALSE): Boot Ack disable"]
pub type BOOT_ACK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_ACK_ENABLE` writer - Boot Acknowledge Enable When this bit set, SDXC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: 0x1 (TRUE): Boot Ack enable 0x0 (FALSE): Boot Ack disable"]
pub type BOOT_ACK_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMMC_BOOT_CTRL_SPEC, bool, O>;
#[doc = "Field `VALIDATE_BOOT` writer - Validate Mandatory Boot Enable bit This bit is used to validate the MAN_BOOT_EN bit. Values: 0x1 (TRUE): Validate Mandatory boot enable bit 0x0 (FALSE): Ignore Mandatory boot Enable bit"]
pub type VALIDATE_BOOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMMC_BOOT_CTRL_SPEC, bool, O>;
#[doc = "Field `MAN_BOOT_EN` reader - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDXC clears this bit after the boot transfer is completed or terminated. Values: 0x1 (MAN_BOOT_EN): Mandatory boot enable 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
pub type MAN_BOOT_EN_R = crate::BitReader<bool>;
#[doc = "Field `MAN_BOOT_EN` writer - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDXC clears this bit after the boot transfer is completed or terminated. Values: 0x1 (MAN_BOOT_EN): Mandatory boot enable 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
pub type MAN_BOOT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMMC_BOOT_CTRL_SPEC, bool, O>;
#[doc = "Field `CQE_PREFETCH_DISABLE` reader - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
pub type CQE_PREFETCH_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `CQE_PREFETCH_DISABLE` writer - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
pub type CQE_PREFETCH_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMMC_BOOT_CTRL_SPEC, bool, O>;
#[doc = "Field `CQE_ALGO_SEL` reader - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
pub type CQE_ALGO_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CQE_ALGO_SEL` writer - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
pub type CQE_ALGO_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMMC_BOOT_CTRL_SPEC, bool, O>;
#[doc = "Field `ENH_STROBE_ENABLE` reader - Enhanced Strobe Enable This bit instructs SDXC to sample the CMD line using data strobe for HS400 mode. Values: 0x1 (ENH_STB_FOR_CMD): CMD line is sampled using data strobe for HS400 mode 0x0 (NO_STB_FOR_CMD): CMD line is sampled using cclk_rx for HS400 mode"]
pub type ENH_STROBE_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENH_STROBE_ENABLE` writer - Enhanced Strobe Enable This bit instructs SDXC to sample the CMD line using data strobe for HS400 mode. Values: 0x1 (ENH_STB_FOR_CMD): CMD line is sampled using data strobe for HS400 mode 0x0 (NO_STB_FOR_CMD): CMD line is sampled using cclk_rx for HS400 mode"]
pub type ENH_STROBE_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMMC_BOOT_CTRL_SPEC, bool, O>;
#[doc = "Field `EMMC_RST_N_OE` reader - Output Enable control for EMMC Device Reset signal PAD control. This field drived sd_rst_n_oe output of SDXC Values: 0x1 (ENABLE): sd_rst_n_oe is 1 0x0 (DISABLE): sd_rst_n_oe is 0"]
pub type EMMC_RST_N_OE_R = crate::BitReader<bool>;
#[doc = "Field `EMMC_RST_N_OE` writer - Output Enable control for EMMC Device Reset signal PAD control. This field drived sd_rst_n_oe output of SDXC Values: 0x1 (ENABLE): sd_rst_n_oe is 1 0x0 (DISABLE): sd_rst_n_oe is 0"]
pub type EMMC_RST_N_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMMC_BOOT_CTRL_SPEC, bool, O>;
#[doc = "Field `EMMC_RST_N` reader - EMMC Device Reset signal control. This register field controls the sd_rst_n output of SDXC Values: 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
pub type EMMC_RST_N_R = crate::BitReader<bool>;
#[doc = "Field `EMMC_RST_N` writer - EMMC Device Reset signal control. This register field controls the sd_rst_n output of SDXC Values: 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
pub type EMMC_RST_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMMC_BOOT_CTRL_SPEC, bool, O>;
#[doc = "Field `DISABLE_DATA_CRC_CHK` reader - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: 0x1 (DISABLE): DATA CRC check is disabled 0x0 (ENABLE): DATA CRC check is enabled"]
pub type DISABLE_DATA_CRC_CHK_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE_DATA_CRC_CHK` writer - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: 0x1 (DISABLE): DATA CRC check is disabled 0x0 (ENABLE): DATA CRC check is enabled"]
pub type DISABLE_DATA_CRC_CHK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMMC_BOOT_CTRL_SPEC, bool, O>;
#[doc = "Field `CARD_IS_EMMC` reader - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDXC. Values: 0x1 (EMMC_CARD): Card connected to SDXC is an eMMC card 0x0 (NON_EMMC_CARD): Card connected to SDXCis a non-eMMC card"]
pub type CARD_IS_EMMC_R = crate::BitReader<bool>;
#[doc = "Field `CARD_IS_EMMC` writer - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDXC. Values: 0x1 (EMMC_CARD): Card connected to SDXC is an eMMC card 0x0 (NON_EMMC_CARD): Card connected to SDXCis a non-eMMC card"]
pub type CARD_IS_EMMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMMC_BOOT_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 28:31 - Boot Ack Timeout Counter Value. This value determines the interval by which boot ack timeout (50 ms) is detected when boot ack is expected during boot operation. 0xF : Reserved 0xE : TMCLK x 2^27 ............ 0x1 : TMCLK x 2^14 0x0 : TMCLK x 2^13"]
    #[inline(always)]
    pub fn boot_tout_cnt(&self) -> BOOT_TOUT_CNT_R {
        BOOT_TOUT_CNT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Boot Acknowledge Enable When this bit set, SDXC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: 0x1 (TRUE): Boot Ack enable 0x0 (FALSE): Boot Ack disable"]
    #[inline(always)]
    pub fn boot_ack_enable(&self) -> BOOT_ACK_ENABLE_R {
        BOOT_ACK_ENABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 16 - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDXC clears this bit after the boot transfer is completed or terminated. Values: 0x1 (MAN_BOOT_EN): Mandatory boot enable 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
    #[inline(always)]
    pub fn man_boot_en(&self) -> MAN_BOOT_EN_R {
        MAN_BOOT_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
    #[inline(always)]
    pub fn cqe_prefetch_disable(&self) -> CQE_PREFETCH_DISABLE_R {
        CQE_PREFETCH_DISABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
    #[inline(always)]
    pub fn cqe_algo_sel(&self) -> CQE_ALGO_SEL_R {
        CQE_ALGO_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Enhanced Strobe Enable This bit instructs SDXC to sample the CMD line using data strobe for HS400 mode. Values: 0x1 (ENH_STB_FOR_CMD): CMD line is sampled using data strobe for HS400 mode 0x0 (NO_STB_FOR_CMD): CMD line is sampled using cclk_rx for HS400 mode"]
    #[inline(always)]
    pub fn enh_strobe_enable(&self) -> ENH_STROBE_ENABLE_R {
        ENH_STROBE_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Enable control for EMMC Device Reset signal PAD control. This field drived sd_rst_n_oe output of SDXC Values: 0x1 (ENABLE): sd_rst_n_oe is 1 0x0 (DISABLE): sd_rst_n_oe is 0"]
    #[inline(always)]
    pub fn emmc_rst_n_oe(&self) -> EMMC_RST_N_OE_R {
        EMMC_RST_N_OE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - EMMC Device Reset signal control. This register field controls the sd_rst_n output of SDXC Values: 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
    #[inline(always)]
    pub fn emmc_rst_n(&self) -> EMMC_RST_N_R {
        EMMC_RST_N_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: 0x1 (DISABLE): DATA CRC check is disabled 0x0 (ENABLE): DATA CRC check is enabled"]
    #[inline(always)]
    pub fn disable_data_crc_chk(&self) -> DISABLE_DATA_CRC_CHK_R {
        DISABLE_DATA_CRC_CHK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDXC. Values: 0x1 (EMMC_CARD): Card connected to SDXC is an eMMC card 0x0 (NON_EMMC_CARD): Card connected to SDXCis a non-eMMC card"]
    #[inline(always)]
    pub fn card_is_emmc(&self) -> CARD_IS_EMMC_R {
        CARD_IS_EMMC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31 - Boot Ack Timeout Counter Value. This value determines the interval by which boot ack timeout (50 ms) is detected when boot ack is expected during boot operation. 0xF : Reserved 0xE : TMCLK x 2^27 ............ 0x1 : TMCLK x 2^14 0x0 : TMCLK x 2^13"]
    #[inline(always)]
    pub fn boot_tout_cnt(&mut self) -> BOOT_TOUT_CNT_W<28> {
        BOOT_TOUT_CNT_W::new(self)
    }
    #[doc = "Bit 24 - Boot Acknowledge Enable When this bit set, SDXC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: 0x1 (TRUE): Boot Ack enable 0x0 (FALSE): Boot Ack disable"]
    #[inline(always)]
    pub fn boot_ack_enable(&mut self) -> BOOT_ACK_ENABLE_W<24> {
        BOOT_ACK_ENABLE_W::new(self)
    }
    #[doc = "Bit 23 - Validate Mandatory Boot Enable bit This bit is used to validate the MAN_BOOT_EN bit. Values: 0x1 (TRUE): Validate Mandatory boot enable bit 0x0 (FALSE): Ignore Mandatory boot Enable bit"]
    #[inline(always)]
    pub fn validate_boot(&mut self) -> VALIDATE_BOOT_W<23> {
        VALIDATE_BOOT_W::new(self)
    }
    #[doc = "Bit 16 - Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDXC clears this bit after the boot transfer is completed or terminated. Values: 0x1 (MAN_BOOT_EN): Mandatory boot enable 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
    #[inline(always)]
    pub fn man_boot_en(&mut self) -> MAN_BOOT_EN_W<16> {
        MAN_BOOT_EN_W::new(self)
    }
    #[doc = "Bit 10 - Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
    #[inline(always)]
    pub fn cqe_prefetch_disable(&mut self) -> CQE_PREFETCH_DISABLE_W<10> {
        CQE_PREFETCH_DISABLE_W::new(self)
    }
    #[doc = "Bit 9 - Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
    #[inline(always)]
    pub fn cqe_algo_sel(&mut self) -> CQE_ALGO_SEL_W<9> {
        CQE_ALGO_SEL_W::new(self)
    }
    #[doc = "Bit 8 - Enhanced Strobe Enable This bit instructs SDXC to sample the CMD line using data strobe for HS400 mode. Values: 0x1 (ENH_STB_FOR_CMD): CMD line is sampled using data strobe for HS400 mode 0x0 (NO_STB_FOR_CMD): CMD line is sampled using cclk_rx for HS400 mode"]
    #[inline(always)]
    pub fn enh_strobe_enable(&mut self) -> ENH_STROBE_ENABLE_W<8> {
        ENH_STROBE_ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Output Enable control for EMMC Device Reset signal PAD control. This field drived sd_rst_n_oe output of SDXC Values: 0x1 (ENABLE): sd_rst_n_oe is 1 0x0 (DISABLE): sd_rst_n_oe is 0"]
    #[inline(always)]
    pub fn emmc_rst_n_oe(&mut self) -> EMMC_RST_N_OE_W<3> {
        EMMC_RST_N_OE_W::new(self)
    }
    #[doc = "Bit 2 - EMMC Device Reset signal control. This register field controls the sd_rst_n output of SDXC Values: 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
    #[inline(always)]
    pub fn emmc_rst_n(&mut self) -> EMMC_RST_N_W<2> {
        EMMC_RST_N_W::new(self)
    }
    #[doc = "Bit 1 - Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: 0x1 (DISABLE): DATA CRC check is disabled 0x0 (ENABLE): DATA CRC check is enabled"]
    #[inline(always)]
    pub fn disable_data_crc_chk(&mut self) -> DISABLE_DATA_CRC_CHK_W<1> {
        DISABLE_DATA_CRC_CHK_W::new(self)
    }
    #[doc = "Bit 0 - eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDXC. Values: 0x1 (EMMC_CARD): Card connected to SDXC is an eMMC card 0x0 (NON_EMMC_CARD): Card connected to SDXCis a non-eMMC card"]
    #[inline(always)]
    pub fn card_is_emmc(&mut self) -> CARD_IS_EMMC_W<0> {
        CARD_IS_EMMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emmc_boot_ctrl](index.html) module"]
pub struct EMMC_BOOT_CTRL_SPEC;
impl crate::RegisterSpec for EMMC_BOOT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emmc_boot_ctrl::R](R) reader structure"]
impl crate::Readable for EMMC_BOOT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emmc_boot_ctrl::W](W) writer structure"]
impl crate::Writable for EMMC_BOOT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMMC_BOOT_CTRL to value 0"]
impl crate::Resettable for EMMC_BOOT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
