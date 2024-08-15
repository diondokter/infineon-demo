#[doc = "Register `SW_RES` reader"]
pub type R = crate::R<SwResSpec>;
#[doc = "Register `SW_RES` writer"]
pub type W = crate::W<SwResSpec>;
#[doc = "Select resistance or low EMI (slow ramp) for the HCAV switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ResHcav {
    #[doc = "0: Low"]
    Low = 0,
    #[doc = "1: Medium"]
    Med = 1,
    #[doc = "2: High"]
    High = 2,
    #[doc = "3: Low EMI (slow ramp: 3 switches closed by fixed delay line)"]
    Lowemi = 3,
}
impl From<ResHcav> for u8 {
    #[inline(always)]
    fn from(variant: ResHcav) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ResHcav {
    type Ux = u8;
}
impl crate::IsEnum for ResHcav {}
#[doc = "Field `RES_HCAV` reader - Select resistance or low EMI (slow ramp) for the HCAV switch"]
pub type ResHcavR = crate::FieldReader<ResHcav>;
impl ResHcavR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResHcav {
        match self.bits {
            0 => ResHcav::Low,
            1 => ResHcav::Med,
            2 => ResHcav::High,
            3 => ResHcav::Lowemi,
            _ => unreachable!(),
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ResHcav::Low
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == ResHcav::Med
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ResHcav::High
    }
    #[doc = "Low EMI (slow ramp: 3 switches closed by fixed delay line)"]
    #[inline(always)]
    pub fn is_lowemi(&self) -> bool {
        *self == ResHcav::Lowemi
    }
}
#[doc = "Field `RES_HCAV` writer - Select resistance or low EMI (slow ramp) for the HCAV switch"]
pub type ResHcavW<'a, REG> = crate::FieldWriter<'a, REG, 2, ResHcav, crate::Safe>;
impl<'a, REG> ResHcavW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(ResHcav::Low)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(ResHcav::Med)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(ResHcav::High)
    }
    #[doc = "Low EMI (slow ramp: 3 switches closed by fixed delay line)"]
    #[inline(always)]
    pub fn lowemi(self) -> &'a mut crate::W<REG> {
        self.variant(ResHcav::Lowemi)
    }
}
#[doc = "Field `RES_HCAG` reader - Select resistance or low EMI for the corresponding switch"]
pub type ResHcagR = crate::FieldReader;
#[doc = "Field `RES_HCAG` writer - Select resistance or low EMI for the corresponding switch"]
pub type ResHcagW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES_HCBV` reader - Select resistance or low EMI for the corresponding switch"]
pub type ResHcbvR = crate::FieldReader;
#[doc = "Field `RES_HCBV` writer - Select resistance or low EMI for the corresponding switch"]
pub type ResHcbvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES_HCBG` reader - Select resistance or low EMI for the corresponding switch"]
pub type ResHcbgR = crate::FieldReader;
#[doc = "Field `RES_HCBG` writer - Select resistance or low EMI for the corresponding switch"]
pub type ResHcbgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Select resistance for the corresponding switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ResF1pm {
    #[doc = "0: Low"]
    Low = 0,
    #[doc = "1: Medium"]
    Med = 1,
    #[doc = "2: High"]
    High = 2,
    #[doc = "3: N/A"]
    Rsvd = 3,
}
impl From<ResF1pm> for u8 {
    #[inline(always)]
    fn from(variant: ResF1pm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ResF1pm {
    type Ux = u8;
}
impl crate::IsEnum for ResF1pm {}
#[doc = "Field `RES_F1PM` reader - Select resistance for the corresponding switch"]
pub type ResF1pmR = crate::FieldReader<ResF1pm>;
impl ResF1pmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResF1pm {
        match self.bits {
            0 => ResF1pm::Low,
            1 => ResF1pm::Med,
            2 => ResF1pm::High,
            3 => ResF1pm::Rsvd,
            _ => unreachable!(),
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ResF1pm::Low
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == ResF1pm::Med
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ResF1pm::High
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == ResF1pm::Rsvd
    }
}
#[doc = "Field `RES_F1PM` writer - Select resistance for the corresponding switch"]
pub type ResF1pmW<'a, REG> = crate::FieldWriter<'a, REG, 2, ResF1pm, crate::Safe>;
impl<'a, REG> ResF1pmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(ResF1pm::Low)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(ResF1pm::Med)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(ResF1pm::High)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(ResF1pm::Rsvd)
    }
}
#[doc = "Field `RES_F2PT` reader - Select resistance for the corresponding switch"]
pub type ResF2ptR = crate::FieldReader;
#[doc = "Field `RES_F2PT` writer - Select resistance for the corresponding switch"]
pub type ResF2ptW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Select resistance or low EMI (slow ramp) for the HCAV switch"]
    #[inline(always)]
    pub fn res_hcav(&self) -> ResHcavR {
        ResHcavR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcag(&self) -> ResHcagR {
        ResHcagR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcbv(&self) -> ResHcbvR {
        ResHcbvR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcbg(&self) -> ResHcbgR {
        ResHcbgR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn res_f1pm(&self) -> ResF1pmR {
        ResF1pmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn res_f2pt(&self) -> ResF2ptR {
        ResF2ptR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select resistance or low EMI (slow ramp) for the HCAV switch"]
    #[inline(always)]
    #[must_use]
    pub fn res_hcav(&mut self) -> ResHcavW<SwResSpec> {
        ResHcavW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn res_hcag(&mut self) -> ResHcagW<SwResSpec> {
        ResHcagW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn res_hcbv(&mut self) -> ResHcbvW<SwResSpec> {
        ResHcbvW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn res_hcbg(&mut self) -> ResHcbgW<SwResSpec> {
        ResHcbgW::new(self, 6)
    }
    #[doc = "Bits 16:17 - Select resistance for the corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn res_f1pm(&mut self) -> ResF1pmW<SwResSpec> {
        ResF1pmW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Select resistance for the corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn res_f2pt(&mut self) -> ResF2ptW<SwResSpec> {
        ResF2ptW::new(self, 18)
    }
}
#[doc = "Switch Resistance configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_res::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_res::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwResSpec;
impl crate::RegisterSpec for SwResSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_res::R`](R) reader structure"]
impl crate::Readable for SwResSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_res::W`](W) writer structure"]
impl crate::Writable for SwResSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_RES to value 0"]
impl crate::Resettable for SwResSpec {
    const RESET_VALUE: u32 = 0;
}
