#[doc = "Register `TB0EX0` reader"]
pub type R = crate::R<Tb0ex0Spec>;
#[doc = "Register `TB0EX0` writer"]
pub type W = crate::W<Tb0ex0Spec>;
#[doc = "Input divider expansion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tbidex {
    #[doc = "0: Divide by 1"]
    _1 = 0,
    #[doc = "1: Divide by 2"]
    _2 = 1,
    #[doc = "2: Divide by 3"]
    _3 = 2,
    #[doc = "3: Divide by 4"]
    _4 = 3,
    #[doc = "4: Divide by 5"]
    _5 = 4,
    #[doc = "5: Divide by 6"]
    _6 = 5,
    #[doc = "6: Divide by 7"]
    _7 = 6,
    #[doc = "7: Divide by 8"]
    _8 = 7,
}
impl From<Tbidex> for u8 {
    #[inline(always)]
    fn from(variant: Tbidex) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tbidex {
    type Ux = u8;
}
impl crate::IsEnum for Tbidex {}
#[doc = "Field `TBIDEX` reader - Input divider expansion"]
pub type TbidexR = crate::FieldReader<Tbidex>;
impl TbidexR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbidex {
        match self.bits {
            0 => Tbidex::_1,
            1 => Tbidex::_2,
            2 => Tbidex::_3,
            3 => Tbidex::_4,
            4 => Tbidex::_5,
            5 => Tbidex::_6,
            6 => Tbidex::_7,
            7 => Tbidex::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tbidex::_1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Tbidex::_2
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == Tbidex::_3
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Tbidex::_4
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == Tbidex::_5
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == Tbidex::_6
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == Tbidex::_7
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Tbidex::_8
    }
}
#[doc = "Field `TBIDEX` writer - Input divider expansion"]
pub type TbidexW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tbidex, crate::Safe>;
impl<'a, REG> TbidexW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tbidex::_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Tbidex::_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(Tbidex::_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Tbidex::_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(Tbidex::_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut crate::W<REG> {
        self.variant(Tbidex::_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut crate::W<REG> {
        self.variant(Tbidex::_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Tbidex::_8)
    }
}
impl R {
    #[doc = "Bits 0:2 - Input divider expansion"]
    #[inline(always)]
    pub fn tbidex(&self) -> TbidexR {
        TbidexR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input divider expansion"]
    #[inline(always)]
    pub fn tbidex(&mut self) -> TbidexW<'_, Tb0ex0Spec> {
        TbidexW::new(self, 0)
    }
}
#[doc = "Timer_Bx Expansion Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tb0ex0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tb0ex0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tb0ex0Spec;
impl crate::RegisterSpec for Tb0ex0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tb0ex0::R`](R) reader structure"]
impl crate::Readable for Tb0ex0Spec {}
#[doc = "`write(|w| ..)` method takes [`tb0ex0::W`](W) writer structure"]
impl crate::Writable for Tb0ex0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TB0EX0 to value 0"]
impl crate::Resettable for Tb0ex0Spec {}
