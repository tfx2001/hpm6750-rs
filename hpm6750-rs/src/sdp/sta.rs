#[doc = "Register `STA` reader"]
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STA` writer"]
pub struct W(crate::W<STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STA_SPEC>;
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
impl From<crate::W<STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERRCHAIN` reader - buffer chain error happen when packet's CHAIN bit=0, but the Packet counter is still not zero."]
pub type ERRCHAIN_R = crate::BitReader<bool>;
#[doc = "Field `ERRCHAIN` writer - buffer chain error happen when packet's CHAIN bit=0, but the Packet counter is still not zero."]
pub type ERRCHAIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `ERRHAS` reader - Hashing Check Error"]
pub type ERRHAS_R = crate::BitReader<bool>;
#[doc = "Field `ERRHAS` writer - Hashing Check Error"]
pub type ERRHAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `ERRDST` reader - Destination Buffer Error"]
pub type ERRDST_R = crate::BitReader<bool>;
#[doc = "Field `ERRDST` writer - Destination Buffer Error"]
pub type ERRDST_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `ERRSRC` reader - Source Buffer Access Error"]
pub type ERRSRC_R = crate::BitReader<bool>;
#[doc = "Field `ERRSRC` writer - Source Buffer Access Error"]
pub type ERRSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `ERRPKT` reader - Packet head access error, or status update error."]
pub type ERRPKT_R = crate::BitReader<bool>;
#[doc = "Field `ERRPKT` writer - Packet head access error, or status update error."]
pub type ERRPKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `ERRSET` reader - Working mode setup error."]
pub type ERRSET_R = crate::BitReader<bool>;
#[doc = "Field `ERRSET` writer - Working mode setup error."]
pub type ERRSET_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `PKTDON` reader - Packet processing done, will trigger this itnerrrupt when the \"PKTINT\" bit set in the packet control word."]
pub type PKTDON_R = crate::BitReader<bool>;
#[doc = "Field `PKTDON` writer - Packet processing done, will trigger this itnerrrupt when the \"PKTINT\" bit set in the packet control word."]
pub type PKTDON_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `PKTCNT0` reader - Packet Counter registers reachs to ZERO now."]
pub type PKTCNT0_R = crate::BitReader<bool>;
#[doc = "Field `PKTCNT0` writer - Packet Counter registers reachs to ZERO now."]
pub type PKTCNT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `HASBSY` reader - Hashing Busy"]
pub type HASBSY_R = crate::BitReader<bool>;
#[doc = "Field `AESBSY` reader - AES Busy"]
pub type AESBSY_R = crate::BitReader<bool>;
#[doc = "Field `CHN1PKT0` reader - the chain buffer \"chain\" bit is \"1\", while packet counter is \"0\", now, waiting for new buffer data."]
pub type CHN1PKT0_R = crate::BitReader<bool>;
#[doc = "Field `CHN1PKT0` writer - the chain buffer \"chain\" bit is \"1\", while packet counter is \"0\", now, waiting for new buffer data."]
pub type CHN1PKT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `IRQ` reader - interrupt Request, requested when error happen, or when packet processing done, packet counter reach to zero."]
pub type IRQ_R = crate::BitReader<bool>;
#[doc = "Field `IRQ` writer - interrupt Request, requested when error happen, or when packet processing done, packet counter reach to zero."]
pub type IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, STA_SPEC, bool, O>;
#[doc = "Field `TAG` reader - packet tag."]
pub type TAG_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - buffer chain error happen when packet's CHAIN bit=0, but the Packet counter is still not zero."]
    #[inline(always)]
    pub fn errchain(&self) -> ERRCHAIN_R {
        ERRCHAIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hashing Check Error"]
    #[inline(always)]
    pub fn errhas(&self) -> ERRHAS_R {
        ERRHAS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Destination Buffer Error"]
    #[inline(always)]
    pub fn errdst(&self) -> ERRDST_R {
        ERRDST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Source Buffer Access Error"]
    #[inline(always)]
    pub fn errsrc(&self) -> ERRSRC_R {
        ERRSRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Packet head access error, or status update error."]
    #[inline(always)]
    pub fn errpkt(&self) -> ERRPKT_R {
        ERRPKT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Working mode setup error."]
    #[inline(always)]
    pub fn errset(&self) -> ERRSET_R {
        ERRSET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Packet processing done, will trigger this itnerrrupt when the \"PKTINT\" bit set in the packet control word."]
    #[inline(always)]
    pub fn pktdon(&self) -> PKTDON_R {
        PKTDON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Packet Counter registers reachs to ZERO now."]
    #[inline(always)]
    pub fn pktcnt0(&self) -> PKTCNT0_R {
        PKTCNT0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Hashing Busy"]
    #[inline(always)]
    pub fn hasbsy(&self) -> HASBSY_R {
        HASBSY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - AES Busy"]
    #[inline(always)]
    pub fn aesbsy(&self) -> AESBSY_R {
        AESBSY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - the chain buffer \"chain\" bit is \"1\", while packet counter is \"0\", now, waiting for new buffer data."]
    #[inline(always)]
    pub fn chn1pkt0(&self) -> CHN1PKT0_R {
        CHN1PKT0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - interrupt Request, requested when error happen, or when packet processing done, packet counter reach to zero."]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - packet tag."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - buffer chain error happen when packet's CHAIN bit=0, but the Packet counter is still not zero."]
    #[inline(always)]
    #[must_use]
    pub fn errchain(&mut self) -> ERRCHAIN_W<0> {
        ERRCHAIN_W::new(self)
    }
    #[doc = "Bit 1 - Hashing Check Error"]
    #[inline(always)]
    #[must_use]
    pub fn errhas(&mut self) -> ERRHAS_W<1> {
        ERRHAS_W::new(self)
    }
    #[doc = "Bit 2 - Destination Buffer Error"]
    #[inline(always)]
    #[must_use]
    pub fn errdst(&mut self) -> ERRDST_W<2> {
        ERRDST_W::new(self)
    }
    #[doc = "Bit 3 - Source Buffer Access Error"]
    #[inline(always)]
    #[must_use]
    pub fn errsrc(&mut self) -> ERRSRC_W<3> {
        ERRSRC_W::new(self)
    }
    #[doc = "Bit 4 - Packet head access error, or status update error."]
    #[inline(always)]
    #[must_use]
    pub fn errpkt(&mut self) -> ERRPKT_W<4> {
        ERRPKT_W::new(self)
    }
    #[doc = "Bit 5 - Working mode setup error."]
    #[inline(always)]
    #[must_use]
    pub fn errset(&mut self) -> ERRSET_W<5> {
        ERRSET_W::new(self)
    }
    #[doc = "Bit 16 - Packet processing done, will trigger this itnerrrupt when the \"PKTINT\" bit set in the packet control word."]
    #[inline(always)]
    #[must_use]
    pub fn pktdon(&mut self) -> PKTDON_W<16> {
        PKTDON_W::new(self)
    }
    #[doc = "Bit 17 - Packet Counter registers reachs to ZERO now."]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt0(&mut self) -> PKTCNT0_W<17> {
        PKTCNT0_W::new(self)
    }
    #[doc = "Bit 20 - the chain buffer \"chain\" bit is \"1\", while packet counter is \"0\", now, waiting for new buffer data."]
    #[inline(always)]
    #[must_use]
    pub fn chn1pkt0(&mut self) -> CHN1PKT0_W<20> {
        CHN1PKT0_W::new(self)
    }
    #[doc = "Bit 23 - interrupt Request, requested when error happen, or when packet processing done, packet counter reach to zero."]
    #[inline(always)]
    #[must_use]
    pub fn irq(&mut self) -> IRQ_W<23> {
        IRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](index.html) module"]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sta::R](R) reader structure"]
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sta::W](W) writer structure"]
impl crate::Writable for STA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
