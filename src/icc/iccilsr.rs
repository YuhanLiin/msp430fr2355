#[doc = "Register `ICCILSR[%s]` reader"]
pub type R = crate::R<IccilsrSpec>;
#[doc = "Register `ICCILSR[%s]` writer"]
pub type W = crate::W<IccilsrSpec>;
#[doc = "Interrupt level of interrupt source 8 x (register index) + (field index)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ilsr {
    #[doc = "0: Level 0, the highest priority"]
    Highest = 0,
    #[doc = "1: Level 1"]
    High = 1,
    #[doc = "2: Level 2"]
    Low = 2,
    #[doc = "3: Level 3, the lowest priority"]
    Lowest = 3,
}
impl From<Ilsr> for u8 {
    #[inline(always)]
    fn from(variant: Ilsr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ilsr {
    type Ux = u8;
}
impl crate::IsEnum for Ilsr {}
#[doc = "Field `ILSR(0-7)` reader - Interrupt level of interrupt source 8 x (register index) + (field index)"]
pub type IlsrR = crate::FieldReader<Ilsr>;
impl IlsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilsr {
        match self.bits {
            0 => Ilsr::Highest,
            1 => Ilsr::High,
            2 => Ilsr::Low,
            3 => Ilsr::Lowest,
            _ => unreachable!(),
        }
    }
    #[doc = "Level 0, the highest priority"]
    #[inline(always)]
    pub fn is_highest(&self) -> bool {
        *self == Ilsr::Highest
    }
    #[doc = "Level 1"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ilsr::High
    }
    #[doc = "Level 2"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ilsr::Low
    }
    #[doc = "Level 3, the lowest priority"]
    #[inline(always)]
    pub fn is_lowest(&self) -> bool {
        *self == Ilsr::Lowest
    }
}
#[doc = "Field `ILSR(0-7)` writer - Interrupt level of interrupt source 8 x (register index) + (field index)"]
pub type IlsrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ilsr, crate::Safe>;
impl<'a, REG> IlsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Level 0, the highest priority"]
    #[inline(always)]
    pub fn highest(self) -> &'a mut crate::W<REG> {
        self.variant(Ilsr::Highest)
    }
    #[doc = "Level 1"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ilsr::High)
    }
    #[doc = "Level 2"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ilsr::Low)
    }
    #[doc = "Level 3, the lowest priority"]
    #[inline(always)]
    pub fn lowest(self) -> &'a mut crate::W<REG> {
        self.variant(Ilsr::Lowest)
    }
}
impl R {
    #[doc = "Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ILSR0` field.</div>"]
    #[inline(always)]
    pub fn ilsr(&self, n: u8) -> IlsrR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        IlsrR::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr_iter(&self) -> impl Iterator<Item = IlsrR> + '_ {
        (0..8).map(move |n| IlsrR::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr0(&self) -> IlsrR {
        IlsrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr1(&self) -> IlsrR {
        IlsrR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr2(&self) -> IlsrR {
        IlsrR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr3(&self) -> IlsrR {
        IlsrR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr4(&self) -> IlsrR {
        IlsrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr5(&self) -> IlsrR {
        IlsrR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr6(&self) -> IlsrR {
        IlsrR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr7(&self) -> IlsrR {
        IlsrR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `ILSR0` field.</div>"]
    #[inline(always)]
    pub fn ilsr(&mut self, n: u8) -> IlsrW<'_, IccilsrSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        IlsrW::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr0(&mut self) -> IlsrW<'_, IccilsrSpec> {
        IlsrW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr1(&mut self) -> IlsrW<'_, IccilsrSpec> {
        IlsrW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr2(&mut self) -> IlsrW<'_, IccilsrSpec> {
        IlsrW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr3(&mut self) -> IlsrW<'_, IccilsrSpec> {
        IlsrW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr4(&mut self) -> IlsrW<'_, IccilsrSpec> {
        IlsrW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr5(&mut self) -> IlsrW<'_, IccilsrSpec> {
        IlsrW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr6(&mut self) -> IlsrW<'_, IccilsrSpec> {
        IlsrW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Interrupt level of interrupt source 8 x (register index) + (field index)"]
    #[inline(always)]
    pub fn ilsr7(&mut self) -> IlsrW<'_, IccilsrSpec> {
        IlsrW::new(self, 14)
    }
}
#[doc = "Interrupt Compare Controller Interrupt Level Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iccilsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccilsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IccilsrSpec;
impl crate::RegisterSpec for IccilsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`iccilsr::R`](R) reader structure"]
impl crate::Readable for IccilsrSpec {}
#[doc = "`write(|w| ..)` method takes [`iccilsr::W`](W) writer structure"]
impl crate::Writable for IccilsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICCILSR[%s] to value 0"]
impl crate::Resettable for IccilsrSpec {}
