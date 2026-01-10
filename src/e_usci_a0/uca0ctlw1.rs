#[doc = "Register `UCA0CTLW1` reader"]
pub type R = crate::R<Uca0ctlw1Spec>;
#[doc = "Register `UCA0CTLW1` writer"]
pub type W = crate::W<Uca0ctlw1Spec>;
#[doc = "Deglitch time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucglit {
    #[doc = "0: Approximately 2 ns (equivalent of 1 delay element)"]
    Ucglit0 = 0,
    #[doc = "1: Approximately 50 ns"]
    Ucglit1 = 1,
    #[doc = "2: Approximately 100 ns"]
    Ucglit2 = 2,
    #[doc = "3: Approximately 200 ns"]
    Ucglit3 = 3,
}
impl From<Ucglit> for u8 {
    #[inline(always)]
    fn from(variant: Ucglit) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucglit {
    type Ux = u8;
}
impl crate::IsEnum for Ucglit {}
#[doc = "Field `UCGLIT` reader - Deglitch time"]
pub type UcglitR = crate::FieldReader<Ucglit>;
impl UcglitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucglit {
        match self.bits {
            0 => Ucglit::Ucglit0,
            1 => Ucglit::Ucglit1,
            2 => Ucglit::Ucglit2,
            3 => Ucglit::Ucglit3,
            _ => unreachable!(),
        }
    }
    #[doc = "Approximately 2 ns (equivalent of 1 delay element)"]
    #[inline(always)]
    pub fn is_ucglit_0(&self) -> bool {
        *self == Ucglit::Ucglit0
    }
    #[doc = "Approximately 50 ns"]
    #[inline(always)]
    pub fn is_ucglit_1(&self) -> bool {
        *self == Ucglit::Ucglit1
    }
    #[doc = "Approximately 100 ns"]
    #[inline(always)]
    pub fn is_ucglit_2(&self) -> bool {
        *self == Ucglit::Ucglit2
    }
    #[doc = "Approximately 200 ns"]
    #[inline(always)]
    pub fn is_ucglit_3(&self) -> bool {
        *self == Ucglit::Ucglit3
    }
}
#[doc = "Field `UCGLIT` writer - Deglitch time"]
pub type UcglitW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucglit, crate::Safe>;
impl<'a, REG> UcglitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Approximately 2 ns (equivalent of 1 delay element)"]
    #[inline(always)]
    pub fn ucglit_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucglit::Ucglit0)
    }
    #[doc = "Approximately 50 ns"]
    #[inline(always)]
    pub fn ucglit_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucglit::Ucglit1)
    }
    #[doc = "Approximately 100 ns"]
    #[inline(always)]
    pub fn ucglit_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucglit::Ucglit2)
    }
    #[doc = "Approximately 200 ns"]
    #[inline(always)]
    pub fn ucglit_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucglit::Ucglit3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Deglitch time"]
    #[inline(always)]
    pub fn ucglit(&self) -> UcglitR {
        UcglitR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Deglitch time"]
    #[inline(always)]
    pub fn ucglit(&mut self) -> UcglitW<'_, Uca0ctlw1Spec> {
        UcglitW::new(self, 0)
    }
}
#[doc = "eUSCI_Ax Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0ctlw1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0ctlw1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0ctlw1Spec;
impl crate::RegisterSpec for Uca0ctlw1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0ctlw1::R`](R) reader structure"]
impl crate::Readable for Uca0ctlw1Spec {}
#[doc = "`write(|w| ..)` method takes [`uca0ctlw1::W`](W) writer structure"]
impl crate::Writable for Uca0ctlw1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0CTLW1 to value 0"]
impl crate::Resettable for Uca0ctlw1Spec {}
