#[doc = "Register `P2IV` reader"]
pub struct R(crate::R<P2IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2IV` writer"]
pub struct W(crate::W<P2IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2IV_SPEC>;
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
impl From<crate::W<P2IV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2IV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2IV` reader - Port 2 interrupt vector value"]
pub type P2IV_R = crate::FieldReader<u8, P2IV_A>;
#[doc = "Port 2 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2IV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Port 2.0 interrupt; Interrupt Flag: P2IFG0; Interrupt Priority: Highest"]
    P2IFG0 = 2,
    #[doc = "4: Interrupt Source: Port 2.1 interrupt; Interrupt Flag: P2IFG1"]
    P2IFG1 = 4,
    #[doc = "6: Interrupt Source: Port 2.2 interrupt; Interrupt Flag: P2IFG2"]
    P2IFG2 = 6,
    #[doc = "8: Interrupt Source: Port 2.3 interrupt; Interrupt Flag: P2IFG3"]
    P2IFG3 = 8,
    #[doc = "10: Interrupt Source: Port 2.4 interrupt; Interrupt Flag: P2IFG4"]
    P2IFG4 = 10,
    #[doc = "12: Interrupt Source: Port 2.5 interrupt; Interrupt Flag: P2IFG5"]
    P2IFG5 = 12,
    #[doc = "14: Interrupt Source: Port 2.6 interrupt; Interrupt Flag: P2IFG6"]
    P2IFG6 = 14,
    #[doc = "16: Interrupt Source: Port 2.7 interrupt; Interrupt Flag: P2IFG7; Interrupt Priority: Lowest"]
    P2IFG7 = 16,
}
impl From<P2IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P2IV_A) -> Self {
        variant as _
    }
}
impl P2IV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P2IV_A> {
        match self.bits {
            0 => Some(P2IV_A::NONE),
            2 => Some(P2IV_A::P2IFG0),
            4 => Some(P2IV_A::P2IFG1),
            6 => Some(P2IV_A::P2IFG2),
            8 => Some(P2IV_A::P2IFG3),
            10 => Some(P2IV_A::P2IFG4),
            12 => Some(P2IV_A::P2IFG5),
            14 => Some(P2IV_A::P2IFG6),
            16 => Some(P2IV_A::P2IFG7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P2IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P2IFG0`"]
    #[inline(always)]
    pub fn is_p2ifg0(&self) -> bool {
        *self == P2IV_A::P2IFG0
    }
    #[doc = "Checks if the value of the field is `P2IFG1`"]
    #[inline(always)]
    pub fn is_p2ifg1(&self) -> bool {
        *self == P2IV_A::P2IFG1
    }
    #[doc = "Checks if the value of the field is `P2IFG2`"]
    #[inline(always)]
    pub fn is_p2ifg2(&self) -> bool {
        *self == P2IV_A::P2IFG2
    }
    #[doc = "Checks if the value of the field is `P2IFG3`"]
    #[inline(always)]
    pub fn is_p2ifg3(&self) -> bool {
        *self == P2IV_A::P2IFG3
    }
    #[doc = "Checks if the value of the field is `P2IFG4`"]
    #[inline(always)]
    pub fn is_p2ifg4(&self) -> bool {
        *self == P2IV_A::P2IFG4
    }
    #[doc = "Checks if the value of the field is `P2IFG5`"]
    #[inline(always)]
    pub fn is_p2ifg5(&self) -> bool {
        *self == P2IV_A::P2IFG5
    }
    #[doc = "Checks if the value of the field is `P2IFG6`"]
    #[inline(always)]
    pub fn is_p2ifg6(&self) -> bool {
        *self == P2IV_A::P2IFG6
    }
    #[doc = "Checks if the value of the field is `P2IFG7`"]
    #[inline(always)]
    pub fn is_p2ifg7(&self) -> bool {
        *self == P2IV_A::P2IFG7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 2 interrupt vector value"]
    #[inline(always)]
    pub fn p2iv(&self) -> P2IV_R {
        P2IV_R::new((self.bits & 0x1f) as u8)
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
#[doc = "Port 2 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2iv](index.html) module"]
pub struct P2IV_SPEC;
impl crate::RegisterSpec for P2IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p2iv::R](R) reader structure"]
impl crate::Readable for P2IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2iv::W](W) writer structure"]
impl crate::Writable for P2IV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2IV to value 0"]
impl crate::Resettable for P2IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
