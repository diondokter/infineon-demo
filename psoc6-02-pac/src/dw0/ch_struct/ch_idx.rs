#[doc = "Register `CH_IDX` reader"]
pub type R = crate::R<ChIdxSpec>;
#[doc = "Register `CH_IDX` writer"]
pub type W = crate::W<ChIdxSpec>;
#[doc = "Field `X_IDX` reader - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
pub type XIdxR = crate::FieldReader;
#[doc = "Field `X_IDX` writer - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
pub type XIdxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Y_IDX` reader - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
pub type YIdxR = crate::FieldReader;
#[doc = "Field `Y_IDX` writer - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
pub type YIdxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn x_idx(&self) -> XIdxR {
        XIdxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    pub fn y_idx(&self) -> YIdxR {
        YIdxR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies the X loop index. In the range of \\[0, X_COUNT\\], with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    #[must_use]
    pub fn x_idx(&mut self) -> XIdxW<ChIdxSpec> {
        XIdxW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Specifies the Y loop index, with X_COUNT taken from the current descriptor. Note: HW sets this field to '0' when it updates the current descriptor pointer CH_CURR_PTR with DESCR_NEXT_PTR after execution of the current descriptor. Note: SW should set this field to '0' when it updates CH_CURR_PTR."]
    #[inline(always)]
    #[must_use]
    pub fn y_idx(&mut self) -> YIdxW<ChIdxSpec> {
        YIdxW::new(self, 8)
    }
}
#[doc = "Channel current indices\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_idx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_idx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChIdxSpec;
impl crate::RegisterSpec for ChIdxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_idx::R`](R) reader structure"]
impl crate::Readable for ChIdxSpec {}
#[doc = "`write(|w| ..)` method takes [`ch_idx::W`](W) writer structure"]
impl crate::Writable for ChIdxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_IDX to value 0"]
impl crate::Resettable for ChIdxSpec {
    const RESET_VALUE: u32 = 0;
}
