#[doc = "Register `MMC_IPC_INTR_RX` reader"]
pub struct R(crate::R<MMC_IPC_INTR_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_IPC_INTR_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_IPC_INTR_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_IPC_INTR_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_IPC_INTR_RX` writer"]
pub struct W(crate::W<MMC_IPC_INTR_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_IPC_INTR_RX_SPEC>;
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
impl From<crate::W<MMC_IPC_INTR_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_IPC_INTR_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXICMPEROIS` reader - MMC Receive ICMP Error Octet Counter Interrupt Status This bit is set when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
pub type RXICMPEROIS_R = crate::BitReader<bool>;
#[doc = "Field `RXICMPEROIS` writer - MMC Receive ICMP Error Octet Counter Interrupt Status This bit is set when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
pub type RXICMPEROIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXICMPGOIS` reader - MMC Receive ICMP Good Octet Counter Interrupt Status This bit is set when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXICMPGOIS_R = crate::BitReader<bool>;
#[doc = "Field `RXICMPGOIS` writer - MMC Receive ICMP Good Octet Counter Interrupt Status This bit is set when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXICMPGOIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXTCPEROIS` reader - MMC Receive TCP Error Octet Counter Interrupt Status This bit is set when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
pub type RXTCPEROIS_R = crate::BitReader<bool>;
#[doc = "Field `RXTCPEROIS` writer - MMC Receive TCP Error Octet Counter Interrupt Status This bit is set when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
pub type RXTCPEROIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXTCPGOIS` reader - MMC Receive TCP Good Octet Counter Interrupt Status This bit is set when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value"]
pub type RXTCPGOIS_R = crate::BitReader<bool>;
#[doc = "Field `RXTCPGOIS` writer - MMC Receive TCP Good Octet Counter Interrupt Status This bit is set when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value"]
pub type RXTCPGOIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXUDPEROIS` reader - MMC Receive UDP Error Octet Counter Interrupt Status This bit is set when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
pub type RXUDPEROIS_R = crate::BitReader<bool>;
#[doc = "Field `RXUDPEROIS` writer - MMC Receive UDP Error Octet Counter Interrupt Status This bit is set when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
pub type RXUDPEROIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXUDPGOIS` reader - MMC Receive UDP Good Octet Counter Interrupt Status This bit is set when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXUDPGOIS_R = crate::BitReader<bool>;
#[doc = "Field `RXUDPGOIS` writer - MMC Receive UDP Good Octet Counter Interrupt Status This bit is set when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXUDPGOIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV6NOPAYOIS` reader - MMC Receive IPV6 No Payload Octet Counter Interrupt Status This bit is set when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6NOPAYOIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV6NOPAYOIS` writer - MMC Receive IPV6 No Payload Octet Counter Interrupt Status This bit is set when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6NOPAYOIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV6HEROIS` reader - MMC Receive IPV6 Header Error Octet Counter Interrupt Status This bit is set when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6HEROIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV6HEROIS` writer - MMC Receive IPV6 Header Error Octet Counter Interrupt Status This bit is set when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6HEROIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV6GOIS` reader - MMC Receive IPV6 Good Octet Counter Interrupt Status This bit is set when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6GOIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV6GOIS` writer - MMC Receive IPV6 Good Octet Counter Interrupt Status This bit is set when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6GOIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4UDSBLOIS` reader - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status This bit is set when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4UDSBLOIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4UDSBLOIS` writer - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status This bit is set when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4UDSBLOIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4FRAGOIS` reader - MMC Receive IPV4 Fragmented Octet Counter Interrupt Status This bit is set when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4FRAGOIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4FRAGOIS` writer - MMC Receive IPV4 Fragmented Octet Counter Interrupt Status This bit is set when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4FRAGOIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4NOPAYOIS` reader - MMC Receive IPV4 No Payload Octet Counter Interrupt Status This bit is set when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4NOPAYOIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4NOPAYOIS` writer - MMC Receive IPV4 No Payload Octet Counter Interrupt Status This bit is set when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4NOPAYOIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4HEROIS` reader - MMC Receive IPV4 Header Error Octet Counter Interrupt Status This bit is set when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4HEROIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4HEROIS` writer - MMC Receive IPV4 Header Error Octet Counter Interrupt Status This bit is set when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4HEROIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4GOIS` reader - MMC Receive IPV4 Good Octet Counter Interrupt Status This bit is set when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4GOIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4GOIS` writer - MMC Receive IPV4 Good Octet Counter Interrupt Status This bit is set when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4GOIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXICMPERFIS` reader - MMC Receive ICMP Error Frame Counter Interrupt Status This bit is set when the rxicmp_err_frms counter reaches half of the maximum value or the maximum value."]
pub type RXICMPERFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXICMPERFIS` writer - MMC Receive ICMP Error Frame Counter Interrupt Status This bit is set when the rxicmp_err_frms counter reaches half of the maximum value or the maximum value."]
pub type RXICMPERFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXICMPGFIS` reader - MMC Receive ICMP Good Frame Counter Interrupt Status This bit is set when the rxicmp_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXICMPGFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXICMPGFIS` writer - MMC Receive ICMP Good Frame Counter Interrupt Status This bit is set when the rxicmp_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXICMPGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXTCPERFIS` reader - MMC Receive TCP Error Frame Counter Interrupt Status This bit is set when the rxtcp_err_frms counter reaches half of the maximum value or the maximum value."]
pub type RXTCPERFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXTCPERFIS` writer - MMC Receive TCP Error Frame Counter Interrupt Status This bit is set when the rxtcp_err_frms counter reaches half of the maximum value or the maximum value."]
pub type RXTCPERFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXTCPGFIS` reader - MMC Receive TCP Good Frame Counter Interrupt Status This bit is set when the rxtcp_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXTCPGFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXTCPGFIS` writer - MMC Receive TCP Good Frame Counter Interrupt Status This bit is set when the rxtcp_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXTCPGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXUDPERFIS` reader - MMC Receive UDP Error Frame Counter Interrupt Status This bit is set when the rxudp_err_frms counter reaches half of the maximum value or the maximum value."]
pub type RXUDPERFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXUDPERFIS` writer - MMC Receive UDP Error Frame Counter Interrupt Status This bit is set when the rxudp_err_frms counter reaches half of the maximum value or the maximum value."]
pub type RXUDPERFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXUDPGFIS` reader - MMC Receive UDP Good Frame Counter Interrupt Status This bit is set when the rxudp_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXUDPGFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXUDPGFIS` writer - MMC Receive UDP Good Frame Counter Interrupt Status This bit is set when the rxudp_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXUDPGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV6NOPAYFIS` reader - MMC Receive IPV6 No Payload Frame Counter Interrupt Status This bit is set when the rxipv6_nopay_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6NOPAYFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV6NOPAYFIS` writer - MMC Receive IPV6 No Payload Frame Counter Interrupt Status This bit is set when the rxipv6_nopay_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6NOPAYFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV6HERFIS` reader - MMC Receive IPV6 Header Error Frame Counter Interrupt Status This bit is set when the rxipv6_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6HERFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV6HERFIS` writer - MMC Receive IPV6 Header Error Frame Counter Interrupt Status This bit is set when the rxipv6_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6HERFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV6GFIS` reader - MMC Receive IPV6 Good Frame Counter Interrupt Status This bit is set when the rxipv6_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6GFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV6GFIS` writer - MMC Receive IPV6 Good Frame Counter Interrupt Status This bit is set when the rxipv6_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV6GFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4UDSBLFIS` reader - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Status This bit is set when the rxipv4_udsbl_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4UDSBLFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4UDSBLFIS` writer - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Status This bit is set when the rxipv4_udsbl_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4UDSBLFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4FRAGFIS` reader - MMC Receive IPV4 Fragmented Frame Counter Interrupt Status This bit is set when the rxipv4_frag_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4FRAGFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4FRAGFIS` writer - MMC Receive IPV4 Fragmented Frame Counter Interrupt Status This bit is set when the rxipv4_frag_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4FRAGFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4NOPAYFIS` reader - MMC Receive IPV4 No Payload Frame Counter Interrupt Status This bit is set when the rxipv4_nopay_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4NOPAYFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4NOPAYFIS` writer - MMC Receive IPV4 No Payload Frame Counter Interrupt Status This bit is set when the rxipv4_nopay_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4NOPAYFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4HERFIS` reader - MMC Receive IPV4 Header Error Frame Counter Interrupt Status This bit is set when the rxipv4_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4HERFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4HERFIS` writer - MMC Receive IPV4 Header Error Frame Counter Interrupt Status This bit is set when the rxipv4_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4HERFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXIPV4GFIS` reader - MMC Receive IPV4 Good Frame Counter Interrupt Status This bit is set when the rxipv4_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4GFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXIPV4GFIS` writer - MMC Receive IPV4 Good Frame Counter Interrupt Status This bit is set when the rxipv4_gd_frms counter reaches half of the maximum value or the maximum value."]
pub type RXIPV4GFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_IPC_INTR_RX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Status This bit is set when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmperois(&self) -> RXICMPEROIS_R {
        RXICMPEROIS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Status This bit is set when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmpgois(&self) -> RXICMPGOIS_R {
        RXICMPGOIS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Status This bit is set when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcperois(&self) -> RXTCPEROIS_R {
        RXTCPEROIS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Status This bit is set when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value"]
    #[inline(always)]
    pub fn rxtcpgois(&self) -> RXTCPGOIS_R {
        RXTCPGOIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Status This bit is set when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudperois(&self) -> RXUDPEROIS_R {
        RXUDPEROIS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Status This bit is set when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudpgois(&self) -> RXUDPGOIS_R {
        RXUDPGOIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Status This bit is set when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6nopayois(&self) -> RXIPV6NOPAYOIS_R {
        RXIPV6NOPAYOIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Status This bit is set when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6herois(&self) -> RXIPV6HEROIS_R {
        RXIPV6HEROIS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Status This bit is set when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6gois(&self) -> RXIPV6GOIS_R {
        RXIPV6GOIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status This bit is set when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4udsblois(&self) -> RXIPV4UDSBLOIS_R {
        RXIPV4UDSBLOIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Status This bit is set when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4fragois(&self) -> RXIPV4FRAGOIS_R {
        RXIPV4FRAGOIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Status This bit is set when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4nopayois(&self) -> RXIPV4NOPAYOIS_R {
        RXIPV4NOPAYOIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Status This bit is set when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4herois(&self) -> RXIPV4HEROIS_R {
        RXIPV4HEROIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Status This bit is set when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4gois(&self) -> RXIPV4GOIS_R {
        RXIPV4GOIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Status This bit is set when the rxicmp_err_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmperfis(&self) -> RXICMPERFIS_R {
        RXICMPERFIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Status This bit is set when the rxicmp_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmpgfis(&self) -> RXICMPGFIS_R {
        RXICMPGFIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Status This bit is set when the rxtcp_err_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcperfis(&self) -> RXTCPERFIS_R {
        RXTCPERFIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Status This bit is set when the rxtcp_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcpgfis(&self) -> RXTCPGFIS_R {
        RXTCPGFIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Status This bit is set when the rxudp_err_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudperfis(&self) -> RXUDPERFIS_R {
        RXUDPERFIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Status This bit is set when the rxudp_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudpgfis(&self) -> RXUDPGFIS_R {
        RXUDPGFIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Status This bit is set when the rxipv6_nopay_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6nopayfis(&self) -> RXIPV6NOPAYFIS_R {
        RXIPV6NOPAYFIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Status This bit is set when the rxipv6_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6herfis(&self) -> RXIPV6HERFIS_R {
        RXIPV6HERFIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Status This bit is set when the rxipv6_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6gfis(&self) -> RXIPV6GFIS_R {
        RXIPV6GFIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Status This bit is set when the rxipv4_udsbl_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4udsblfis(&self) -> RXIPV4UDSBLFIS_R {
        RXIPV4UDSBLFIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Status This bit is set when the rxipv4_frag_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4fragfis(&self) -> RXIPV4FRAGFIS_R {
        RXIPV4FRAGFIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Status This bit is set when the rxipv4_nopay_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4nopayfis(&self) -> RXIPV4NOPAYFIS_R {
        RXIPV4NOPAYFIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Status This bit is set when the rxipv4_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4herfis(&self) -> RXIPV4HERFIS_R {
        RXIPV4HERFIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Status This bit is set when the rxipv4_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4gfis(&self) -> RXIPV4GFIS_R {
        RXIPV4GFIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - MMC Receive ICMP Error Octet Counter Interrupt Status This bit is set when the rxicmp_err_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmperois(&mut self) -> RXICMPEROIS_W<29> {
        RXICMPEROIS_W::new(self)
    }
    #[doc = "Bit 28 - MMC Receive ICMP Good Octet Counter Interrupt Status This bit is set when the rxicmp_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmpgois(&mut self) -> RXICMPGOIS_W<28> {
        RXICMPGOIS_W::new(self)
    }
    #[doc = "Bit 27 - MMC Receive TCP Error Octet Counter Interrupt Status This bit is set when the rxtcp_err_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcperois(&mut self) -> RXTCPEROIS_W<27> {
        RXTCPEROIS_W::new(self)
    }
    #[doc = "Bit 26 - MMC Receive TCP Good Octet Counter Interrupt Status This bit is set when the rxtcp_gd_octets counter reaches half of the maximum value or the maximum value"]
    #[inline(always)]
    pub fn rxtcpgois(&mut self) -> RXTCPGOIS_W<26> {
        RXTCPGOIS_W::new(self)
    }
    #[doc = "Bit 25 - MMC Receive UDP Error Octet Counter Interrupt Status This bit is set when the rxudp_err_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudperois(&mut self) -> RXUDPEROIS_W<25> {
        RXUDPEROIS_W::new(self)
    }
    #[doc = "Bit 24 - MMC Receive UDP Good Octet Counter Interrupt Status This bit is set when the rxudp_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudpgois(&mut self) -> RXUDPGOIS_W<24> {
        RXUDPGOIS_W::new(self)
    }
    #[doc = "Bit 23 - MMC Receive IPV6 No Payload Octet Counter Interrupt Status This bit is set when the rxipv6_nopay_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6nopayois(&mut self) -> RXIPV6NOPAYOIS_W<23> {
        RXIPV6NOPAYOIS_W::new(self)
    }
    #[doc = "Bit 22 - MMC Receive IPV6 Header Error Octet Counter Interrupt Status This bit is set when the rxipv6_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6herois(&mut self) -> RXIPV6HEROIS_W<22> {
        RXIPV6HEROIS_W::new(self)
    }
    #[doc = "Bit 21 - MMC Receive IPV6 Good Octet Counter Interrupt Status This bit is set when the rxipv6_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6gois(&mut self) -> RXIPV6GOIS_W<21> {
        RXIPV6GOIS_W::new(self)
    }
    #[doc = "Bit 20 - MMC Receive IPV4 UDP Checksum Disabled Octet Counter Interrupt Status This bit is set when the rxipv4_udsbl_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4udsblois(&mut self) -> RXIPV4UDSBLOIS_W<20> {
        RXIPV4UDSBLOIS_W::new(self)
    }
    #[doc = "Bit 19 - MMC Receive IPV4 Fragmented Octet Counter Interrupt Status This bit is set when the rxipv4_frag_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4fragois(&mut self) -> RXIPV4FRAGOIS_W<19> {
        RXIPV4FRAGOIS_W::new(self)
    }
    #[doc = "Bit 18 - MMC Receive IPV4 No Payload Octet Counter Interrupt Status This bit is set when the rxipv4_nopay_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4nopayois(&mut self) -> RXIPV4NOPAYOIS_W<18> {
        RXIPV4NOPAYOIS_W::new(self)
    }
    #[doc = "Bit 17 - MMC Receive IPV4 Header Error Octet Counter Interrupt Status This bit is set when the rxipv4_hdrerr_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4herois(&mut self) -> RXIPV4HEROIS_W<17> {
        RXIPV4HEROIS_W::new(self)
    }
    #[doc = "Bit 16 - MMC Receive IPV4 Good Octet Counter Interrupt Status This bit is set when the rxipv4_gd_octets counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4gois(&mut self) -> RXIPV4GOIS_W<16> {
        RXIPV4GOIS_W::new(self)
    }
    #[doc = "Bit 13 - MMC Receive ICMP Error Frame Counter Interrupt Status This bit is set when the rxicmp_err_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmperfis(&mut self) -> RXICMPERFIS_W<13> {
        RXICMPERFIS_W::new(self)
    }
    #[doc = "Bit 12 - MMC Receive ICMP Good Frame Counter Interrupt Status This bit is set when the rxicmp_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxicmpgfis(&mut self) -> RXICMPGFIS_W<12> {
        RXICMPGFIS_W::new(self)
    }
    #[doc = "Bit 11 - MMC Receive TCP Error Frame Counter Interrupt Status This bit is set when the rxtcp_err_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcperfis(&mut self) -> RXTCPERFIS_W<11> {
        RXTCPERFIS_W::new(self)
    }
    #[doc = "Bit 10 - MMC Receive TCP Good Frame Counter Interrupt Status This bit is set when the rxtcp_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxtcpgfis(&mut self) -> RXTCPGFIS_W<10> {
        RXTCPGFIS_W::new(self)
    }
    #[doc = "Bit 9 - MMC Receive UDP Error Frame Counter Interrupt Status This bit is set when the rxudp_err_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudperfis(&mut self) -> RXUDPERFIS_W<9> {
        RXUDPERFIS_W::new(self)
    }
    #[doc = "Bit 8 - MMC Receive UDP Good Frame Counter Interrupt Status This bit is set when the rxudp_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxudpgfis(&mut self) -> RXUDPGFIS_W<8> {
        RXUDPGFIS_W::new(self)
    }
    #[doc = "Bit 7 - MMC Receive IPV6 No Payload Frame Counter Interrupt Status This bit is set when the rxipv6_nopay_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6nopayfis(&mut self) -> RXIPV6NOPAYFIS_W<7> {
        RXIPV6NOPAYFIS_W::new(self)
    }
    #[doc = "Bit 6 - MMC Receive IPV6 Header Error Frame Counter Interrupt Status This bit is set when the rxipv6_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6herfis(&mut self) -> RXIPV6HERFIS_W<6> {
        RXIPV6HERFIS_W::new(self)
    }
    #[doc = "Bit 5 - MMC Receive IPV6 Good Frame Counter Interrupt Status This bit is set when the rxipv6_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv6gfis(&mut self) -> RXIPV6GFIS_W<5> {
        RXIPV6GFIS_W::new(self)
    }
    #[doc = "Bit 4 - MMC Receive IPV4 UDP Checksum Disabled Frame Counter Interrupt Status This bit is set when the rxipv4_udsbl_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4udsblfis(&mut self) -> RXIPV4UDSBLFIS_W<4> {
        RXIPV4UDSBLFIS_W::new(self)
    }
    #[doc = "Bit 3 - MMC Receive IPV4 Fragmented Frame Counter Interrupt Status This bit is set when the rxipv4_frag_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4fragfis(&mut self) -> RXIPV4FRAGFIS_W<3> {
        RXIPV4FRAGFIS_W::new(self)
    }
    #[doc = "Bit 2 - MMC Receive IPV4 No Payload Frame Counter Interrupt Status This bit is set when the rxipv4_nopay_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4nopayfis(&mut self) -> RXIPV4NOPAYFIS_W<2> {
        RXIPV4NOPAYFIS_W::new(self)
    }
    #[doc = "Bit 1 - MMC Receive IPV4 Header Error Frame Counter Interrupt Status This bit is set when the rxipv4_hdrerr_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4herfis(&mut self) -> RXIPV4HERFIS_W<1> {
        RXIPV4HERFIS_W::new(self)
    }
    #[doc = "Bit 0 - MMC Receive IPV4 Good Frame Counter Interrupt Status This bit is set when the rxipv4_gd_frms counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxipv4gfis(&mut self) -> RXIPV4GFIS_W<0> {
        RXIPV4GFIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Receive Checksum Offload Interrupt maintains the interrupt that the receive IPC statistic counters generate. See Table 4-25 for further detail.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_ipc_intr_rx](index.html) module"]
pub struct MMC_IPC_INTR_RX_SPEC;
impl crate::RegisterSpec for MMC_IPC_INTR_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_ipc_intr_rx::R](R) reader structure"]
impl crate::Readable for MMC_IPC_INTR_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_ipc_intr_rx::W](W) writer structure"]
impl crate::Writable for MMC_IPC_INTR_RX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMC_IPC_INTR_RX to value 0"]
impl crate::Resettable for MMC_IPC_INTR_RX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
