#[doc = "Register `MACCFG` reader"]
pub struct R(crate::R<MACCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACCFG` writer"]
pub struct W(crate::W<MACCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACCFG_SPEC>;
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
impl From<crate::W<MACCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARC` reader - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted frames. Bit 30 specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: - 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. - 2'b10: - If Bit 30 is set to 0, the MAC inserts the content of the MAC Address 0 registers (registers 16 and 17) in the SA field of all transmitted frames. - If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC inserts the content of the MAC Address 1 registers (registers 18 and 19) in the SA field of all transmitted frames. - 2'b11: - If Bit 30 is set to 0, the MAC replaces the content of the MAC Address 0 registers (registers 16 and 17) in the SA field of all transmitted frames. - If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC replaces the content of the MAC Address 1 registers (registers 18 and 19) in the SA field of all transmitted frames. Note: - Changes to this field take effect only on the start of a frame. If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value. - These bits are reserved and RO when the Enable SA, VLAN, and CRC Insertion on TX feature is not selected during core configuration."]
pub type SARC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SARC` writer - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted frames. Bit 30 specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: - 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. - 2'b10: - If Bit 30 is set to 0, the MAC inserts the content of the MAC Address 0 registers (registers 16 and 17) in the SA field of all transmitted frames. - If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC inserts the content of the MAC Address 1 registers (registers 18 and 19) in the SA field of all transmitted frames. - 2'b11: - If Bit 30 is set to 0, the MAC replaces the content of the MAC Address 0 registers (registers 16 and 17) in the SA field of all transmitted frames. - If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC replaces the content of the MAC Address 1 registers (registers 18 and 19) in the SA field of all transmitted frames. Note: - Changes to this field take effect only on the start of a frame. If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value. - These bits are reserved and RO when the Enable SA, VLAN, and CRC Insertion on TX feature is not selected during core configuration."]
pub type SARC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `TWOKPE` reader - IEEE 802.3as Support for 2K Packets When set, the MAC considers all frames, with up to 2,000 bytes length, as normal packets. When Bit 20 (JE) is not set, the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit 20 (JE) is not set, the MAC considers all received frames of size more than 1,518 bytes (1,522 bytes for tagged) as Giant frames. When Bit 20 is set, setting this bit has no effect on Giant Frame status."]
pub type TWOKPE_R = crate::BitReader<bool>;
#[doc = "Field `TWOKPE` writer - IEEE 802.3as Support for 2K Packets When set, the MAC considers all frames, with up to 2,000 bytes length, as normal packets. When Bit 20 (JE) is not set, the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit 20 (JE) is not set, the MAC considers all received frames of size more than 1,518 bytes (1,522 bytes for tagged) as Giant frames. When Bit 20 is set, setting this bit has no effect on Giant Frame status."]
pub type TWOKPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `SFTERR` reader - SMII Force Transmit Error When set, this bit indicates to the PHY to force a transmit error in the SMII frame being transmitted. This bit is reserved if the SMII PHY port is not selected during core configuration."]
pub type SFTERR_R = crate::BitReader<bool>;
#[doc = "Field `SFTERR` writer - SMII Force Transmit Error When set, this bit indicates to the PHY to force a transmit error in the SMII frame being transmitted. This bit is reserved if the SMII PHY port is not selected during core configuration."]
pub type SFTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `CST` reader - CRC Stripping for Type Frames When this bit is set, the last 4 bytes (FCS) of all frames of Ether type (Length/Type field greater than or equal to 1,536) are stripped and dropped before forwarding the frame to the application. This function is not valid when the IP Checksum Engine (Type 1) is enabled in the MAC receiver. This function is valid when Type 2 Checksum Offload Engine is enabled."]
pub type CST_R = crate::BitReader<bool>;
#[doc = "Field `CST` writer - CRC Stripping for Type Frames When this bit is set, the last 4 bytes (FCS) of all frames of Ether type (Length/Type field greater than or equal to 1,536) are stripped and dropped before forwarding the frame to the application. This function is not valid when the IP Checksum Engine (Type 1) is enabled in the MAC receiver. This function is valid when Type 2 Checksum Offload Engine is enabled."]
pub type CST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `TC` reader - Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port. When this bit is reset, no such information is driven to the PHY. This bit is reserved (and RO) if the RGMII, SMII, or SGMII PHY port is not selected during core configuration."]
pub type TC_R = crate::BitReader<bool>;
#[doc = "Field `TC` writer - Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port. When this bit is reset, no such information is driven to the PHY. This bit is reserved (and RO) if the RGMII, SMII, or SGMII PHY port is not selected during core configuration."]
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `WD` reader - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16,383 bytes."]
pub type WD_R = crate::BitReader<bool>;
#[doc = "Field `WD` writer - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16,383 bytes."]
pub type WD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `JD` reader - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16,383 bytes. When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data (10,240 if JE is set high) during transmission."]
pub type JD_R = crate::BitReader<bool>;
#[doc = "Field `JD` writer - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16,383 bytes. When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data (10,240 if JE is set high) during transmission."]
pub type JD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `BE` reader - Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII half-duplex mode. This bit is reserved (and RO) in the 10/100 Mbps only or full-duplex-only configurations."]
pub type BE_R = crate::BitReader<bool>;
#[doc = "Field `BE` writer - Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII half-duplex mode. This bit is reserved (and RO) in the 10/100 Mbps only or full-duplex-only configurations."]
pub type BE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `JE` reader - Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
pub type JE_R = crate::BitReader<bool>;
#[doc = "Field `JE` writer - Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
pub type JE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `IFG` reader - Inter-Frame Gap These bits control the minimum IFG between frames during transmission. - 000: 96 bit times - 001: 88 bit times - 010: 80 bit times - ... - 111: 40 bit times In the half-duplex mode, the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered. In the 1000-Mbps mode, the minimum IFG supported is 64 bit times (and above) in the GMAC-CORE configuration and 80 bit times (and above) in other configurations. When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG."]
pub type IFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFG` writer - Inter-Frame Gap These bits control the minimum IFG between frames during transmission. - 000: 96 bit times - 001: 88 bit times - 010: 80 bit times - ... - 111: 40 bit times In the half-duplex mode, the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered. In the 1000-Mbps mode, the minimum IFG supported is 64 bit times (and above) in the GMAC-CORE configuration and 80 bit times (and above) in other configurations. When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG."]
pub type IFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `DCRS` reader - Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the (G)MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
pub type DCRS_R = crate::BitReader<bool>;
#[doc = "Field `DCRS` writer - Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the (G)MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
pub type DCRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `PS` reader - Port Select This bit selects the Ethernet line speed. - 0: For 1000 Mbps operations - 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed. In the 10/100 Mbps-only (always 1) or 1000 Mbps-only (always 0) configurations, this bit is read-only with the appropriate value. In default 10/100/1000 Mbps configuration, this bit is R_W. The mac_portselect_o or mac_speed_o\\[1\\]
signal reflects the value of this bit."]
pub type PS_R = crate::BitReader<bool>;
#[doc = "Field `PS` writer - Port Select This bit selects the Ethernet line speed. - 0: For 1000 Mbps operations - 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed. In the 10/100 Mbps-only (always 1) or 1000 Mbps-only (always 0) configurations, this bit is read-only with the appropriate value. In default 10/100/1000 Mbps configuration, this bit is R_W. The mac_portselect_o or mac_speed_o\\[1\\]
signal reflects the value of this bit."]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `FES` reader - Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: - 0: 10 Mbps - 1: 100 Mbps This bit is reserved (RO) by default and is enabled only when the parameter SPEED_SELECT = Enabled. This bit generates link speed encoding when Bit 24 (TC) is set in the RGMII, SMII, or SGMII mode. This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface. In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal (mac_speed_o\\[0\\]) to reflect the value of this bit in the mac_speed_o signal. In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal (mac_speed_o\\[0\\]) to reflect its value in the mac_speed_o signal."]
pub type FES_R = crate::BitReader<bool>;
#[doc = "Field `FES` writer - Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: - 0: 10 Mbps - 1: 100 Mbps This bit is reserved (RO) by default and is enabled only when the parameter SPEED_SELECT = Enabled. This bit generates link speed encoding when Bit 24 (TC) is set in the RGMII, SMII, or SGMII mode. This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface. In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal (mac_speed_o\\[0\\]) to reflect the value of this bit in the mac_speed_o signal. In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal (mac_speed_o\\[0\\]) to reflect its value in the mac_speed_o signal."]
pub type FES_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `DO` reader - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the half-duplex mode. When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full-duplex mode. This bit is reserved (RO with default value) if the MAC is configured for the full-duplex-only operation."]
pub type DO_R = crate::BitReader<bool>;
#[doc = "Field `DO` writer - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the half-duplex mode. When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full-duplex mode. This bit is reserved (RO with default value) if the MAC is configured for the full-duplex-only operation."]
pub type DO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `LM` reader - Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII. The (G)MII Receive clock input (clk_rx_i) is required for the loopback to work properly, because the Transmit clock is not looped-back internally."]
pub type LM_R = crate::BitReader<bool>;
#[doc = "Field `LM` writer - Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII. The (G)MII Receive clock input (clk_rx_i) is required for the loopback to work properly, because the Transmit clock is not looped-back internally."]
pub type LM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `DM` reader - Duplex Mode When this bit is set, the MAC operates in the full-duplex mode where it can transmit and receive simultaneously. This bit is RO with default value of 1'b1 in the full-duplex-only configuration."]
pub type DM_R = crate::BitReader<bool>;
#[doc = "Field `DM` writer - Duplex Mode When this bit is set, the MAC operates in the full-duplex mode where it can transmit and receive simultaneously. This bit is RO with default value of 1'b1 in the full-duplex-only configuration."]
pub type DM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `IPC` reader - Checksum Offload When this bit is set, the MAC calculates the 16-bit one’s complement of the one’s complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25–26 or 29–30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset, this function is disabled. When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
pub type IPC_R = crate::BitReader<bool>;
#[doc = "Field `IPC` writer - Checksum Offload When this bit is set, the MAC calculates the 16-bit one’s complement of the one’s complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25–26 or 29–30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset, this function is disabled. When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
pub type IPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `DR` reader - Disable Retry When this bit is set, the MAC attempts only one transmission. When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset, the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\])."]
pub type DR_R = crate::BitReader<bool>;
#[doc = "Field `DR` writer - Disable Retry When this bit is set, the MAC attempts only one transmission. When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset, the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\])."]
pub type DR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `LUD` reader - Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: - 0: Link Down - 1: Link Up"]
pub type LUD_R = crate::BitReader<bool>;
#[doc = "Field `LUD` writer - Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: - 0: Link Down - 1: Link Up"]
pub type LUD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `ACS` reader - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes. All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host."]
pub type ACS_R = crate::BitReader<bool>;
#[doc = "Field `ACS` writer - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes. All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host."]
pub type ACS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `BL` reader - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode and is reserved (RO) in the full-duplex-only configuration. - 00: k= min (n, 10) - 01: k = min (n, 8) - 10: k = min (n, 4) - 11: k = min (n, 1) where n = retransmission attempt. The random integer r takes the value in the range 0 ≤ r < 2k"]
pub type BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BL` writer - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode and is reserved (RO) in the full-duplex-only configuration. - 00: k= min (n, 10) - 01: k = min (n, 8) - 10: k = min (n, 4) - 11: k = min (n, 1) where n = retransmission attempt. The random integer r takes the value in the range 0 ≤ r < 2k"]
pub type BL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `DC` reader - Deferral Check When this bit is set, the deferral check function is enabled in the MAC. The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode. If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times. Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal (CRS) on GMII or MII. The defer time is not cumulative. For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens. Because of collision, the transmitter needs to back off and then defer again after back off completion. In such a scenario, the deferral timer is reset to 0 and it is restarted."]
pub type DC_R = crate::BitReader<bool>;
#[doc = "Field `DC` writer - Deferral Check When this bit is set, the deferral check function is enabled in the MAC. The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode. If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times. Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal (CRS) on GMII or MII. The defer time is not cumulative. For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens. Because of collision, the transmitter needs to back off and then defer again after back off completion. In such a scenario, the deferral timer is reset to 0 and it is restarted."]
pub type DC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `TE` reader - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII. When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames."]
pub type TE_R = crate::BitReader<bool>;
#[doc = "Field `TE` writer - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII. When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames."]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `RE` reader - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII. When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII."]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII. When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII."]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCFG_SPEC, bool, O>;
#[doc = "Field `PRELEN` reader - Preamble Length for Transmit frames These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode. - 2'b00: 7 bytes of preamble - 2'b01: 5 bytes of preamble - 2'b10: 3 bytes of preamble - 2'b11: Reserved"]
pub type PRELEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRELEN` writer - Preamble Length for Transmit frames These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode. - 2'b00: 7 bytes of preamble - 2'b01: 5 bytes of preamble - 2'b10: 3 bytes of preamble - 2'b11: Reserved"]
pub type PRELEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACCFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted frames. Bit 30 specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: - 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. - 2'b10: - If Bit 30 is set to 0, the MAC inserts the content of the MAC Address 0 registers (registers 16 and 17) in the SA field of all transmitted frames. - If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC inserts the content of the MAC Address 1 registers (registers 18 and 19) in the SA field of all transmitted frames. - 2'b11: - If Bit 30 is set to 0, the MAC replaces the content of the MAC Address 0 registers (registers 16 and 17) in the SA field of all transmitted frames. - If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC replaces the content of the MAC Address 1 registers (registers 18 and 19) in the SA field of all transmitted frames. Note: - Changes to this field take effect only on the start of a frame. If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value. - These bits are reserved and RO when the Enable SA, VLAN, and CRC Insertion on TX feature is not selected during core configuration."]
    #[inline(always)]
    pub fn sarc(&self) -> SARC_R {
        SARC_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 27 - IEEE 802.3as Support for 2K Packets When set, the MAC considers all frames, with up to 2,000 bytes length, as normal packets. When Bit 20 (JE) is not set, the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit 20 (JE) is not set, the MAC considers all received frames of size more than 1,518 bytes (1,522 bytes for tagged) as Giant frames. When Bit 20 is set, setting this bit has no effect on Giant Frame status."]
    #[inline(always)]
    pub fn twokpe(&self) -> TWOKPE_R {
        TWOKPE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - SMII Force Transmit Error When set, this bit indicates to the PHY to force a transmit error in the SMII frame being transmitted. This bit is reserved if the SMII PHY port is not selected during core configuration."]
    #[inline(always)]
    pub fn sfterr(&self) -> SFTERR_R {
        SFTERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames When this bit is set, the last 4 bytes (FCS) of all frames of Ether type (Length/Type field greater than or equal to 1,536) are stripped and dropped before forwarding the frame to the application. This function is not valid when the IP Checksum Engine (Type 1) is enabled in the MAC receiver. This function is valid when Type 2 Checksum Offload Engine is enabled."]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port. When this bit is reset, no such information is driven to the PHY. This bit is reserved (and RO) if the RGMII, SMII, or SGMII PHY port is not selected during core configuration."]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16,383 bytes."]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16,383 bytes. When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data (10,240 if JE is set high) during transmission."]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII half-duplex mode. This bit is reserved (and RO) in the 10/100 Mbps only or full-duplex-only configurations."]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
    #[inline(always)]
    pub fn je(&self) -> JE_R {
        JE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap These bits control the minimum IFG between frames during transmission. - 000: 96 bit times - 001: 88 bit times - 010: 80 bit times - ... - 111: 40 bit times In the half-duplex mode, the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered. In the 1000-Mbps mode, the minimum IFG supported is 64 bit times (and above) in the GMAC-CORE configuration and 80 bit times (and above) in other configurations. When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG."]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the (G)MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
    #[inline(always)]
    pub fn dcrs(&self) -> DCRS_R {
        DCRS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Select This bit selects the Ethernet line speed. - 0: For 1000 Mbps operations - 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed. In the 10/100 Mbps-only (always 1) or 1000 Mbps-only (always 0) configurations, this bit is read-only with the appropriate value. In default 10/100/1000 Mbps configuration, this bit is R_W. The mac_portselect_o or mac_speed_o\\[1\\]
signal reflects the value of this bit."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: - 0: 10 Mbps - 1: 100 Mbps This bit is reserved (RO) by default and is enabled only when the parameter SPEED_SELECT = Enabled. This bit generates link speed encoding when Bit 24 (TC) is set in the RGMII, SMII, or SGMII mode. This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface. In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal (mac_speed_o\\[0\\]) to reflect the value of this bit in the mac_speed_o signal. In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal (mac_speed_o\\[0\\]) to reflect its value in the mac_speed_o signal."]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the half-duplex mode. When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full-duplex mode. This bit is reserved (RO with default value) if the MAC is configured for the full-duplex-only operation."]
    #[inline(always)]
    pub fn do_(&self) -> DO_R {
        DO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII. The (G)MII Receive clock input (clk_rx_i) is required for the loopback to work properly, because the Transmit clock is not looped-back internally."]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex Mode When this bit is set, the MAC operates in the full-duplex mode where it can transmit and receive simultaneously. This bit is RO with default value of 1'b1 in the full-duplex-only configuration."]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Checksum Offload When this bit is set, the MAC calculates the 16-bit one’s complement of the one’s complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25–26 or 29–30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset, this function is disabled. When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable Retry When this bit is set, the MAC attempts only one transmission. When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset, the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\])."]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: - 0: Link Down - 1: Link Up"]
    #[inline(always)]
    pub fn lud(&self) -> LUD_R {
        LUD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes. All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host."]
    #[inline(always)]
    pub fn acs(&self) -> ACS_R {
        ACS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode and is reserved (RO) in the full-duplex-only configuration. - 00: k= min (n, 10) - 01: k = min (n, 8) - 10: k = min (n, 4) - 11: k = min (n, 1) where n = retransmission attempt. The random integer r takes the value in the range 0 ≤ r < 2k"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC. The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode. If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times. Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal (CRS) on GMII or MII. The defer time is not cumulative. For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens. Because of collision, the transmitter needs to back off and then defer again after back off completion. In such a scenario, the deferral timer is reset to 0 and it is restarted."]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII. When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames."]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII. When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII."]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Preamble Length for Transmit frames These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode. - 2'b00: 7 bytes of preamble - 2'b01: 5 bytes of preamble - 2'b10: 3 bytes of preamble - 2'b11: Reserved"]
    #[inline(always)]
    pub fn prelen(&self) -> PRELEN_R {
        PRELEN_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30 - Source Address Insertion or Replacement Control This field controls the source address insertion or replacement for all transmitted frames. Bit 30 specifies which MAC Address register (0 or 1) is used for source address insertion or replacement based on the values of Bits \\[29:28\\]: - 2'b0x: The input signals mti_sa_ctrl_i and ati_sa_ctrl_i control the SA field generation. - 2'b10: - If Bit 30 is set to 0, the MAC inserts the content of the MAC Address 0 registers (registers 16 and 17) in the SA field of all transmitted frames. - If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC inserts the content of the MAC Address 1 registers (registers 18 and 19) in the SA field of all transmitted frames. - 2'b11: - If Bit 30 is set to 0, the MAC replaces the content of the MAC Address 0 registers (registers 16 and 17) in the SA field of all transmitted frames. - If Bit 30 is set to 1 and the Enable MAC Address Register 1 option is selected during core configuration, the MAC replaces the content of the MAC Address 1 registers (registers 18 and 19) in the SA field of all transmitted frames. Note: - Changes to this field take effect only on the start of a frame. If you write this register field when a frame is being transmitted, only the subsequent frame can use the updated value, that is, the current frame does not use the updated value. - These bits are reserved and RO when the Enable SA, VLAN, and CRC Insertion on TX feature is not selected during core configuration."]
    #[inline(always)]
    pub fn sarc(&mut self) -> SARC_W<28> {
        SARC_W::new(self)
    }
    #[doc = "Bit 27 - IEEE 802.3as Support for 2K Packets When set, the MAC considers all frames, with up to 2,000 bytes length, as normal packets. When Bit 20 (JE) is not set, the MAC considers all received frames of size more than 2K bytes as Giant frames. When this bit is reset and Bit 20 (JE) is not set, the MAC considers all received frames of size more than 1,518 bytes (1,522 bytes for tagged) as Giant frames. When Bit 20 is set, setting this bit has no effect on Giant Frame status."]
    #[inline(always)]
    pub fn twokpe(&mut self) -> TWOKPE_W<27> {
        TWOKPE_W::new(self)
    }
    #[doc = "Bit 26 - SMII Force Transmit Error When set, this bit indicates to the PHY to force a transmit error in the SMII frame being transmitted. This bit is reserved if the SMII PHY port is not selected during core configuration."]
    #[inline(always)]
    pub fn sfterr(&mut self) -> SFTERR_W<26> {
        SFTERR_W::new(self)
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames When this bit is set, the last 4 bytes (FCS) of all frames of Ether type (Length/Type field greater than or equal to 1,536) are stripped and dropped before forwarding the frame to the application. This function is not valid when the IP Checksum Engine (Type 1) is enabled in the MAC receiver. This function is valid when Type 2 Checksum Offload Engine is enabled."]
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W<25> {
        CST_W::new(self)
    }
    #[doc = "Bit 24 - Transmit Configuration in RGMII, SGMII, or SMII When set, this bit enables the transmission of duplex mode, link speed, and link up or down information to the PHY in the RGMII, SMII, or SGMII port. When this bit is reset, no such information is driven to the PHY. This bit is reserved (and RO) if the RGMII, SMII, or SGMII PHY port is not selected during core configuration."]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<24> {
        TC_W::new(self)
    }
    #[doc = "Bit 23 - Watchdog Disable When this bit is set, the MAC disables the watchdog timer on the receiver. The MAC can receive frames of up to 16,383 bytes."]
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W<23> {
        WD_W::new(self)
    }
    #[doc = "Bit 22 - Jabber Disable When this bit is set, the MAC disables the jabber timer on the transmitter. The MAC can transfer frames of up to 16,383 bytes. When this bit is reset, the MAC cuts off the transmitter if the application sends out more than 2,048 bytes of data (10,240 if JE is set high) during transmission."]
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W<22> {
        JD_W::new(self)
    }
    #[doc = "Bit 21 - Frame Burst Enable When this bit is set, the MAC allows frame bursting during transmission in the GMII half-duplex mode. This bit is reserved (and RO) in the 10/100 Mbps only or full-duplex-only configurations."]
    #[inline(always)]
    pub fn be(&mut self) -> BE_W<21> {
        BE_W::new(self)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable When this bit is set, the MAC allows Jumbo frames of 9,018 bytes (9,022 bytes for VLAN tagged frames) without reporting a giant frame error in the receive frame status."]
    #[inline(always)]
    pub fn je(&mut self) -> JE_W<20> {
        JE_W::new(self)
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap These bits control the minimum IFG between frames during transmission. - 000: 96 bit times - 001: 88 bit times - 010: 80 bit times - ... - 111: 40 bit times In the half-duplex mode, the minimum IFG can be configured only for 64 bit times (IFG = 100). Lower values are not considered. In the 1000-Mbps mode, the minimum IFG supported is 64 bit times (and above) in the GMAC-CORE configuration and 80 bit times (and above) in other configurations. When a JAM pattern is being transmitted because of backpressure activation, the MAC does not consider the minimum IFG."]
    #[inline(always)]
    pub fn ifg(&mut self) -> IFG_W<17> {
        IFG_W::new(self)
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission When set high, this bit makes the MAC transmitter ignore the (G)MII CRS signal during frame transmission in the half-duplex mode. This request results in no errors generated because of Loss of Carrier or No Carrier during such transmission. When this bit is low, the MAC transmitter generates such errors because of Carrier Sense and can even abort the transmissions."]
    #[inline(always)]
    pub fn dcrs(&mut self) -> DCRS_W<16> {
        DCRS_W::new(self)
    }
    #[doc = "Bit 15 - Port Select This bit selects the Ethernet line speed. - 0: For 1000 Mbps operations - 1: For 10 or 100 Mbps operations In 10 or 100 Mbps operations, this bit, along with FES bit, selects the exact line speed. In the 10/100 Mbps-only (always 1) or 1000 Mbps-only (always 0) configurations, this bit is read-only with the appropriate value. In default 10/100/1000 Mbps configuration, this bit is R_W. The mac_portselect_o or mac_speed_o\\[1\\]
signal reflects the value of this bit."]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<15> {
        PS_W::new(self)
    }
    #[doc = "Bit 14 - Speed This bit selects the speed in the MII, RMII, SMII, RGMII, SGMII, or RevMII interface: - 0: 10 Mbps - 1: 100 Mbps This bit is reserved (RO) by default and is enabled only when the parameter SPEED_SELECT = Enabled. This bit generates link speed encoding when Bit 24 (TC) is set in the RGMII, SMII, or SGMII mode. This bit is always enabled for RGMII, SGMII, SMII, or RevMII interface. In configurations with RGMII, SGMII, SMII, or RevMII interface, this bit is driven as an output signal (mac_speed_o\\[0\\]) to reflect the value of this bit in the mac_speed_o signal. In configurations with RMII, MII, or GMII interface, you can optionally drive this bit as an output signal (mac_speed_o\\[0\\]) to reflect its value in the mac_speed_o signal."]
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W<14> {
        FES_W::new(self)
    }
    #[doc = "Bit 13 - Disable Receive Own When this bit is set, the MAC disables the reception of frames when the phy_txen_o is asserted in the half-duplex mode. When this bit is reset, the MAC receives all packets that are given by the PHY while transmitting. This bit is not applicable if the MAC is operating in the full-duplex mode. This bit is reserved (RO with default value) if the MAC is configured for the full-duplex-only operation."]
    #[inline(always)]
    pub fn do_(&mut self) -> DO_W<13> {
        DO_W::new(self)
    }
    #[doc = "Bit 12 - Loopback Mode When this bit is set, the MAC operates in the loopback mode at GMII or MII. The (G)MII Receive clock input (clk_rx_i) is required for the loopback to work properly, because the Transmit clock is not looped-back internally."]
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W<12> {
        LM_W::new(self)
    }
    #[doc = "Bit 11 - Duplex Mode When this bit is set, the MAC operates in the full-duplex mode where it can transmit and receive simultaneously. This bit is RO with default value of 1'b1 in the full-duplex-only configuration."]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W<11> {
        DM_W::new(self)
    }
    #[doc = "Bit 10 - Checksum Offload When this bit is set, the MAC calculates the 16-bit one’s complement of the one’s complement sum of all received Ethernet frame payloads. It also checks whether the IPv4 Header checksum (assumed to be bytes 25–26 or 29–30 (VLAN-tagged) of the received Ethernet frame) is correct for the received frame and gives the status in the receive status word. The MAC also appends the 16-bit checksum calculated for the IP header datagram payload (bytes after the IPv4 header) and appends it to the Ethernet frame transferred to the application (when Type 2 COE is deselected). When this bit is reset, this function is disabled. When Type 2 COE is selected, this bit, when set, enables the IPv4 header checksum checking and IPv4 or IPv6 TCP, UDP, or ICMP payload checksum checking."]
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W<10> {
        IPC_W::new(self)
    }
    #[doc = "Bit 9 - Disable Retry When this bit is set, the MAC attempts only one transmission. When a collision occurs on the GMII or MII interface, the MAC ignores the current frame transmission and reports a Frame Abort with excessive collision error in the transmit frame status. When this bit is reset, the MAC attempts retries based on the settings of the BL field (Bits \\[6:5\\])."]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W<9> {
        DR_W::new(self)
    }
    #[doc = "Bit 8 - Link Up or Down This bit indicates whether the link is up or down during the transmission of configuration in the RGMII, SGMII, or SMII interface: - 0: Link Down - 1: Link Up"]
    #[inline(always)]
    pub fn lud(&mut self) -> LUD_W<8> {
        LUD_W::new(self)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping When this bit is set, the MAC strips the Pad or FCS field on the incoming frames only if the value of the length field is less than 1,536 bytes. All received frames with length field greater than or equal to 1,536 bytes are passed to the application without stripping the Pad or FCS field. When this bit is reset, the MAC passes all incoming frames, without modifying them, to the Host."]
    #[inline(always)]
    pub fn acs(&mut self) -> ACS_W<7> {
        ACS_W::new(self)
    }
    #[doc = "Bits 5:6 - Back-Off Limit The Back-Off limit determines the random integer number (r) of slot time delays (4,096 bit times for 1000 Mbps and 512 bit times for 10/100 Mbps) for which the MAC waits before rescheduling a transmission attempt during retries after a collision. This bit is applicable only in the half-duplex mode and is reserved (RO) in the full-duplex-only configuration. - 00: k= min (n, 10) - 01: k = min (n, 8) - 10: k = min (n, 4) - 11: k = min (n, 1) where n = retransmission attempt. The random integer r takes the value in the range 0 ≤ r < 2k"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W<5> {
        BL_W::new(self)
    }
    #[doc = "Bit 4 - Deferral Check When this bit is set, the deferral check function is enabled in the MAC. The MAC issues a Frame Abort status, along with the excessive deferral error bit set in the transmit frame status, when the transmit state machine is deferred for more than 24,288 bit times in the 10 or 100 Mbps mode. If the MAC is configured for 1000 Mbps operation or if the Jumbo frame mode is enabled in the 10 or 100 Mbps mode, the threshold for deferral is 155,680 bits times. Deferral begins when the transmitter is ready to transmit, but it is prevented because of an active carrier sense signal (CRS) on GMII or MII. The defer time is not cumulative. For example, if the transmitter defers for 10,000 bit times because the CRS signal is active and then the CRS signal becomes inactive, the transmitter transmits and collision happens. Because of collision, the transmitter needs to back off and then defer again after back off completion. In such a scenario, the deferral timer is reset to 0 and it is restarted."]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W<4> {
        DC_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter Enable When this bit is set, the transmit state machine of the MAC is enabled for transmission on the GMII or MII. When this bit is reset, the MAC transmit state machine is disabled after the completion of the transmission of the current frame, and does not transmit any further frames."]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    #[doc = "Bit 2 - Receiver Enable When this bit is set, the receiver state machine of the MAC is enabled for receiving frames from the GMII or MII. When this bit is reset, the MAC receive state machine is disabled after the completion of the reception of the current frame, and does not receive any further frames from the GMII or MII."]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    #[doc = "Bits 0:1 - Preamble Length for Transmit frames These bits control the number of preamble bytes that are added to the beginning of every Transmit frame. The preamble reduction occurs only when the MAC is operating in the full-duplex mode. - 2'b00: 7 bytes of preamble - 2'b01: 5 bytes of preamble - 2'b10: 3 bytes of preamble - 2'b11: Reserved"]
    #[inline(always)]
    pub fn prelen(&mut self) -> PRELEN_W<0> {
        PRELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maccfg](index.html) module"]
pub struct MACCFG_SPEC;
impl crate::RegisterSpec for MACCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maccfg::R](R) reader structure"]
impl crate::Readable for MACCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maccfg::W](W) writer structure"]
impl crate::Writable for MACCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACCFG to value 0"]
impl crate::Resettable for MACCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
