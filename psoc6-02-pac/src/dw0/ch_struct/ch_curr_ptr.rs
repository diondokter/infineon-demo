#[doc = "Register `CH_CURR_PTR` reader"]
pub type R = crate::R<ChCurrPtrSpec>;
#[doc = "Register `CH_CURR_PTR` writer"]
pub type W = crate::W<ChCurrPtrSpec>;
#[doc = "Field `ADDR` reader - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Address of current descriptor. When this field is '0', there is no valid descriptor. Note: HW updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: Typically, when SW updates the current descriptor pointer CH_CURR_PTR, it also sets CH_IDX.X_IDX and CH_IDX.Y_IDX to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<ChCurrPtrSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "Channel current descriptor pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_curr_ptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_curr_ptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChCurrPtrSpec;
impl crate::RegisterSpec for ChCurrPtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_curr_ptr::R`](R) reader structure"]
impl crate::Readable for ChCurrPtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch_curr_ptr::W`](W) writer structure"]
impl crate::Writable for ChCurrPtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_CURR_PTR to value 0"]
impl crate::Resettable for ChCurrPtrSpec {
    const RESET_VALUE: u32 = 0;
}
