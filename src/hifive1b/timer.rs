use crate::events::{Event, IrqCause};
use crate::periph::MmapPeripheral;
use core::time;
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub struct Timer {
    data_available: Arc<(Mutex<Option<u64>>, Condvar)>,
    irq_pending: Arc<Mutex<Option<IrqCause>>>,
    time_passed: Arc<Mutex<u64>>,
    reg: [u8; 8],
}

impl Timer {
    pub fn default(interrupts: mpsc::Sender<Event>) -> Self {
        let data_mux: Arc<(Mutex<Option<u64>>, Condvar)> =
            Arc::new((Mutex::new(None), Condvar::new()));
        let data_mux_clone = data_mux.clone();
        let irq_pending = Arc::new(Mutex::new(None));
        let irq_pending_clone = irq_pending.clone();
        let time_passed = Arc::new(Mutex::new(0));
        let time_passed_clone = time_passed.clone();

        thread::spawn(move || loop {
            let (lock, cvar) = &*data_mux_clone;
            let dur = {
                let data_available = lock.lock().unwrap();
                //let data = cvar.wait(data_available).unwrap();
                if data_available.is_some() {
                    let cycles = data_available.unwrap();
                    let cycles_per_ms = 33; // ~32.768

                    time::Duration::from_millis(cycles / cycles_per_ms)
                } else {
                    time::Duration::from_secs(999999)
                }
            };
            println!("Sleeping for {:?}", dur);
            match cvar.wait_timeout(lock.lock().unwrap(), dur) {
                Ok((mut guard, timeout)) => {
                    if timeout.timed_out() {
                        println!("Interrupt: Timer");
                        *guard = None;
                        *time_passed_clone.lock().unwrap() += dur.as_millis() as u64;
                        *irq_pending_clone.lock().unwrap() = Some(IrqCause::Timer);
                        *irq_pending_clone.lock().unwrap() = Some(IrqCause::Timer);
                        interrupts.send(Event::Interrupt(IrqCause::Timer)).unwrap();
                    }
                }
                Err(_) => (),
            }
        });
        Self {
            data_available: data_mux,
            irq_pending,
            time_passed,
            reg: [0; 8],
        }
    }

    pub fn set_timer(&mut self, time: u64) {
        let mut data = self.data_available.0.lock().unwrap();
        *self.irq_pending.lock().unwrap() = None;
        *data = Some(time);
        println!("Notifying that we can sleep for {time}");
        self.data_available.1.notify_one();
    }
}

impl MmapPeripheral for Timer {
    fn read(&self, offset: usize) -> u8 {
        (*self.time_passed.lock().unwrap() >> offset * 8) as u8
    }
    fn write(&mut self, offset: usize, value: u8) {
        self.reg[offset] = value;
    }
    fn pending_interrupt(&self) -> Option<IrqCause> {
        self.irq_pending.lock().unwrap().clone()
    }
}
