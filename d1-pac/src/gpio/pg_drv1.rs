#[doc = "Register `pg_drv1` reader"]
pub struct R(crate::R<PG_DRV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_DRV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_DRV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_DRV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pg_drv1` writer"]
pub struct W(crate::W<PG_DRV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_DRV1_SPEC>;
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
impl From<crate::W<PG_DRV1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_DRV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PG Multi_Driving Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PG_DRV_A {
    #[doc = "0: `0`"]
    L0 = 0,
    #[doc = "1: `1`"]
    L1 = 1,
    #[doc = "2: `10`"]
    L2 = 2,
    #[doc = "3: `11`"]
    L3 = 3,
}
impl From<PG_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: PG_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Fields `PG(8-15)_DRV` reader - PG Multi_Driving Select"]
pub struct PG_DRV_R(crate::FieldReader<u8>);
impl PG_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PG_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PG_DRV_A {
        match self.bits {
            0 => PG_DRV_A::L0,
            1 => PG_DRV_A::L1,
            2 => PG_DRV_A::L2,
            3 => PG_DRV_A::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `L0`"]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        **self == PG_DRV_A::L0
    }
    #[doc = "Checks if the value of the field is `L1`"]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        **self == PG_DRV_A::L1
    }
    #[doc = "Checks if the value of the field is `L2`"]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        **self == PG_DRV_A::L2
    }
    #[doc = "Checks if the value of the field is `L3`"]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        **self == PG_DRV_A::L3
    }
}
impl core::ops::Deref for PG_DRV_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `PG(8-15)_DRV` const generic writer - PG Multi_Driving Select"]
pub struct PG_DRV_W<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> PG_DRV_W<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PG_DRV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn l0(self) -> &'a mut W {
        self.variant(PG_DRV_A::L0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn l1(self) -> &'a mut W {
        self.variant(PG_DRV_A::L1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn l2(self) -> &'a mut W {
        self.variant(PG_DRV_A::L2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn l3(self) -> &'a mut W {
        self.variant(PG_DRV_A::L3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << O)) | ((value as u32 & 3) << O);
        self.w
    }
}
impl R {
    #[doc = "PG Multi_Driving Select"]
    #[inline(always)]
    pub unsafe fn pg_drv(&self, n: usize) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> ((n - 8) * 4)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg8_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg9_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg10_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg11_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg12_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg13_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg14_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg15_drv(&self) -> PG_DRV_R {
        PG_DRV_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "PG Multi_Driving Select"]
    #[inline(always)]
    pub unsafe fn pg_drv<const O: usize>(&mut self) -> PG_DRV_W<O> {
        PG_DRV_W { w: self }
    }
    #[doc = "Bits 0:1 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg8_drv(&mut self) -> PG_DRV_W<0> {
        PG_DRV_W { w: self }
    }
    #[doc = "Bits 4:5 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg9_drv(&mut self) -> PG_DRV_W<4> {
        PG_DRV_W { w: self }
    }
    #[doc = "Bits 8:9 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg10_drv(&mut self) -> PG_DRV_W<8> {
        PG_DRV_W { w: self }
    }
    #[doc = "Bits 12:13 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg11_drv(&mut self) -> PG_DRV_W<12> {
        PG_DRV_W { w: self }
    }
    #[doc = "Bits 16:17 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg12_drv(&mut self) -> PG_DRV_W<16> {
        PG_DRV_W { w: self }
    }
    #[doc = "Bits 20:21 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg13_drv(&mut self) -> PG_DRV_W<20> {
        PG_DRV_W { w: self }
    }
    #[doc = "Bits 24:25 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg14_drv(&mut self) -> PG_DRV_W<24> {
        PG_DRV_W { w: self }
    }
    #[doc = "Bits 28:29 - PG Multi_Driving Select"]
    #[inline(always)]
    pub fn pg15_drv(&mut self) -> PG_DRV_W<28> {
        PG_DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PG Multi_Driving Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_drv1](index.html) module"]
pub struct PG_DRV1_SPEC;
impl crate::RegisterSpec for PG_DRV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg_drv1::R](R) reader structure"]
impl crate::Readable for PG_DRV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg_drv1::W](W) writer structure"]
impl crate::Writable for PG_DRV1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pg_drv1 to value 0"]
impl crate::Resettable for PG_DRV1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}