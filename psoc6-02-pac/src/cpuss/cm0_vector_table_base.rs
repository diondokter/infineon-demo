#[doc = "Register `CM0_VECTOR_TABLE_BASE` reader"]
pub type R = crate::R<Cm0VectorTableBaseSpec>;
#[doc = "Register `CM0_VECTOR_TABLE_BASE` writer"]
pub type W = crate::W<Cm0VectorTableBaseSpec>;
#[doc = "Field `ADDR24` reader - Address of CM0+ vector table. This register is used for CM0+ warm boot purposes: the CM0+ warm boot code uses the register to initialize the CM0+ internal VTOR register. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
pub type Addr24R = crate::FieldReader<u32>;
#[doc = "Field `ADDR24` writer - Address of CM0+ vector table. This register is used for CM0+ warm boot purposes: the CM0+ warm boot code uses the register to initialize the CM0+ internal VTOR register. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
pub type Addr24W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - Address of CM0+ vector table. This register is used for CM0+ warm boot purposes: the CM0+ warm boot code uses the register to initialize the CM0+ internal VTOR register. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
    #[inline(always)]
    pub fn addr24(&self) -> Addr24R {
        Addr24R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Address of CM0+ vector table. This register is used for CM0+ warm boot purposes: the CM0+ warm boot code uses the register to initialize the CM0+ internal VTOR register. Note: the CM0+ vector table is at an address that is a 256 B multiple."]
    #[inline(always)]
    #[must_use]
    pub fn addr24(&mut self) -> Addr24W<Cm0VectorTableBaseSpec> {
        Addr24W::new(self, 8)
    }
}
#[doc = "CM0+ vector table base\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_vector_table_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_vector_table_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0VectorTableBaseSpec;
impl crate::RegisterSpec for Cm0VectorTableBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_vector_table_base::R`](R) reader structure"]
impl crate::Readable for Cm0VectorTableBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`cm0_vector_table_base::W`](W) writer structure"]
impl crate::Writable for Cm0VectorTableBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM0_VECTOR_TABLE_BASE to value 0"]
impl crate::Resettable for Cm0VectorTableBaseSpec {
    const RESET_VALUE: u32 = 0;
}
