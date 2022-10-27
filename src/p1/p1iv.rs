#[doc = "Register `P1IV` reader"]
pub struct R(crate::R<P1IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1IV` writer"]
pub struct W(crate::W<P1IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1IV_SPEC>;
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
impl From<crate::W<P1IV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1IV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1IV` reader - Port 1 interrupt vector value"]
pub type P1IV_R = crate::FieldReader<u8, P1IV_A>;
#[doc = "Port 1 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1IV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Port 1.0 interrupt; Interrupt Flag: P1IFG0; Interrupt Priority: Highest"]
    P1IFG0 = 2,
    #[doc = "4: Interrupt Source: Port 1.1 interrupt; Interrupt Flag: P1IFG1"]
    P1IFG1 = 4,
    #[doc = "6: Interrupt Source: Port 1.2 interrupt; Interrupt Flag: P1IFG2"]
    P1IFG2 = 6,
    #[doc = "8: Interrupt Source: Port 1.3 interrupt; Interrupt Flag: P1IFG3"]
    P1IFG3 = 8,
    #[doc = "10: Interrupt Source: Port 1.4 interrupt; Interrupt Flag: P1IFG4"]
    P1IFG4 = 10,
    #[doc = "12: Interrupt Source: Port 1.5 interrupt; Interrupt Flag: P1IFG5"]
    P1IFG5 = 12,
    #[doc = "14: Interrupt Source: Port 1.6 interrupt; Interrupt Flag: P1IFG6"]
    P1IFG6 = 14,
    #[doc = "16: Interrupt Source: Port 1.7 interrupt; Interrupt Flag: P1IFG7; Interrupt Priority: Lowest"]
    P1IFG7 = 16,
}
impl From<P1IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P1IV_A) -> Self {
        variant as _
    }
}
impl P1IV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P1IV_A> {
        match self.bits {
            0 => Some(P1IV_A::NONE),
            2 => Some(P1IV_A::P1IFG0),
            4 => Some(P1IV_A::P1IFG1),
            6 => Some(P1IV_A::P1IFG2),
            8 => Some(P1IV_A::P1IFG3),
            10 => Some(P1IV_A::P1IFG4),
            12 => Some(P1IV_A::P1IFG5),
            14 => Some(P1IV_A::P1IFG6),
            16 => Some(P1IV_A::P1IFG7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P1IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P1IFG0`"]
    #[inline(always)]
    pub fn is_p1ifg0(&self) -> bool {
        *self == P1IV_A::P1IFG0
    }
    #[doc = "Checks if the value of the field is `P1IFG1`"]
    #[inline(always)]
    pub fn is_p1ifg1(&self) -> bool {
        *self == P1IV_A::P1IFG1
    }
    #[doc = "Checks if the value of the field is `P1IFG2`"]
    #[inline(always)]
    pub fn is_p1ifg2(&self) -> bool {
        *self == P1IV_A::P1IFG2
    }
    #[doc = "Checks if the value of the field is `P1IFG3`"]
    #[inline(always)]
    pub fn is_p1ifg3(&self) -> bool {
        *self == P1IV_A::P1IFG3
    }
    #[doc = "Checks if the value of the field is `P1IFG4`"]
    #[inline(always)]
    pub fn is_p1ifg4(&self) -> bool {
        *self == P1IV_A::P1IFG4
    }
    #[doc = "Checks if the value of the field is `P1IFG5`"]
    #[inline(always)]
    pub fn is_p1ifg5(&self) -> bool {
        *self == P1IV_A::P1IFG5
    }
    #[doc = "Checks if the value of the field is `P1IFG6`"]
    #[inline(always)]
    pub fn is_p1ifg6(&self) -> bool {
        *self == P1IV_A::P1IFG6
    }
    #[doc = "Checks if the value of the field is `P1IFG7`"]
    #[inline(always)]
    pub fn is_p1ifg7(&self) -> bool {
        *self == P1IV_A::P1IFG7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 1 interrupt vector value"]
    #[inline(always)]
    pub fn p1iv(&self) -> P1IV_R {
        P1IV_R::new((self.bits & 0x1f) as u8)
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
#[doc = "Port 1 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1iv](index.html) module"]
pub struct P1IV_SPEC;
impl crate::RegisterSpec for P1IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p1iv::R](R) reader structure"]
impl crate::Readable for P1IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1iv::W](W) writer structure"]
impl crate::Writable for P1IV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1IV to value 0"]
impl crate::Resettable for P1IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
