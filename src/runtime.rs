use crate::{Program, Op, Mem, mem};
use std::io::{self, Read, Write};

pub struct Runtime<'a> {
    mem: Mem,

    reader: &'a mut dyn Read,
    writer: &'a mut dyn Write,

    ctrl_stack: Vec<usize>, // for control flow

    ip: usize, // program counter
}

impl<'a> Runtime<'a> {
    pub fn new(len: usize, reader: &'a mut dyn Read, writer: &'a mut dyn Write) -> Self {
        Self {
            mem: Mem::new(len),
            ip: 0,

            reader,
            writer,

            ctrl_stack: vec![],
        }
    }

    pub fn exec(&mut self, prog: Program) -> Result<()> {
        loop {
            if let Some(op) = prog.get(self.ip) {
                match op {
                    Op::Next => self.mem.inc_ptr().map_err(|e| Error::Mem(e))?,
                    Op::Prev => self.mem.dec_ptr().map_err(|e| Error::Mem(e))?,
                    Op::Inc => self.mem.inc_cur().map_err(|e| Error::Mem(e))?,
                    Op::Dec => self.mem.dec_cur().map_err(|e| Error::Mem(e))?,
                    Op::Write => {
                        self.writer.write(&[self.mem.get_cur().map_err(|e| Error::Mem(e))?]).map_err(|e| Error::Io(e))?;
                    },
                    Op::Read => if let Some(b) = self.reader.bytes().next() {
                        self.mem.set_cur(b.map_err(|e| Error::Io(e))?).map_err(|e| Error::Mem(e))?;
                    },
                    Op::Skip => {
                        if self.mem.get_cur().map_err(|e| Error::Mem(e))? != 0 {
                            self.ctrl_stack.push(self.ip);
                        } else {
                            let mut count = 0;
                            'ctrl: loop {
                                self.ip += 1;
                                if let Some(op) = prog.get(self.ip) {
                                    match op {
                                        Op::Skip => count += 1,
                                        Op::Back => {
                                            if count == 0 {
                                                break 'ctrl;
                                            }
                                            count -= 1;
                                        },
                                        _ => (),
                                    }
                                } else {
                                    break 'ctrl;
                                }
                            }
                        }
                    },
                    Op::Back => {
                        self.ip = self.ctrl_stack.pop().unwrap() - 1;
    
                    },
                };
    
                self.ip += 1;
            }
        };
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Mem(mem::Error),
    Io(io::Error),
}

impl From<mem::Error> for Error {
    fn from(value: mem::Error) -> Self {
        Self::Mem(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
