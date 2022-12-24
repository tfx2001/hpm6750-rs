#[doc = "Register `L3_L4_CFG_1_L3_L4_CTRL` reader"]
pub struct R(crate::R<L3_L4_CFG_1_L3_L4_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L3_L4_CFG_1_L3_L4_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L3_L4_CFG_1_L3_L4_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L3_L4_CFG_1_L3_L4_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L3_L4_CFG_1_L3_L4_CTRL` writer"]
pub struct W(crate::W<L3_L4_CFG_1_L3_L4_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L3_L4_CFG_1_L3_L4_CTRL_SPEC>;
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
impl From<crate::W<L3_L4_CFG_1_L3_L4_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L3_L4_CFG_1_L3_L4_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L3PEN0` reader - Layer 3 Protocol Enable When set, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv6 frames. When reset, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv4 frames. The Layer 3 matching is done only when either L3SAM0 or L3DAM0 bit is set high."]
pub type L3PEN0_R = crate::BitReader<bool>;
#[doc = "Field `L3PEN0` writer - Layer 3 Protocol Enable When set, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv6 frames. When reset, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv4 frames. The Layer 3 matching is done only when either L3SAM0 or L3DAM0 bit is set high."]
pub type L3PEN0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L3_L4_CFG_1_L3_L4_CTRL_SPEC, bool, O>;
#[doc = "Field `L3SAM0` reader - Layer 3 IP SA Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for matching. When reset, the MAC ignores the Layer 3 IP Source Address field for matching."]
pub type L3SAM0_R = crate::BitReader<bool>;
#[doc = "Field `L3SAM0` writer - Layer 3 IP SA Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for matching. When reset, the MAC ignores the Layer 3 IP Source Address field for matching."]
pub type L3SAM0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L3_L4_CFG_1_L3_L4_CTRL_SPEC, bool, O>;
#[doc = "Field `L3SAIM0` reader - Layer 3 IP SA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for inverse matching. When reset, this bit indicates that the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when Bit 2 (L3SAM0) is set high."]
pub type L3SAIM0_R = crate::BitReader<bool>;
#[doc = "Field `L3SAIM0` writer - Layer 3 IP SA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for inverse matching. When reset, this bit indicates that the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when Bit 2 (L3SAM0) is set high."]
pub type L3SAIM0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L3_L4_CFG_1_L3_L4_CTRL_SPEC, bool, O>;
#[doc = "Field `L3DAM0` reader - Layer 3 IP DA Match Enable When set, this bit indicates that Layer 3 IP Destination Address field is enabled for matching. When reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When Bit 0 (L3PEN0) is set, you should set either this bit or Bit 2 (L3SAM0) because either IPv6 DA or SA can be checked for filtering."]
pub type L3DAM0_R = crate::BitReader<bool>;
#[doc = "Field `L3DAM0` writer - Layer 3 IP DA Match Enable When set, this bit indicates that Layer 3 IP Destination Address field is enabled for matching. When reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When Bit 0 (L3PEN0) is set, you should set either this bit or Bit 2 (L3SAM0) because either IPv6 DA or SA can be checked for filtering."]
pub type L3DAM0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L3_L4_CFG_1_L3_L4_CTRL_SPEC, bool, O>;
#[doc = "Field `L3DAIM0` reader - Layer 3 IP DA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Destination Address field is enabled for inverse matching. When reset, this bit indicates that the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when Bit 4 (L3DAM0) is set high."]
pub type L3DAIM0_R = crate::BitReader<bool>;
#[doc = "Field `L3DAIM0` writer - Layer 3 IP DA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Destination Address field is enabled for inverse matching. When reset, this bit indicates that the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when Bit 4 (L3DAM0) is set high."]
pub type L3DAIM0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L3_L4_CFG_1_L3_L4_CTRL_SPEC, bool, O>;
#[doc = "Field `L3HSBM0` reader - Layer 3 IP SA Higher Bits Match IPv4 Frames: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 frames. The following list describes the values of this field: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - ... - 31: All bits except MSb are masked. IPv6 Frames: This field contains Bits \\[4:0\\]
of the field that indicates the number of higher bits of IP Source or Destination Address matched in the IPv6 frames. This field is valid and applicable only if L3DAM0 or L3SAM0 is set high."]
pub type L3HSBM0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `L3HSBM0` writer - Layer 3 IP SA Higher Bits Match IPv4 Frames: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 frames. The following list describes the values of this field: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - ... - 31: All bits except MSb are masked. IPv6 Frames: This field contains Bits \\[4:0\\]
of the field that indicates the number of higher bits of IP Source or Destination Address matched in the IPv6 frames. This field is valid and applicable only if L3DAM0 or L3SAM0 is set high."]
pub type L3HSBM0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L3_L4_CFG_1_L3_L4_CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `L3HDBM0` reader - Layer 3 IP DA Higher Bits Match IPv4 Frames: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 frames. The following list describes the values of this field: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - ... - 31: All bits except MSb are masked. IPv6 Frames: Bits \\[12:11\\]
of this field correspond to Bits \\[6:5\\]
of L3HSBM0, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 frames. The following list describes the concatenated values of the L3HDBM0\\[1:0\\]
and L3HSBM0 bits: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - … - 127: All bits except MSb are masked. This field is valid and applicable only if L3DAM0 or L3SAM0 is set high."]
pub type L3HDBM0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `L3HDBM0` writer - Layer 3 IP DA Higher Bits Match IPv4 Frames: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 frames. The following list describes the values of this field: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - ... - 31: All bits except MSb are masked. IPv6 Frames: Bits \\[12:11\\]
of this field correspond to Bits \\[6:5\\]
of L3HSBM0, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 frames. The following list describes the concatenated values of the L3HDBM0\\[1:0\\]
and L3HSBM0 bits: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - … - 127: All bits except MSb are masked. This field is valid and applicable only if L3DAM0 or L3SAM0 is set high."]
pub type L3HDBM0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L3_L4_CFG_1_L3_L4_CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `L4PEN0` reader - Layer 4 Protocol Enable When set, this bit indicates that the Source and Destination Port number fields for UDP frames are used for matching. When reset, this bit indicates that the Source and Destination Port number fields for TCP frames are used for matching. The Layer 4 matching is done only when either L4SPM0 or L4DPM0 bit is set high."]
pub type L4PEN0_R = crate::BitReader<bool>;
#[doc = "Field `L4PEN0` writer - Layer 4 Protocol Enable When set, this bit indicates that the Source and Destination Port number fields for UDP frames are used for matching. When reset, this bit indicates that the Source and Destination Port number fields for TCP frames are used for matching. The Layer 4 matching is done only when either L4SPM0 or L4DPM0 bit is set high."]
pub type L4PEN0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L3_L4_CFG_1_L3_L4_CTRL_SPEC, bool, O>;
#[doc = "Field `L4SPM0` reader - Layer 4 Source Port Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for matching. When reset, the MAC ignores the Layer 4 Source Port number field for matching."]
pub type L4SPM0_R = crate::BitReader<bool>;
#[doc = "Field `L4SPM0` writer - Layer 4 Source Port Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for matching. When reset, the MAC ignores the Layer 4 Source Port number field for matching."]
pub type L4SPM0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L3_L4_CFG_1_L3_L4_CTRL_SPEC, bool, O>;
#[doc = "Field `L4SPIM0` reader - Layer 4 Source Port Inverse Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for inverse matching. When reset, this bit indicates that the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when Bit 18 (L4SPM0) is set high."]
pub type L4SPIM0_R = crate::BitReader<bool>;
#[doc = "Field `L4SPIM0` writer - Layer 4 Source Port Inverse Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for inverse matching. When reset, this bit indicates that the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when Bit 18 (L4SPM0) is set high."]
pub type L4SPIM0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L3_L4_CFG_1_L3_L4_CTRL_SPEC, bool, O>;
#[doc = "Field `L4DPM0` reader - Layer 4 Destination Port Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for matching. When reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
pub type L4DPM0_R = crate::BitReader<bool>;
#[doc = "Field `L4DPM0` writer - Layer 4 Destination Port Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for matching. When reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
pub type L4DPM0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L3_L4_CFG_1_L3_L4_CTRL_SPEC, bool, O>;
#[doc = "Field `L4DPIM0` reader - Layer 4 Destination Port Inverse Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for inverse matching. When reset, this bit indicates that the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when Bit 20 (L4DPM0) is set high."]
pub type L4DPIM0_R = crate::BitReader<bool>;
#[doc = "Field `L4DPIM0` writer - Layer 4 Destination Port Inverse Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for inverse matching. When reset, this bit indicates that the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when Bit 20 (L4DPM0) is set high."]
pub type L4DPIM0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, L3_L4_CFG_1_L3_L4_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Layer 3 Protocol Enable When set, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv6 frames. When reset, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv4 frames. The Layer 3 matching is done only when either L3SAM0 or L3DAM0 bit is set high."]
    #[inline(always)]
    pub fn l3pen0(&self) -> L3PEN0_R {
        L3PEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Layer 3 IP SA Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for matching. When reset, the MAC ignores the Layer 3 IP Source Address field for matching."]
    #[inline(always)]
    pub fn l3sam0(&self) -> L3SAM0_R {
        L3SAM0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Layer 3 IP SA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for inverse matching. When reset, this bit indicates that the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when Bit 2 (L3SAM0) is set high."]
    #[inline(always)]
    pub fn l3saim0(&self) -> L3SAIM0_R {
        L3SAIM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Layer 3 IP DA Match Enable When set, this bit indicates that Layer 3 IP Destination Address field is enabled for matching. When reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When Bit 0 (L3PEN0) is set, you should set either this bit or Bit 2 (L3SAM0) because either IPv6 DA or SA can be checked for filtering."]
    #[inline(always)]
    pub fn l3dam0(&self) -> L3DAM0_R {
        L3DAM0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Layer 3 IP DA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Destination Address field is enabled for inverse matching. When reset, this bit indicates that the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when Bit 4 (L3DAM0) is set high."]
    #[inline(always)]
    pub fn l3daim0(&self) -> L3DAIM0_R {
        L3DAIM0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - Layer 3 IP SA Higher Bits Match IPv4 Frames: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 frames. The following list describes the values of this field: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - ... - 31: All bits except MSb are masked. IPv6 Frames: This field contains Bits \\[4:0\\]
of the field that indicates the number of higher bits of IP Source or Destination Address matched in the IPv6 frames. This field is valid and applicable only if L3DAM0 or L3SAM0 is set high."]
    #[inline(always)]
    pub fn l3hsbm0(&self) -> L3HSBM0_R {
        L3HSBM0_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Layer 3 IP DA Higher Bits Match IPv4 Frames: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 frames. The following list describes the values of this field: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - ... - 31: All bits except MSb are masked. IPv6 Frames: Bits \\[12:11\\]
of this field correspond to Bits \\[6:5\\]
of L3HSBM0, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 frames. The following list describes the concatenated values of the L3HDBM0\\[1:0\\]
and L3HSBM0 bits: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - … - 127: All bits except MSb are masked. This field is valid and applicable only if L3DAM0 or L3SAM0 is set high."]
    #[inline(always)]
    pub fn l3hdbm0(&self) -> L3HDBM0_R {
        L3HDBM0_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Layer 4 Protocol Enable When set, this bit indicates that the Source and Destination Port number fields for UDP frames are used for matching. When reset, this bit indicates that the Source and Destination Port number fields for TCP frames are used for matching. The Layer 4 matching is done only when either L4SPM0 or L4DPM0 bit is set high."]
    #[inline(always)]
    pub fn l4pen0(&self) -> L4PEN0_R {
        L4PEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Layer 4 Source Port Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for matching. When reset, the MAC ignores the Layer 4 Source Port number field for matching."]
    #[inline(always)]
    pub fn l4spm0(&self) -> L4SPM0_R {
        L4SPM0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Layer 4 Source Port Inverse Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for inverse matching. When reset, this bit indicates that the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when Bit 18 (L4SPM0) is set high."]
    #[inline(always)]
    pub fn l4spim0(&self) -> L4SPIM0_R {
        L4SPIM0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Layer 4 Destination Port Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for matching. When reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
    #[inline(always)]
    pub fn l4dpm0(&self) -> L4DPM0_R {
        L4DPM0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Layer 4 Destination Port Inverse Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for inverse matching. When reset, this bit indicates that the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when Bit 20 (L4DPM0) is set high."]
    #[inline(always)]
    pub fn l4dpim0(&self) -> L4DPIM0_R {
        L4DPIM0_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Layer 3 Protocol Enable When set, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv6 frames. When reset, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv4 frames. The Layer 3 matching is done only when either L3SAM0 or L3DAM0 bit is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l3pen0(&mut self) -> L3PEN0_W<0> {
        L3PEN0_W::new(self)
    }
    #[doc = "Bit 2 - Layer 3 IP SA Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for matching. When reset, the MAC ignores the Layer 3 IP Source Address field for matching."]
    #[inline(always)]
    #[must_use]
    pub fn l3sam0(&mut self) -> L3SAM0_W<2> {
        L3SAM0_W::new(self)
    }
    #[doc = "Bit 3 - Layer 3 IP SA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for inverse matching. When reset, this bit indicates that the Layer 3 IP Source Address field is enabled for perfect matching. This bit is valid and applicable only when Bit 2 (L3SAM0) is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l3saim0(&mut self) -> L3SAIM0_W<3> {
        L3SAIM0_W::new(self)
    }
    #[doc = "Bit 4 - Layer 3 IP DA Match Enable When set, this bit indicates that Layer 3 IP Destination Address field is enabled for matching. When reset, the MAC ignores the Layer 3 IP Destination Address field for matching. Note: When Bit 0 (L3PEN0) is set, you should set either this bit or Bit 2 (L3SAM0) because either IPv6 DA or SA can be checked for filtering."]
    #[inline(always)]
    #[must_use]
    pub fn l3dam0(&mut self) -> L3DAM0_W<4> {
        L3DAM0_W::new(self)
    }
    #[doc = "Bit 5 - Layer 3 IP DA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Destination Address field is enabled for inverse matching. When reset, this bit indicates that the Layer 3 IP Destination Address field is enabled for perfect matching. This bit is valid and applicable only when Bit 4 (L3DAM0) is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l3daim0(&mut self) -> L3DAIM0_W<5> {
        L3DAIM0_W::new(self)
    }
    #[doc = "Bits 6:10 - Layer 3 IP SA Higher Bits Match IPv4 Frames: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 frames. The following list describes the values of this field: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - ... - 31: All bits except MSb are masked. IPv6 Frames: This field contains Bits \\[4:0\\]
of the field that indicates the number of higher bits of IP Source or Destination Address matched in the IPv6 frames. This field is valid and applicable only if L3DAM0 or L3SAM0 is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l3hsbm0(&mut self) -> L3HSBM0_W<6> {
        L3HSBM0_W::new(self)
    }
    #[doc = "Bits 11:15 - Layer 3 IP DA Higher Bits Match IPv4 Frames: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 frames. The following list describes the values of this field: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - ... - 31: All bits except MSb are masked. IPv6 Frames: Bits \\[12:11\\]
of this field correspond to Bits \\[6:5\\]
of L3HSBM0, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 frames. The following list describes the concatenated values of the L3HDBM0\\[1:0\\]
and L3HSBM0 bits: - 0: No bits are masked. - 1: LSb\\[0\\]
is masked. - 2: Two LSbs \\[1:0\\]
are masked. - … - 127: All bits except MSb are masked. This field is valid and applicable only if L3DAM0 or L3SAM0 is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l3hdbm0(&mut self) -> L3HDBM0_W<11> {
        L3HDBM0_W::new(self)
    }
    #[doc = "Bit 16 - Layer 4 Protocol Enable When set, this bit indicates that the Source and Destination Port number fields for UDP frames are used for matching. When reset, this bit indicates that the Source and Destination Port number fields for TCP frames are used for matching. The Layer 4 matching is done only when either L4SPM0 or L4DPM0 bit is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l4pen0(&mut self) -> L4PEN0_W<16> {
        L4PEN0_W::new(self)
    }
    #[doc = "Bit 18 - Layer 4 Source Port Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for matching. When reset, the MAC ignores the Layer 4 Source Port number field for matching."]
    #[inline(always)]
    #[must_use]
    pub fn l4spm0(&mut self) -> L4SPM0_W<18> {
        L4SPM0_W::new(self)
    }
    #[doc = "Bit 19 - Layer 4 Source Port Inverse Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for inverse matching. When reset, this bit indicates that the Layer 4 Source Port number field is enabled for perfect matching. This bit is valid and applicable only when Bit 18 (L4SPM0) is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l4spim0(&mut self) -> L4SPIM0_W<19> {
        L4SPIM0_W::new(self)
    }
    #[doc = "Bit 20 - Layer 4 Destination Port Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for matching. When reset, the MAC ignores the Layer 4 Destination Port number field for matching."]
    #[inline(always)]
    #[must_use]
    pub fn l4dpm0(&mut self) -> L4DPM0_W<20> {
        L4DPM0_W::new(self)
    }
    #[doc = "Bit 21 - Layer 4 Destination Port Inverse Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for inverse matching. When reset, this bit indicates that the Layer 4 Destination Port number field is enabled for perfect matching. This bit is valid and applicable only when Bit 20 (L4DPM0) is set high."]
    #[inline(always)]
    #[must_use]
    pub fn l4dpim0(&mut self) -> L4DPIM0_W<21> {
        L4DPIM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer 3 and Layer 4 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l3_l4_cfg_1_l3_l4_ctrl](index.html) module"]
pub struct L3_L4_CFG_1_L3_L4_CTRL_SPEC;
impl crate::RegisterSpec for L3_L4_CFG_1_L3_L4_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l3_l4_cfg_1_l3_l4_ctrl::R](R) reader structure"]
impl crate::Readable for L3_L4_CFG_1_L3_L4_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l3_l4_cfg_1_l3_l4_ctrl::W](W) writer structure"]
impl crate::Writable for L3_L4_CFG_1_L3_L4_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L3_L4_CFG_1_L3_L4_CTRL to value 0"]
impl crate::Resettable for L3_L4_CFG_1_L3_L4_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
