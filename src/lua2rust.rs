// controlling lua scripts from rust
use rlua::{
    Function,
    Lua,
    Result,
};
use std::{
    fs::File,
    io::Read,
    sync::mpsc,
    thread,
};


// data returned from a lua state
// label: the type of data
// from: where the data came from
// data: the data itself
pub struct FromLua {
    pub label: String,
    pub from: String,
    pub data: String,
}

// holds the current state of the Lua to Rust bindings
// loaded: true if lua scripts have already been loaded
// core_receiver: mpsc receiver for collecting data from communication thread
// comm_sender: mpsc sender for communication thread to send data to core
// core_sender: mpsc sender for core to send commands with (WIP)
// comm_receiver: mpsc receiver for communication thread to receive commands with (WIP)
pub struct Lua2Rust {
    loaded: bool,
    core_receiver: mpsc::Receiver<FromLua>,
    comm_sender: mpsc::Sender<FromLua>,
    core_sender: mpsc::Sender<String>,
    comm_receiver: mpsc::Receiver<String>,
}

impl Default for Lua2Rust {
    fn default() -> Lua2Rust {
        // mpsc channel for communication thread to send retrived data back to Conerobo core
        let (tx1, rx1) = mpsc::channel();

        // mpsc channel to allow Conerobo core to send commands to communication thread
        let (tx2, rx2) = mpsc::channel();

        Lua2Rust {
            loaded: false,
            core_receiver: rx1,
            comm_sender: tx1,
            core_sender: tx2,
            comm_receiver: rx2,
        }
    }
}

impl Lua2Rust {
    // open script and read the source into a buffer
    // returns buffer of source code
    fn read_source(&self, lua: String) -> String {
        let mut file = File::open(lua).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        buf
    }

    // returns the name of the script, path and extension removed
    fn get_name(&self, lua: String) -> String {
        let start = lua.rfind('/').unwrap();
        let end = lua.rfind('.').unwrap();

        lua.get(start..end).unwrap().to_owned()
    }

    // load scripts, communication threads, and other resources for launch
    pub fn load(&mut self, luas: Vec<String>) {
        for f in luas.iter() {
            let source = self.read_source(f.to_owned());
            let name = self.get_name(f.to_owned());

            let comm_sender_clone = self.comm_sender.clone();
            
            // move resources to their own thread for launch (communication thread)
            thread::spawn(move || {
                load_script(name, source, comm_sender_clone).unwrap();
            });
        }

        self.loaded = true;
    }

    // check for any data received from communication threads
    pub fn get_from_lua(&self) -> Vec<FromLua> {
        let mut from_lua: Vec<FromLua> = Vec::new();

        loop {
            let received = self.core_receiver.try_recv();
            if received.is_err() {
                break;
            } else {
                from_lua.push(received.unwrap())
            }
        }

        from_lua
    }

    // returns whether scripts have already been loaded
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }
}

// script and corresponding lua state will run in here
// name: name of script being ran
// source: sourcecode of script
// sender: mpsc sender for sending values to core
fn load_script(
    name: String,
    source: String,
    sender: mpsc::Sender<FromLua>,
) -> Result<()> {
    // load lua standard library
    let lua = Lua::new();

    // current state of communication thread
    let mut state = "load";
    
    lua.context(|lua_ctx| {
        // create reference to globals table
        let globals = lua_ctx.globals();

        loop {
            // current state of communication thread
            match state {
                "load" => {
                    // load code into environment
                    lua_ctx
                        .load(&*source)
                        .set_name(name.as_str())?
                        .exec()?;
                    
                    // transition to the setup state
                    state = "setup";
                }
                "setup" => {
                    // get the Setup lua function to be called from communication thread
                    let setup: Function = globals.get("Setup")?;
                    setup.call::<_, ()>(())?;
                    
                    // check for any new messages from lua
                    let msg = globals.get::<_, String>("Message")?;
                    if msg != "".to_owned() {
                        let to_core = FromLua {
                            label: "MSG".to_owned(),
                            from: name.to_owned(),
                            data: msg
                        };

                        sender.send(to_core).unwrap();
                    }

                    state = "waiting";
                },
                "waiting" => {}
                _ => {
                    break;
                }
            }
        }

        Ok(())
    })?;

    Ok(())
}
