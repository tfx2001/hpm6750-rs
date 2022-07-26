#[doc = "Register `PKTCTL` reader"]
pub struct R(crate::R<PKTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKTCTL` writer"]
pub struct W(crate::W<PKTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKTCTL_SPEC>;
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
impl From<crate::W<PKTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKTTAG` reader - packet tag"]
pub type PKTTAG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PKTTAG` writer - packet tag"]
pub type PKTTAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PKTCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CIPHIV` reader - Load Initial Vector for the AES in this packet."]
pub type CIPHIV_R = crate::BitReader<bool>;
#[doc = "Field `CIPHIV` writer - Load Initial Vector for the AES in this packet."]
pub type CIPHIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKTCTL_SPEC, bool, O>;
#[doc = "Field `HASFNL` reader - Hash Termination packet"]
pub type HASFNL_R = crate::BitReader<bool>;
#[doc = "Field `HASFNL` writer - Hash Termination packet"]
pub type HASFNL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKTCTL_SPEC, bool, O>;
#[doc = "Field `HASINI` reader - Hash Initialization packat"]
pub type HASINI_R = crate::BitReader<bool>;
#[doc = "Field `HASINI` writer - Hash Initialization packat"]
pub type HASINI_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKTCTL_SPEC, bool, O>;
#[doc = "Field `CHAIN` reader - whether the next command pointer register must be loaded into the channel's current descriptor pointer."]
pub type CHAIN_R = crate::BitReader<bool>;
#[doc = "Field `CHAIN` writer - whether the next command pointer register must be loaded into the channel's current descriptor pointer."]
pub type CHAIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKTCTL_SPEC, bool, O>;
#[doc = "Field `DCRSEMA` reader - whether the channel's semaphore must be decremented at the end of the current operation. When the semaphore reaches a value of zero, no more operations are issued from the channel."]
pub type DCRSEMA_R = crate::BitReader<bool>;
#[doc = "Field `DCRSEMA` writer - whether the channel's semaphore must be decremented at the end of the current operation. When the semaphore reaches a value of zero, no more operations are issued from the channel."]
pub type DCRSEMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKTCTL_SPEC, bool, O>;
#[doc = "Field `PKTINT` reader - Reflects whether the channel must issue an interrupt upon the completion of the packet"]
pub type PKTINT_R = crate::BitReader<bool>;
#[doc = "Field `PKTINT` writer - Reflects whether the channel must issue an interrupt upon the completion of the packet"]
pub type PKTINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKTCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 24:31 - packet tag"]
    #[inline(always)]
    pub fn pkttag(&self) -> PKTTAG_R {
        PKTTAG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 6 - Load Initial Vector for the AES in this packet."]
    #[inline(always)]
    pub fn ciphiv(&self) -> CIPHIV_R {
        CIPHIV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Hash Termination packet"]
    #[inline(always)]
    pub fn hasfnl(&self) -> HASFNL_R {
        HASFNL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Hash Initialization packat"]
    #[inline(always)]
    pub fn hasini(&self) -> HASINI_R {
        HASINI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - whether the next command pointer register must be loaded into the channel's current descriptor pointer."]
    #[inline(always)]
    pub fn chain(&self) -> CHAIN_R {
        CHAIN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - whether the channel's semaphore must be decremented at the end of the current operation. When the semaphore reaches a value of zero, no more operations are issued from the channel."]
    #[inline(always)]
    pub fn dcrsema(&self) -> DCRSEMA_R {
        DCRSEMA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Reflects whether the channel must issue an interrupt upon the completion of the packet"]
    #[inline(always)]
    pub fn pktint(&self) -> PKTINT_R {
        PKTINT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - packet tag"]
    #[inline(always)]
    pub fn pkttag(&mut self) -> PKTTAG_W<24> {
        PKTTAG_W::new(self)
    }
    #[doc = "Bit 6 - Load Initial Vector for the AES in this packet."]
    #[inline(always)]
    pub fn ciphiv(&mut self) -> CIPHIV_W<6> {
        CIPHIV_W::new(self)
    }
    #[doc = "Bit 5 - Hash Termination packet"]
    #[inline(always)]
    pub fn hasfnl(&mut self) -> HASFNL_W<5> {
        HASFNL_W::new(self)
    }
    #[doc = "Bit 4 - Hash Initialization packat"]
    #[inline(always)]
    pub fn hasini(&mut self) -> HASINI_W<4> {
        HASINI_W::new(self)
    }
    #[doc = "Bit 3 - whether the next command pointer register must be loaded into the channel's current descriptor pointer."]
    #[inline(always)]
    pub fn chain(&mut self) -> CHAIN_W<3> {
        CHAIN_W::new(self)
    }
    #[doc = "Bit 2 - whether the channel's semaphore must be decremented at the end of the current operation. When the semaphore reaches a value of zero, no more operations are issued from the channel."]
    #[inline(always)]
    pub fn dcrsema(&mut self) -> DCRSEMA_W<2> {
        DCRSEMA_W::new(self)
    }
    #[doc = "Bit 1 - Reflects whether the channel must issue an interrupt upon the completion of the packet"]
    #[inline(always)]
    pub fn pktint(&mut self) -> PKTINT_W<1> {
        PKTINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pktctl](index.html) module"]
pub struct PKTCTL_SPEC;
impl crate::RegisterSpec for PKTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pktctl::R](R) reader structure"]
impl crate::Readable for PKTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pktctl::W](W) writer structure"]
impl crate::Writable for PKTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKTCTL to value 0"]
impl crate::Resettable for PKTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
