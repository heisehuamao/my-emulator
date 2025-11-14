use std::cell::Cell;
use std::time::{Duration, Instant};

pub(crate) struct TscClock;

thread_local! {
    static ACCUMULATED_TSC: Cell<u64> = Cell::new(0);
    static LAST_TSC: Cell<u64> = Cell::new(TscClock::inner_rdtsc());
    static HZ: Cell<u64> = Cell::new(TscClock::measure_hz());
}

impl TscClock {
    pub(crate) fn rdtsc_usec() -> u64 {
        let last_tsc = LAST_TSC.get();
        let hz = HZ.get();
        let curr_tsc = Self::inner_rdtsc();
        let mut diff = 0;
        if curr_tsc > last_tsc {
            diff = curr_tsc - last_tsc;
            LAST_TSC.replace(curr_tsc);
        } else {
            // to evade tsc inverse
            LAST_TSC.replace(curr_tsc);
        }
        let old_accum_tsc = ACCUMULATED_TSC.get();
        let new_accum_tsc = old_accum_tsc + diff;
        ACCUMULATED_TSC.replace(new_accum_tsc);
        new_accum_tsc * 1_000_000 / hz
    }

    /// Read the architecture-specific cycle counter.
    #[inline]
    fn inner_rdtsc() -> u64 {
        #[cfg(target_arch = "x86")]
        unsafe {
            // x86: RDTSC intrinsic
            core::arch::x86::_rdtsc() as u64
        }

        #[cfg(target_arch = "x86_64")]
        unsafe {
            // x86_64: RDTSC intrinsic
            core::arch::x86_64::_rdtsc() as u64
        }

        #[cfg(target_arch = "aarch64")]
        unsafe {
            // ARM64: read CNTVCT_EL0 (virtual count register)
            let mut vct: u64;
            core::arch::asm!("mrs {v}, cntvct_el0", v = out(reg) vct);
            vct
        }

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
        {
            // Convert Instant to a monotonic nanosecond count
            Instant::now()
                .duration_since(Instant::now())
                .as_nanos() as u64
        }
    }

    /// Estimate ticks per second by measuring over ~1s.
    fn measure_hz() -> u64 {
        let start = Instant::now();
        let t0 = Self::inner_rdtsc();

        // Sleep ~950ms, then busy-wait until ~1s
        std::thread::sleep(Duration::from_millis(950));
        while start.elapsed() < Duration::from_secs(1) {
            std::hint::spin_loop();
        }

        let t1 = Self::inner_rdtsc();
        let elapsed = start.elapsed().as_secs_f64();

        let hz = (t1 - t0) as f64 / elapsed;
        println!("measure_hz(): {}", hz);
        hz as u64
    }
}
