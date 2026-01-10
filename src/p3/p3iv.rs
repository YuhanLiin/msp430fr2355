#[doc = "Register `P3IV` reader"]
pub type R = crate::R<P3ivSpec>;
#[doc = "Register `P3IV` writer"]
pub type W = crate::W<P3ivSpec>;
#[doc = "Port 3 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P3iv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Port 3.0 interrupt; Interrupt Flag: P3IFG0; Interrupt Priority: Highest"]
    P3ifg0 = 2,
    #[doc = "4: Interrupt Source: Port 3.1 interrupt; Interrupt Flag: P3IFG1"]
    P3ifg1 = 4,
    #[doc = "6: Interrupt Source: Port 3.2 interrupt; Interrupt Flag: P3IFG2"]
    P3ifg2 = 6,
    #[doc = "8: Interrupt Source: Port 3.3 interrupt; Interrupt Flag: P3IFG3"]
    P3ifg3 = 8,
    #[doc = "10: Interrupt Source: Port 3.4 interrupt; Interrupt Flag: P3IFG4"]
    P3ifg4 = 10,
    #[doc = "12: Interrupt Source: Port 3.5 interrupt; Interrupt Flag: P3IFG5"]
    P3ifg5 = 12,
    #[doc = "14: Interrupt Source: Port 3.6 interrupt; Interrupt Flag: P3IFG6"]
    P3ifg6 = 14,
    #[doc = "16: Interrupt Source: Port 3.7 interrupt; Interrupt Flag: P3IFG7; Interrupt Priority: Lowest"]
    P3ifg7 = 16,
}
impl From<P3iv> for u8 {
    #[inline(always)]
    fn from(variant: P3iv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P3iv {
    type Ux = u8;
}
impl crate::IsEnum for P3iv {}
#[doc = "Field `P3IV` reader - Port 3 interrupt vector value"]
pub type P3ivR = crate::FieldReader<P3iv>;
impl P3ivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<P3iv> {
        match self.bits {
            0 => Some(P3iv::None),
            2 => Some(P3iv::P3ifg0),
            4 => Some(P3iv::P3ifg1),
            6 => Some(P3iv::P3ifg2),
            8 => Some(P3iv::P3ifg3),
            10 => Some(P3iv::P3ifg4),
            12 => Some(P3iv::P3ifg5),
            14 => Some(P3iv::P3ifg6),
            16 => Some(P3iv::P3ifg7),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P3iv::None
    }
    #[doc = "Interrupt Source: Port 3.0 interrupt; Interrupt Flag: P3IFG0; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_p3ifg0(&self) -> bool {
        *self == P3iv::P3ifg0
    }
    #[doc = "Interrupt Source: Port 3.1 interrupt; Interrupt Flag: P3IFG1"]
    #[inline(always)]
    pub fn is_p3ifg1(&self) -> bool {
        *self == P3iv::P3ifg1
    }
    #[doc = "Interrupt Source: Port 3.2 interrupt; Interrupt Flag: P3IFG2"]
    #[inline(always)]
    pub fn is_p3ifg2(&self) -> bool {
        *self == P3iv::P3ifg2
    }
    #[doc = "Interrupt Source: Port 3.3 interrupt; Interrupt Flag: P3IFG3"]
    #[inline(always)]
    pub fn is_p3ifg3(&self) -> bool {
        *self == P3iv::P3ifg3
    }
    #[doc = "Interrupt Source: Port 3.4 interrupt; Interrupt Flag: P3IFG4"]
    #[inline(always)]
    pub fn is_p3ifg4(&self) -> bool {
        *self == P3iv::P3ifg4
    }
    #[doc = "Interrupt Source: Port 3.5 interrupt; Interrupt Flag: P3IFG5"]
    #[inline(always)]
    pub fn is_p3ifg5(&self) -> bool {
        *self == P3iv::P3ifg5
    }
    #[doc = "Interrupt Source: Port 3.6 interrupt; Interrupt Flag: P3IFG6"]
    #[inline(always)]
    pub fn is_p3ifg6(&self) -> bool {
        *self == P3iv::P3ifg6
    }
    #[doc = "Interrupt Source: Port 3.7 interrupt; Interrupt Flag: P3IFG7; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_p3ifg7(&self) -> bool {
        *self == P3iv::P3ifg7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 3 interrupt vector value"]
    #[inline(always)]
    pub fn p3iv(&self) -> P3ivR {
        P3ivR::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "Port 3 Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p3iv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3iv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P3ivSpec;
impl crate::RegisterSpec for P3ivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`p3iv::R`](R) reader structure"]
impl crate::Readable for P3ivSpec {}
#[doc = "`write(|w| ..)` method takes [`p3iv::W`](W) writer structure"]
impl crate::Writable for P3ivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P3IV to value 0"]
impl crate::Resettable for P3ivSpec {}
