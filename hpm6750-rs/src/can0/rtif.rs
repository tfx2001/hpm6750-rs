#[doc = "Register `RTIF` reader"]
pub struct R(crate::R<RTIF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTIF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTIF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTIF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTIF` writer"]
pub struct W(crate::W<RTIF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTIF_SPEC>;
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
impl From<crate::W<RTIF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTIF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIF` reader - Receive Interrupt Flag 1 - Data or a remote frame has been received and is available in the receive buffer. 0 - No frame has been received."]
pub type RIF_R = crate::BitReader<bool>;
#[doc = "Field `RIF` writer - Receive Interrupt Flag 1 - Data or a remote frame has been received and is available in the receive buffer. 0 - No frame has been received."]
pub type RIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `ROIF` reader - RB Overrun Interrupt Flag 1 - At least one received message has been overwritten in the RB. 0 - No RB overwritten. In case of an overrun both ROIF and RFIF will be set."]
pub type ROIF_R = crate::BitReader<bool>;
#[doc = "Field `ROIF` writer - RB Overrun Interrupt Flag 1 - At least one received message has been overwritten in the RB. 0 - No RB overwritten. In case of an overrun both ROIF and RFIF will be set."]
pub type ROIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `RFIF` reader - RB Full Interrupt Flag 1 - All RBs are full. If no RB will be released until the next valid message is received, the oldest message will be lost. 0 - The RB FIFO is not full."]
pub type RFIF_R = crate::BitReader<bool>;
#[doc = "Field `RFIF` writer - RB Full Interrupt Flag 1 - All RBs are full. If no RB will be released until the next valid message is received, the oldest message will be lost. 0 - The RB FIFO is not full."]
pub type RFIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `RAFIF` reader - RB Almost Full Interrupt Flag 1 - number of filled RB slots >= AFWL_i 0 - number of filled RB slots < AFWL_i"]
pub type RAFIF_R = crate::BitReader<bool>;
#[doc = "Field `RAFIF` writer - RB Almost Full Interrupt Flag 1 - number of filled RB slots >= AFWL_i 0 - number of filled RB slots < AFWL_i"]
pub type RAFIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `TPIF` reader - Transmission Primary Interrupt Flag 1 - The requested transmission of the PTB has been successfully completed. 0 - No transmission of the PTB has been completed. In TTCAN mode, TPIF will never be set. Then only TSIF is valid."]
pub type TPIF_R = crate::BitReader<bool>;
#[doc = "Field `TPIF` writer - Transmission Primary Interrupt Flag 1 - The requested transmission of the PTB has been successfully completed. 0 - No transmission of the PTB has been completed. In TTCAN mode, TPIF will never be set. Then only TSIF is valid."]
pub type TPIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `TSIF` reader - Transmission Secondary Interrupt Flag 1 - The requested transmission of the STB has been successfully completed. 0 - No transmission of the STB has been completed successfully. In TTCAN mode TSIF will signal all successful transmissions, regardless of storage location of the message."]
pub type TSIF_R = crate::BitReader<bool>;
#[doc = "Field `TSIF` writer - Transmission Secondary Interrupt Flag 1 - The requested transmission of the STB has been successfully completed. 0 - No transmission of the STB has been completed successfully. In TTCAN mode TSIF will signal all successful transmissions, regardless of storage location of the message."]
pub type TSIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `EIF` reader - Error Interrupt Flag 1 - The border of the error warning limit has been crossed in either direction, or the BUSOFF bit has been changed in either direction. 0 - There has been no change."]
pub type EIF_R = crate::BitReader<bool>;
#[doc = "Field `EIF` writer - Error Interrupt Flag 1 - The border of the error warning limit has been crossed in either direction, or the BUSOFF bit has been changed in either direction. 0 - There has been no change."]
pub type EIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
#[doc = "Field `AIF` reader - Abort Interrupt Flag 1 - After setting TPA or TSA the appropriated message(s) have been aborted. It is recommended to not set both TPA and TSA simultaneously because both source AIF. 0 - No abort has been executed. The AIF does not have an associated enable register."]
pub type AIF_R = crate::BitReader<bool>;
#[doc = "Field `AIF` writer - Abort Interrupt Flag 1 - After setting TPA or TSA the appropriated message(s) have been aborted. It is recommended to not set both TPA and TSA simultaneously because both source AIF. 0 - No abort has been executed. The AIF does not have an associated enable register."]
pub type AIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RTIF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - Receive Interrupt Flag 1 - Data or a remote frame has been received and is available in the receive buffer. 0 - No frame has been received."]
    #[inline(always)]
    pub fn rif(&self) -> RIF_R {
        RIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - RB Overrun Interrupt Flag 1 - At least one received message has been overwritten in the RB. 0 - No RB overwritten. In case of an overrun both ROIF and RFIF will be set."]
    #[inline(always)]
    pub fn roif(&self) -> ROIF_R {
        ROIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - RB Full Interrupt Flag 1 - All RBs are full. If no RB will be released until the next valid message is received, the oldest message will be lost. 0 - The RB FIFO is not full."]
    #[inline(always)]
    pub fn rfif(&self) -> RFIF_R {
        RFIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - RB Almost Full Interrupt Flag 1 - number of filled RB slots >= AFWL_i 0 - number of filled RB slots < AFWL_i"]
    #[inline(always)]
    pub fn rafif(&self) -> RAFIF_R {
        RAFIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Primary Interrupt Flag 1 - The requested transmission of the PTB has been successfully completed. 0 - No transmission of the PTB has been completed. In TTCAN mode, TPIF will never be set. Then only TSIF is valid."]
    #[inline(always)]
    pub fn tpif(&self) -> TPIF_R {
        TPIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Secondary Interrupt Flag 1 - The requested transmission of the STB has been successfully completed. 0 - No transmission of the STB has been completed successfully. In TTCAN mode TSIF will signal all successful transmissions, regardless of storage location of the message."]
    #[inline(always)]
    pub fn tsif(&self) -> TSIF_R {
        TSIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Error Interrupt Flag 1 - The border of the error warning limit has been crossed in either direction, or the BUSOFF bit has been changed in either direction. 0 - There has been no change."]
    #[inline(always)]
    pub fn eif(&self) -> EIF_R {
        EIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Abort Interrupt Flag 1 - After setting TPA or TSA the appropriated message(s) have been aborted. It is recommended to not set both TPA and TSA simultaneously because both source AIF. 0 - No abort has been executed. The AIF does not have an associated enable register."]
    #[inline(always)]
    pub fn aif(&self) -> AIF_R {
        AIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Receive Interrupt Flag 1 - Data or a remote frame has been received and is available in the receive buffer. 0 - No frame has been received."]
    #[inline(always)]
    pub fn rif(&mut self) -> RIF_W<7> {
        RIF_W::new(self)
    }
    #[doc = "Bit 6 - RB Overrun Interrupt Flag 1 - At least one received message has been overwritten in the RB. 0 - No RB overwritten. In case of an overrun both ROIF and RFIF will be set."]
    #[inline(always)]
    pub fn roif(&mut self) -> ROIF_W<6> {
        ROIF_W::new(self)
    }
    #[doc = "Bit 5 - RB Full Interrupt Flag 1 - All RBs are full. If no RB will be released until the next valid message is received, the oldest message will be lost. 0 - The RB FIFO is not full."]
    #[inline(always)]
    pub fn rfif(&mut self) -> RFIF_W<5> {
        RFIF_W::new(self)
    }
    #[doc = "Bit 4 - RB Almost Full Interrupt Flag 1 - number of filled RB slots >= AFWL_i 0 - number of filled RB slots < AFWL_i"]
    #[inline(always)]
    pub fn rafif(&mut self) -> RAFIF_W<4> {
        RAFIF_W::new(self)
    }
    #[doc = "Bit 3 - Transmission Primary Interrupt Flag 1 - The requested transmission of the PTB has been successfully completed. 0 - No transmission of the PTB has been completed. In TTCAN mode, TPIF will never be set. Then only TSIF is valid."]
    #[inline(always)]
    pub fn tpif(&mut self) -> TPIF_W<3> {
        TPIF_W::new(self)
    }
    #[doc = "Bit 2 - Transmission Secondary Interrupt Flag 1 - The requested transmission of the STB has been successfully completed. 0 - No transmission of the STB has been completed successfully. In TTCAN mode TSIF will signal all successful transmissions, regardless of storage location of the message."]
    #[inline(always)]
    pub fn tsif(&mut self) -> TSIF_W<2> {
        TSIF_W::new(self)
    }
    #[doc = "Bit 1 - Error Interrupt Flag 1 - The border of the error warning limit has been crossed in either direction, or the BUSOFF bit has been changed in either direction. 0 - There has been no change."]
    #[inline(always)]
    pub fn eif(&mut self) -> EIF_W<1> {
        EIF_W::new(self)
    }
    #[doc = "Bit 0 - Abort Interrupt Flag 1 - After setting TPA or TSA the appropriated message(s) have been aborted. It is recommended to not set both TPA and TSA simultaneously because both source AIF. 0 - No abort has been executed. The AIF does not have an associated enable register."]
    #[inline(always)]
    pub fn aif(&mut self) -> AIF_W<0> {
        AIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive and Transmit Interrupt Flag Register RTIF (0xa5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtif](index.html) module"]
pub struct RTIF_SPEC;
impl crate::RegisterSpec for RTIF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rtif::R](R) reader structure"]
impl crate::Readable for RTIF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtif::W](W) writer structure"]
impl crate::Writable for RTIF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTIF to value 0"]
impl crate::Resettable for RTIF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
