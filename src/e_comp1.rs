# [ doc = r"Register block" ] # [ repr ( C ) ] pub struct RegisterBlock { # [ doc = "0x00 - Comparator Interrupt Vector Word Register" ] pub cp1iv : CP1IV , _reserved1 : [ u8 ; 6usize ] , # [ doc = "0x08 - 6-bit Comparator built-in DAC Control Register" ] pub cp1dacctl : CP1DACCTL , # [ doc = "0x0a - 6-bit Comparator built-in DAC Data Register" ] pub cp1dacdata : CP1DACDATA , } # [ doc = "Comparator Interrupt Vector Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cp1iv](cp1iv) module" ] pub type CP1IV = crate :: Reg < u16 , _CP1IV > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _CP1IV ; # [ doc = "`read()` method returns [cp1iv::R](cp1iv::R) reader structure" ] impl crate :: Readable for CP1IV { } # [ doc = "`write(|w| ..)` method takes [cp1iv::W](cp1iv::W) writer structure" ] impl crate :: Writable for CP1IV { } # [ doc = "Comparator Interrupt Vector Word Register" ] pub mod cp1iv ; # [ doc = "6-bit Comparator built-in DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cp1dacctl](cp1dacctl) module" ] pub type CP1DACCTL = crate :: Reg < u16 , _CP1DACCTL > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _CP1DACCTL ; # [ doc = "`read()` method returns [cp1dacctl::R](cp1dacctl::R) reader structure" ] impl crate :: Readable for CP1DACCTL { } # [ doc = "`write(|w| ..)` method takes [cp1dacctl::W](cp1dacctl::W) writer structure" ] impl crate :: Writable for CP1DACCTL { } # [ doc = "6-bit Comparator built-in DAC Control Register" ] pub mod cp1dacctl ; # [ doc = "6-bit Comparator built-in DAC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cp1dacdata](cp1dacdata) module" ] pub type CP1DACDATA = crate :: Reg < u16 , _CP1DACDATA > ; # [ allow ( missing_docs ) ] # [ doc ( hidden ) ] pub struct _CP1DACDATA ; # [ doc = "`read()` method returns [cp1dacdata::R](cp1dacdata::R) reader structure" ] impl crate :: Readable for CP1DACDATA { } # [ doc = "`write(|w| ..)` method takes [cp1dacdata::W](cp1dacdata::W) writer structure" ] impl crate :: Writable for CP1DACDATA { } # [ doc = "6-bit Comparator built-in DAC Data Register" ] pub mod cp1dacdata ;