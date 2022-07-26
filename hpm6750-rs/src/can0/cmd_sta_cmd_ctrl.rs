#[doc = "Register `CMD_STA_CMD_CTRL` reader"]
pub struct R(crate::R<CMD_STA_CMD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_STA_CMD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_STA_CMD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_STA_CMD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD_STA_CMD_CTRL` writer"]
pub struct W(crate::W<CMD_STA_CMD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_STA_CMD_CTRL_SPEC>;
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
impl From<crate::W<CMD_STA_CMD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_STA_CMD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SACK` reader - Self-ACKnowledge 0 – no self-ACK 1 – self-ACK when LBME=1"]
pub type SACK_R = crate::BitReader<bool>;
#[doc = "Field `SACK` writer - Self-ACKnowledge 0 – no self-ACK 1 – self-ACK when LBME=1"]
pub type SACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `ROM` reader - Receive buffer Overflow Mode In case of a full RBUF when a new message is received, then ROM selects the following: 1 – The new message will not be stored. 0 – The oldest message will be overwritten."]
pub type ROM_R = crate::BitReader<bool>;
#[doc = "Field `ROM` writer - Receive buffer Overflow Mode In case of a full RBUF when a new message is received, then ROM selects the following: 1 – The new message will not be stored. 0 – The oldest message will be overwritten."]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `ROV` reader - Receive buffer OVerflow 1 – Overflow. At least one message is lost. 0 – No Overflow. ROV is cleared by setting RREL=1."]
pub type ROV_R = crate::BitReader<bool>;
#[doc = "Field `RREL` reader - Receive buffer RELease The host controller has read the actual RB slot and releases it. Afterwards the CAN-CTRL core points to the next RB slot. RSTAT gets updated. 1 – Release: The host has read the RB. 0 – No release"]
pub type RREL_R = crate::BitReader<bool>;
#[doc = "Field `RREL` writer - Receive buffer RELease The host controller has read the actual RB slot and releases it. Afterwards the CAN-CTRL core points to the next RB slot. RSTAT gets updated. 1 – Release: The host has read the RB. 0 – No release"]
pub type RREL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `RBALL` reader - Receive Buffer stores ALL data frames 0 – normal operation 1 – RB stores correct data frames as well as data frames with error"]
pub type RBALL_R = crate::BitReader<bool>;
#[doc = "Field `RBALL` writer - Receive Buffer stores ALL data frames 0 – normal operation 1 – RB stores correct data frames as well as data frames with error"]
pub type RBALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `RSTAT` reader - Receive buffer STATus 00 - empty 01 - > empty and < almost full (AFWL) 10 - \u{f0b3} almost full (programmable threshold by AFWL) but not full and no overflow 11 - full (stays set in case of overflow – for overflow signaling see ROV)"]
pub type RSTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FD_ISO` reader - CAN FD ISO mode 0 - Bosch CAN FD (non-ISO) mode 1 - ISO CAN FD mode (ISO 11898-1:2015) ISO CAN FD mode has a different CRC initialization value and an additional stuff bit count. Both modes are incompatible and must not be mixed in one CAN network. This bit has no impact to CAN 2.0B. This bit is only writeable if RESET=1."]
pub type FD_ISO_R = crate::BitReader<bool>;
#[doc = "Field `FD_ISO` writer - CAN FD ISO mode 0 - Bosch CAN FD (non-ISO) mode 1 - ISO CAN FD mode (ISO 11898-1:2015) ISO CAN FD mode has a different CRC initialization value and an additional stuff bit count. Both modes are incompatible and must not be mixed in one CAN network. This bit has no impact to CAN 2.0B. This bit is only writeable if RESET=1."]
pub type FD_ISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `TSNEXT` reader - Transmit buffer Secondary NEXT 0 - no action 1 - STB slot filled, select next slot. After all frame bytes are written to the TBUF registers, the host controller has to set TSNEXT to signal that this slot has been filled. Then the CAN-CTRL core connects the TBUF registers to the next slot. Once a slot is marked as filled a transmission can be started using TSONE or TSALL. It is possible to set TSNEXT and TSONE or TSALL together in one write access. TSNEXT has to be set by the host controller and is automatically reset by the CAN-CTRL core immediately after it was set. Setting TSNEXT is meaningless if TBSEL=0. In this case TSNEXT is ignored and automatically cleared. It does not do any harm. If all slots of the STB are filled, TSNEXT stays set until a slot becomes free. TSNEXT has no meaning in TTCAN mode and is fixed to 0."]
pub type TSNEXT_R = crate::BitReader<bool>;
#[doc = "Field `TSNEXT` writer - Transmit buffer Secondary NEXT 0 - no action 1 - STB slot filled, select next slot. After all frame bytes are written to the TBUF registers, the host controller has to set TSNEXT to signal that this slot has been filled. Then the CAN-CTRL core connects the TBUF registers to the next slot. Once a slot is marked as filled a transmission can be started using TSONE or TSALL. It is possible to set TSNEXT and TSONE or TSALL together in one write access. TSNEXT has to be set by the host controller and is automatically reset by the CAN-CTRL core immediately after it was set. Setting TSNEXT is meaningless if TBSEL=0. In this case TSNEXT is ignored and automatically cleared. It does not do any harm. If all slots of the STB are filled, TSNEXT stays set until a slot becomes free. TSNEXT has no meaning in TTCAN mode and is fixed to 0."]
pub type TSNEXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `TSMODE` reader - Transmit buffer Secondary operation MODE 0 - FIFO mode 1 - priority decision mode In FIFO mode frames are transmitted in the order in that they are written into the STB. In priority decision mode the frame with the highest priority in the STB is automatically transmitted first. The ID of a frame is used for the priority decision. A lower ID means a higher priority of a frame. A frame in the PTB has always the highest priority regardless of the ID. TSMODE shall be switched only if the STB if empty"]
pub type TSMODE_R = crate::BitReader<bool>;
#[doc = "Field `TSMODE` writer - Transmit buffer Secondary operation MODE 0 - FIFO mode 1 - priority decision mode In FIFO mode frames are transmitted in the order in that they are written into the STB. In priority decision mode the frame with the highest priority in the STB is automatically transmitted first. The ID of a frame is used for the priority decision. A lower ID means a higher priority of a frame. A frame in the PTB has always the highest priority regardless of the ID. TSMODE shall be switched only if the STB if empty"]
pub type TSMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `TTTBM` reader - TTCAN Transmit Buffer Mode If TTEN=0 then TTTBM is ignored, otherwise the following is valid: 0 - separate PTB and STB, behavior defined by TSMODE 1 - full TTCAN support: buffer slots selectable by TBPTR and TTPTR For event-driven CAN communication (TTEN=0), the system provides PTB and STB and the behavior of the STB is defined by TSMODE. Then TTTBM is ignored. For time-triggered CAN communication (TTEN=1) with full support of all features including time-triggered transmissions, TTTBM=1 needs to be chosen. Then the all TB slots are addressable using TTPTR and TBPTR. For time-triggered CAN communication (TTEN=1) with only support of reception timestamps, TTTBM=0 can be chosen. Then the transmit buffer acts as in event-driven mode and the behavior can be selected by TSMODE. TTTBM shall be switched only if the TBUF is empty."]
pub type TTTBM_R = crate::BitReader<bool>;
#[doc = "Field `TTTBM` writer - TTCAN Transmit Buffer Mode If TTEN=0 then TTTBM is ignored, otherwise the following is valid: 0 - separate PTB and STB, behavior defined by TSMODE 1 - full TTCAN support: buffer slots selectable by TBPTR and TTPTR For event-driven CAN communication (TTEN=0), the system provides PTB and STB and the behavior of the STB is defined by TSMODE. Then TTTBM is ignored. For time-triggered CAN communication (TTEN=1) with full support of all features including time-triggered transmissions, TTTBM=1 needs to be chosen. Then the all TB slots are addressable using TTPTR and TBPTR. For time-triggered CAN communication (TTEN=1) with only support of reception timestamps, TTTBM=0 can be chosen. Then the transmit buffer acts as in event-driven mode and the behavior can be selected by TSMODE. TTTBM shall be switched only if the TBUF is empty."]
pub type TTTBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `TSSTAT` reader - Transmission Secondary STATus bits If TTEN=0 or TTTBM=0: 00 – STB is empty 01 – STB is less than or equal to half full 10 – STB is more than half full 11 – STB is full If the STB is disabled using STB_DISABLE, then TSSTAT=00. If TTEN=1 and TTTBM=1: 00 – PTB and STB are empty 01 – PTB and STB are not empty and not full 11 – PTB and STB are full"]
pub type TSSTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TBSEL` reader - Transmit Buffer Select Selects the transmit buffer to be loaded with a message. Use the TBUF registers for access. TBSEL needs to be stable all the time the TBUF registers are written and when TSNEXT is set. 0 - PTB (high-priority buffer) 1 - STB The bit will be reset to the hardware reset value if (TTEN=1 and TTTBM=1)"]
pub type TBSEL_R = crate::BitReader<bool>;
#[doc = "Field `TBSEL` writer - Transmit Buffer Select Selects the transmit buffer to be loaded with a message. Use the TBUF registers for access. TBSEL needs to be stable all the time the TBUF registers are written and when TSNEXT is set. 0 - PTB (high-priority buffer) 1 - STB The bit will be reset to the hardware reset value if (TTEN=1 and TTTBM=1)"]
pub type TBSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `LOM` reader - Listen Only Mode 0 - Disabled 1 - Enabled LOM cannot be set if TPE, TSONE or TSALL is set. No transmission can be started if LOM is enabled and LBME is disabled. LOM=1 and LBME=0 disables all transmissions. LOM=1 and LBME=1 disables the ACK for received frames and error frames, but enables the transmission of own frames."]
pub type LOM_R = crate::BitReader<bool>;
#[doc = "Field `LOM` writer - Listen Only Mode 0 - Disabled 1 - Enabled LOM cannot be set if TPE, TSONE or TSALL is set. No transmission can be started if LOM is enabled and LBME is disabled. LOM=1 and LBME=0 disables all transmissions. LOM=1 and LBME=1 disables the ACK for received frames and error frames, but enables the transmission of own frames."]
pub type LOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `STBY` reader - Transceiver Standby Mode 0 - Disabled 1 - Enabled This register bit is connected to the output signal stby which can be used to control a standby mode of a transceiver. STBY cannot be set to 1 if TPE=1, TSONE=1 or TSALL=1. If the host sets STBY to 0 then the host needs to wait for the time required by the transceiver to start up before the host requests a new transmission."]
pub type STBY_R = crate::BitReader<bool>;
#[doc = "Field `STBY` writer - Transceiver Standby Mode 0 - Disabled 1 - Enabled This register bit is connected to the output signal stby which can be used to control a standby mode of a transceiver. STBY cannot be set to 1 if TPE=1, TSONE=1 or TSALL=1. If the host sets STBY to 0 then the host needs to wait for the time required by the transceiver to start up before the host requests a new transmission."]
pub type STBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `TPE` reader - Transmit Primary Enable 1 - Transmission enable for the message in the high-priority PTB 0 - No transmission for the PTB If TPE is set, the message from the PTB will be transmitted at the next possible transmit position. A started transmission from the STB will be completed before, but pending new messages are delayed until the PTB message has been transmitted. TPE stays set until the message has been transmitted successfully or it is aborted using TPA. The host controller can set TPE to 1 but can not reset it to 0. This would only be possible using TPA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
pub type TPE_R = crate::BitReader<bool>;
#[doc = "Field `TPE` writer - Transmit Primary Enable 1 - Transmission enable for the message in the high-priority PTB 0 - No transmission for the PTB If TPE is set, the message from the PTB will be transmitted at the next possible transmit position. A started transmission from the STB will be completed before, but pending new messages are delayed until the PTB message has been transmitted. TPE stays set until the message has been transmitted successfully or it is aborted using TPA. The host controller can set TPE to 1 but can not reset it to 0. This would only be possible using TPA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
pub type TPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `TPA` reader - Transmit Primary Abort 1 – Aborts a transmission from PTB which has been requested by TPE=1 but not started yet. (The data bytes of the message remains in the PTB.) 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TPA automatically de-asserts TPE. The host controller can set TPA to 1 but can not reset it to 0. During the short time while the CAN-CTRL core resets the bit, it cannot be set by the host. The bit will be reset to the hardware reset value if RESET=1 or (TTEN=1 and TTTBM=1). TPA should not be set simultaneously with TPE."]
pub type TPA_R = crate::BitReader<bool>;
#[doc = "Field `TPA` writer - Transmit Primary Abort 1 – Aborts a transmission from PTB which has been requested by TPE=1 but not started yet. (The data bytes of the message remains in the PTB.) 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TPA automatically de-asserts TPE. The host controller can set TPA to 1 but can not reset it to 0. During the short time while the CAN-CTRL core resets the bit, it cannot be set by the host. The bit will be reset to the hardware reset value if RESET=1 or (TTEN=1 and TTTBM=1). TPA should not be set simultaneously with TPE."]
pub type TPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `TSONE` reader - Transmit Secondary ONE frame 1 – Transmission enable of one in the STB. In FIFO mode this is the oldest message and in priority mode this is the one with the highest priority. TSONE in priority mode is difficult to handle, because it is not always clear which message will be transmitted if new messages are written to the STB meanwhile. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSONE stays set until the message has been transmitted successfully or it is aborted using TSA. The host controller can set TSONE to 1 but can not reset it to 0. This would only be possible using TSA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
pub type TSONE_R = crate::BitReader<bool>;
#[doc = "Field `TSONE` writer - Transmit Secondary ONE frame 1 – Transmission enable of one in the STB. In FIFO mode this is the oldest message and in priority mode this is the one with the highest priority. TSONE in priority mode is difficult to handle, because it is not always clear which message will be transmitted if new messages are written to the STB meanwhile. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSONE stays set until the message has been transmitted successfully or it is aborted using TSA. The host controller can set TSONE to 1 but can not reset it to 0. This would only be possible using TSA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
pub type TSONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `TSALL` reader - Transmit Secondary ALL frames 1 – Transmission enable of all messages in the STB. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSALL stays set until all messages have been transmitted successfully or they are aborted using TSA. The host controller can set TSALL to 1 but can not reset it to 0. This would only be possible using TSA and aborting the messages. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1). If during a transmission the STB is loaded with a new frame then the new frame will be transmitted too. In other words: a transmission initiated by TSALL is finished when the STB becomes empty."]
pub type TSALL_R = crate::BitReader<bool>;
#[doc = "Field `TSALL` writer - Transmit Secondary ALL frames 1 – Transmission enable of all messages in the STB. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSALL stays set until all messages have been transmitted successfully or they are aborted using TSA. The host controller can set TSALL to 1 but can not reset it to 0. This would only be possible using TSA and aborting the messages. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1). If during a transmission the STB is loaded with a new frame then the new frame will be transmitted too. In other words: a transmission initiated by TSALL is finished when the STB becomes empty."]
pub type TSALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `TSA` reader - Transmit Secondary Abort 1 – Aborts a transmission from STB which has been requested but not started yet. For a TSONE transmission, only one frame is aborted while for a TSALL Transmission, all frames are aborted. One or all message slots will be released which updates TSSTAT. All aborted messages are lost because they are not accessible any more. If in priority mode a TSONE transmission is aborted, then it is not clear which frame will be aborted if new frames are written to the STB meanwhile. 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TSA,automatically de-asserts TSONE or TSALL respectively. The host controller can set TSA to 1 but can not reset it to 0. The bit will be reset to the hardware reset value if RESET=1. TSA should not be set simultaneously with TSONE or TSALL."]
pub type TSA_R = crate::BitReader<bool>;
#[doc = "Field `TSA` writer - Transmit Secondary Abort 1 – Aborts a transmission from STB which has been requested but not started yet. For a TSONE transmission, only one frame is aborted while for a TSALL Transmission, all frames are aborted. One or all message slots will be released which updates TSSTAT. All aborted messages are lost because they are not accessible any more. If in priority mode a TSONE transmission is aborted, then it is not clear which frame will be aborted if new frames are written to the STB meanwhile. 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TSA,automatically de-asserts TSONE or TSALL respectively. The host controller can set TSA to 1 but can not reset it to 0. The bit will be reset to the hardware reset value if RESET=1. TSA should not be set simultaneously with TSONE or TSALL."]
pub type TSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `RESET` reader - RESET request bit 1 - The host controller performs a local reset of CAN-CTRL. 0 - no local reset of CAN-CTRLThe some register (e.g for node configuration) can only be modified if RESET=1. Bit RESET forces several components to a reset state. RESET is automatically set if the node enters “bus off” state. Note that a CAN node will participate in CAN communication after RESET is switched to 0after 11 CAN bit times. This delay is required by the CAN standard (bus idle time).If RESET is set to 1 and immediately set to 0, then it takes some time until RESET can beread as 0 and becomes inactive. The reason is clock domain crossing from host to CAN clockdomain. RESET is held active as long as needed depending on the relation between host andCAN clock."]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - RESET request bit 1 - The host controller performs a local reset of CAN-CTRL. 0 - no local reset of CAN-CTRLThe some register (e.g for node configuration) can only be modified if RESET=1. Bit RESET forces several components to a reset state. RESET is automatically set if the node enters “bus off” state. Note that a CAN node will participate in CAN communication after RESET is switched to 0after 11 CAN bit times. This delay is required by the CAN standard (bus idle time).If RESET is set to 1 and immediately set to 0, then it takes some time until RESET can beread as 0 and becomes inactive. The reason is clock domain crossing from host to CAN clockdomain. RESET is held active as long as needed depending on the relation between host andCAN clock."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `LBME` reader - Loop Back Mode, External 0 - Disabled 1 - EnabledLBME should not be enabled while a transmission is active"]
pub type LBME_R = crate::BitReader<bool>;
#[doc = "Field `LBME` writer - Loop Back Mode, External 0 - Disabled 1 - EnabledLBME should not be enabled while a transmission is active"]
pub type LBME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `LBMI` reader - Loop Back Mode, Internal 0 - Disabled1 - EnabledLBMI should not be enabled while a transmission is active."]
pub type LBMI_R = crate::BitReader<bool>;
#[doc = "Field `LBMI` writer - Loop Back Mode, Internal 0 - Disabled1 - EnabledLBMI should not be enabled while a transmission is active."]
pub type LBMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `TPSS` reader - Transmission Primary Single Shot mode for PTB 0 - Disabled 1 - Enabled"]
pub type TPSS_R = crate::BitReader<bool>;
#[doc = "Field `TPSS` writer - Transmission Primary Single Shot mode for PTB 0 - Disabled 1 - Enabled"]
pub type TPSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `TSSS` reader - Transmission Secondary Single Shot mode for STB 0 - Disabled 1 - Enabled"]
pub type TSSS_R = crate::BitReader<bool>;
#[doc = "Field `TSSS` writer - Transmission Secondary Single Shot mode for STB 0 - Disabled 1 - Enabled"]
pub type TSSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
#[doc = "Field `RACTIVE` reader - Reception ACTIVE (Receive Status bit) 1 - The controller is currently receiving a frame. 0 - No receive activity."]
pub type RACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `TACTIVE` reader - Transmission ACTIVE (Transmit Status bit) 1 - The controller is currently transmitting a frame. 0 - No transmit activity."]
pub type TACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `BUSOFF` reader - Bus Off (Bus Status bit) 1 - The controller status is “bus off”. 0 - The controller status is “bus on”. Writing a 1 to BUSOFF will reset TECNT and RECNT. This should be done only for debugging. See Chapter 3.9.10.6 for details."]
pub type BUSOFF_R = crate::BitReader<bool>;
#[doc = "Field `BUSOFF` writer - Bus Off (Bus Status bit) 1 - The controller status is “bus off”. 0 - The controller status is “bus on”. Writing a 1 to BUSOFF will reset TECNT and RECNT. This should be done only for debugging. See Chapter 3.9.10.6 for details."]
pub type BUSOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_STA_CMD_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Self-ACKnowledge 0 – no self-ACK 1 – self-ACK when LBME=1"]
    #[inline(always)]
    pub fn sack(&self) -> SACK_R {
        SACK_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Receive buffer Overflow Mode In case of a full RBUF when a new message is received, then ROM selects the following: 1 – The new message will not be stored. 0 – The oldest message will be overwritten."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - Receive buffer OVerflow 1 – Overflow. At least one message is lost. 0 – No Overflow. ROV is cleared by setting RREL=1."]
    #[inline(always)]
    pub fn rov(&self) -> ROV_R {
        ROV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - Receive buffer RELease The host controller has read the actual RB slot and releases it. Afterwards the CAN-CTRL core points to the next RB slot. RSTAT gets updated. 1 – Release: The host has read the RB. 0 – No release"]
    #[inline(always)]
    pub fn rrel(&self) -> RREL_R {
        RREL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive Buffer stores ALL data frames 0 – normal operation 1 – RB stores correct data frames as well as data frames with error"]
    #[inline(always)]
    pub fn rball(&self) -> RBALL_R {
        RBALL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Receive buffer STATus 00 - empty 01 - > empty and < almost full (AFWL) 10 - \u{f0b3} almost full (programmable threshold by AFWL) but not full and no overflow 11 - full (stays set in case of overflow – for overflow signaling see ROV)"]
    #[inline(always)]
    pub fn rstat(&self) -> RSTAT_R {
        RSTAT_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 23 - CAN FD ISO mode 0 - Bosch CAN FD (non-ISO) mode 1 - ISO CAN FD mode (ISO 11898-1:2015) ISO CAN FD mode has a different CRC initialization value and an additional stuff bit count. Both modes are incompatible and must not be mixed in one CAN network. This bit has no impact to CAN 2.0B. This bit is only writeable if RESET=1."]
    #[inline(always)]
    pub fn fd_iso(&self) -> FD_ISO_R {
        FD_ISO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmit buffer Secondary NEXT 0 - no action 1 - STB slot filled, select next slot. After all frame bytes are written to the TBUF registers, the host controller has to set TSNEXT to signal that this slot has been filled. Then the CAN-CTRL core connects the TBUF registers to the next slot. Once a slot is marked as filled a transmission can be started using TSONE or TSALL. It is possible to set TSNEXT and TSONE or TSALL together in one write access. TSNEXT has to be set by the host controller and is automatically reset by the CAN-CTRL core immediately after it was set. Setting TSNEXT is meaningless if TBSEL=0. In this case TSNEXT is ignored and automatically cleared. It does not do any harm. If all slots of the STB are filled, TSNEXT stays set until a slot becomes free. TSNEXT has no meaning in TTCAN mode and is fixed to 0."]
    #[inline(always)]
    pub fn tsnext(&self) -> TSNEXT_R {
        TSNEXT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit buffer Secondary operation MODE 0 - FIFO mode 1 - priority decision mode In FIFO mode frames are transmitted in the order in that they are written into the STB. In priority decision mode the frame with the highest priority in the STB is automatically transmitted first. The ID of a frame is used for the priority decision. A lower ID means a higher priority of a frame. A frame in the PTB has always the highest priority regardless of the ID. TSMODE shall be switched only if the STB if empty"]
    #[inline(always)]
    pub fn tsmode(&self) -> TSMODE_R {
        TSMODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - TTCAN Transmit Buffer Mode If TTEN=0 then TTTBM is ignored, otherwise the following is valid: 0 - separate PTB and STB, behavior defined by TSMODE 1 - full TTCAN support: buffer slots selectable by TBPTR and TTPTR For event-driven CAN communication (TTEN=0), the system provides PTB and STB and the behavior of the STB is defined by TSMODE. Then TTTBM is ignored. For time-triggered CAN communication (TTEN=1) with full support of all features including time-triggered transmissions, TTTBM=1 needs to be chosen. Then the all TB slots are addressable using TTPTR and TBPTR. For time-triggered CAN communication (TTEN=1) with only support of reception timestamps, TTTBM=0 can be chosen. Then the transmit buffer acts as in event-driven mode and the behavior can be selected by TSMODE. TTTBM shall be switched only if the TBUF is empty."]
    #[inline(always)]
    pub fn tttbm(&self) -> TTTBM_R {
        TTTBM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Transmission Secondary STATus bits If TTEN=0 or TTTBM=0: 00 – STB is empty 01 – STB is less than or equal to half full 10 – STB is more than half full 11 – STB is full If the STB is disabled using STB_DISABLE, then TSSTAT=00. If TTEN=1 and TTTBM=1: 00 – PTB and STB are empty 01 – PTB and STB are not empty and not full 11 – PTB and STB are full"]
    #[inline(always)]
    pub fn tsstat(&self) -> TSSTAT_R {
        TSSTAT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 15 - Transmit Buffer Select Selects the transmit buffer to be loaded with a message. Use the TBUF registers for access. TBSEL needs to be stable all the time the TBUF registers are written and when TSNEXT is set. 0 - PTB (high-priority buffer) 1 - STB The bit will be reset to the hardware reset value if (TTEN=1 and TTTBM=1)"]
    #[inline(always)]
    pub fn tbsel(&self) -> TBSEL_R {
        TBSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Listen Only Mode 0 - Disabled 1 - Enabled LOM cannot be set if TPE, TSONE or TSALL is set. No transmission can be started if LOM is enabled and LBME is disabled. LOM=1 and LBME=0 disables all transmissions. LOM=1 and LBME=1 disables the ACK for received frames and error frames, but enables the transmission of own frames."]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Transceiver Standby Mode 0 - Disabled 1 - Enabled This register bit is connected to the output signal stby which can be used to control a standby mode of a transceiver. STBY cannot be set to 1 if TPE=1, TSONE=1 or TSALL=1. If the host sets STBY to 0 then the host needs to wait for the time required by the transceiver to start up before the host requests a new transmission."]
    #[inline(always)]
    pub fn stby(&self) -> STBY_R {
        STBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Primary Enable 1 - Transmission enable for the message in the high-priority PTB 0 - No transmission for the PTB If TPE is set, the message from the PTB will be transmitted at the next possible transmit position. A started transmission from the STB will be completed before, but pending new messages are delayed until the PTB message has been transmitted. TPE stays set until the message has been transmitted successfully or it is aborted using TPA. The host controller can set TPE to 1 but can not reset it to 0. This would only be possible using TPA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Primary Abort 1 – Aborts a transmission from PTB which has been requested by TPE=1 but not started yet. (The data bytes of the message remains in the PTB.) 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TPA automatically de-asserts TPE. The host controller can set TPA to 1 but can not reset it to 0. During the short time while the CAN-CTRL core resets the bit, it cannot be set by the host. The bit will be reset to the hardware reset value if RESET=1 or (TTEN=1 and TTTBM=1). TPA should not be set simultaneously with TPE."]
    #[inline(always)]
    pub fn tpa(&self) -> TPA_R {
        TPA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Secondary ONE frame 1 – Transmission enable of one in the STB. In FIFO mode this is the oldest message and in priority mode this is the one with the highest priority. TSONE in priority mode is difficult to handle, because it is not always clear which message will be transmitted if new messages are written to the STB meanwhile. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSONE stays set until the message has been transmitted successfully or it is aborted using TSA. The host controller can set TSONE to 1 but can not reset it to 0. This would only be possible using TSA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
    #[inline(always)]
    pub fn tsone(&self) -> TSONE_R {
        TSONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Secondary ALL frames 1 – Transmission enable of all messages in the STB. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSALL stays set until all messages have been transmitted successfully or they are aborted using TSA. The host controller can set TSALL to 1 but can not reset it to 0. This would only be possible using TSA and aborting the messages. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1). If during a transmission the STB is loaded with a new frame then the new frame will be transmitted too. In other words: a transmission initiated by TSALL is finished when the STB becomes empty."]
    #[inline(always)]
    pub fn tsall(&self) -> TSALL_R {
        TSALL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Secondary Abort 1 – Aborts a transmission from STB which has been requested but not started yet. For a TSONE transmission, only one frame is aborted while for a TSALL Transmission, all frames are aborted. One or all message slots will be released which updates TSSTAT. All aborted messages are lost because they are not accessible any more. If in priority mode a TSONE transmission is aborted, then it is not clear which frame will be aborted if new frames are written to the STB meanwhile. 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TSA,automatically de-asserts TSONE or TSALL respectively. The host controller can set TSA to 1 but can not reset it to 0. The bit will be reset to the hardware reset value if RESET=1. TSA should not be set simultaneously with TSONE or TSALL."]
    #[inline(always)]
    pub fn tsa(&self) -> TSA_R {
        TSA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - RESET request bit 1 - The host controller performs a local reset of CAN-CTRL. 0 - no local reset of CAN-CTRLThe some register (e.g for node configuration) can only be modified if RESET=1. Bit RESET forces several components to a reset state. RESET is automatically set if the node enters “bus off” state. Note that a CAN node will participate in CAN communication after RESET is switched to 0after 11 CAN bit times. This delay is required by the CAN standard (bus idle time).If RESET is set to 1 and immediately set to 0, then it takes some time until RESET can beread as 0 and becomes inactive. The reason is clock domain crossing from host to CAN clockdomain. RESET is held active as long as needed depending on the relation between host andCAN clock."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Loop Back Mode, External 0 - Disabled 1 - EnabledLBME should not be enabled while a transmission is active"]
    #[inline(always)]
    pub fn lbme(&self) -> LBME_R {
        LBME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Loop Back Mode, Internal 0 - Disabled1 - EnabledLBMI should not be enabled while a transmission is active."]
    #[inline(always)]
    pub fn lbmi(&self) -> LBMI_R {
        LBMI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmission Primary Single Shot mode for PTB 0 - Disabled 1 - Enabled"]
    #[inline(always)]
    pub fn tpss(&self) -> TPSS_R {
        TPSS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Secondary Single Shot mode for STB 0 - Disabled 1 - Enabled"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Reception ACTIVE (Receive Status bit) 1 - The controller is currently receiving a frame. 0 - No receive activity."]
    #[inline(always)]
    pub fn ractive(&self) -> RACTIVE_R {
        RACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission ACTIVE (Transmit Status bit) 1 - The controller is currently transmitting a frame. 0 - No transmit activity."]
    #[inline(always)]
    pub fn tactive(&self) -> TACTIVE_R {
        TACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Bus Off (Bus Status bit) 1 - The controller status is “bus off”. 0 - The controller status is “bus on”. Writing a 1 to BUSOFF will reset TECNT and RECNT. This should be done only for debugging. See Chapter 3.9.10.6 for details."]
    #[inline(always)]
    pub fn busoff(&self) -> BUSOFF_R {
        BUSOFF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Self-ACKnowledge 0 – no self-ACK 1 – self-ACK when LBME=1"]
    #[inline(always)]
    pub fn sack(&mut self) -> SACK_W<31> {
        SACK_W::new(self)
    }
    #[doc = "Bit 30 - Receive buffer Overflow Mode In case of a full RBUF when a new message is received, then ROM selects the following: 1 – The new message will not be stored. 0 – The oldest message will be overwritten."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W<30> {
        ROM_W::new(self)
    }
    #[doc = "Bit 28 - Receive buffer RELease The host controller has read the actual RB slot and releases it. Afterwards the CAN-CTRL core points to the next RB slot. RSTAT gets updated. 1 – Release: The host has read the RB. 0 – No release"]
    #[inline(always)]
    pub fn rrel(&mut self) -> RREL_W<28> {
        RREL_W::new(self)
    }
    #[doc = "Bit 27 - Receive Buffer stores ALL data frames 0 – normal operation 1 – RB stores correct data frames as well as data frames with error"]
    #[inline(always)]
    pub fn rball(&mut self) -> RBALL_W<27> {
        RBALL_W::new(self)
    }
    #[doc = "Bit 23 - CAN FD ISO mode 0 - Bosch CAN FD (non-ISO) mode 1 - ISO CAN FD mode (ISO 11898-1:2015) ISO CAN FD mode has a different CRC initialization value and an additional stuff bit count. Both modes are incompatible and must not be mixed in one CAN network. This bit has no impact to CAN 2.0B. This bit is only writeable if RESET=1."]
    #[inline(always)]
    pub fn fd_iso(&mut self) -> FD_ISO_W<23> {
        FD_ISO_W::new(self)
    }
    #[doc = "Bit 22 - Transmit buffer Secondary NEXT 0 - no action 1 - STB slot filled, select next slot. After all frame bytes are written to the TBUF registers, the host controller has to set TSNEXT to signal that this slot has been filled. Then the CAN-CTRL core connects the TBUF registers to the next slot. Once a slot is marked as filled a transmission can be started using TSONE or TSALL. It is possible to set TSNEXT and TSONE or TSALL together in one write access. TSNEXT has to be set by the host controller and is automatically reset by the CAN-CTRL core immediately after it was set. Setting TSNEXT is meaningless if TBSEL=0. In this case TSNEXT is ignored and automatically cleared. It does not do any harm. If all slots of the STB are filled, TSNEXT stays set until a slot becomes free. TSNEXT has no meaning in TTCAN mode and is fixed to 0."]
    #[inline(always)]
    pub fn tsnext(&mut self) -> TSNEXT_W<22> {
        TSNEXT_W::new(self)
    }
    #[doc = "Bit 21 - Transmit buffer Secondary operation MODE 0 - FIFO mode 1 - priority decision mode In FIFO mode frames are transmitted in the order in that they are written into the STB. In priority decision mode the frame with the highest priority in the STB is automatically transmitted first. The ID of a frame is used for the priority decision. A lower ID means a higher priority of a frame. A frame in the PTB has always the highest priority regardless of the ID. TSMODE shall be switched only if the STB if empty"]
    #[inline(always)]
    pub fn tsmode(&mut self) -> TSMODE_W<21> {
        TSMODE_W::new(self)
    }
    #[doc = "Bit 20 - TTCAN Transmit Buffer Mode If TTEN=0 then TTTBM is ignored, otherwise the following is valid: 0 - separate PTB and STB, behavior defined by TSMODE 1 - full TTCAN support: buffer slots selectable by TBPTR and TTPTR For event-driven CAN communication (TTEN=0), the system provides PTB and STB and the behavior of the STB is defined by TSMODE. Then TTTBM is ignored. For time-triggered CAN communication (TTEN=1) with full support of all features including time-triggered transmissions, TTTBM=1 needs to be chosen. Then the all TB slots are addressable using TTPTR and TBPTR. For time-triggered CAN communication (TTEN=1) with only support of reception timestamps, TTTBM=0 can be chosen. Then the transmit buffer acts as in event-driven mode and the behavior can be selected by TSMODE. TTTBM shall be switched only if the TBUF is empty."]
    #[inline(always)]
    pub fn tttbm(&mut self) -> TTTBM_W<20> {
        TTTBM_W::new(self)
    }
    #[doc = "Bit 15 - Transmit Buffer Select Selects the transmit buffer to be loaded with a message. Use the TBUF registers for access. TBSEL needs to be stable all the time the TBUF registers are written and when TSNEXT is set. 0 - PTB (high-priority buffer) 1 - STB The bit will be reset to the hardware reset value if (TTEN=1 and TTTBM=1)"]
    #[inline(always)]
    pub fn tbsel(&mut self) -> TBSEL_W<15> {
        TBSEL_W::new(self)
    }
    #[doc = "Bit 14 - Listen Only Mode 0 - Disabled 1 - Enabled LOM cannot be set if TPE, TSONE or TSALL is set. No transmission can be started if LOM is enabled and LBME is disabled. LOM=1 and LBME=0 disables all transmissions. LOM=1 and LBME=1 disables the ACK for received frames and error frames, but enables the transmission of own frames."]
    #[inline(always)]
    pub fn lom(&mut self) -> LOM_W<14> {
        LOM_W::new(self)
    }
    #[doc = "Bit 13 - Transceiver Standby Mode 0 - Disabled 1 - Enabled This register bit is connected to the output signal stby which can be used to control a standby mode of a transceiver. STBY cannot be set to 1 if TPE=1, TSONE=1 or TSALL=1. If the host sets STBY to 0 then the host needs to wait for the time required by the transceiver to start up before the host requests a new transmission."]
    #[inline(always)]
    pub fn stby(&mut self) -> STBY_W<13> {
        STBY_W::new(self)
    }
    #[doc = "Bit 12 - Transmit Primary Enable 1 - Transmission enable for the message in the high-priority PTB 0 - No transmission for the PTB If TPE is set, the message from the PTB will be transmitted at the next possible transmit position. A started transmission from the STB will be completed before, but pending new messages are delayed until the PTB message has been transmitted. TPE stays set until the message has been transmitted successfully or it is aborted using TPA. The host controller can set TPE to 1 but can not reset it to 0. This would only be possible using TPA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
    #[inline(always)]
    pub fn tpe(&mut self) -> TPE_W<12> {
        TPE_W::new(self)
    }
    #[doc = "Bit 11 - Transmit Primary Abort 1 – Aborts a transmission from PTB which has been requested by TPE=1 but not started yet. (The data bytes of the message remains in the PTB.) 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TPA automatically de-asserts TPE. The host controller can set TPA to 1 but can not reset it to 0. During the short time while the CAN-CTRL core resets the bit, it cannot be set by the host. The bit will be reset to the hardware reset value if RESET=1 or (TTEN=1 and TTTBM=1). TPA should not be set simultaneously with TPE."]
    #[inline(always)]
    pub fn tpa(&mut self) -> TPA_W<11> {
        TPA_W::new(self)
    }
    #[doc = "Bit 10 - Transmit Secondary ONE frame 1 – Transmission enable of one in the STB. In FIFO mode this is the oldest message and in priority mode this is the one with the highest priority. TSONE in priority mode is difficult to handle, because it is not always clear which message will be transmitted if new messages are written to the STB meanwhile. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSONE stays set until the message has been transmitted successfully or it is aborted using TSA. The host controller can set TSONE to 1 but can not reset it to 0. This would only be possible using TSA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
    #[inline(always)]
    pub fn tsone(&mut self) -> TSONE_W<10> {
        TSONE_W::new(self)
    }
    #[doc = "Bit 9 - Transmit Secondary ALL frames 1 – Transmission enable of all messages in the STB. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSALL stays set until all messages have been transmitted successfully or they are aborted using TSA. The host controller can set TSALL to 1 but can not reset it to 0. This would only be possible using TSA and aborting the messages. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1). If during a transmission the STB is loaded with a new frame then the new frame will be transmitted too. In other words: a transmission initiated by TSALL is finished when the STB becomes empty."]
    #[inline(always)]
    pub fn tsall(&mut self) -> TSALL_W<9> {
        TSALL_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Secondary Abort 1 – Aborts a transmission from STB which has been requested but not started yet. For a TSONE transmission, only one frame is aborted while for a TSALL Transmission, all frames are aborted. One or all message slots will be released which updates TSSTAT. All aborted messages are lost because they are not accessible any more. If in priority mode a TSONE transmission is aborted, then it is not clear which frame will be aborted if new frames are written to the STB meanwhile. 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TSA,automatically de-asserts TSONE or TSALL respectively. The host controller can set TSA to 1 but can not reset it to 0. The bit will be reset to the hardware reset value if RESET=1. TSA should not be set simultaneously with TSONE or TSALL."]
    #[inline(always)]
    pub fn tsa(&mut self) -> TSA_W<8> {
        TSA_W::new(self)
    }
    #[doc = "Bit 7 - RESET request bit 1 - The host controller performs a local reset of CAN-CTRL. 0 - no local reset of CAN-CTRLThe some register (e.g for node configuration) can only be modified if RESET=1. Bit RESET forces several components to a reset state. RESET is automatically set if the node enters “bus off” state. Note that a CAN node will participate in CAN communication after RESET is switched to 0after 11 CAN bit times. This delay is required by the CAN standard (bus idle time).If RESET is set to 1 and immediately set to 0, then it takes some time until RESET can beread as 0 and becomes inactive. The reason is clock domain crossing from host to CAN clockdomain. RESET is held active as long as needed depending on the relation between host andCAN clock."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<7> {
        RESET_W::new(self)
    }
    #[doc = "Bit 6 - Loop Back Mode, External 0 - Disabled 1 - EnabledLBME should not be enabled while a transmission is active"]
    #[inline(always)]
    pub fn lbme(&mut self) -> LBME_W<6> {
        LBME_W::new(self)
    }
    #[doc = "Bit 5 - Loop Back Mode, Internal 0 - Disabled1 - EnabledLBMI should not be enabled while a transmission is active."]
    #[inline(always)]
    pub fn lbmi(&mut self) -> LBMI_W<5> {
        LBMI_W::new(self)
    }
    #[doc = "Bit 4 - Transmission Primary Single Shot mode for PTB 0 - Disabled 1 - Enabled"]
    #[inline(always)]
    pub fn tpss(&mut self) -> TPSS_W<4> {
        TPSS_W::new(self)
    }
    #[doc = "Bit 3 - Transmission Secondary Single Shot mode for STB 0 - Disabled 1 - Enabled"]
    #[inline(always)]
    pub fn tsss(&mut self) -> TSSS_W<3> {
        TSSS_W::new(self)
    }
    #[doc = "Bit 0 - Bus Off (Bus Status bit) 1 - The controller status is “bus off”. 0 - The controller status is “bus on”. Writing a 1 to BUSOFF will reset TECNT and RECNT. This should be done only for debugging. See Chapter 3.9.10.6 for details."]
    #[inline(always)]
    pub fn busoff(&mut self) -> BUSOFF_W<0> {
        BUSOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "config, status, command and control bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_sta_cmd_ctrl](index.html) module"]
pub struct CMD_STA_CMD_CTRL_SPEC;
impl crate::RegisterSpec for CMD_STA_CMD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_sta_cmd_ctrl::R](R) reader structure"]
impl crate::Readable for CMD_STA_CMD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd_sta_cmd_ctrl::W](W) writer structure"]
impl crate::Writable for CMD_STA_CMD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD_STA_CMD_CTRL to value 0x0090_0080"]
impl crate::Resettable for CMD_STA_CMD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0090_0080
    }
}
