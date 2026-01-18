#[doc = "Register `CPDACDATA` reader"]
pub type R = crate::R<CpdacdataSpec>;
#[doc = "Register `CPDACDATA` writer"]
pub type W = crate::W<CpdacdataSpec>;
#[doc = "1st 6-bit DAC buffer Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpdacbuf1 {
    #[doc = "0: 0v"]
    Cpdacbuf1_0 = 0,
    #[doc = "1: selected reference voltage * 1/64"]
    Cpdacbuf1_1 = 1,
    #[doc = "2: selected reference voltage * 2/64"]
    Cpdacbuf1_2 = 2,
    #[doc = "3: selected reference voltage * 3/64"]
    Cpdacbuf1_3 = 3,
    #[doc = "4: selected reference voltage * 4/64"]
    Cpdacbuf1_4 = 4,
    #[doc = "5: selected reference voltage * 5/64"]
    Cpdacbuf1_5 = 5,
    #[doc = "6: selected reference voltage * 6/64"]
    Cpdacbuf1_6 = 6,
    #[doc = "7: selected reference voltage * 7/64"]
    Cpdacbuf1_7 = 7,
    #[doc = "8: selected reference voltage * 8/64"]
    Cpdacbuf1_8 = 8,
    #[doc = "9: selected reference voltage *9/64"]
    Cpdacbuf1_9 = 9,
    #[doc = "10: selected reference voltage * 10/64"]
    Cpdacbuf1_10 = 10,
    #[doc = "11: selected reference voltage * 11/64"]
    Cpdacbuf1_11 = 11,
    #[doc = "12: selected reference voltage * 12/64"]
    Cpdacbuf1_12 = 12,
    #[doc = "13: selected reference voltage * 13/64"]
    Cpdacbuf1_13 = 13,
    #[doc = "14: selected reference voltage * 14/64"]
    Cpdacbuf1_14 = 14,
    #[doc = "15: selected reference voltage * 15/64"]
    Cpdacbuf1_15 = 15,
    #[doc = "16: selected reference voltage * 16/64"]
    Cpdacbuf1_16 = 16,
    #[doc = "17: selected reference voltage * 17/64"]
    Cpdacbuf1_17 = 17,
    #[doc = "18: selected reference voltage * 18/64"]
    Cpdacbuf1_18 = 18,
    #[doc = "19: selected reference voltage * 19/64"]
    Cpdacbuf1_19 = 19,
    #[doc = "20: selected reference voltage * 20/64"]
    Cpdacbuf1_20 = 20,
    #[doc = "21: selected reference voltage * 21/64"]
    Cpdacbuf1_21 = 21,
    #[doc = "22: selected reference voltage * 22/64"]
    Cpdacbuf1_22 = 22,
    #[doc = "23: selected reference voltage * 23/64"]
    Cpdacbuf1_23 = 23,
    #[doc = "24: selected reference voltage * 24/64"]
    Cpdacbuf1_24 = 24,
    #[doc = "25: selected reference voltage * 25/64"]
    Cpdacbuf1_25 = 25,
    #[doc = "26: selected reference voltage * 26/64"]
    Cpdacbuf1_26 = 26,
    #[doc = "27: selected reference voltage * 27/64"]
    Cpdacbuf1_27 = 27,
    #[doc = "28: selected reference voltage * 28/64"]
    Cpdacbuf1_28 = 28,
    #[doc = "29: selected reference voltage * 29/64"]
    Cpdacbuf1_29 = 29,
    #[doc = "30: selected reference voltage * 30/64"]
    Cpdacbuf1_30 = 30,
    #[doc = "31: selected reference voltage * 31/64"]
    Cpdacbuf1_31 = 31,
    #[doc = "32: selected reference voltage * 32/64"]
    Cpdacbuf1_32 = 32,
    #[doc = "33: selected reference voltage * 33/64"]
    Cpdacbuf1_33 = 33,
    #[doc = "34: selected reference voltage * 34/64"]
    Cpdacbuf1_34 = 34,
    #[doc = "35: selected reference voltage * 35/64"]
    Cpdacbuf1_35 = 35,
    #[doc = "36: selected reference voltage * 36/64"]
    Cpdacbuf1_36 = 36,
    #[doc = "37: selected reference voltage * 37/64"]
    Cpdacbuf1_37 = 37,
    #[doc = "38: selected reference voltage * 38/64"]
    Cpdacbuf1_38 = 38,
    #[doc = "39: selected reference voltage * 39/64"]
    Cpdacbuf1_39 = 39,
    #[doc = "40: selected reference voltage * 40/64"]
    Cpdacbuf1_40 = 40,
    #[doc = "41: selected reference voltage * 41/64"]
    Cpdacbuf1_41 = 41,
    #[doc = "42: selected reference voltage * 42/64"]
    Cpdacbuf1_42 = 42,
    #[doc = "43: selected reference voltage * 43/64"]
    Cpdacbuf1_43 = 43,
    #[doc = "44: selected reference voltage * 44/64"]
    Cpdacbuf1_44 = 44,
    #[doc = "45: selected reference voltage * 45/64"]
    Cpdacbuf1_45 = 45,
    #[doc = "46: selected reference voltage * 46/64"]
    Cpdacbuf1_46 = 46,
    #[doc = "47: selected reference voltage * 47/64"]
    Cpdacbuf1_47 = 47,
    #[doc = "48: selected reference voltage * 48/64"]
    Cpdacbuf1_48 = 48,
    #[doc = "49: selected reference voltage * 49/64"]
    Cpdacbuf1_49 = 49,
    #[doc = "50: selected reference voltage * 50/64"]
    Cpdacbuf1_50 = 50,
    #[doc = "51: selected reference voltage * 51/64"]
    Cpdacbuf1_51 = 51,
    #[doc = "52: selected reference voltage * 52/64"]
    Cpdacbuf1_52 = 52,
    #[doc = "53: selected reference voltage * 53/64"]
    Cpdacbuf1_53 = 53,
    #[doc = "54: selected reference voltage * 54/64"]
    Cpdacbuf1_54 = 54,
    #[doc = "55: selected reference voltage * 55/64"]
    Cpdacbuf1_55 = 55,
    #[doc = "56: selected reference voltage * 56/64"]
    Cpdacbuf1_56 = 56,
    #[doc = "57: selected reference voltage * 57/64"]
    Cpdacbuf1_57 = 57,
    #[doc = "58: selected reference voltage * 58/64"]
    Cpdacbuf1_58 = 58,
    #[doc = "59: selected reference voltage * 59/64"]
    Cpdacbuf1_59 = 59,
    #[doc = "60: selected reference voltage * 60/64"]
    Cpdacbuf1_60 = 60,
    #[doc = "61: selected reference voltage * 61/64"]
    Cpdacbuf1_61 = 61,
    #[doc = "62: selected reference voltage * 62/64"]
    Cpdacbuf1_62 = 62,
    #[doc = "63: selected reference voltage * 63/64"]
    Cpdacbuf1_63 = 63,
}
impl From<Cpdacbuf1> for u8 {
    #[inline(always)]
    fn from(variant: Cpdacbuf1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpdacbuf1 {
    type Ux = u8;
}
impl crate::IsEnum for Cpdacbuf1 {}
#[doc = "Field `CPDACBUF1` reader - 1st 6-bit DAC buffer Data"]
pub type Cpdacbuf1R = crate::FieldReader<Cpdacbuf1>;
impl Cpdacbuf1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpdacbuf1 {
        match self.bits {
            0 => Cpdacbuf1::Cpdacbuf1_0,
            1 => Cpdacbuf1::Cpdacbuf1_1,
            2 => Cpdacbuf1::Cpdacbuf1_2,
            3 => Cpdacbuf1::Cpdacbuf1_3,
            4 => Cpdacbuf1::Cpdacbuf1_4,
            5 => Cpdacbuf1::Cpdacbuf1_5,
            6 => Cpdacbuf1::Cpdacbuf1_6,
            7 => Cpdacbuf1::Cpdacbuf1_7,
            8 => Cpdacbuf1::Cpdacbuf1_8,
            9 => Cpdacbuf1::Cpdacbuf1_9,
            10 => Cpdacbuf1::Cpdacbuf1_10,
            11 => Cpdacbuf1::Cpdacbuf1_11,
            12 => Cpdacbuf1::Cpdacbuf1_12,
            13 => Cpdacbuf1::Cpdacbuf1_13,
            14 => Cpdacbuf1::Cpdacbuf1_14,
            15 => Cpdacbuf1::Cpdacbuf1_15,
            16 => Cpdacbuf1::Cpdacbuf1_16,
            17 => Cpdacbuf1::Cpdacbuf1_17,
            18 => Cpdacbuf1::Cpdacbuf1_18,
            19 => Cpdacbuf1::Cpdacbuf1_19,
            20 => Cpdacbuf1::Cpdacbuf1_20,
            21 => Cpdacbuf1::Cpdacbuf1_21,
            22 => Cpdacbuf1::Cpdacbuf1_22,
            23 => Cpdacbuf1::Cpdacbuf1_23,
            24 => Cpdacbuf1::Cpdacbuf1_24,
            25 => Cpdacbuf1::Cpdacbuf1_25,
            26 => Cpdacbuf1::Cpdacbuf1_26,
            27 => Cpdacbuf1::Cpdacbuf1_27,
            28 => Cpdacbuf1::Cpdacbuf1_28,
            29 => Cpdacbuf1::Cpdacbuf1_29,
            30 => Cpdacbuf1::Cpdacbuf1_30,
            31 => Cpdacbuf1::Cpdacbuf1_31,
            32 => Cpdacbuf1::Cpdacbuf1_32,
            33 => Cpdacbuf1::Cpdacbuf1_33,
            34 => Cpdacbuf1::Cpdacbuf1_34,
            35 => Cpdacbuf1::Cpdacbuf1_35,
            36 => Cpdacbuf1::Cpdacbuf1_36,
            37 => Cpdacbuf1::Cpdacbuf1_37,
            38 => Cpdacbuf1::Cpdacbuf1_38,
            39 => Cpdacbuf1::Cpdacbuf1_39,
            40 => Cpdacbuf1::Cpdacbuf1_40,
            41 => Cpdacbuf1::Cpdacbuf1_41,
            42 => Cpdacbuf1::Cpdacbuf1_42,
            43 => Cpdacbuf1::Cpdacbuf1_43,
            44 => Cpdacbuf1::Cpdacbuf1_44,
            45 => Cpdacbuf1::Cpdacbuf1_45,
            46 => Cpdacbuf1::Cpdacbuf1_46,
            47 => Cpdacbuf1::Cpdacbuf1_47,
            48 => Cpdacbuf1::Cpdacbuf1_48,
            49 => Cpdacbuf1::Cpdacbuf1_49,
            50 => Cpdacbuf1::Cpdacbuf1_50,
            51 => Cpdacbuf1::Cpdacbuf1_51,
            52 => Cpdacbuf1::Cpdacbuf1_52,
            53 => Cpdacbuf1::Cpdacbuf1_53,
            54 => Cpdacbuf1::Cpdacbuf1_54,
            55 => Cpdacbuf1::Cpdacbuf1_55,
            56 => Cpdacbuf1::Cpdacbuf1_56,
            57 => Cpdacbuf1::Cpdacbuf1_57,
            58 => Cpdacbuf1::Cpdacbuf1_58,
            59 => Cpdacbuf1::Cpdacbuf1_59,
            60 => Cpdacbuf1::Cpdacbuf1_60,
            61 => Cpdacbuf1::Cpdacbuf1_61,
            62 => Cpdacbuf1::Cpdacbuf1_62,
            63 => Cpdacbuf1::Cpdacbuf1_63,
            _ => unreachable!(),
        }
    }
    #[doc = "0v"]
    #[inline(always)]
    pub fn is_cpdacbuf1_0(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_0
    }
    #[doc = "selected reference voltage * 1/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_1(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_1
    }
    #[doc = "selected reference voltage * 2/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_2(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_2
    }
    #[doc = "selected reference voltage * 3/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_3(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_3
    }
    #[doc = "selected reference voltage * 4/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_4(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_4
    }
    #[doc = "selected reference voltage * 5/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_5(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_5
    }
    #[doc = "selected reference voltage * 6/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_6(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_6
    }
    #[doc = "selected reference voltage * 7/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_7(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_7
    }
    #[doc = "selected reference voltage * 8/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_8(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_8
    }
    #[doc = "selected reference voltage *9/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_9(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_9
    }
    #[doc = "selected reference voltage * 10/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_10(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_10
    }
    #[doc = "selected reference voltage * 11/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_11(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_11
    }
    #[doc = "selected reference voltage * 12/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_12(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_12
    }
    #[doc = "selected reference voltage * 13/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_13(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_13
    }
    #[doc = "selected reference voltage * 14/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_14(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_14
    }
    #[doc = "selected reference voltage * 15/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_15(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_15
    }
    #[doc = "selected reference voltage * 16/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_16(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_16
    }
    #[doc = "selected reference voltage * 17/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_17(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_17
    }
    #[doc = "selected reference voltage * 18/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_18(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_18
    }
    #[doc = "selected reference voltage * 19/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_19(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_19
    }
    #[doc = "selected reference voltage * 20/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_20(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_20
    }
    #[doc = "selected reference voltage * 21/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_21(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_21
    }
    #[doc = "selected reference voltage * 22/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_22(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_22
    }
    #[doc = "selected reference voltage * 23/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_23(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_23
    }
    #[doc = "selected reference voltage * 24/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_24(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_24
    }
    #[doc = "selected reference voltage * 25/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_25(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_25
    }
    #[doc = "selected reference voltage * 26/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_26(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_26
    }
    #[doc = "selected reference voltage * 27/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_27(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_27
    }
    #[doc = "selected reference voltage * 28/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_28(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_28
    }
    #[doc = "selected reference voltage * 29/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_29(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_29
    }
    #[doc = "selected reference voltage * 30/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_30(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_30
    }
    #[doc = "selected reference voltage * 31/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_31(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_31
    }
    #[doc = "selected reference voltage * 32/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_32(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_32
    }
    #[doc = "selected reference voltage * 33/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_33(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_33
    }
    #[doc = "selected reference voltage * 34/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_34(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_34
    }
    #[doc = "selected reference voltage * 35/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_35(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_35
    }
    #[doc = "selected reference voltage * 36/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_36(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_36
    }
    #[doc = "selected reference voltage * 37/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_37(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_37
    }
    #[doc = "selected reference voltage * 38/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_38(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_38
    }
    #[doc = "selected reference voltage * 39/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_39(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_39
    }
    #[doc = "selected reference voltage * 40/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_40(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_40
    }
    #[doc = "selected reference voltage * 41/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_41(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_41
    }
    #[doc = "selected reference voltage * 42/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_42(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_42
    }
    #[doc = "selected reference voltage * 43/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_43(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_43
    }
    #[doc = "selected reference voltage * 44/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_44(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_44
    }
    #[doc = "selected reference voltage * 45/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_45(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_45
    }
    #[doc = "selected reference voltage * 46/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_46(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_46
    }
    #[doc = "selected reference voltage * 47/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_47(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_47
    }
    #[doc = "selected reference voltage * 48/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_48(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_48
    }
    #[doc = "selected reference voltage * 49/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_49(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_49
    }
    #[doc = "selected reference voltage * 50/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_50(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_50
    }
    #[doc = "selected reference voltage * 51/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_51(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_51
    }
    #[doc = "selected reference voltage * 52/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_52(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_52
    }
    #[doc = "selected reference voltage * 53/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_53(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_53
    }
    #[doc = "selected reference voltage * 54/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_54(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_54
    }
    #[doc = "selected reference voltage * 55/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_55(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_55
    }
    #[doc = "selected reference voltage * 56/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_56(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_56
    }
    #[doc = "selected reference voltage * 57/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_57(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_57
    }
    #[doc = "selected reference voltage * 58/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_58(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_58
    }
    #[doc = "selected reference voltage * 59/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_59(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_59
    }
    #[doc = "selected reference voltage * 60/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_60(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_60
    }
    #[doc = "selected reference voltage * 61/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_61(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_61
    }
    #[doc = "selected reference voltage * 62/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_62(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_62
    }
    #[doc = "selected reference voltage * 63/64"]
    #[inline(always)]
    pub fn is_cpdacbuf1_63(&self) -> bool {
        *self == Cpdacbuf1::Cpdacbuf1_63
    }
}
#[doc = "Field `CPDACBUF1` writer - 1st 6-bit DAC buffer Data"]
pub type Cpdacbuf1W<'a, REG> = crate::FieldWriter<'a, REG, 6, Cpdacbuf1, crate::Safe>;
impl<'a, REG> Cpdacbuf1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0v"]
    #[inline(always)]
    pub fn cpdacbuf1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_0)
    }
    #[doc = "selected reference voltage * 1/64"]
    #[inline(always)]
    pub fn cpdacbuf1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_1)
    }
    #[doc = "selected reference voltage * 2/64"]
    #[inline(always)]
    pub fn cpdacbuf1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_2)
    }
    #[doc = "selected reference voltage * 3/64"]
    #[inline(always)]
    pub fn cpdacbuf1_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_3)
    }
    #[doc = "selected reference voltage * 4/64"]
    #[inline(always)]
    pub fn cpdacbuf1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_4)
    }
    #[doc = "selected reference voltage * 5/64"]
    #[inline(always)]
    pub fn cpdacbuf1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_5)
    }
    #[doc = "selected reference voltage * 6/64"]
    #[inline(always)]
    pub fn cpdacbuf1_6(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_6)
    }
    #[doc = "selected reference voltage * 7/64"]
    #[inline(always)]
    pub fn cpdacbuf1_7(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_7)
    }
    #[doc = "selected reference voltage * 8/64"]
    #[inline(always)]
    pub fn cpdacbuf1_8(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_8)
    }
    #[doc = "selected reference voltage *9/64"]
    #[inline(always)]
    pub fn cpdacbuf1_9(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_9)
    }
    #[doc = "selected reference voltage * 10/64"]
    #[inline(always)]
    pub fn cpdacbuf1_10(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_10)
    }
    #[doc = "selected reference voltage * 11/64"]
    #[inline(always)]
    pub fn cpdacbuf1_11(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_11)
    }
    #[doc = "selected reference voltage * 12/64"]
    #[inline(always)]
    pub fn cpdacbuf1_12(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_12)
    }
    #[doc = "selected reference voltage * 13/64"]
    #[inline(always)]
    pub fn cpdacbuf1_13(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_13)
    }
    #[doc = "selected reference voltage * 14/64"]
    #[inline(always)]
    pub fn cpdacbuf1_14(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_14)
    }
    #[doc = "selected reference voltage * 15/64"]
    #[inline(always)]
    pub fn cpdacbuf1_15(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_15)
    }
    #[doc = "selected reference voltage * 16/64"]
    #[inline(always)]
    pub fn cpdacbuf1_16(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_16)
    }
    #[doc = "selected reference voltage * 17/64"]
    #[inline(always)]
    pub fn cpdacbuf1_17(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_17)
    }
    #[doc = "selected reference voltage * 18/64"]
    #[inline(always)]
    pub fn cpdacbuf1_18(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_18)
    }
    #[doc = "selected reference voltage * 19/64"]
    #[inline(always)]
    pub fn cpdacbuf1_19(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_19)
    }
    #[doc = "selected reference voltage * 20/64"]
    #[inline(always)]
    pub fn cpdacbuf1_20(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_20)
    }
    #[doc = "selected reference voltage * 21/64"]
    #[inline(always)]
    pub fn cpdacbuf1_21(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_21)
    }
    #[doc = "selected reference voltage * 22/64"]
    #[inline(always)]
    pub fn cpdacbuf1_22(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_22)
    }
    #[doc = "selected reference voltage * 23/64"]
    #[inline(always)]
    pub fn cpdacbuf1_23(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_23)
    }
    #[doc = "selected reference voltage * 24/64"]
    #[inline(always)]
    pub fn cpdacbuf1_24(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_24)
    }
    #[doc = "selected reference voltage * 25/64"]
    #[inline(always)]
    pub fn cpdacbuf1_25(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_25)
    }
    #[doc = "selected reference voltage * 26/64"]
    #[inline(always)]
    pub fn cpdacbuf1_26(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_26)
    }
    #[doc = "selected reference voltage * 27/64"]
    #[inline(always)]
    pub fn cpdacbuf1_27(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_27)
    }
    #[doc = "selected reference voltage * 28/64"]
    #[inline(always)]
    pub fn cpdacbuf1_28(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_28)
    }
    #[doc = "selected reference voltage * 29/64"]
    #[inline(always)]
    pub fn cpdacbuf1_29(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_29)
    }
    #[doc = "selected reference voltage * 30/64"]
    #[inline(always)]
    pub fn cpdacbuf1_30(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_30)
    }
    #[doc = "selected reference voltage * 31/64"]
    #[inline(always)]
    pub fn cpdacbuf1_31(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_31)
    }
    #[doc = "selected reference voltage * 32/64"]
    #[inline(always)]
    pub fn cpdacbuf1_32(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_32)
    }
    #[doc = "selected reference voltage * 33/64"]
    #[inline(always)]
    pub fn cpdacbuf1_33(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_33)
    }
    #[doc = "selected reference voltage * 34/64"]
    #[inline(always)]
    pub fn cpdacbuf1_34(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_34)
    }
    #[doc = "selected reference voltage * 35/64"]
    #[inline(always)]
    pub fn cpdacbuf1_35(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_35)
    }
    #[doc = "selected reference voltage * 36/64"]
    #[inline(always)]
    pub fn cpdacbuf1_36(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_36)
    }
    #[doc = "selected reference voltage * 37/64"]
    #[inline(always)]
    pub fn cpdacbuf1_37(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_37)
    }
    #[doc = "selected reference voltage * 38/64"]
    #[inline(always)]
    pub fn cpdacbuf1_38(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_38)
    }
    #[doc = "selected reference voltage * 39/64"]
    #[inline(always)]
    pub fn cpdacbuf1_39(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_39)
    }
    #[doc = "selected reference voltage * 40/64"]
    #[inline(always)]
    pub fn cpdacbuf1_40(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_40)
    }
    #[doc = "selected reference voltage * 41/64"]
    #[inline(always)]
    pub fn cpdacbuf1_41(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_41)
    }
    #[doc = "selected reference voltage * 42/64"]
    #[inline(always)]
    pub fn cpdacbuf1_42(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_42)
    }
    #[doc = "selected reference voltage * 43/64"]
    #[inline(always)]
    pub fn cpdacbuf1_43(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_43)
    }
    #[doc = "selected reference voltage * 44/64"]
    #[inline(always)]
    pub fn cpdacbuf1_44(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_44)
    }
    #[doc = "selected reference voltage * 45/64"]
    #[inline(always)]
    pub fn cpdacbuf1_45(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_45)
    }
    #[doc = "selected reference voltage * 46/64"]
    #[inline(always)]
    pub fn cpdacbuf1_46(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_46)
    }
    #[doc = "selected reference voltage * 47/64"]
    #[inline(always)]
    pub fn cpdacbuf1_47(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_47)
    }
    #[doc = "selected reference voltage * 48/64"]
    #[inline(always)]
    pub fn cpdacbuf1_48(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_48)
    }
    #[doc = "selected reference voltage * 49/64"]
    #[inline(always)]
    pub fn cpdacbuf1_49(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_49)
    }
    #[doc = "selected reference voltage * 50/64"]
    #[inline(always)]
    pub fn cpdacbuf1_50(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_50)
    }
    #[doc = "selected reference voltage * 51/64"]
    #[inline(always)]
    pub fn cpdacbuf1_51(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_51)
    }
    #[doc = "selected reference voltage * 52/64"]
    #[inline(always)]
    pub fn cpdacbuf1_52(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_52)
    }
    #[doc = "selected reference voltage * 53/64"]
    #[inline(always)]
    pub fn cpdacbuf1_53(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_53)
    }
    #[doc = "selected reference voltage * 54/64"]
    #[inline(always)]
    pub fn cpdacbuf1_54(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_54)
    }
    #[doc = "selected reference voltage * 55/64"]
    #[inline(always)]
    pub fn cpdacbuf1_55(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_55)
    }
    #[doc = "selected reference voltage * 56/64"]
    #[inline(always)]
    pub fn cpdacbuf1_56(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_56)
    }
    #[doc = "selected reference voltage * 57/64"]
    #[inline(always)]
    pub fn cpdacbuf1_57(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_57)
    }
    #[doc = "selected reference voltage * 58/64"]
    #[inline(always)]
    pub fn cpdacbuf1_58(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_58)
    }
    #[doc = "selected reference voltage * 59/64"]
    #[inline(always)]
    pub fn cpdacbuf1_59(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_59)
    }
    #[doc = "selected reference voltage * 60/64"]
    #[inline(always)]
    pub fn cpdacbuf1_60(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_60)
    }
    #[doc = "selected reference voltage * 61/64"]
    #[inline(always)]
    pub fn cpdacbuf1_61(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_61)
    }
    #[doc = "selected reference voltage * 62/64"]
    #[inline(always)]
    pub fn cpdacbuf1_62(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_62)
    }
    #[doc = "selected reference voltage * 63/64"]
    #[inline(always)]
    pub fn cpdacbuf1_63(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf1::Cpdacbuf1_63)
    }
}
#[doc = "2nd 6-bit DAC buffer Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpdacbuf2 {
    #[doc = "0: 0v"]
    Cpdacbuf2_0 = 0,
    #[doc = "1: selected reference voltage * 1/64"]
    Cpdacbuf2_1 = 1,
    #[doc = "2: selected reference voltage * 2/64"]
    Cpdacbuf2_2 = 2,
    #[doc = "3: selected reference voltage * 3/64"]
    Cpdacbuf2_3 = 3,
    #[doc = "4: selected reference voltage * 4/64"]
    Cpdacbuf2_4 = 4,
    #[doc = "5: selected reference voltage * 5/64"]
    Cpdacbuf2_5 = 5,
    #[doc = "6: selected reference voltage * 6/64"]
    Cpdacbuf2_6 = 6,
    #[doc = "7: selected reference voltage * 7/64"]
    Cpdacbuf2_7 = 7,
    #[doc = "8: selected reference voltage * 8/64"]
    Cpdacbuf2_8 = 8,
    #[doc = "9: selected reference voltage * 9/64"]
    Cpdacbuf2_9 = 9,
    #[doc = "10: selected reference voltage * 10/64"]
    Cpdacbuf2_10 = 10,
    #[doc = "11: selected reference voltage * 11/64"]
    Cpdacbuf2_11 = 11,
    #[doc = "12: selected reference voltage * 12/64"]
    Cpdacbuf2_12 = 12,
    #[doc = "13: selected reference voltage * 13/64"]
    Cpdacbuf2_13 = 13,
    #[doc = "14: selected reference voltage * 14/64"]
    Cpdacbuf2_14 = 14,
    #[doc = "15: selected reference voltage * 15/64"]
    Cpdacbuf2_15 = 15,
    #[doc = "16: selected reference voltage * 16/64"]
    Cpdacbuf2_16 = 16,
    #[doc = "17: selected reference voltage * 17/64"]
    Cpdacbuf2_17 = 17,
    #[doc = "18: selected reference voltage * 18/64"]
    Cpdacbuf2_18 = 18,
    #[doc = "19: selected reference voltage * 19/64"]
    Cpdacbuf2_19 = 19,
    #[doc = "20: selected reference voltage * 20/64"]
    Cpdacbuf2_20 = 20,
    #[doc = "21: selected reference voltage * 21/64"]
    Cpdacbuf2_21 = 21,
    #[doc = "22: selected reference voltage * 22/64"]
    Cpdacbuf2_22 = 22,
    #[doc = "23: selected reference voltage * 23/64"]
    Cpdacbuf2_23 = 23,
    #[doc = "24: selected reference voltage * 24/64"]
    Cpdacbuf2_24 = 24,
    #[doc = "25: selected reference voltage * 25/64"]
    Cpdacbuf2_25 = 25,
    #[doc = "26: selected reference voltage * 26/64"]
    Cpdacbuf2_26 = 26,
    #[doc = "27: selected reference voltage * 27/64"]
    Cpdacbuf2_27 = 27,
    #[doc = "28: selected reference voltage * 28/64"]
    Cpdacbuf2_28 = 28,
    #[doc = "29: selected reference voltage * 29/64"]
    Cpdacbuf2_29 = 29,
    #[doc = "30: selected reference voltage * 30/64"]
    Cpdacbuf2_30 = 30,
    #[doc = "31: selected reference voltage * 31/64"]
    Cpdacbuf2_31 = 31,
    #[doc = "32: selected reference voltage * 32/64"]
    Cpdacbuf2_32 = 32,
    #[doc = "33: selected reference voltage * 33/64"]
    Cpdacbuf2_33 = 33,
    #[doc = "34: selected reference voltage * 34/64"]
    Cpdacbuf2_34 = 34,
    #[doc = "35: selected reference voltage * 35/64"]
    Cpdacbuf2_35 = 35,
    #[doc = "36: selected reference voltage * 36/64"]
    Cpdacbuf2_36 = 36,
    #[doc = "37: selected reference voltage * 37/64"]
    Cpdacbuf2_37 = 37,
    #[doc = "38: selected reference voltage * 38/64"]
    Cpdacbuf2_38 = 38,
    #[doc = "39: selected reference voltage * 39/64"]
    Cpdacbuf2_39 = 39,
    #[doc = "40: selected reference voltage * 40/64"]
    Cpdacbuf2_40 = 40,
    #[doc = "41: selected reference voltage * 41/64"]
    Cpdacbuf2_41 = 41,
    #[doc = "42: selected reference voltage * 42/64"]
    Cpdacbuf2_42 = 42,
    #[doc = "43: selected reference voltage * 43/64"]
    Cpdacbuf2_43 = 43,
    #[doc = "44: selected reference voltage * 44/64"]
    Cpdacbuf2_44 = 44,
    #[doc = "45: selected reference voltage * 45/64"]
    Cpdacbuf2_45 = 45,
    #[doc = "46: selected reference voltage * 46/64"]
    Cpdacbuf2_46 = 46,
    #[doc = "47: selected reference voltage * 47/64"]
    Cpdacbuf2_47 = 47,
    #[doc = "48: selected reference voltage * 48/64"]
    Cpdacbuf2_48 = 48,
    #[doc = "49: selected reference voltage * 49/64"]
    Cpdacbuf2_49 = 49,
    #[doc = "50: selected reference voltage * 50/64"]
    Cpdacbuf2_50 = 50,
    #[doc = "51: selected reference voltage * 51/64"]
    Cpdacbuf2_51 = 51,
    #[doc = "52: selected reference voltage * 52/64"]
    Cpdacbuf2_52 = 52,
    #[doc = "53: selected reference voltage * 53/64"]
    Cpdacbuf2_53 = 53,
    #[doc = "54: selected reference voltage * 54/64"]
    Cpdacbuf2_54 = 54,
    #[doc = "55: selected reference voltage * 55/64"]
    Cpdacbuf2_55 = 55,
    #[doc = "56: selected reference voltage * 56/64"]
    Cpdacbuf2_56 = 56,
    #[doc = "57: selected reference voltage * 57/64"]
    Cpdacbuf2_57 = 57,
    #[doc = "58: selected reference voltage * 58/64"]
    Cpdacbuf2_58 = 58,
    #[doc = "59: selected reference voltage * 59/64"]
    Cpdacbuf2_59 = 59,
    #[doc = "60: selected reference voltage * 60/64"]
    Cpdacbuf2_60 = 60,
    #[doc = "61: selected reference voltage * 61/64"]
    Cpdacbuf2_61 = 61,
    #[doc = "62: selected reference voltage * 62/64"]
    Cpdacbuf2_62 = 62,
    #[doc = "63: selected reference voltage * 63/64"]
    Cpdacbuf2_63 = 63,
}
impl From<Cpdacbuf2> for u8 {
    #[inline(always)]
    fn from(variant: Cpdacbuf2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpdacbuf2 {
    type Ux = u8;
}
impl crate::IsEnum for Cpdacbuf2 {}
#[doc = "Field `CPDACBUF2` reader - 2nd 6-bit DAC buffer Data"]
pub type Cpdacbuf2R = crate::FieldReader<Cpdacbuf2>;
impl Cpdacbuf2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpdacbuf2 {
        match self.bits {
            0 => Cpdacbuf2::Cpdacbuf2_0,
            1 => Cpdacbuf2::Cpdacbuf2_1,
            2 => Cpdacbuf2::Cpdacbuf2_2,
            3 => Cpdacbuf2::Cpdacbuf2_3,
            4 => Cpdacbuf2::Cpdacbuf2_4,
            5 => Cpdacbuf2::Cpdacbuf2_5,
            6 => Cpdacbuf2::Cpdacbuf2_6,
            7 => Cpdacbuf2::Cpdacbuf2_7,
            8 => Cpdacbuf2::Cpdacbuf2_8,
            9 => Cpdacbuf2::Cpdacbuf2_9,
            10 => Cpdacbuf2::Cpdacbuf2_10,
            11 => Cpdacbuf2::Cpdacbuf2_11,
            12 => Cpdacbuf2::Cpdacbuf2_12,
            13 => Cpdacbuf2::Cpdacbuf2_13,
            14 => Cpdacbuf2::Cpdacbuf2_14,
            15 => Cpdacbuf2::Cpdacbuf2_15,
            16 => Cpdacbuf2::Cpdacbuf2_16,
            17 => Cpdacbuf2::Cpdacbuf2_17,
            18 => Cpdacbuf2::Cpdacbuf2_18,
            19 => Cpdacbuf2::Cpdacbuf2_19,
            20 => Cpdacbuf2::Cpdacbuf2_20,
            21 => Cpdacbuf2::Cpdacbuf2_21,
            22 => Cpdacbuf2::Cpdacbuf2_22,
            23 => Cpdacbuf2::Cpdacbuf2_23,
            24 => Cpdacbuf2::Cpdacbuf2_24,
            25 => Cpdacbuf2::Cpdacbuf2_25,
            26 => Cpdacbuf2::Cpdacbuf2_26,
            27 => Cpdacbuf2::Cpdacbuf2_27,
            28 => Cpdacbuf2::Cpdacbuf2_28,
            29 => Cpdacbuf2::Cpdacbuf2_29,
            30 => Cpdacbuf2::Cpdacbuf2_30,
            31 => Cpdacbuf2::Cpdacbuf2_31,
            32 => Cpdacbuf2::Cpdacbuf2_32,
            33 => Cpdacbuf2::Cpdacbuf2_33,
            34 => Cpdacbuf2::Cpdacbuf2_34,
            35 => Cpdacbuf2::Cpdacbuf2_35,
            36 => Cpdacbuf2::Cpdacbuf2_36,
            37 => Cpdacbuf2::Cpdacbuf2_37,
            38 => Cpdacbuf2::Cpdacbuf2_38,
            39 => Cpdacbuf2::Cpdacbuf2_39,
            40 => Cpdacbuf2::Cpdacbuf2_40,
            41 => Cpdacbuf2::Cpdacbuf2_41,
            42 => Cpdacbuf2::Cpdacbuf2_42,
            43 => Cpdacbuf2::Cpdacbuf2_43,
            44 => Cpdacbuf2::Cpdacbuf2_44,
            45 => Cpdacbuf2::Cpdacbuf2_45,
            46 => Cpdacbuf2::Cpdacbuf2_46,
            47 => Cpdacbuf2::Cpdacbuf2_47,
            48 => Cpdacbuf2::Cpdacbuf2_48,
            49 => Cpdacbuf2::Cpdacbuf2_49,
            50 => Cpdacbuf2::Cpdacbuf2_50,
            51 => Cpdacbuf2::Cpdacbuf2_51,
            52 => Cpdacbuf2::Cpdacbuf2_52,
            53 => Cpdacbuf2::Cpdacbuf2_53,
            54 => Cpdacbuf2::Cpdacbuf2_54,
            55 => Cpdacbuf2::Cpdacbuf2_55,
            56 => Cpdacbuf2::Cpdacbuf2_56,
            57 => Cpdacbuf2::Cpdacbuf2_57,
            58 => Cpdacbuf2::Cpdacbuf2_58,
            59 => Cpdacbuf2::Cpdacbuf2_59,
            60 => Cpdacbuf2::Cpdacbuf2_60,
            61 => Cpdacbuf2::Cpdacbuf2_61,
            62 => Cpdacbuf2::Cpdacbuf2_62,
            63 => Cpdacbuf2::Cpdacbuf2_63,
            _ => unreachable!(),
        }
    }
    #[doc = "0v"]
    #[inline(always)]
    pub fn is_cpdacbuf2_0(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_0
    }
    #[doc = "selected reference voltage * 1/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_1(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_1
    }
    #[doc = "selected reference voltage * 2/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_2(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_2
    }
    #[doc = "selected reference voltage * 3/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_3(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_3
    }
    #[doc = "selected reference voltage * 4/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_4(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_4
    }
    #[doc = "selected reference voltage * 5/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_5(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_5
    }
    #[doc = "selected reference voltage * 6/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_6(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_6
    }
    #[doc = "selected reference voltage * 7/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_7(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_7
    }
    #[doc = "selected reference voltage * 8/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_8(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_8
    }
    #[doc = "selected reference voltage * 9/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_9(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_9
    }
    #[doc = "selected reference voltage * 10/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_10(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_10
    }
    #[doc = "selected reference voltage * 11/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_11(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_11
    }
    #[doc = "selected reference voltage * 12/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_12(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_12
    }
    #[doc = "selected reference voltage * 13/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_13(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_13
    }
    #[doc = "selected reference voltage * 14/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_14(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_14
    }
    #[doc = "selected reference voltage * 15/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_15(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_15
    }
    #[doc = "selected reference voltage * 16/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_16(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_16
    }
    #[doc = "selected reference voltage * 17/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_17(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_17
    }
    #[doc = "selected reference voltage * 18/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_18(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_18
    }
    #[doc = "selected reference voltage * 19/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_19(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_19
    }
    #[doc = "selected reference voltage * 20/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_20(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_20
    }
    #[doc = "selected reference voltage * 21/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_21(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_21
    }
    #[doc = "selected reference voltage * 22/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_22(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_22
    }
    #[doc = "selected reference voltage * 23/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_23(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_23
    }
    #[doc = "selected reference voltage * 24/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_24(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_24
    }
    #[doc = "selected reference voltage * 25/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_25(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_25
    }
    #[doc = "selected reference voltage * 26/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_26(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_26
    }
    #[doc = "selected reference voltage * 27/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_27(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_27
    }
    #[doc = "selected reference voltage * 28/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_28(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_28
    }
    #[doc = "selected reference voltage * 29/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_29(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_29
    }
    #[doc = "selected reference voltage * 30/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_30(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_30
    }
    #[doc = "selected reference voltage * 31/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_31(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_31
    }
    #[doc = "selected reference voltage * 32/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_32(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_32
    }
    #[doc = "selected reference voltage * 33/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_33(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_33
    }
    #[doc = "selected reference voltage * 34/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_34(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_34
    }
    #[doc = "selected reference voltage * 35/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_35(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_35
    }
    #[doc = "selected reference voltage * 36/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_36(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_36
    }
    #[doc = "selected reference voltage * 37/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_37(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_37
    }
    #[doc = "selected reference voltage * 38/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_38(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_38
    }
    #[doc = "selected reference voltage * 39/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_39(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_39
    }
    #[doc = "selected reference voltage * 40/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_40(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_40
    }
    #[doc = "selected reference voltage * 41/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_41(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_41
    }
    #[doc = "selected reference voltage * 42/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_42(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_42
    }
    #[doc = "selected reference voltage * 43/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_43(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_43
    }
    #[doc = "selected reference voltage * 44/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_44(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_44
    }
    #[doc = "selected reference voltage * 45/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_45(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_45
    }
    #[doc = "selected reference voltage * 46/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_46(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_46
    }
    #[doc = "selected reference voltage * 47/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_47(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_47
    }
    #[doc = "selected reference voltage * 48/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_48(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_48
    }
    #[doc = "selected reference voltage * 49/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_49(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_49
    }
    #[doc = "selected reference voltage * 50/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_50(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_50
    }
    #[doc = "selected reference voltage * 51/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_51(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_51
    }
    #[doc = "selected reference voltage * 52/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_52(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_52
    }
    #[doc = "selected reference voltage * 53/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_53(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_53
    }
    #[doc = "selected reference voltage * 54/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_54(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_54
    }
    #[doc = "selected reference voltage * 55/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_55(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_55
    }
    #[doc = "selected reference voltage * 56/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_56(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_56
    }
    #[doc = "selected reference voltage * 57/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_57(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_57
    }
    #[doc = "selected reference voltage * 58/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_58(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_58
    }
    #[doc = "selected reference voltage * 59/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_59(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_59
    }
    #[doc = "selected reference voltage * 60/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_60(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_60
    }
    #[doc = "selected reference voltage * 61/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_61(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_61
    }
    #[doc = "selected reference voltage * 62/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_62(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_62
    }
    #[doc = "selected reference voltage * 63/64"]
    #[inline(always)]
    pub fn is_cpdacbuf2_63(&self) -> bool {
        *self == Cpdacbuf2::Cpdacbuf2_63
    }
}
#[doc = "Field `CPDACBUF2` writer - 2nd 6-bit DAC buffer Data"]
pub type Cpdacbuf2W<'a, REG> = crate::FieldWriter<'a, REG, 6, Cpdacbuf2, crate::Safe>;
impl<'a, REG> Cpdacbuf2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0v"]
    #[inline(always)]
    pub fn cpdacbuf2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_0)
    }
    #[doc = "selected reference voltage * 1/64"]
    #[inline(always)]
    pub fn cpdacbuf2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_1)
    }
    #[doc = "selected reference voltage * 2/64"]
    #[inline(always)]
    pub fn cpdacbuf2_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_2)
    }
    #[doc = "selected reference voltage * 3/64"]
    #[inline(always)]
    pub fn cpdacbuf2_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_3)
    }
    #[doc = "selected reference voltage * 4/64"]
    #[inline(always)]
    pub fn cpdacbuf2_4(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_4)
    }
    #[doc = "selected reference voltage * 5/64"]
    #[inline(always)]
    pub fn cpdacbuf2_5(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_5)
    }
    #[doc = "selected reference voltage * 6/64"]
    #[inline(always)]
    pub fn cpdacbuf2_6(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_6)
    }
    #[doc = "selected reference voltage * 7/64"]
    #[inline(always)]
    pub fn cpdacbuf2_7(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_7)
    }
    #[doc = "selected reference voltage * 8/64"]
    #[inline(always)]
    pub fn cpdacbuf2_8(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_8)
    }
    #[doc = "selected reference voltage * 9/64"]
    #[inline(always)]
    pub fn cpdacbuf2_9(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_9)
    }
    #[doc = "selected reference voltage * 10/64"]
    #[inline(always)]
    pub fn cpdacbuf2_10(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_10)
    }
    #[doc = "selected reference voltage * 11/64"]
    #[inline(always)]
    pub fn cpdacbuf2_11(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_11)
    }
    #[doc = "selected reference voltage * 12/64"]
    #[inline(always)]
    pub fn cpdacbuf2_12(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_12)
    }
    #[doc = "selected reference voltage * 13/64"]
    #[inline(always)]
    pub fn cpdacbuf2_13(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_13)
    }
    #[doc = "selected reference voltage * 14/64"]
    #[inline(always)]
    pub fn cpdacbuf2_14(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_14)
    }
    #[doc = "selected reference voltage * 15/64"]
    #[inline(always)]
    pub fn cpdacbuf2_15(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_15)
    }
    #[doc = "selected reference voltage * 16/64"]
    #[inline(always)]
    pub fn cpdacbuf2_16(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_16)
    }
    #[doc = "selected reference voltage * 17/64"]
    #[inline(always)]
    pub fn cpdacbuf2_17(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_17)
    }
    #[doc = "selected reference voltage * 18/64"]
    #[inline(always)]
    pub fn cpdacbuf2_18(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_18)
    }
    #[doc = "selected reference voltage * 19/64"]
    #[inline(always)]
    pub fn cpdacbuf2_19(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_19)
    }
    #[doc = "selected reference voltage * 20/64"]
    #[inline(always)]
    pub fn cpdacbuf2_20(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_20)
    }
    #[doc = "selected reference voltage * 21/64"]
    #[inline(always)]
    pub fn cpdacbuf2_21(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_21)
    }
    #[doc = "selected reference voltage * 22/64"]
    #[inline(always)]
    pub fn cpdacbuf2_22(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_22)
    }
    #[doc = "selected reference voltage * 23/64"]
    #[inline(always)]
    pub fn cpdacbuf2_23(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_23)
    }
    #[doc = "selected reference voltage * 24/64"]
    #[inline(always)]
    pub fn cpdacbuf2_24(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_24)
    }
    #[doc = "selected reference voltage * 25/64"]
    #[inline(always)]
    pub fn cpdacbuf2_25(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_25)
    }
    #[doc = "selected reference voltage * 26/64"]
    #[inline(always)]
    pub fn cpdacbuf2_26(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_26)
    }
    #[doc = "selected reference voltage * 27/64"]
    #[inline(always)]
    pub fn cpdacbuf2_27(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_27)
    }
    #[doc = "selected reference voltage * 28/64"]
    #[inline(always)]
    pub fn cpdacbuf2_28(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_28)
    }
    #[doc = "selected reference voltage * 29/64"]
    #[inline(always)]
    pub fn cpdacbuf2_29(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_29)
    }
    #[doc = "selected reference voltage * 30/64"]
    #[inline(always)]
    pub fn cpdacbuf2_30(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_30)
    }
    #[doc = "selected reference voltage * 31/64"]
    #[inline(always)]
    pub fn cpdacbuf2_31(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_31)
    }
    #[doc = "selected reference voltage * 32/64"]
    #[inline(always)]
    pub fn cpdacbuf2_32(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_32)
    }
    #[doc = "selected reference voltage * 33/64"]
    #[inline(always)]
    pub fn cpdacbuf2_33(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_33)
    }
    #[doc = "selected reference voltage * 34/64"]
    #[inline(always)]
    pub fn cpdacbuf2_34(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_34)
    }
    #[doc = "selected reference voltage * 35/64"]
    #[inline(always)]
    pub fn cpdacbuf2_35(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_35)
    }
    #[doc = "selected reference voltage * 36/64"]
    #[inline(always)]
    pub fn cpdacbuf2_36(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_36)
    }
    #[doc = "selected reference voltage * 37/64"]
    #[inline(always)]
    pub fn cpdacbuf2_37(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_37)
    }
    #[doc = "selected reference voltage * 38/64"]
    #[inline(always)]
    pub fn cpdacbuf2_38(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_38)
    }
    #[doc = "selected reference voltage * 39/64"]
    #[inline(always)]
    pub fn cpdacbuf2_39(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_39)
    }
    #[doc = "selected reference voltage * 40/64"]
    #[inline(always)]
    pub fn cpdacbuf2_40(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_40)
    }
    #[doc = "selected reference voltage * 41/64"]
    #[inline(always)]
    pub fn cpdacbuf2_41(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_41)
    }
    #[doc = "selected reference voltage * 42/64"]
    #[inline(always)]
    pub fn cpdacbuf2_42(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_42)
    }
    #[doc = "selected reference voltage * 43/64"]
    #[inline(always)]
    pub fn cpdacbuf2_43(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_43)
    }
    #[doc = "selected reference voltage * 44/64"]
    #[inline(always)]
    pub fn cpdacbuf2_44(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_44)
    }
    #[doc = "selected reference voltage * 45/64"]
    #[inline(always)]
    pub fn cpdacbuf2_45(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_45)
    }
    #[doc = "selected reference voltage * 46/64"]
    #[inline(always)]
    pub fn cpdacbuf2_46(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_46)
    }
    #[doc = "selected reference voltage * 47/64"]
    #[inline(always)]
    pub fn cpdacbuf2_47(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_47)
    }
    #[doc = "selected reference voltage * 48/64"]
    #[inline(always)]
    pub fn cpdacbuf2_48(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_48)
    }
    #[doc = "selected reference voltage * 49/64"]
    #[inline(always)]
    pub fn cpdacbuf2_49(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_49)
    }
    #[doc = "selected reference voltage * 50/64"]
    #[inline(always)]
    pub fn cpdacbuf2_50(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_50)
    }
    #[doc = "selected reference voltage * 51/64"]
    #[inline(always)]
    pub fn cpdacbuf2_51(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_51)
    }
    #[doc = "selected reference voltage * 52/64"]
    #[inline(always)]
    pub fn cpdacbuf2_52(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_52)
    }
    #[doc = "selected reference voltage * 53/64"]
    #[inline(always)]
    pub fn cpdacbuf2_53(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_53)
    }
    #[doc = "selected reference voltage * 54/64"]
    #[inline(always)]
    pub fn cpdacbuf2_54(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_54)
    }
    #[doc = "selected reference voltage * 55/64"]
    #[inline(always)]
    pub fn cpdacbuf2_55(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_55)
    }
    #[doc = "selected reference voltage * 56/64"]
    #[inline(always)]
    pub fn cpdacbuf2_56(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_56)
    }
    #[doc = "selected reference voltage * 57/64"]
    #[inline(always)]
    pub fn cpdacbuf2_57(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_57)
    }
    #[doc = "selected reference voltage * 58/64"]
    #[inline(always)]
    pub fn cpdacbuf2_58(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_58)
    }
    #[doc = "selected reference voltage * 59/64"]
    #[inline(always)]
    pub fn cpdacbuf2_59(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_59)
    }
    #[doc = "selected reference voltage * 60/64"]
    #[inline(always)]
    pub fn cpdacbuf2_60(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_60)
    }
    #[doc = "selected reference voltage * 61/64"]
    #[inline(always)]
    pub fn cpdacbuf2_61(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_61)
    }
    #[doc = "selected reference voltage * 62/64"]
    #[inline(always)]
    pub fn cpdacbuf2_62(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_62)
    }
    #[doc = "selected reference voltage * 63/64"]
    #[inline(always)]
    pub fn cpdacbuf2_63(self) -> &'a mut crate::W<REG> {
        self.variant(Cpdacbuf2::Cpdacbuf2_63)
    }
}
impl R {
    #[doc = "Bits 0:5 - 1st 6-bit DAC buffer Data"]
    #[inline(always)]
    pub fn cpdacbuf1(&self) -> Cpdacbuf1R {
        Cpdacbuf1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 2nd 6-bit DAC buffer Data"]
    #[inline(always)]
    pub fn cpdacbuf2(&self) -> Cpdacbuf2R {
        Cpdacbuf2R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 1st 6-bit DAC buffer Data"]
    #[inline(always)]
    pub fn cpdacbuf1(&mut self) -> Cpdacbuf1W<'_, CpdacdataSpec> {
        Cpdacbuf1W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 2nd 6-bit DAC buffer Data"]
    #[inline(always)]
    pub fn cpdacbuf2(&mut self) -> Cpdacbuf2W<'_, CpdacdataSpec> {
        Cpdacbuf2W::new(self, 8)
    }
}
#[doc = "6-bit Comparator built-in DAC Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpdacdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpdacdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpdacdataSpec;
impl crate::RegisterSpec for CpdacdataSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cpdacdata::R`](R) reader structure"]
impl crate::Readable for CpdacdataSpec {}
#[doc = "`write(|w| ..)` method takes [`cpdacdata::W`](W) writer structure"]
impl crate::Writable for CpdacdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPDACDATA to value 0"]
impl crate::Resettable for CpdacdataSpec {}
