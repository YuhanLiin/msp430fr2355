#[doc = "Reader of register P4IV"]
pub type R = crate::R<u16, super::P4IV>;
#[doc = "Writer for register P4IV"]
pub type W = crate::W<u16, super::P4IV>;
#[doc = "Register P4IV `reset()`'s with value 0"]
impl crate::ResetValue for super::P4IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Port 4 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P4IV_A {
    #[doc = "0: No interrupt pending"]
    NONE,
    #[doc = "2: Interrupt Source: Port 4.0 interrupt; Interrupt Flag: P4IFG0; Interrupt Priority: Highest"]
    P4IFG0,
    #[doc = "4: Interrupt Source: Port 4.1 interrupt; Interrupt Flag: P4IFG1"]
    P4IFG1,
    #[doc = "6: Interrupt Source: Port 4.2 interrupt; Interrupt Flag: P4IFG2"]
    P4IFG2,
    #[doc = "8: Interrupt Source: Port 4.3 interrupt; Interrupt Flag: P4IFG3"]
    P4IFG3,
    #[doc = "10: Interrupt Source: Port 4.4 interrupt; Interrupt Flag: P4IFG4"]
    P4IFG4,
    #[doc = "12: Interrupt Source: Port 4.5 interrupt; Interrupt Flag: P4IFG5"]
    P4IFG5,
    #[doc = "14: Interrupt Source: Port 4.6 interrupt; Interrupt Flag: P4IFG6"]
    P4IFG6,
    #[doc = "16: Interrupt Source: Port 4.7 interrupt; Interrupt Flag: P4IFG7; Interrupt Priority: Lowest"]
    P4IFG7,
}
impl From<P4IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P4IV_A) -> Self {
        match variant {
            P4IV_A::NONE => 0,
            P4IV_A::P4IFG0 => 2,
            P4IV_A::P4IFG1 => 4,
            P4IV_A::P4IFG2 => 6,
            P4IV_A::P4IFG3 => 8,
            P4IV_A::P4IFG4 => 10,
            P4IV_A::P4IFG5 => 12,
            P4IV_A::P4IFG6 => 14,
            P4IV_A::P4IFG7 => 16,
        }
    }
}
#[doc = "Reader of field `P4IV`"]
pub type P4IV_R = crate::R<u8, P4IV_A>;
impl P4IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P4IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P4IV_A::NONE),
            2 => Val(P4IV_A::P4IFG0),
            4 => Val(P4IV_A::P4IFG1),
            6 => Val(P4IV_A::P4IFG2),
            8 => Val(P4IV_A::P4IFG3),
            10 => Val(P4IV_A::P4IFG4),
            12 => Val(P4IV_A::P4IFG5),
            14 => Val(P4IV_A::P4IFG6),
            16 => Val(P4IV_A::P4IFG7),
            i => Res(i),
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
#[doc = "Write proxy for field `P4IV`"]
pub struct P4IV_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P4IV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(P4IV_A::NONE)
    }
    #[doc = "Interrupt Source: Port 4.0 interrupt; Interrupt Flag: P4IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn p4ifg0(self) -> &'a mut W {
        self.variant(P4IV_A::P4IFG0)
    }
    #[doc = "Interrupt Source: Port 4.1 interrupt; Interrupt Flag: P4IFG1"]
    #[inline(always)]
    pub fn p4ifg1(self) -> &'a mut W {
        self.variant(P4IV_A::P4IFG1)
    }
    #[doc = "Interrupt Source: Port 4.2 interrupt; Interrupt Flag: P4IFG2"]
    #[inline(always)]
    pub fn p4ifg2(self) -> &'a mut W {
        self.variant(P4IV_A::P4IFG2)
    }
    #[doc = "Interrupt Source: Port 4.3 interrupt; Interrupt Flag: P4IFG3"]
    #[inline(always)]
    pub fn p4ifg3(self) -> &'a mut W {
        self.variant(P4IV_A::P4IFG3)
    }
    #[doc = "Interrupt Source: Port 4.4 interrupt; Interrupt Flag: P4IFG4"]
    #[inline(always)]
    pub fn p4ifg4(self) -> &'a mut W {
        self.variant(P4IV_A::P4IFG4)
    }
    #[doc = "Interrupt Source: Port 4.5 interrupt; Interrupt Flag: P4IFG5"]
    #[inline(always)]
    pub fn p4ifg5(self) -> &'a mut W {
        self.variant(P4IV_A::P4IFG5)
    }
    #[doc = "Interrupt Source: Port 4.6 interrupt; Interrupt Flag: P4IFG6"]
    #[inline(always)]
    pub fn p4ifg6(self) -> &'a mut W {
        self.variant(P4IV_A::P4IFG6)
    }
    #[doc = "Interrupt Source: Port 4.7 interrupt; Interrupt Flag: P4IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn p4ifg7(self) -> &'a mut W {
        self.variant(P4IV_A::P4IFG7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
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
    #[doc = "Bits 0:4 - Port 4 interrupt vector value"]
    #[inline(always)]
    pub fn p4iv(&mut self) -> P4IV_W {
        P4IV_W { w: self }
    }
}
