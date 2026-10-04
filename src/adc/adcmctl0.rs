#[doc = "Register `ADCMCTL0` reader"]
pub type R = crate::R<Adcmctl0Spec>;
#[doc = "Register `ADCMCTL0` writer"]
pub type W = crate::W<Adcmctl0Spec>;
#[doc = "Input channel select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcinch {
    #[doc = "0: A0 - see device-specific data sheet"]
    Adcinch0 = 0,
    #[doc = "1: A1 - see device-specific data sheet"]
    Adcinch1 = 1,
    #[doc = "2: A2 - see device-specific data sheet"]
    Adcinch2 = 2,
    #[doc = "3: A3 - see device-specific data sheet"]
    Adcinch3 = 3,
    #[doc = "4: A4 - see device-specific data sheet"]
    Adcinch4 = 4,
    #[doc = "5: A5 - see device-specific data sheet"]
    Adcinch5 = 5,
    #[doc = "6: A2 - see device-specific data sheet"]
    Adcinch6 = 6,
    #[doc = "7: A7 - see device-specific data sheet"]
    Adcinch7 = 7,
    #[doc = "8: A8 - see device-specific data sheet"]
    Adcinch8 = 8,
    #[doc = "9: A9 - see device-specific data sheet"]
    Adcinch9 = 9,
    #[doc = "10: A10 - see device-specific data sheet"]
    Adcinch10 = 10,
    #[doc = "11: A11 - see device-specific data sheet"]
    Adcinch11 = 11,
    #[doc = "12: A12 - see device-specific data sheet"]
    Adcinch12 = 12,
    #[doc = "13: A13 - see device-specific data sheet"]
    Adcinch13 = 13,
    #[doc = "14: A14 - see device-specific data sheet"]
    Adcinch14 = 14,
    #[doc = "15: A15 - see device-specific data sheet"]
    Adcinch15 = 15,
}
impl From<Adcinch> for u8 {
    #[inline(always)]
    fn from(variant: Adcinch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcinch {
    type Ux = u8;
}
impl crate::IsEnum for Adcinch {}
#[doc = "Field `ADCINCH` reader - Input channel select"]
pub type AdcinchR = crate::FieldReader<Adcinch>;
impl AdcinchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcinch {
        match self.bits {
            0 => Adcinch::Adcinch0,
            1 => Adcinch::Adcinch1,
            2 => Adcinch::Adcinch2,
            3 => Adcinch::Adcinch3,
            4 => Adcinch::Adcinch4,
            5 => Adcinch::Adcinch5,
            6 => Adcinch::Adcinch6,
            7 => Adcinch::Adcinch7,
            8 => Adcinch::Adcinch8,
            9 => Adcinch::Adcinch9,
            10 => Adcinch::Adcinch10,
            11 => Adcinch::Adcinch11,
            12 => Adcinch::Adcinch12,
            13 => Adcinch::Adcinch13,
            14 => Adcinch::Adcinch14,
            15 => Adcinch::Adcinch15,
            _ => unreachable!(),
        }
    }
    #[doc = "A0 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_0(&self) -> bool {
        *self == Adcinch::Adcinch0
    }
    #[doc = "A1 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_1(&self) -> bool {
        *self == Adcinch::Adcinch1
    }
    #[doc = "A2 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_2(&self) -> bool {
        *self == Adcinch::Adcinch2
    }
    #[doc = "A3 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_3(&self) -> bool {
        *self == Adcinch::Adcinch3
    }
    #[doc = "A4 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_4(&self) -> bool {
        *self == Adcinch::Adcinch4
    }
    #[doc = "A5 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_5(&self) -> bool {
        *self == Adcinch::Adcinch5
    }
    #[doc = "A2 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_6(&self) -> bool {
        *self == Adcinch::Adcinch6
    }
    #[doc = "A7 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_7(&self) -> bool {
        *self == Adcinch::Adcinch7
    }
    #[doc = "A8 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_8(&self) -> bool {
        *self == Adcinch::Adcinch8
    }
    #[doc = "A9 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_9(&self) -> bool {
        *self == Adcinch::Adcinch9
    }
    #[doc = "A10 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_10(&self) -> bool {
        *self == Adcinch::Adcinch10
    }
    #[doc = "A11 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_11(&self) -> bool {
        *self == Adcinch::Adcinch11
    }
    #[doc = "A12 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_12(&self) -> bool {
        *self == Adcinch::Adcinch12
    }
    #[doc = "A13 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_13(&self) -> bool {
        *self == Adcinch::Adcinch13
    }
    #[doc = "A14 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_14(&self) -> bool {
        *self == Adcinch::Adcinch14
    }
    #[doc = "A15 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_15(&self) -> bool {
        *self == Adcinch::Adcinch15
    }
}
#[doc = "Field `ADCINCH` writer - Input channel select"]
pub type AdcinchW<'a, REG> = crate::FieldWriter<'a, REG, 4, Adcinch, crate::Safe>;
impl<'a, REG> AdcinchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A0 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch0)
    }
    #[doc = "A1 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch1)
    }
    #[doc = "A2 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch2)
    }
    #[doc = "A3 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch3)
    }
    #[doc = "A4 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch4)
    }
    #[doc = "A5 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch5)
    }
    #[doc = "A2 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch6)
    }
    #[doc = "A7 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch7)
    }
    #[doc = "A8 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch8)
    }
    #[doc = "A9 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_9(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch9)
    }
    #[doc = "A10 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_10(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch10)
    }
    #[doc = "A11 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_11(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch11)
    }
    #[doc = "A12 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_12(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch12)
    }
    #[doc = "A13 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_13(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch13)
    }
    #[doc = "A14 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_14(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch14)
    }
    #[doc = "A15 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_15(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch15)
    }
}
#[doc = "Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcsref {
    #[doc = "0: VR+ = AVCC and VR- = AVSS"]
    AvccAvss = 0,
    #[doc = "1: VR+ = VREF and VR- = AVSS"]
    VrefAvss = 1,
    #[doc = "2: VR+ = VEREF+ buffered and VR- = AVSS"]
    VerefPlusBufferedAvss = 2,
    #[doc = "3: VR+ = VEREF+ and VR- = AVSS"]
    VerefPlusAvss = 3,
    #[doc = "4: VR+ = AVCC and VR- = VEREF-"]
    AvccVerefMinus = 4,
    #[doc = "5: VR+ = VREF and VR- = VEREF-"]
    VrefVerefMinus = 5,
    #[doc = "6: VR+ = VEREF+ buffered and VR- = VEREF-"]
    VerefPlusBufferedVerefMinus = 6,
    #[doc = "7: VR+ = VEREF+ and VR- = VEREF-"]
    VerefPlusVerefMinus = 7,
}
impl From<Adcsref> for u8 {
    #[inline(always)]
    fn from(variant: Adcsref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcsref {
    type Ux = u8;
}
impl crate::IsEnum for Adcsref {}
#[doc = "Field `ADCSREF` reader - Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active."]
pub type AdcsrefR = crate::FieldReader<Adcsref>;
impl AdcsrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcsref {
        match self.bits {
            0 => Adcsref::AvccAvss,
            1 => Adcsref::VrefAvss,
            2 => Adcsref::VerefPlusBufferedAvss,
            3 => Adcsref::VerefPlusAvss,
            4 => Adcsref::AvccVerefMinus,
            5 => Adcsref::VrefVerefMinus,
            6 => Adcsref::VerefPlusBufferedVerefMinus,
            7 => Adcsref::VerefPlusVerefMinus,
            _ => unreachable!(),
        }
    }
    #[doc = "VR+ = AVCC and VR- = AVSS"]
    #[inline(always)]
    pub fn is_avcc_avss(&self) -> bool {
        *self == Adcsref::AvccAvss
    }
    #[doc = "VR+ = VREF and VR- = AVSS"]
    #[inline(always)]
    pub fn is_vref_avss(&self) -> bool {
        *self == Adcsref::VrefAvss
    }
    #[doc = "VR+ = VEREF+ buffered and VR- = AVSS"]
    #[inline(always)]
    pub fn is_veref_plus_buffered_avss(&self) -> bool {
        *self == Adcsref::VerefPlusBufferedAvss
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn is_veref_plus_avss(&self) -> bool {
        *self == Adcsref::VerefPlusAvss
    }
    #[doc = "VR+ = AVCC and VR- = VEREF-"]
    #[inline(always)]
    pub fn is_avcc_veref_minus(&self) -> bool {
        *self == Adcsref::AvccVerefMinus
    }
    #[doc = "VR+ = VREF and VR- = VEREF-"]
    #[inline(always)]
    pub fn is_vref_veref_minus(&self) -> bool {
        *self == Adcsref::VrefVerefMinus
    }
    #[doc = "VR+ = VEREF+ buffered and VR- = VEREF-"]
    #[inline(always)]
    pub fn is_veref_plus_buffered_veref_minus(&self) -> bool {
        *self == Adcsref::VerefPlusBufferedVerefMinus
    }
    #[doc = "VR+ = VEREF+ and VR- = VEREF-"]
    #[inline(always)]
    pub fn is_veref_plus_veref_minus(&self) -> bool {
        *self == Adcsref::VerefPlusVerefMinus
    }
}
#[doc = "Field `ADCSREF` writer - Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active."]
pub type AdcsrefW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adcsref, crate::Safe>;
impl<'a, REG> AdcsrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VR+ = AVCC and VR- = AVSS"]
    #[inline(always)]
    pub fn avcc_avss(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::AvccAvss)
    }
    #[doc = "VR+ = VREF and VR- = AVSS"]
    #[inline(always)]
    pub fn vref_avss(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::VrefAvss)
    }
    #[doc = "VR+ = VEREF+ buffered and VR- = AVSS"]
    #[inline(always)]
    pub fn veref_plus_buffered_avss(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::VerefPlusBufferedAvss)
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn veref_plus_avss(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::VerefPlusAvss)
    }
    #[doc = "VR+ = AVCC and VR- = VEREF-"]
    #[inline(always)]
    pub fn avcc_veref_minus(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::AvccVerefMinus)
    }
    #[doc = "VR+ = VREF and VR- = VEREF-"]
    #[inline(always)]
    pub fn vref_veref_minus(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::VrefVerefMinus)
    }
    #[doc = "VR+ = VEREF+ buffered and VR- = VEREF-"]
    #[inline(always)]
    pub fn veref_plus_buffered_veref_minus(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::VerefPlusBufferedVerefMinus)
    }
    #[doc = "VR+ = VEREF+ and VR- = VEREF-"]
    #[inline(always)]
    pub fn veref_plus_veref_minus(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::VerefPlusVerefMinus)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input channel select"]
    #[inline(always)]
    pub fn adcinch(&self) -> AdcinchR {
        AdcinchR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active."]
    #[inline(always)]
    pub fn adcsref(&self) -> AdcsrefR {
        AdcsrefR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input channel select"]
    #[inline(always)]
    pub fn adcinch(&mut self) -> AdcinchW<'_, Adcmctl0Spec> {
        AdcinchW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active."]
    #[inline(always)]
    pub fn adcsref(&mut self) -> AdcsrefW<'_, Adcmctl0Spec> {
        AdcsrefW::new(self, 4)
    }
}
#[doc = "ADC Conversion Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcmctl0Spec;
impl crate::RegisterSpec for Adcmctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmctl0::R`](R) reader structure"]
impl crate::Readable for Adcmctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`adcmctl0::W`](W) writer structure"]
impl crate::Writable for Adcmctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMCTL0 to value 0"]
impl crate::Resettable for Adcmctl0Spec {}
