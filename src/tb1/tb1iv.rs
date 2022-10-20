#[doc = "Register `TB1IV` reader"]
pub struct R(crate::R<TB1IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TB1IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TB1IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TB1IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TB1IV` writer"]
pub struct W(crate::W<TB1IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TB1IV_SPEC>;
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
impl From<crate::W<TB1IV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TB1IV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBIV` reader - Timer_B interrupt vector value"]
pub type TBIV_R = crate::FieldReader<u16, TBIV_A>;
#[doc = "Timer_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TBIV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Capture/compare 1; Interrupt Flag: TBxCCR1 CCIFG; Interrupt Priority: Highest"]
    TBCCR1 = 2,
    #[doc = "4: Interrupt Source: Capture/compare 2; Interrupt Flag: TBxCCR2 CCIFG"]
    TBCCR2 = 4,
    #[doc = "6: Interrupt Source: Capture/compare 3; Interrupt Flag: TBxCCR3 CCIFG"]
    TBCCR3 = 6,
    #[doc = "8: Interrupt Source: Capture/compare 4; Interrupt Flag: TBxCCR4 CCIFG"]
    TBCCR4 = 8,
    #[doc = "10: Interrupt Source: Capture/compare 5; Interrupt Flag: TBxCCR5 CCIFG"]
    TBCCR5 = 10,
    #[doc = "12: Interrupt Source: Capture/compare 6; Interrupt Flag: TBxCCR6 CCIFG"]
    TBCCR6 = 12,
    #[doc = "14: Interrupt Source: Timer overflow; Interrupt Flag: TBxCTL TBIFG; Interrupt Priority: Lowest"]
    TBIFG = 14,
}
impl From<TBIV_A> for u16 {
    #[inline(always)]
    fn from(variant: TBIV_A) -> Self {
        variant as _
    }
}
impl TBIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TBIV_A> {
        match self.bits {
            0 => Some(TBIV_A::NONE),
            2 => Some(TBIV_A::TBCCR1),
            4 => Some(TBIV_A::TBCCR2),
            6 => Some(TBIV_A::TBCCR3),
            8 => Some(TBIV_A::TBCCR4),
            10 => Some(TBIV_A::TBCCR5),
            12 => Some(TBIV_A::TBCCR6),
            14 => Some(TBIV_A::TBIFG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TBIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `TBCCR1`"]
    #[inline(always)]
    pub fn is_tbccr1(&self) -> bool {
        *self == TBIV_A::TBCCR1
    }
    #[doc = "Checks if the value of the field is `TBCCR2`"]
    #[inline(always)]
    pub fn is_tbccr2(&self) -> bool {
        *self == TBIV_A::TBCCR2
    }
    #[doc = "Checks if the value of the field is `TBCCR3`"]
    #[inline(always)]
    pub fn is_tbccr3(&self) -> bool {
        *self == TBIV_A::TBCCR3
    }
    #[doc = "Checks if the value of the field is `TBCCR4`"]
    #[inline(always)]
    pub fn is_tbccr4(&self) -> bool {
        *self == TBIV_A::TBCCR4
    }
    #[doc = "Checks if the value of the field is `TBCCR5`"]
    #[inline(always)]
    pub fn is_tbccr5(&self) -> bool {
        *self == TBIV_A::TBCCR5
    }
    #[doc = "Checks if the value of the field is `TBCCR6`"]
    #[inline(always)]
    pub fn is_tbccr6(&self) -> bool {
        *self == TBIV_A::TBCCR6
    }
    #[doc = "Checks if the value of the field is `TBIFG`"]
    #[inline(always)]
    pub fn is_tbifg(&self) -> bool {
        *self == TBIV_A::TBIFG
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer_B interrupt vector value"]
    #[inline(always)]
    pub fn tbiv(&self) -> TBIV_R {
        TBIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tb1iv](index.html) module"]
pub struct TB1IV_SPEC;
impl crate::RegisterSpec for TB1IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tb1iv::R](R) reader structure"]
impl crate::Readable for TB1IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tb1iv::W](W) writer structure"]
impl crate::Writable for TB1IV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TB1IV to value 0"]
impl crate::Resettable for TB1IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
