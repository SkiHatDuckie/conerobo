use rlua::{
    Lua, Result,
};
use std::fs::File;
use std::io::Read;
use std::thread;

// initialize lua states
pub fn launch(luas: &Vec<String>) {
    for lua in luas.iter() {
        // giving the thread a clone of the lua directory so that borrow checker stops
        // complaining about non-matching explicit lifetimes (threads are static by default)
        let lua_clone = lua.clone();
        thread::spawn(move || {
            load_script(lua_clone).unwrap();
        });
    }
}

// run lua states in seperate threads
fn load_script(lua: String) -> Result<()> {
    // create new state for script
    let lua_state = Lua::new();

    lua_state.context(|lua_ctx| {
        // open script file and read the source into a buffer
        let mut file = File::open(lua).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        // load and execute the code
        lua_ctx
            .load(&*buf)
            .set_name("test")?
            .exec()?;
        
        Ok(())
    })?;

    Ok(())
}
