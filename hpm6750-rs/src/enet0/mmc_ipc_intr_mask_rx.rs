#[doc = "Register `MMC_IPC_INTR_MASK_RX` reader"]
pub struct R(crate::R<MMC_IPC_INTR_MASK_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_IPC_INTR_MASK_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_IPC_INTR_MASK_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_IPC_INTR_MASK_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_IPC_INTR_MASK_RX` writer"]
pub struct W(crate::W<MMC_IPC_INTR_MASK_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_IPC_INTR_MASK_RX_SPEC>;
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
impl From<crate::W<MMC_IPC_INTR_MASK_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_IPC_INTR_MASK_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXICMPEROIM` reader - MMC Receive ICMP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
pub type RXICMPEROIM_R = crate::BitReader<bool>;
#[doc = "Field `RXICMPEROIM` writer - MMC Receive ICMP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
pub type RXICMPEROIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXICMPGOIM` reader - MMC Receive ICMP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXICMPGOIM_R = crate::BitReader<bool>;
#[doc = "Field `RXICMPGOIM` writer - MMC Receive ICMP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXICMPGOIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXTCPEROIM` reader - MMC Receive TCP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
pub type RXTCPEROIM_R = crate::BitReader<bool>;
#[doc = "Field `RXTCPEROIM` writer - MMC Receive TCP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
pub type RXTCPEROIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXTCPGOIM` reader - MMC Receive TCP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXTCPGOIM_R = crate::BitReader<bool>;
#[doc = "Field `RXTCPGOIM` writer - MMC Receive TCP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXTCPGOIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXUDPEROIM` reader - MMC Receive UDP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
pub type RXUDPEROIM_R = crate::BitReader<bool>;
#[doc = "Field `RXUDPEROIM` writer - MMC Receive UDP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
pub type RXUDPEROIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXUDPGOIM` reader - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXUDPGOIM_R = crate::BitReader<bool>;
#[doc = "Field `RXUDPGOIM` writer - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXUDPGOIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV6NOPAYOIM` reader - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6NOPAYOIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV6NOPAYOIM` writer - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6NOPAYOIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV6HEROIM` reader - MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6HEROIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV6HEROIM` writer - MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6HEROIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV6GOIM` reader - MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6GOIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV6GOIM` writer - MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6GOIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4UDSBLOIM` reader - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4UDSBLOIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4UDSBLOIM` writer - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4UDSBLOIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4FRAGOIM` reader - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4FRAGOIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4FRAGOIM` writer - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4FRAGOIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4NOPAYOIM` reader - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4NOPAYOIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4NOPAYOIM` writer - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4NOPAYOIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4HEROIM` reader - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4HEROIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4HEROIM` writer - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4HEROIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4GOIM` reader - MMC Receive IPV4 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4GOIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4GOIM` writer - MMC Receive IPV4 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4GOIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXICMPERFIM` reader - MMC Receive ICMP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_frms counter reaches half of the maximum value or the maximum value."]
pub type RXICMPERFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXICMPERFIM` writer - MMC Receive ICMP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_frms counter reaches half of the maximum value or the maximum value."]
pub type RXICMPERFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXICMPGFIM` reader - MMC Receive ICMP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXICMPGFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXICMPGFIM` writer - MMC Receive ICMP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXICMPGFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXTCPERFIM` reader - MMC Receive TCP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_frms counter reaches half of the maximum value or the maximum value."]
pub type RXTCPERFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXTCPERFIM` writer - MMC Receive TCP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_frms counter reaches half of the maximum value or the maximum value."]
pub type RXTCPERFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXTCPGFIM` reader - MMC Receive TCP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXTCPGFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXTCPGFIM` writer - MMC Receive TCP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXTCPGFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXUDPERFIM` reader - MMC Receive UDP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_frms counter reaches half of the maximum value or the maximum value."]
pub type RXUDPERFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXUDPERFIM` writer - MMC Receive UDP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_frms counter reaches half of the maximum value or the maximum value."]
pub type RXUDPERFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXUDPGFIM` reader - MMC Receive UDP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXUDPGFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXUDPGFIM` writer - MMC Receive UDP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXUDPGFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV6NOPAYFIM` reader - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6NOPAYFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV6NOPAYFIM` writer - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6NOPAYFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV6HERFIM` reader - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6HERFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV6HERFIM` writer - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6HERFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV6GFIM` reader - MMC Receive IPV6 Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6GFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV6GFIM` writer - MMC Receive IPV6 Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6GFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4UDSBLFIM` reader - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4UDSBLFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4UDSBLFIM` writer - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4UDSBLFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4FRAGFIM` reader - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4FRAGFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4FRAGFIM` writer - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4FRAGFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4NOPAYFIM` reader - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4NOPAYFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4NOPAYFIM` writer - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4NOPAYFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4HERFIM` reader - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4HERFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4HERFIM` writer - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4HERFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4GFIM` reader - MMC Receive IPV4 Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4GFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4GFIM` writer - MMC Receive IPV4 Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4GFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_MASK_RX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmperoim(&self) -> RXICMPEROIM_R {
        RXICMPEROIM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmpgoim(&self) -> RXICMPGOIM_R {
        RXICMPGOIM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcperoim(&self) -> RXTCPEROIM_R {
        RXTCPEROIM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcpgoim(&self) -> RXTCPGOIM_R {
        RXTCPGOIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Receive UDP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudperoim(&self) -> RXUDPEROIM_R {
        RXUDPEROIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudpgoim(&self) -> RXUDPGOIM_R {
        RXUDPGOIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6nopayoim(&self) -> RXIPV6NOPAYOIM_R {
        RXIPV6NOPAYOIM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6heroim(&self) -> RXIPV6HEROIM_R {
        RXIPV6HEROIM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6goim(&self) -> RXIPV6GOIM_R {
        RXIPV6GOIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4udsbloim(&self) -> RXIPV4UDSBLOIM_R {
        RXIPV4UDSBLOIM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4fragoim(&self) -> RXIPV4FRAGOIM_R {
        RXIPV4FRAGOIM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4nopayoim(&self) -> RXIPV4NOPAYOIM_R {
        RXIPV4NOPAYOIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4heroim(&self) -> RXIPV4HEROIM_R {
        RXIPV4HEROIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4goim(&self) -> RXIPV4GOIM_R {
        RXIPV4GOIM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmperfim(&self) -> RXICMPERFIM_R {
        RXICMPERFIM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmpgfim(&self) -> RXICMPGFIM_R {
        RXICMPGFIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcperfim(&self) -> RXTCPERFIM_R {
        RXTCPERFIM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcpgfim(&self) -> RXTCPGFIM_R {
        RXTCPGFIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudperfim(&self) -> RXUDPERFIM_R {
        RXUDPERFIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudpgfim(&self) -> RXUDPGFIM_R {
        RXUDPGFIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6nopayfim(&self) -> RXIPV6NOPAYFIM_R {
        RXIPV6NOPAYFIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6herfim(&self) -> RXIPV6HERFIM_R {
        RXIPV6HERFIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6gfim(&self) -> RXIPV6GFIM_R {
        RXIPV6GFIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4udsblfim(&self) -> RXIPV4UDSBLFIM_R {
        RXIPV4UDSBLFIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4fragfim(&self) -> RXIPV4FRAGFIM_R {
        RXIPV4FRAGFIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4nopayfim(&self) -> RXIPV4NOPAYFIM_R {
        RXIPV4NOPAYFIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4herfim(&self) -> RXIPV4HERFIM_R {
        RXIPV4HERFIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4gfim(&self) -> RXIPV4GFIM_R {
        RXIPV4GFIM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmperoim(&mut self) -> RXICMPEROIM_W<29> {
        RXICMPEROIM_W::new(self)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmpgoim(&mut self) -> RXICMPGOIM_W<28> {
        RXICMPGOIM_W::new(self)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcperoim(&mut self) -> RXTCPEROIM_W<27> {
        RXTCPEROIM_W::new(self)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcpgoim(&mut self) -> RXTCPGOIM_W<26> {
        RXTCPGOIM_W::new(self)
    }
    #[doc = "Bit 25 - MMC Receive UDP Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudperoim(&mut self) -> RXUDPEROIM_W<25> {
        RXUDPEROIM_W::new(self)
    }
    #[doc = "Bit 24 - MMC Receive IPV6 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudpgoim(&mut self) -> RXUDPGOIM_W<24> {
        RXUDPGOIM_W::new(self)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6nopayoim(&mut self) -> RXIPV6NOPAYOIM_W<23> {
        RXIPV6NOPAYOIM_W::new(self)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6heroim(&mut self) -> RXIPV6HEROIM_W<22> {
        RXIPV6HEROIM_W::new(self)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6goim(&mut self) -> RXIPV6GOIM_W<21> {
        RXIPV6GOIM_W::new(self)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4udsbloim(&mut self) -> RXIPV4UDSBLOIM_W<20> {
        RXIPV4UDSBLOIM_W::new(self)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4fragoim(&mut self) -> RXIPV4FRAGOIM_W<19> {
        RXIPV4FRAGOIM_W::new(self)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4nopayoim(&mut self) -> RXIPV4NOPAYOIM_W<18> {
        RXIPV4NOPAYOIM_W::new(self)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4heroim(&mut self) -> RXIPV4HEROIM_W<17> {
        RXIPV4HEROIM_W::new(self)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4goim(&mut self) -> RXIPV4GOIM_W<16> {
        RXIPV4GOIM_W::new(self)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_err_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmperfim(&mut self) -> RXICMPERFIM_W<13> {
        RXICMPERFIM_W::new(self)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxicmp_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmpgfim(&mut self) -> RXICMPGFIM_W<12> {
        RXICMPGFIM_W::new(self)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_err_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcperfim(&mut self) -> RXTCPERFIM_W<11> {
        RXTCPERFIM_W::new(self)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxtcp_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcpgfim(&mut self) -> RXTCPGFIM_W<10> {
        RXTCPGFIM_W::new(self)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_err_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudperfim(&mut self) -> RXUDPERFIM_W<9> {
        RXUDPERFIM_W::new(self)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxudp_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudpgfim(&mut self) -> RXUDPGFIM_W<8> {
        RXUDPGFIM_W::new(self)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_nopay_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6nopayfim(&mut self) -> RXIPV6NOPAYFIM_W<7> {
        RXIPV6NOPAYFIM_W::new(self)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6herfim(&mut self) -> RXIPV6HERFIM_W<6> {
        RXIPV6HERFIM_W::new(self)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv6_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6gfim(&mut self) -> RXIPV6GFIM_W<5> {
        RXIPV6GFIM_W::new(self)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_udsbl_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4udsblfim(&mut self) -> RXIPV4UDSBLFIM_W<4> {
        RXIPV4UDSBLFIM_W::new(self)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_frag_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4fragfim(&mut self) -> RXIPV4FRAGFIM_W<3> {
        RXIPV4FRAGFIM_W::new(self)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_nopay_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4nopayfim(&mut self) -> RXIPV4NOPAYFIM_W<2> {
        RXIPV4NOPAYFIM_W::new(self)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4herfim(&mut self) -> RXIPV4HERFIM_W<1> {
        RXIPV4HERFIM_W::new(self)
    }
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxipv4_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4gfim(&mut self) -> RXIPV4GFIM_W<0> {
        RXIPV4GFIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC IPC Receive Checksum Offload Interrupt Mask maintains the mask for the interrupt generated from the receive IPC statistic counters.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_ipc_intr_mask_rx](index.html) module"]
pub struct MMC_IPC_INTR_MASK_RX_SPEC;
impl crate::RegisterSpec for MMC_IPC_INTR_MASK_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_ipc_intr_mask_rx::R](R) reader structure"]
impl crate::Readable for MMC_IPC_INTR_MASK_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_ipc_intr_mask_rx::W](W) writer structure"]
impl crate::Writable for MMC_IPC_INTR_MASK_RX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMC_IPC_INTR_MASK_RX to value 0"]
impl crate::Resettable for MMC_IPC_INTR_MASK_RX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
