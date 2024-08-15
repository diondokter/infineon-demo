#[doc = "Register `CMP0_SW_CLEAR` reader"]
pub type R = crate::R<Cmp0SwClearSpec>;
#[doc = "Register `CMP0_SW_CLEAR` writer"]
pub type W = crate::W<Cmp0SwClearSpec>;
#[doc = "Field `CMP0_IP0` reader - see corresponding bit in CMP0_SW"]
pub type Cmp0Ip0R = crate::BitReader;
#[doc = "Field `CMP0_IP0` writer - see corresponding bit in CMP0_SW"]
pub type Cmp0Ip0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0_AP0` reader - see corresponding bit in CMP0_SW"]
pub type Cmp0Ap0R = crate::BitReader;
#[doc = "Field `CMP0_AP0` writer - see corresponding bit in CMP0_SW"]
pub type Cmp0Ap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0_BP0` reader - see corresponding bit in CMP0_SW"]
pub type Cmp0Bp0R = crate::BitReader;
#[doc = "Field `CMP0_BP0` writer - see corresponding bit in CMP0_SW"]
pub type Cmp0Bp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0_IN0` reader - see corresponding bit in CMP0_SW"]
pub type Cmp0In0R = crate::BitReader;
#[doc = "Field `CMP0_IN0` writer - see corresponding bit in CMP0_SW"]
pub type Cmp0In0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0_AN0` reader - see corresponding bit in CMP0_SW"]
pub type Cmp0An0R = crate::BitReader;
#[doc = "Field `CMP0_AN0` writer - see corresponding bit in CMP0_SW"]
pub type Cmp0An0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0_BN0` reader - see corresponding bit in CMP0_SW"]
pub type Cmp0Bn0R = crate::BitReader;
#[doc = "Field `CMP0_BN0` writer - see corresponding bit in CMP0_SW"]
pub type Cmp0Bn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0_VN0` reader - see corresponding bit in CMP0_SW"]
pub type Cmp0Vn0R = crate::BitReader;
#[doc = "Field `CMP0_VN0` writer - see corresponding bit in CMP0_SW"]
pub type Cmp0Vn0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_ip0(&self) -> Cmp0Ip0R {
        Cmp0Ip0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_ap0(&self) -> Cmp0Ap0R {
        Cmp0Ap0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_bp0(&self) -> Cmp0Bp0R {
        Cmp0Bp0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_in0(&self) -> Cmp0In0R {
        Cmp0In0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_an0(&self) -> Cmp0An0R {
        Cmp0An0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_bn0(&self) -> Cmp0Bn0R {
        Cmp0Bn0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    pub fn cmp0_vn0(&self) -> Cmp0Vn0R {
        Cmp0Vn0R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_ip0(&mut self) -> Cmp0Ip0W<Cmp0SwClearSpec> {
        Cmp0Ip0W::new(self, 0)
    }
    #[doc = "Bit 1 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_ap0(&mut self) -> Cmp0Ap0W<Cmp0SwClearSpec> {
        Cmp0Ap0W::new(self, 1)
    }
    #[doc = "Bit 2 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_bp0(&mut self) -> Cmp0Bp0W<Cmp0SwClearSpec> {
        Cmp0Bp0W::new(self, 2)
    }
    #[doc = "Bit 4 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_in0(&mut self) -> Cmp0In0W<Cmp0SwClearSpec> {
        Cmp0In0W::new(self, 4)
    }
    #[doc = "Bit 5 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_an0(&mut self) -> Cmp0An0W<Cmp0SwClearSpec> {
        Cmp0An0W::new(self, 5)
    }
    #[doc = "Bit 6 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_bn0(&mut self) -> Cmp0Bn0W<Cmp0SwClearSpec> {
        Cmp0Bn0W::new(self, 6)
    }
    #[doc = "Bit 7 - see corresponding bit in CMP0_SW"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0_vn0(&mut self) -> Cmp0Vn0W<Cmp0SwClearSpec> {
        Cmp0Vn0W::new(self, 7)
    }
}
#[doc = "Comparator 0 switch control clear\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp0_sw_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp0_sw_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp0SwClearSpec;
impl crate::RegisterSpec for Cmp0SwClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp0_sw_clear::R`](R) reader structure"]
impl crate::Readable for Cmp0SwClearSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp0_sw_clear::W`](W) writer structure"]
impl crate::Writable for Cmp0SwClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP0_SW_CLEAR to value 0"]
impl crate::Resettable for Cmp0SwClearSpec {
    const RESET_VALUE: u32 = 0;
}
