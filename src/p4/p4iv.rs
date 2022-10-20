#[doc = "Register `P4IV` reader"]
pub struct R(crate::R<P4IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P4IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P4IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P4IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P4IV` writer"]
pub struct W(crate::W<P4IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P4IV_SPEC>;
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
impl From<crate::W<P4IV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P4IV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P4IV` reader - Port 4 interrupt vector value"]
pub type P4IV_R = crate::FieldReader<u8, P4IV_A>;
#[doc = "Port 4 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P4IV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Port 4.0 interrupt; Interrupt Flag: P4IFG0; Interrupt Priority: Highest"]
    P4IFG0 = 2,
    #[doc = "4: Interrupt Source: Port 4.1 interrupt; Interrupt Flag: P4IFG1"]
    P4IFG1 = 4,
    #[doc = "6: Interrupt Source: Port 4.2 interrupt; Interrupt Flag: P4IFG2"]
    P4IFG2 = 6,
    #[doc = "8: Interrupt Source: Port 4.3 interrupt; Interrupt Flag: P4IFG3"]
    P4IFG3 = 8,
    #[doc = "10: Interrupt Source: Port 4.4 interrupt; Interrupt Flag: P4IFG4"]
    P4IFG4 = 10,
    #[doc = "12: Interrupt Source: Port 4.5 interrupt; Interrupt Flag: P4IFG5"]
    P4IFG5 = 12,
    #[doc = "14: Interrupt Source: Port 4.6 interrupt; Interrupt Flag: P4IFG6"]
    P4IFG6 = 14,
    #[doc = "16: Interrupt Source: Port 4.7 interrupt; Interrupt Flag: P4IFG7; Interrupt Priority: Lowest"]
    P4IFG7 = 16,
}
impl From<P4IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P4IV_A) -> Self {
        variant as _
    }
}
impl P4IV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P4IV_A> {
        match self.bits {
            0 => Some(P4IV_A::NONE),
            2 => Some(P4IV_A::P4IFG0),
            4 => Some(P4IV_A::P4IFG1),
            6 => Some(P4IV_A::P4IFG2),
            8 => Some(P4IV_A::P4IFG3),
            10 => Some(P4IV_A::P4IFG4),
            12 => Some(P4IV_A::P4IFG5),
            14 => Some(P4IV_A::P4IFG6),
            16 => Some(P4IV_A::P4IFG7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P4IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P4IFG0`"]
    #[inline(always)]
    pub fn is_p4ifg0(&self) -> bool {
        *self == P4IV_A::P4IFG0
    }
    #[doc = "Checks if the value of the field is `P4IFG1`"]
    #[inline(always)]
    pub fn is_p4ifg1(&self) -> bool {
        *self == P4IV_A::P4IFG1
    }
    #[doc = "Checks if the value of the field is `P4IFG2`"]
    #[inline(always)]
    pub fn is_p4ifg2(&self) -> bool {
        *self == P4IV_A::P4IFG2
    }
    #[doc = "Checks if the value of the field is `P4IFG3`"]
    #[inline(always)]
    pub fn is_p4ifg3(&self) -> bool {
        *self == P4IV_A::P4IFG3
    }
    #[doc = "Checks if the value of the field is `P4IFG4`"]
    #[inline(always)]
    pub fn is_p4ifg4(&self) -> bool {
        *self == P4IV_A::P4IFG4
    }
    #[doc = "Checks if the value of the field is `P4IFG5`"]
    #[inline(always)]
    pub fn is_p4ifg5(&self) -> bool {
        *self == P4IV_A::P4IFG5
    }
    #[doc = "Checks if the value of the field is `P4IFG6`"]
    #[inline(always)]
    pub fn is_p4ifg6(&self) -> bool {
        *self == P4IV_A::P4IFG6
    }
    #[doc = "Checks if the value of the field is `P4IFG7`"]
    #[inline(always)]
    pub fn is_p4ifg7(&self) -> bool {
        *self == P4IV_A::P4IFG7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 4 interrupt vector value"]
    #[inline(always)]
    pub fn p4iv(&self) -> P4IV_R {
        P4IV_R::new((self.bits & 0x1f) as u8)
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
#[doc = "Port 4 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4iv](index.html) module"]
pub struct P4IV_SPEC;
impl crate::RegisterSpec for P4IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p4iv::R](R) reader structure"]
impl crate::Readable for P4IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p4iv::W](W) writer structure"]
impl crate::Writable for P4IV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P4IV to value 0"]
impl crate::Resettable for P4IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
