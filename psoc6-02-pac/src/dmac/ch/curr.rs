#[doc = "Register `CURR` reader"]
pub type R = crate::R<CurrSpec>;
#[doc = "Register `CURR` writer"]
pub type W = crate::W<CurrSpec>;
#[doc = "Field `PTR` reader - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
pub type PtrR = crate::FieldReader<u32>;
#[doc = "Field `PTR` writer - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
pub type PtrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
    #[inline(always)]
    pub fn ptr(&self) -> PtrR {
        PtrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn ptr(&mut self) -> PtrW<CurrSpec> {
        PtrW::new(self, 2)
    }
}
#[doc = "Channel current descriptor pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`curr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`curr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrSpec;
impl crate::RegisterSpec for CurrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr::R`](R) reader structure"]
impl crate::Readable for CurrSpec {}
#[doc = "`write(|w| ..)` method takes [`curr::W`](W) writer structure"]
impl crate::Writable for CurrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CURR to value 0"]
impl crate::Resettable for CurrSpec {
    const RESET_VALUE: u32 = 0;
}
