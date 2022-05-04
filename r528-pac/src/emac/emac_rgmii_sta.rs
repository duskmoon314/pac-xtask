#[doc = "Register `EMAC_RGMII_STA` reader"]
pub struct R(crate::R<EMAC_RGMII_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_RGMII_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_RGMII_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_RGMII_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_RGMII_STA` writer"]
pub struct W(crate::W<EMAC_RGMII_STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_RGMII_STA_SPEC>;
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
impl From<crate::W<EMAC_RGMII_STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_RGMII_STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "The link status of the RGMII interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGMII_LINK_A {
    #[doc = "0: `0`"]
    DOWN = 0,
    #[doc = "1: `1`"]
    UP = 1,
}
impl From<RGMII_LINK_A> for bool {
    #[inline(always)]
    fn from(variant: RGMII_LINK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGMII_LINK` reader - The link status of the RGMII interface"]
pub struct RGMII_LINK_R(crate::FieldReader<bool, RGMII_LINK_A>);
impl RGMII_LINK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RGMII_LINK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGMII_LINK_A {
        match self.bits {
            false => RGMII_LINK_A::DOWN,
            true => RGMII_LINK_A::UP,
        }
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        **self == RGMII_LINK_A::DOWN
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        **self == RGMII_LINK_A::UP
    }
}
impl core::ops::Deref for RGMII_LINK_R {
    type Target = crate::FieldReader<bool, RGMII_LINK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGMII_LINK` writer - The link status of the RGMII interface"]
pub struct RGMII_LINK_W<'a> {
    w: &'a mut W,
}
impl<'a> RGMII_LINK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGMII_LINK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(RGMII_LINK_A::DOWN)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(RGMII_LINK_A::UP)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "The link speed of the RGMII interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RGMII_LINK_SPD_A {
    #[doc = "0: `0`"]
    S2_5 = 0,
    #[doc = "1: `1`"]
    S25 = 1,
    #[doc = "2: `10`"]
    S125 = 2,
}
impl From<RGMII_LINK_SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: RGMII_LINK_SPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RGMII_LINK_SPD` reader - The link speed of the RGMII interface"]
pub struct RGMII_LINK_SPD_R(crate::FieldReader<u8, RGMII_LINK_SPD_A>);
impl RGMII_LINK_SPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RGMII_LINK_SPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RGMII_LINK_SPD_A> {
        match self.bits {
            0 => Some(RGMII_LINK_SPD_A::S2_5),
            1 => Some(RGMII_LINK_SPD_A::S25),
            2 => Some(RGMII_LINK_SPD_A::S125),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `S2_5`"]
    #[inline(always)]
    pub fn is_s2_5(&self) -> bool {
        **self == RGMII_LINK_SPD_A::S2_5
    }
    #[doc = "Checks if the value of the field is `S25`"]
    #[inline(always)]
    pub fn is_s25(&self) -> bool {
        **self == RGMII_LINK_SPD_A::S25
    }
    #[doc = "Checks if the value of the field is `S125`"]
    #[inline(always)]
    pub fn is_s125(&self) -> bool {
        **self == RGMII_LINK_SPD_A::S125
    }
}
impl core::ops::Deref for RGMII_LINK_SPD_R {
    type Target = crate::FieldReader<u8, RGMII_LINK_SPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGMII_LINK_SPD` writer - The link speed of the RGMII interface"]
pub struct RGMII_LINK_SPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RGMII_LINK_SPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGMII_LINK_SPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn s2_5(self) -> &'a mut W {
        self.variant(RGMII_LINK_SPD_A::S2_5)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn s25(self) -> &'a mut W {
        self.variant(RGMII_LINK_SPD_A::S25)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn s125(self) -> &'a mut W {
        self.variant(RGMII_LINK_SPD_A::S125)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "The link mode of the RGMII interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGMII_LINK_MD_A {
    #[doc = "0: `0`"]
    HALF_DUPLEX = 0,
    #[doc = "1: `1`"]
    FULL_DUPLEX = 1,
}
impl From<RGMII_LINK_MD_A> for bool {
    #[inline(always)]
    fn from(variant: RGMII_LINK_MD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGMII_LINK_MD` reader - The link mode of the RGMII interface"]
pub struct RGMII_LINK_MD_R(crate::FieldReader<bool, RGMII_LINK_MD_A>);
impl RGMII_LINK_MD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RGMII_LINK_MD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGMII_LINK_MD_A {
        match self.bits {
            false => RGMII_LINK_MD_A::HALF_DUPLEX,
            true => RGMII_LINK_MD_A::FULL_DUPLEX,
        }
    }
    #[doc = "Checks if the value of the field is `HALF_DUPLEX`"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        **self == RGMII_LINK_MD_A::HALF_DUPLEX
    }
    #[doc = "Checks if the value of the field is `FULL_DUPLEX`"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        **self == RGMII_LINK_MD_A::FULL_DUPLEX
    }
}
impl core::ops::Deref for RGMII_LINK_MD_R {
    type Target = crate::FieldReader<bool, RGMII_LINK_MD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGMII_LINK_MD` writer - The link mode of the RGMII interface"]
pub struct RGMII_LINK_MD_W<'a> {
    w: &'a mut W,
}
impl<'a> RGMII_LINK_MD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGMII_LINK_MD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut W {
        self.variant(RGMII_LINK_MD_A::HALF_DUPLEX)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(RGMII_LINK_MD_A::FULL_DUPLEX)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - The link status of the RGMII interface"]
    #[inline(always)]
    pub fn rgmii_link(&self) -> RGMII_LINK_R {
        RGMII_LINK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - The link speed of the RGMII interface"]
    #[inline(always)]
    pub fn rgmii_link_spd(&self) -> RGMII_LINK_SPD_R {
        RGMII_LINK_SPD_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - The link mode of the RGMII interface"]
    #[inline(always)]
    pub fn rgmii_link_md(&self) -> RGMII_LINK_MD_R {
        RGMII_LINK_MD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - The link status of the RGMII interface"]
    #[inline(always)]
    pub fn rgmii_link(&mut self) -> RGMII_LINK_W {
        RGMII_LINK_W { w: self }
    }
    #[doc = "Bits 1:2 - The link speed of the RGMII interface"]
    #[inline(always)]
    pub fn rgmii_link_spd(&mut self) -> RGMII_LINK_SPD_W {
        RGMII_LINK_SPD_W { w: self }
    }
    #[doc = "Bit 0 - The link mode of the RGMII interface"]
    #[inline(always)]
    pub fn rgmii_link_md(&mut self) -> RGMII_LINK_MD_W {
        RGMII_LINK_MD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC RGMII Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_rgmii_sta](index.html) module"]
pub struct EMAC_RGMII_STA_SPEC;
impl crate::RegisterSpec for EMAC_RGMII_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_rgmii_sta::R](R) reader structure"]
impl crate::Readable for EMAC_RGMII_STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_rgmii_sta::W](W) writer structure"]
impl crate::Writable for EMAC_RGMII_STA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_RGMII_STA to value 0"]
impl crate::Resettable for EMAC_RGMII_STA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}