#[doc = "Register `CM4_VECTOR_TABLE_BASE` reader"]
pub type R = crate::R<Cm4VectorTableBaseSpec>;
#[doc = "Register `CM4_VECTOR_TABLE_BASE` writer"]
pub type W = crate::W<Cm4VectorTableBaseSpec>;
#[doc = "Field `ADDR22` reader - Address of CM4 vector table. This register is used for CM4 warm and cold boot purposes: the CM0+ CPU initializes the CM4_VECTOR_TABLE_BASE register and the CM4 boot code uses the register to initialize the CM4 internal VTOR register. Note: the CM4 vector table is at an address that is a 1024 B multiple."]
pub type Addr22R = crate::FieldReader<u32>;
#[doc = "Field `ADDR22` writer - Address of CM4 vector table. This register is used for CM4 warm and cold boot purposes: the CM0+ CPU initializes the CM4_VECTOR_TABLE_BASE register and the CM4 boot code uses the register to initialize the CM4 internal VTOR register. Note: the CM4 vector table is at an address that is a 1024 B multiple."]
pub type Addr22W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - Address of CM4 vector table. This register is used for CM4 warm and cold boot purposes: the CM0+ CPU initializes the CM4_VECTOR_TABLE_BASE register and the CM4 boot code uses the register to initialize the CM4 internal VTOR register. Note: the CM4 vector table is at an address that is a 1024 B multiple."]
    #[inline(always)]
    pub fn addr22(&self) -> Addr22R {
        Addr22R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 10:31 - Address of CM4 vector table. This register is used for CM4 warm and cold boot purposes: the CM0+ CPU initializes the CM4_VECTOR_TABLE_BASE register and the CM4 boot code uses the register to initialize the CM4 internal VTOR register. Note: the CM4 vector table is at an address that is a 1024 B multiple."]
    #[inline(always)]
    #[must_use]
    pub fn addr22(&mut self) -> Addr22W<Cm4VectorTableBaseSpec> {
        Addr22W::new(self, 10)
    }
}
#[doc = "CM4 vector table base\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_vector_table_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_vector_table_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4VectorTableBaseSpec;
impl crate::RegisterSpec for Cm4VectorTableBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_vector_table_base::R`](R) reader structure"]
impl crate::Readable for Cm4VectorTableBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`cm4_vector_table_base::W`](W) writer structure"]
impl crate::Writable for Cm4VectorTableBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM4_VECTOR_TABLE_BASE to value 0"]
impl crate::Resettable for Cm4VectorTableBaseSpec {
    const RESET_VALUE: u32 = 0;
}
