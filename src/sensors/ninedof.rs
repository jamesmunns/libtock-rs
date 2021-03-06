use core::cell::Cell;
use core::fmt;
use core::mem;
use syscalls::{self, yieldk_for};

const DRIVER_NUM: u32 = 11;

pub struct Ninedof;

#[derive(Copy, Clone, Default, Debug)]
pub struct NinedofReading {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl fmt::Display for NinedofReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Default)]
struct CbData {
    res: Cell<NinedofReading>,
    ready: Cell<bool>
}

impl Ninedof {
    pub unsafe fn new() -> Ninedof {
        Ninedof
    }

    pub fn read_acceleration(&mut self) -> NinedofReading {
        let res: CbData = Default::default();
        unsafe {
            subscribe(Self::cb, mem::transmute(&res));
            start_accel_reading();
            yieldk_for(|| res.ready.get() );
        }
        res.res.get() 
    }

    pub fn read_magnetometer(&mut self) -> NinedofReading {
        let res: CbData = Default::default();
        unsafe {
            subscribe(Self::cb, mem::transmute(&res));
            start_magnetometer_reading();
            yieldk_for(|| res.ready.get() );
        }
        res.res.get() 
    }

    extern fn cb(x: usize, y: usize, z: usize, ptr: usize) {
        let res: &CbData = unsafe {
            mem::transmute(ptr)
        };
        res.res.set(NinedofReading {
            x: x as i32, y: y as i32, z: z as i32
        });
        res.ready.set(true);
    }
}

pub unsafe fn subscribe(cb: extern fn(usize, usize, usize, usize), ud: usize) {
    syscalls::subscribe(DRIVER_NUM, 0, cb, ud);
}

pub unsafe fn start_accel_reading() {
    syscalls::command(DRIVER_NUM, 1, 0);
}

pub unsafe fn start_magnetometer_reading() {
    syscalls::command(DRIVER_NUM, 1, 0);
}

