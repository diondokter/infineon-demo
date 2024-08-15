#[doc = "Register `CMP1_SW_CLEAR` reader"]
pub type R = crate::R<Cmp1SwClearSpec>;
#[doc = "Register `CMP1_SW_CLEAR` writer"]
pub type W = crate::W<Cmp1SwClearSpec>;
#[doc = "Field `CMP1_IP1` reader - see corresponding bit in CMP1_SW"]
pub type Cmp1Ip1R = crate::BitReader;
#[doc = "Field `CMP1_IP1` writer - see corresponding bit in CMP1_SW"]
pub type Cmp1Ip1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_AP1` reader - see corresponding bit in CMP1_SW"]
pub type Cmp1Ap1R = crate::BitReader;
#[doc = "Field `CMP1_AP1` writer - see corresponding bit in CMP1_SW"]
pub type Cmp1Ap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_BP1` reader - see corresponding bit in CMP1_SW"]
pub type Cmp1Bp1R = crate::BitReader;
#[doc = "Field `CMP1_BP1` writer - see corresponding bit in CMP1_SW"]
pub type Cmp1Bp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_IN1` reader - see corresponding bit in CMP1_SW"]
pub type Cmp1In1R = crate::BitReader;
#[doc = "Field `CMP1_IN1` writer - see corresponding bit in CMP1_SW"]
pub type Cmp1In1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_AN1` reader - see corresponding bit in CMP1_SW"]
pub type Cmp1An1R = crate::BitReader;
#[doc = "Field `CMP1_AN1` writer - see corresponding bit in CMP1_SW"]
pub type Cmp1An1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_BN1` reader - see corresponding bit in CMP1_SW"]
pub type Cmp1Bn1R = crate::BitReader;
#[doc = "Field `CMP1_BN1` writer - see corresponding bit in CMP1_SW"]
pub type Cmp1Bn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_VN1` reader - see corresponding bit in CMP1_SW"]
pub type Cmp1Vn1R = crate::BitReader;
#[doc = "Field `CMP1_VN1` writer - see corresponding bit in CMP1_SW"]
pub type Cmp1Vn1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_ip1(&self) -> Cmp1Ip1R {
        Cmp1Ip1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_ap1(&self) -> Cmp1Ap1R {
        Cmp1Ap1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_bp1(&self) -> Cmp1Bp1R {
        Cmp1Bp1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_in1(&self) -> Cmp1In1R {
        Cmp1In1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_an1(&self) -> Cmp1An1R {
        Cmp1An1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_bn1(&self) -> Cmp1Bn1R {
        Cmp1Bn1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    pub fn cmp1_vn1(&self) -> Cmp1Vn1R {
        Cmp1Vn1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_ip1(&mut self) -> Cmp1Ip1W<Cmp1SwClearSpec> {
        Cmp1Ip1W::new(self, 0)
    }
    #[doc = "Bit 1 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_ap1(&mut self) -> Cmp1Ap1W<Cmp1SwClearSpec> {
        Cmp1Ap1W::new(self, 1)
    }
    #[doc = "Bit 2 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_bp1(&mut self) -> Cmp1Bp1W<Cmp1SwClearSpec> {
        Cmp1Bp1W::new(self, 2)
    }
    #[doc = "Bit 4 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_in1(&mut self) -> Cmp1In1W<Cmp1SwClearSpec> {
        Cmp1In1W::new(self, 4)
    }
    #[doc = "Bit 5 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_an1(&mut self) -> Cmp1An1W<Cmp1SwClearSpec> {
        Cmp1An1W::new(self, 5)
    }
    #[doc = "Bit 6 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_bn1(&mut self) -> Cmp1Bn1W<Cmp1SwClearSpec> {
        Cmp1Bn1W::new(self, 6)
    }
    #[doc = "Bit 7 - see corresponding bit in CMP1_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_vn1(&mut self) -> Cmp1Vn1W<Cmp1SwClearSpec> {
        Cmp1Vn1W::new(self, 7)
    }
}
#[doc = "Comparator 1 switch control clear\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1_sw_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1_sw_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp1SwClearSpec;
impl crate::RegisterSpec for Cmp1SwClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1_sw_clear::R`](R) reader structure"]
impl crate::Readable for Cmp1SwClearSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp1_sw_clear::W`](W) writer structure"]
impl crate::Writable for Cmp1SwClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP1_SW_CLEAR to value 0"]
impl crate::Resettable for Cmp1SwClearSpec {
    const RESET_VALUE: u32 = 0;
}
