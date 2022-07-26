#[doc = "Register `OTGSC` reader"]
pub struct R(crate::R<OTGSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTGSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTGSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTGSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTGSC` writer"]
pub struct W(crate::W<OTGSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTGSC_SPEC>;
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
impl From<crate::W<OTGSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTGSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASVIE` reader - ASVIE A Session Valid Interrupt Enable - Read/Write."]
pub type ASVIE_R = crate::BitReader<bool>;
#[doc = "Field `ASVIE` writer - ASVIE A Session Valid Interrupt Enable - Read/Write."]
pub type ASVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `AVVIE` reader - AVVIE A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
pub type AVVIE_R = crate::BitReader<bool>;
#[doc = "Field `AVVIE` writer - AVVIE A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
pub type AVVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `IDIE` reader - IDIE USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
pub type IDIE_R = crate::BitReader<bool>;
#[doc = "Field `IDIE` writer - IDIE USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
pub type IDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `ASVIS` reader - ASVIS A Session Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the A session valid threshold. Software must write a one to clear this bit."]
pub type ASVIS_R = crate::BitReader<bool>;
#[doc = "Field `ASVIS` writer - ASVIS A Session Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the A session valid threshold. Software must write a one to clear this bit."]
pub type ASVIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `AVVIS` reader - AVVIS A VBus Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the VBus valid threshold on an A device. Software must write a one to clear this bit."]
pub type AVVIS_R = crate::BitReader<bool>;
#[doc = "Field `AVVIS` writer - AVVIS A VBus Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the VBus valid threshold on an A device. Software must write a one to clear this bit."]
pub type AVVIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `IDIS` reader - IDIS USB ID Interrupt Status - Read/Write. This bit is set when a change on the ID input has been detected. Software must write a one to clear this bit."]
pub type IDIS_R = crate::BitReader<bool>;
#[doc = "Field `IDIS` writer - IDIS USB ID Interrupt Status - Read/Write. This bit is set when a change on the ID input has been detected. Software must write a one to clear this bit."]
pub type IDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `ASV` reader - ASV A Session Valid - Read Only. Indicates VBus is above the A session valid threshold."]
pub type ASV_R = crate::BitReader<bool>;
#[doc = "Field `AVV` reader - AVV A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold."]
pub type AVV_R = crate::BitReader<bool>;
#[doc = "Field `ID` reader - ID USB ID - Read Only. 0 = A device, 1 = B device"]
pub type ID_R = crate::BitReader<bool>;
#[doc = "Field `IDPU` reader - IDPU ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]. When this bit is 0, the ID input will not be sampled."]
pub type IDPU_R = crate::BitReader<bool>;
#[doc = "Field `IDPU` writer - IDPU ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]. When this bit is 0, the ID input will not be sampled."]
pub type IDPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `VC` reader - VC VBUS Charge - Read/Write. Setting this bit causes the VBus line to be charged. This is used for VBus pulsing during SRP."]
pub type VC_R = crate::BitReader<bool>;
#[doc = "Field `VC` writer - VC VBUS Charge - Read/Write. Setting this bit causes the VBus line to be charged. This is used for VBus pulsing during SRP."]
pub type VC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `VD` reader - VD VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
pub type VD_R = crate::BitReader<bool>;
#[doc = "Field `VD` writer - VD VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
pub type VD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 26 - ASVIE A Session Valid Interrupt Enable - Read/Write."]
    #[inline(always)]
    pub fn asvie(&self) -> ASVIE_R {
        ASVIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - AVVIE A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
    #[inline(always)]
    pub fn avvie(&self) -> AVVIE_R {
        AVVIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - IDIE USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
    #[inline(always)]
    pub fn idie(&self) -> IDIE_R {
        IDIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 18 - ASVIS A Session Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the A session valid threshold. Software must write a one to clear this bit."]
    #[inline(always)]
    pub fn asvis(&self) -> ASVIS_R {
        ASVIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - AVVIS A VBus Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the VBus valid threshold on an A device. Software must write a one to clear this bit."]
    #[inline(always)]
    pub fn avvis(&self) -> AVVIS_R {
        AVVIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - IDIS USB ID Interrupt Status - Read/Write. This bit is set when a change on the ID input has been detected. Software must write a one to clear this bit."]
    #[inline(always)]
    pub fn idis(&self) -> IDIS_R {
        IDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 10 - ASV A Session Valid - Read Only. Indicates VBus is above the A session valid threshold."]
    #[inline(always)]
    pub fn asv(&self) -> ASV_R {
        ASV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - AVV A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold."]
    #[inline(always)]
    pub fn avv(&self) -> AVV_R {
        AVV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - ID USB ID - Read Only. 0 = A device, 1 = B device"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 5 - IDPU ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]. When this bit is 0, the ID input will not be sampled."]
    #[inline(always)]
    pub fn idpu(&self) -> IDPU_R {
        IDPU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 1 - VC VBUS Charge - Read/Write. Setting this bit causes the VBus line to be charged. This is used for VBus pulsing during SRP."]
    #[inline(always)]
    pub fn vc(&self) -> VC_R {
        VC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - VD VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
    #[inline(always)]
    pub fn vd(&self) -> VD_R {
        VD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - ASVIE A Session Valid Interrupt Enable - Read/Write."]
    #[inline(always)]
    pub fn asvie(&mut self) -> ASVIE_W<26> {
        ASVIE_W::new(self)
    }
    #[doc = "Bit 25 - AVVIE A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
    #[inline(always)]
    pub fn avvie(&mut self) -> AVVIE_W<25> {
        AVVIE_W::new(self)
    }
    #[doc = "Bit 24 - IDIE USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
    #[inline(always)]
    pub fn idie(&mut self) -> IDIE_W<24> {
        IDIE_W::new(self)
    }
    #[doc = "Bit 18 - ASVIS A Session Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the A session valid threshold. Software must write a one to clear this bit."]
    #[inline(always)]
    pub fn asvis(&mut self) -> ASVIS_W<18> {
        ASVIS_W::new(self)
    }
    #[doc = "Bit 17 - AVVIS A VBus Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the VBus valid threshold on an A device. Software must write a one to clear this bit."]
    #[inline(always)]
    pub fn avvis(&mut self) -> AVVIS_W<17> {
        AVVIS_W::new(self)
    }
    #[doc = "Bit 16 - IDIS USB ID Interrupt Status - Read/Write. This bit is set when a change on the ID input has been detected. Software must write a one to clear this bit."]
    #[inline(always)]
    pub fn idis(&mut self) -> IDIS_W<16> {
        IDIS_W::new(self)
    }
    #[doc = "Bit 5 - IDPU ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]. When this bit is 0, the ID input will not be sampled."]
    #[inline(always)]
    pub fn idpu(&mut self) -> IDPU_W<5> {
        IDPU_W::new(self)
    }
    #[doc = "Bit 1 - VC VBUS Charge - Read/Write. Setting this bit causes the VBus line to be charged. This is used for VBus pulsing during SRP."]
    #[inline(always)]
    pub fn vc(&mut self) -> VC_W<1> {
        VC_W::new(self)
    }
    #[doc = "Bit 0 - VD VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
    #[inline(always)]
    pub fn vd(&mut self) -> VD_W<0> {
        VD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "On-The-Go Status & control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgsc](index.html) module"]
pub struct OTGSC_SPEC;
impl crate::RegisterSpec for OTGSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otgsc::R](R) reader structure"]
impl crate::Readable for OTGSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otgsc::W](W) writer structure"]
impl crate::Writable for OTGSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTGSC to value 0"]
impl crate::Resettable for OTGSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
