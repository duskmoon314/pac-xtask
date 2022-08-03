#[doc = "Register `ths_alarm_ints` reader"]
pub struct R(crate::R<THS_ALARM_INTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THS_ALARM_INTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THS_ALARM_INTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THS_ALARM_INTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ths_alarm_ints` writer"]
pub struct W(crate::W<THS_ALARM_INTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THS_ALARM_INTS_SPEC>;
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
impl From<crate::W<THS_ALARM_INTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THS_ALARM_INTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `alarm_int_sts` reader - Alarm interrupt pending for sensor\n\nWrite 1 to clear the pending status."]
pub type ALARM_INT_STS_R = crate::BitReader<ALARM_INT_STS_A>;
#[doc = "Alarm interrupt pending for sensor\n\nWrite 1 to clear the pending status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM_INT_STS_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Pending"]
    PENDING = 1,
}
impl From<ALARM_INT_STS_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM_INT_STS_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM_INT_STS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM_INT_STS_A {
        match self.bits {
            false => ALARM_INT_STS_A::NO_EFFECT,
            true => ALARM_INT_STS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ALARM_INT_STS_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ALARM_INT_STS_A::PENDING
    }
}
#[doc = "Field `alarm_int_sts` writer - Alarm interrupt pending for sensor\n\nWrite 1 to clear the pending status."]
pub type ALARM_INT_STS_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, THS_ALARM_INTS_SPEC, ALARM_INT_STS_A, O>;
impl<'a, const O: u8> ALARM_INT_STS_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ALARM_INT_STS_A::NO_EFFECT)
    }
    #[doc = "Pending"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(ALARM_INT_STS_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - Alarm interrupt pending for sensor\n\nWrite 1 to clear the pending status."]
    #[inline(always)]
    pub fn alarm_int_sts(&self) -> ALARM_INT_STS_R {
        ALARM_INT_STS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alarm interrupt pending for sensor\n\nWrite 1 to clear the pending status."]
    #[inline(always)]
    pub fn alarm_int_sts(&mut self) -> ALARM_INT_STS_W<0> {
        ALARM_INT_STS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "THS Alarm Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ths_alarm_ints](index.html) module"]
pub struct THS_ALARM_INTS_SPEC;
impl crate::RegisterSpec for THS_ALARM_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ths_alarm_ints::R](R) reader structure"]
impl crate::Readable for THS_ALARM_INTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ths_alarm_ints::W](W) writer structure"]
impl crate::Writable for THS_ALARM_INTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ths_alarm_ints to value 0"]
impl crate::Resettable for THS_ALARM_INTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
