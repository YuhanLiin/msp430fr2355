#[doc = "Register `P3IV` reader"]
pub struct R(crate::R<P3IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P3IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P3IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P3IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P3IV` writer"]
pub struct W(crate::W<P3IV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P3IV_SPEC>;
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
impl From<crate::W<P3IV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P3IV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3IV` reader - Port 3 interrupt vector value"]
pub type P3IV_R = crate::FieldReader<u8, P3IV_A>;
#[doc = "Port 3 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P3IV_A {
    #[doc = "0: No interrupt pending"]
    NONE = 0,
    #[doc = "2: Interrupt Source: Port 3.0 interrupt; Interrupt Flag: P3IFG0; Interrupt Priority: Highest"]
    P3IFG0 = 2,
    #[doc = "4: Interrupt Source: Port 3.1 interrupt; Interrupt Flag: P3IFG1"]
    P3IFG1 = 4,
    #[doc = "6: Interrupt Source: Port 3.2 interrupt; Interrupt Flag: P3IFG2"]
    P3IFG2 = 6,
    #[doc = "8: Interrupt Source: Port 3.3 interrupt; Interrupt Flag: P3IFG3"]
    P3IFG3 = 8,
    #[doc = "10: Interrupt Source: Port 3.4 interrupt; Interrupt Flag: P3IFG4"]
    P3IFG4 = 10,
    #[doc = "12: Interrupt Source: Port 3.5 interrupt; Interrupt Flag: P3IFG5"]
    P3IFG5 = 12,
    #[doc = "14: Interrupt Source: Port 3.6 interrupt; Interrupt Flag: P3IFG6"]
    P3IFG6 = 14,
    #[doc = "16: Interrupt Source: Port 3.7 interrupt; Interrupt Flag: P3IFG7; Interrupt Priority: Lowest"]
    P3IFG7 = 16,
}
impl From<P3IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P3IV_A) -> Self {
        variant as _
    }
}
impl P3IV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P3IV_A> {
        match self.bits {
            0 => Some(P3IV_A::NONE),
            2 => Some(P3IV_A::P3IFG0),
            4 => Some(P3IV_A::P3IFG1),
            6 => Some(P3IV_A::P3IFG2),
            8 => Some(P3IV_A::P3IFG3),
            10 => Some(P3IV_A::P3IFG4),
            12 => Some(P3IV_A::P3IFG5),
            14 => Some(P3IV_A::P3IFG6),
            16 => Some(P3IV_A::P3IFG7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P3IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P3IFG0`"]
    #[inline(always)]
    pub fn is_p3ifg0(&self) -> bool {
        *self == P3IV_A::P3IFG0
    }
    #[doc = "Checks if the value of the field is `P3IFG1`"]
    #[inline(always)]
    pub fn is_p3ifg1(&self) -> bool {
        *self == P3IV_A::P3IFG1
    }
    #[doc = "Checks if the value of the field is `P3IFG2`"]
    #[inline(always)]
    pub fn is_p3ifg2(&self) -> bool {
        *self == P3IV_A::P3IFG2
    }
    #[doc = "Checks if the value of the field is `P3IFG3`"]
    #[inline(always)]
    pub fn is_p3ifg3(&self) -> bool {
        *self == P3IV_A::P3IFG3
    }
    #[doc = "Checks if the value of the field is `P3IFG4`"]
    #[inline(always)]
    pub fn is_p3ifg4(&self) -> bool {
        *self == P3IV_A::P3IFG4
    }
    #[doc = "Checks if the value of the field is `P3IFG5`"]
    #[inline(always)]
    pub fn is_p3ifg5(&self) -> bool {
        *self == P3IV_A::P3IFG5
    }
    #[doc = "Checks if the value of the field is `P3IFG6`"]
    #[inline(always)]
    pub fn is_p3ifg6(&self) -> bool {
        *self == P3IV_A::P3IFG6
    }
    #[doc = "Checks if the value of the field is `P3IFG7`"]
    #[inline(always)]
    pub fn is_p3ifg7(&self) -> bool {
        *self == P3IV_A::P3IFG7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 3 interrupt vector value"]
    #[inline(always)]
    pub fn p3iv(&self) -> P3IV_R {
        P3IV_R::new((self.bits & 0x1f) as u8)
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
#[doc = "Port 3 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3iv](index.html) module"]
pub struct P3IV_SPEC;
impl crate::RegisterSpec for P3IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p3iv::R](R) reader structure"]
impl crate::Readable for P3IV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p3iv::W](W) writer structure"]
impl crate::Writable for P3IV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P3IV to value 0"]
impl crate::Resettable for P3IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
