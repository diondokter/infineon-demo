#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `CNT_OVFLW` reader - This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
pub type CntOvflwR = crate::FieldReader<u32>;
#[doc = "Field `CNT_OVFLW` writer - This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
pub type CntOvflwW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
    #[inline(always)]
    pub fn cnt_ovflw(&self) -> CntOvflwR {
        CntOvflwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This interrupt cause field is activated (HW sets the field to '1') when an profiling counter overflow (from 0xFFFFFFFF to 0x00000000) is captured. There is one bit per profling counter. SW writes a '1' to a bit of this field to clear this bit to '0' (writing 0xFFFFFFFF clears all interrupt causes to '0')."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_ovflw(&mut self) -> CntOvflwW<IntrSpec> {
        CntOvflwW::new(self, 0)
    }
}
#[doc = "Profile interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
