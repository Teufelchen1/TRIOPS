use crate::events::{Event, IrqCause};
use crate::periph::MmapPeripheral;
use core::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct TimeReg {
    last_read: u64,
    cycles_passed: u64,
    cycles_per_ms: u64,
    deadline: Option<u64>,
}

impl TimeReg {
    fn new() -> Self {
        Self {
            last_read: 0,
            cycles_passed: 0,
            cycles_per_ms: 33,
            deadline: None,
        }
    }

    fn read(&mut self) -> u64 {
        // todo: instead of +1, calculate actually passed time
        let now = self.last_read + 1;
        let time_passed = now - self.last_read;
        self.cycles_passed += time_passed * self.cycles_per_ms;
        self.last_read = now;
        self.cycles_passed
    }

    fn set(&mut self, cycles_passed: u64) {
        self.cycles_passed = cycles_passed;
    }

    fn set_deadline(&mut self, deadline_cycles: u64) {
        self.deadline = Some(deadline_cycles);
    }

    fn complete_deadline(&mut self) {
        if let Some(deadline) = self.deadline {
            if self.cycles_passed < deadline {
                self.cycles_passed = deadline;
            }
        }
        self.deadline = None;
    }

    fn time_to_wait_until_deadline(&mut self) -> Option<Duration> {
        if let Some(deadline) = self.deadline {
            let cycles_passed = self.read();
            let cycles_remain = deadline.saturating_sub(cycles_passed);
            let ms = cycles_remain / self.cycles_per_ms;
            return Some(Duration::from_millis(ms));
        }
        None
    }

    fn is_deadline_over(&mut self) -> bool {
        if let Some(deadline) = self.deadline {
            let cycles_passed = self.read();
            let cycles_remain = deadline.saturating_sub(cycles_passed);
            cycles_remain == 0
        } else {
            false
        }
    }
}

pub struct Timer {
    data_available: Arc<(Mutex<Option<Duration>>, Condvar)>,
    irq_pending: Arc<Mutex<Option<IrqCause>>>,
    time: Arc<Mutex<TimeReg>>,
}

impl Timer {
    fn start_waiter_thread(
        notify_wait_duration: Arc<(Mutex<Option<Duration>>, Condvar)>,
        time: Arc<Mutex<TimeReg>>,
        irq_pending: Arc<Mutex<Option<IrqCause>>>,
        interrupts: mpsc::Sender<Event>,
    ) {
        thread::spawn(move || loop {
            let (lock, cvar) = &*notify_wait_duration;
            let duration_to_wait = *lock.lock().unwrap();
            if let Some(duration) = duration_to_wait {
                if let Ok((mut duration_to_wait, timeout)) =
                    cvar.wait_timeout(lock.lock().unwrap(), duration)
                {
                    if timeout.timed_out() {
                        *duration_to_wait = None;
                        let mut time = time.lock().unwrap();
                        time.complete_deadline();
                        *irq_pending.lock().unwrap() = Some(IrqCause::Timer);
                        interrupts.send(Event::Interrupt(IrqCause::Timer)).unwrap();
                    }
                }
            } else {
                drop(cvar.wait(lock.lock().unwrap()));
            }
        });
    }

    pub fn default(interrupts: mpsc::Sender<Event>) -> Self {
        let notify_wait_duration: Arc<(Mutex<Option<Duration>>, Condvar)> =
            Arc::new((Mutex::new(None), Condvar::new()));
        let irq_pending = Arc::new(Mutex::new(None));
        let time = Arc::new(Mutex::new(TimeReg::new()));

        Self::start_waiter_thread(
            notify_wait_duration.clone(),
            time.clone(),
            irq_pending.clone(),
            interrupts,
        );

        Self {
            data_available: notify_wait_duration,
            irq_pending,
            time,
        }
    }

    pub fn set_timecmp(&mut self, deadline: u64) {
        let mut time = self.time.lock().unwrap();

        time.set_deadline(deadline);
        let duration = time.time_to_wait_until_deadline();

        let (data_available_lock, cvar) = &*self.data_available;
        let mut duration_to_wait = data_available_lock.lock().unwrap();
        *self.irq_pending.lock().unwrap() = None;
        *duration_to_wait = duration;

        cvar.notify_one();
    }

    fn set_time(&mut self, cycles_passed: u64) {
        let mut time = self.time.lock().unwrap();

        time.set(cycles_passed);

        // We updated the absolute time, does our deadline still hold?
        if time.is_deadline_over() {
            todo!();
            // emit irq
        } else {
            // Clear pending irq if any
            *self.irq_pending.lock().unwrap() = None;

            // Since the absolute time changed, the time we have to wait also changed
            let duration = time.time_to_wait_until_deadline();

            // Update the wait time
            let (data_available_lock, cvar) = &*self.data_available;
            let mut duration_to_wait = data_available_lock.lock().unwrap();
            *duration_to_wait = duration;

            // Notify waiting thread
            cvar.notify_one();
        }
    }
}

impl MmapPeripheral for Timer {
    fn read_byte(&self, byte_offset: usize) -> u8 {
        let bit_offset = byte_offset * 8;
        let mut time = self.time.lock().unwrap();
        ((time.read() >> bit_offset) & 0xff) as u8
    }

    fn write_byte(&mut self, byte_offset: usize, value: u8) {
        let bit_offset = byte_offset * 8;

        let cycles_passed = {
            let mut time = self.time.lock().unwrap();
            let mut cycles_passed = time.read();

            // Clear the byte in question
            cycles_passed &= !(0xff << bit_offset);
            // Set the byte in question
            cycles_passed |= u64::from(value) << bit_offset;

            cycles_passed
        };

        self.set_time(cycles_passed);
    }

    fn write_word(&mut self, byte_offset: usize, value: u32) {
        assert!(byte_offset == 0);

        let cycles_passed = {
            let mut time = self.time.lock().unwrap();
            let mut cycles_passed = time.read();

            // Clear the lower half
            cycles_passed &= !(0xffff_ffff);
            // Set the lower half
            cycles_passed |= u64::from(value);

            cycles_passed
        };

        self.set_time(cycles_passed);
    }

    fn pending_interrupt(&self) -> Option<IrqCause> {
        self.irq_pending.lock().unwrap().clone()
    }
}
