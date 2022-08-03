#[doc = "Register `pf_eint_cfg0` reader"]
pub struct R(crate::R<PF_EINT_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_EINT_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_EINT_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_EINT_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pf_eint_cfg0` writer"]
pub struct W(crate::W<PF_EINT_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_EINT_CFG0_SPEC>;
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
impl From<crate::W<PF_EINT_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_EINT_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `eint_cfg[0-6]` reader - External INT Mode"]
pub type EINT_CFG_R = crate::FieldReader<u8, EINT_CFG_A>;
#[doc = "External INT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EINT_CFG_A {
    #[doc = "0: `0`"]
    POSITIVE_EDGE = 0,
    #[doc = "1: `1`"]
    NEGATIVE_EDGE = 1,
    #[doc = "2: `10`"]
    HIGH_LEVEL = 2,
    #[doc = "3: `11`"]
    LOW_LEVEL = 3,
    #[doc = "4: `100`"]
    DOUBLE_EDGE = 4,
}
impl From<EINT_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: EINT_CFG_A) -> Self {
        variant as _
    }
}
impl EINT_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EINT_CFG_A> {
        match self.bits {
            0 => Some(EINT_CFG_A::POSITIVE_EDGE),
            1 => Some(EINT_CFG_A::NEGATIVE_EDGE),
            2 => Some(EINT_CFG_A::HIGH_LEVEL),
            3 => Some(EINT_CFG_A::LOW_LEVEL),
            4 => Some(EINT_CFG_A::DOUBLE_EDGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE_EDGE`"]
    #[inline(always)]
    pub fn is_positive_edge(&self) -> bool {
        *self == EINT_CFG_A::POSITIVE_EDGE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_EDGE`"]
    #[inline(always)]
    pub fn is_negative_edge(&self) -> bool {
        *self == EINT_CFG_A::NEGATIVE_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == EINT_CFG_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == EINT_CFG_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE`"]
    #[inline(always)]
    pub fn is_double_edge(&self) -> bool {
        *self == EINT_CFG_A::DOUBLE_EDGE
    }
}
#[doc = "Field `eint_cfg[0-6]` writer - External INT Mode"]
pub type EINT_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PF_EINT_CFG0_SPEC, u8, EINT_CFG_A, 4, O>;
impl<'a, const O: u8> EINT_CFG_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn positive_edge(self) -> &'a mut W {
        self.variant(EINT_CFG_A::POSITIVE_EDGE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn negative_edge(self) -> &'a mut W {
        self.variant(EINT_CFG_A::NEGATIVE_EDGE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(EINT_CFG_A::HIGH_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(EINT_CFG_A::LOW_LEVEL)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn double_edge(self) -> &'a mut W {
        self.variant(EINT_CFG_A::DOUBLE_EDGE)
    }
}
impl R {
    #[doc = "External INT Mode"]
    #[inline(always)]
    pub unsafe fn eint_cfg(&self, n: u8) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - External INT Mode"]
    #[inline(always)]
    pub fn eint0_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External INT Mode"]
    #[inline(always)]
    pub fn eint1_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External INT Mode"]
    #[inline(always)]
    pub fn eint2_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External INT Mode"]
    #[inline(always)]
    pub fn eint3_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External INT Mode"]
    #[inline(always)]
    pub fn eint4_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - External INT Mode"]
    #[inline(always)]
    pub fn eint5_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - External INT Mode"]
    #[inline(always)]
    pub fn eint6_cfg(&self) -> EINT_CFG_R {
        EINT_CFG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "External INT Mode"]
    #[inline(always)]
    pub unsafe fn eint_cfg<const O: u8>(&mut self) -> EINT_CFG_W<O> {
        EINT_CFG_W::new(self)
    }
    #[doc = "Bits 0:3 - External INT Mode"]
    #[inline(always)]
    pub fn eint0_cfg(&mut self) -> EINT_CFG_W<0> {
        EINT_CFG_W::new(self)
    }
    #[doc = "Bits 4:7 - External INT Mode"]
    #[inline(always)]
    pub fn eint1_cfg(&mut self) -> EINT_CFG_W<4> {
        EINT_CFG_W::new(self)
    }
    #[doc = "Bits 8:11 - External INT Mode"]
    #[inline(always)]
    pub fn eint2_cfg(&mut self) -> EINT_CFG_W<8> {
        EINT_CFG_W::new(self)
    }
    #[doc = "Bits 12:15 - External INT Mode"]
    #[inline(always)]
    pub fn eint3_cfg(&mut self) -> EINT_CFG_W<12> {
        EINT_CFG_W::new(self)
    }
    #[doc = "Bits 16:19 - External INT Mode"]
    #[inline(always)]
    pub fn eint4_cfg(&mut self) -> EINT_CFG_W<16> {
        EINT_CFG_W::new(self)
    }
    #[doc = "Bits 20:23 - External INT Mode"]
    #[inline(always)]
    pub fn eint5_cfg(&mut self) -> EINT_CFG_W<20> {
        EINT_CFG_W::new(self)
    }
    #[doc = "Bits 24:27 - External INT Mode"]
    #[inline(always)]
    pub fn eint6_cfg(&mut self) -> EINT_CFG_W<24> {
        EINT_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PF External Interrupt Configure Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_eint_cfg0](index.html) module"]
pub struct PF_EINT_CFG0_SPEC;
impl crate::RegisterSpec for PF_EINT_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_eint_cfg0::R](R) reader structure"]
impl crate::Readable for PF_EINT_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_eint_cfg0::W](W) writer structure"]
impl crate::Writable for PF_EINT_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pf_eint_cfg0 to value 0"]
impl crate::Resettable for PF_EINT_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
