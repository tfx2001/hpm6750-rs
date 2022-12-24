#[doc = "Register `REF_MSG` reader"]
pub struct R(crate::R<REF_MSG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REF_MSG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REF_MSG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REF_MSG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REF_MSG` writer"]
pub struct W(crate::W<REF_MSG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REF_MSG_SPEC>;
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
impl From<crate::W<REF_MSG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REF_MSG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REF_MSG` reader - REFerence message IDentifier. If REF_IDE is 1 - REF_ID(28:0) is valid (extended ID) 0 - REF_ID(10:0) is valid (standard ID) REF_ID is used in TTCAN mode to detect a reference message. This holds for time slaves (reception) as well as for the time master (transmission). If the reference message is detected and there are no errors, then the Sync_Mark of this frame will become the Ref_Mark. REF_ID(2:0) is not tested and therefore the appropriate register bits are forced to 0. These bits are used for up to 8 potential time masters. CAN-CTRL recognizes the reference message only by ID. The payload is not tested. Additional note: A time master will transmit a reference message in the same way as a normal frame. REF_ID is intended for detection of a successful transmission of a reference message."]
pub type REF_MSG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REF_MSG` writer - REFerence message IDentifier. If REF_IDE is 1 - REF_ID(28:0) is valid (extended ID) 0 - REF_ID(10:0) is valid (standard ID) REF_ID is used in TTCAN mode to detect a reference message. This holds for time slaves (reception) as well as for the time master (transmission). If the reference message is detected and there are no errors, then the Sync_Mark of this frame will become the Ref_Mark. REF_ID(2:0) is not tested and therefore the appropriate register bits are forced to 0. These bits are used for up to 8 potential time masters. CAN-CTRL recognizes the reference message only by ID. The payload is not tested. Additional note: A time master will transmit a reference message in the same way as a normal frame. REF_ID is intended for detection of a successful transmission of a reference message."]
pub type REF_MSG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REF_MSG_SPEC, u32, u32, 29, O>;
#[doc = "Field `REF_IDE` reader - REFerence message IDE bit."]
pub type REF_IDE_R = crate::BitReader<bool>;
#[doc = "Field `REF_IDE` writer - REFerence message IDE bit."]
pub type REF_IDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, REF_MSG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:28 - REFerence message IDentifier. If REF_IDE is 1 - REF_ID(28:0) is valid (extended ID) 0 - REF_ID(10:0) is valid (standard ID) REF_ID is used in TTCAN mode to detect a reference message. This holds for time slaves (reception) as well as for the time master (transmission). If the reference message is detected and there are no errors, then the Sync_Mark of this frame will become the Ref_Mark. REF_ID(2:0) is not tested and therefore the appropriate register bits are forced to 0. These bits are used for up to 8 potential time masters. CAN-CTRL recognizes the reference message only by ID. The payload is not tested. Additional note: A time master will transmit a reference message in the same way as a normal frame. REF_ID is intended for detection of a successful transmission of a reference message."]
    #[inline(always)]
    pub fn ref_msg(&self) -> REF_MSG_R {
        REF_MSG_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 31 - REFerence message IDE bit."]
    #[inline(always)]
    pub fn ref_ide(&self) -> REF_IDE_R {
        REF_IDE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - REFerence message IDentifier. If REF_IDE is 1 - REF_ID(28:0) is valid (extended ID) 0 - REF_ID(10:0) is valid (standard ID) REF_ID is used in TTCAN mode to detect a reference message. This holds for time slaves (reception) as well as for the time master (transmission). If the reference message is detected and there are no errors, then the Sync_Mark of this frame will become the Ref_Mark. REF_ID(2:0) is not tested and therefore the appropriate register bits are forced to 0. These bits are used for up to 8 potential time masters. CAN-CTRL recognizes the reference message only by ID. The payload is not tested. Additional note: A time master will transmit a reference message in the same way as a normal frame. REF_ID is intended for detection of a successful transmission of a reference message."]
    #[inline(always)]
    #[must_use]
    pub fn ref_msg(&mut self) -> REF_MSG_W<0> {
        REF_MSG_W::new(self)
    }
    #[doc = "Bit 31 - REFerence message IDE bit."]
    #[inline(always)]
    #[must_use]
    pub fn ref_ide(&mut self) -> REF_IDE_W<31> {
        REF_IDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TTCAN: Reference Message REF_MSG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_msg](index.html) module"]
pub struct REF_MSG_SPEC;
impl crate::RegisterSpec for REF_MSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ref_msg::R](R) reader structure"]
impl crate::Readable for REF_MSG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ref_msg::W](W) writer structure"]
impl crate::Writable for REF_MSG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REF_MSG to value 0"]
impl crate::Resettable for REF_MSG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
