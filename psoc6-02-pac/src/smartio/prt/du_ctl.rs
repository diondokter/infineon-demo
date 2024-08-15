#[doc = "Register `DU_CTL` reader"]
pub type R = crate::R<DuCtlSpec>;
#[doc = "Register `DU_CTL` writer"]
pub type W = crate::W<DuCtlSpec>;
#[doc = "Field `DU_SIZE` reader - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
pub type DuSizeR = crate::FieldReader;
#[doc = "Field `DU_SIZE` writer - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
pub type DuSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DU_OPC` reader - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
pub type DuOpcR = crate::FieldReader;
#[doc = "Field `DU_OPC` writer - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
pub type DuOpcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
    #[inline(always)]
    pub fn du_size(&self) -> DuSizeR {
        DuSizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
    #[inline(always)]
    pub fn du_opc(&self) -> DuOpcR {
        DuOpcR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn du_size(&mut self) -> DuSizeW<DuCtlSpec> {
        DuSizeW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
    #[inline(always)]
    #[must_use]
    pub fn du_opc(&mut self) -> DuOpcW<DuCtlSpec> {
        DuOpcW::new(self, 8)
    }
}
#[doc = "Data unit component control register\n\nYou can [`read`](crate::Reg::read) this register and get [`du_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`du_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DuCtlSpec;
impl crate::RegisterSpec for DuCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`du_ctl::R`](R) reader structure"]
impl crate::Readable for DuCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`du_ctl::W`](W) writer structure"]
impl crate::Writable for DuCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DU_CTL to value 0"]
impl crate::Resettable for DuCtlSpec {
    const RESET_VALUE: u32 = 0;
}
