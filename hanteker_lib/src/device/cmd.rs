pub type RawCommand = [u8; 10];

#[derive(Clone)]
pub enum Val {
    ValU8([u8; 4]),
    ValU16([u16; 2]),
    ValU32(u32),
}

#[derive(Clone)]
pub struct HantekCommand {
    idx: u8,
    boh: u8,
    func: u16,
    cmd: u8,
    val: Val,
    last: u8,
}

impl Into<RawCommand> for HantekCommand {
    fn into(self) -> RawCommand {
        let mut as_array = [0u8; 10];
        as_array[0] = self.idx;
        as_array[1] = self.boh;
        as_array[2] = self.func.to_be_bytes()[1];
        as_array[3] = self.func.to_be_bytes()[0];
        as_array[4] = self.cmd;
        match self.val {
            Val::ValU8(v) => {
                as_array[5] = v[0];
                as_array[6] = v[1];
                as_array[7] = v[2];
                as_array[8] = v[3];
            }
            Val::ValU16(v) => {
                as_array[5] = v[0].to_le_bytes()[0];
                as_array[6] = v[0].to_le_bytes()[1];
                as_array[7] = v[1].to_le_bytes()[0];
                as_array[8] = v[1].to_le_bytes()[1];
            }
            Val::ValU32(v) => {
                as_array[5] = v.to_le_bytes()[0];
                as_array[6] = v.to_le_bytes()[1];
                as_array[7] = v.to_le_bytes()[2];
                as_array[8] = v.to_le_bytes()[3];
            }
        }
        as_array[9] = self.last;

        as_array
    }
}

impl HantekCommand {
    pub fn dump(&self) -> String {
        let val = match self.val {
            Val::ValU8(v) => format!("{}-{}-{}-{}", v[0], v[1], v[2], v[3]),
            Val::ValU16(v) => format!("{}-{}", v[0], v[1]),
            Val::ValU32(v) => format!("{}", v),
        };

        format!(
            "idx={}\nboh={}\nfunc={}\ncmd={}\nval={}\nlast={}",
            self.idx, self.boh, self.func, self.cmd, val, self.last
        )
    }

    pub fn dump_raw(&self) -> String {
        let cloned = self.clone();
        let raw: RawCommand = cloned.into();
        format!(
            "idx={}\nboh={}\nfunc={}-{}\ncmd={}\nval={}-{}-{}-{}\nlast={}",
            raw[0], // idx
            raw[1], // boh
            raw[2],
            raw[3], // func
            raw[4], // cmd
            raw[5],
            raw[6],
            raw[7],
            raw[8], // val
            raw[9]  // last
        )
    }

    pub fn print_dump(self) -> Self {
        let val = match self.val {
            Val::ValU8(v) => format!("{}-{}-{}-{}", v[0], v[1], v[2], v[3]),
            Val::ValU16(v) => format!("{}-{}", v[0], v[1]),
            Val::ValU32(v) => format!("{}", v),
        };

        println!(
            "idx={}\nboh={}\nfunc={}\ncmd={}\nval={}\nlast={}",
            self.idx, self.boh, self.func, self.cmd, val, self.last
        );
        self
    }

    pub fn print_dump_raw(self) -> Self {
        let cloned = self.clone();
        let raw: RawCommand = cloned.into();
        println!(
            "idx={}\nboh={}\nfunc={}-{}\ncmd={}\nval={}-{}-{}-{}\nlast={}",
            raw[0], // idx
            raw[1], // boh
            raw[2],
            raw[3], // func
            raw[4], // cmd
            raw[5],
            raw[6],
            raw[7],
            raw[8], // val
            raw[9]  // last
        );
        self
    }
}

pub struct HantekCommandBuilder {
    idx: Option<u8>,
    boh: Option<u8>,
    func: Option<u16>,
    cmd: Option<u8>,
    val: Option<Val>,
    last: Option<u8>,
}

impl HantekCommandBuilder {
    pub fn new() -> Self {
        Self {
            idx: None,
            boh: None,
            func: None,
            cmd: None,
            val: None,
            last: None,
        }
    }

    pub fn set_idx(mut self, idx: u8) -> Self {
        self.idx = Some(idx);
        self
    }

    pub fn set_boh(mut self, boh: u8) -> Self {
        self.boh = Some(boh);
        self
    }

    pub fn set_func(mut self, func: u16) -> Self {
        self.func = Some(func);
        self
    }

    pub fn set_cmd(mut self, cmd: u8) -> Self {
        self.cmd = Some(cmd);
        self
    }

    pub fn set_val0(self, v0: u8) -> Self {
        self.set_val_u8(v0, 0, 0, 0)
    }

    pub fn set_val_u8(mut self, v0: u8, v1: u8, v2: u8, v3: u8) -> Self {
        self.val = Some(Val::ValU8([v0, v1, v2, v3]));
        self
    }

    pub fn set_val_u32(mut self, val: u32) -> Self {
        self.val = Some(Val::ValU32(val));
        self
    }

    pub fn set_val_u16(mut self, val0: u16, val1: u16) -> Self {
        self.val = Some(Val::ValU16([val0, val1]));
        self
    }

    pub fn set_last(mut self, last: u8) -> Self {
        self.last = Some(last);
        self
    }

    pub fn build(self) -> HantekCommand {
        HantekCommand {
            idx: self.idx.expect("idx not set"),
            boh: self.boh.expect("boh not set"),
            func: self.func.expect("func not set"),
            cmd: self.cmd.expect("cmd not set"),
            val: self.val.expect("val not set"),
            last: self.last.expect("last not set"),
        }
    }
}
