#[doc = "Register `BDCHR` reader"]
pub struct R(crate::R<BDCHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDCHR` writer"]
pub struct W(crate::W<BDCHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCHR_SPEC>;
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
impl From<crate::W<BDCHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDCH` reader - "]
pub struct BDCH_R(crate::FieldReader<u8, u8>);
impl BDCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BDCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BDCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BDCH` writer - "]
pub struct BDCH_W<'a> {
    w: &'a mut W,
}
impl<'a> BDCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bdch(&self) -> BDCH_R {
        BDCH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bdch(&mut self) -> BDCH_W {
        BDCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Baudrate Detection Counter High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdchr](index.html) module"]
pub struct BDCHR_SPEC;
impl crate::RegisterSpec for BDCHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdchr::R](R) reader structure"]
impl crate::Readable for BDCHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdchr::W](W) writer structure"]
impl crate::Writable for BDCHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDCHR to value 0"]
impl crate::Resettable for BDCHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
