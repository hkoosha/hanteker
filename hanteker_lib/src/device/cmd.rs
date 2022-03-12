pub type RawCommand = [u8; 10];

#[derive(Clone)]
pub enum Val {
    ValU8([u8; 4]),
    ValU16([u16; 2]),
    ValU32(u32),
}

#[derive(Clone)]
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

    // =================================================================== DEBUG

    pub fn dump(&self) -> String {
        let idx = match self.idx {
            Some(v) => v.to_string(),
            None => "?".to_string(),
        };
        let boh = match self.boh {
            Some(v) => v.to_string(),
            None => "?".to_string(),
        };
        let func = match self.func {
            Some(v) => v.to_string(),
            None => "?".to_string(),
        };
        let cmd = match self.cmd {
            Some(v) => v.to_string(),
            None => "?".to_string(),
        };
        let val = match self.val {
            Some(Val::ValU8(v)) => format!("{}-{}-{}-{}", v[0], v[1], v[2], v[3]),
            Some(Val::ValU16(v)) => format!("{}-{}", v[0], v[1]),
            Some(Val::ValU32(v)) => format!("{}", v),
            None => "?".to_string(),
        };
        let last = match self.last {
            Some(v) => v.to_string(),
            None => "?".to_string(),
        };

        format!(
            "idx={}\nboh={}\nfunc={}\ncmd={}\nval={}\nlast={}",
            idx, boh, func, cmd, val, last
        )
    }

    pub fn dump_raw(&self) -> String {
        let cloned = self.clone();
        let raw: RawCommand = cloned.into();
        format!(
            "idx={}\nboh={}\nfunc={}-{}\ncmd={}\nval={}-{}-{}-{}\nlast={}",
            raw[0], // idx
            raw[1], // boh
            raw[2], // func0
            raw[3], // func1
            raw[4], // cmd
            raw[5], // val0
            raw[6], // val1
            raw[7], // val2
            raw[8], // val3
            raw[9]  // last
        )
    }

    pub fn print_dump(self) -> Self {
        println!("{}", self.dump());
        self
    }

    pub fn print_dump_raw(self) -> Self {
        println!("{}", self.dump_raw());
        self
    }
}

impl Default for HantekCommandBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::from_over_into)]
impl Into<RawCommand> for HantekCommandBuilder {
    fn into(self) -> RawCommand {
        let mut as_array = [0u8; 10];
        as_array[0] = self.idx.expect("idx not set");
        as_array[1] = self.boh.expect("boh not set");
        as_array[2] = self.func.expect("func not set").to_be_bytes()[1];
        as_array[3] = self.func.expect("func not set").to_be_bytes()[0];
        as_array[4] = self.cmd.expect("cmd not set");
        match self.val.expect("val not set") {
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
        as_array[9] = self.last.expect("last not set");

        as_array
    }
}
