// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use core::result::Result::Ok;

use kernel::prelude::*;
use kernel::{chrdev, file, task::Task};
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};
use kernel::sync::{CondVar, Mutex};

module! {
    type: RustCompletiondev,
    name: "rust_completion",
    author: "Tester",
    description: "Example of Kernel's completion mechanism",
    license: "GPL",
}

kernel::init_static_sync! {
    static CM_MUTEX: Mutex::<bool> = true;
    static CM_CONDVAR: CondVar;
}

struct Cdev;

#[vtable]
impl file::Operations for Cdev {
    type Data = Box<Self>;

    type OpenData = ();

    fn open(_context: &Self::OpenData, _file: &file::File) -> Result<Self::Data> {
        pr_info!("open is invoked\n");
        Ok(Box::try_new(Cdev)?)
    }
    fn read(
        _data: &Self,
        _file: &file::File,
        _writer: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("read is invoked\n");
        pr_info!("process {} is going to sleep\n", Task::current().pid());
        let mut lock = CM_MUTEX.lock();
        while *lock  {
            let _ = CM_CONDVAR.wait(&mut lock);
        }
        pr_info!("awoken {}\n", Task::current().pid());
        *lock = true;
        Ok(0)
    }

    fn write(
        _data: &Self,
        _file: &file::File,
        _reader: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("write is invoked\n");
        let mut lock = CM_MUTEX.lock();
        *lock = false;
        CM_CONDVAR.notify_all();
        pr_info!("prosess {} awakening the readers...\n", Task::current().pid());
        Ok(_reader.len())
    }
}

struct RustCompletiondev {
    _dev: Pin<Box<chrdev::Registration<1>>>,
}

impl kernel::Module for RustCompletiondev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust completion sample (init)\n");

        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;

        chrdev_reg.as_mut().register::<Cdev>()?;

        Ok(RustCompletiondev { _dev: chrdev_reg })
    }
}

impl Drop for RustCompletiondev {
    fn drop(&mut self) {
        pr_info!("Rust completion sample (exit)\n");
        
    }
}