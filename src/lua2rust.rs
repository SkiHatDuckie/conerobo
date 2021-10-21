use rlua::{
    Lua, Result,
};
use std::fs::File;
use std::io::Read;


// initialize lua states
pub fn load_scripts(luas: &Vec<String>) -> Result<()> {
    for l in luas.iter() {
        // create new state for script
        let lua = Lua::new();

        lua.context(|lua_ctx| {
            // open script file and read the source into a buffer
            let mut file = File::open(l).unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();

            // load and execute the code
            lua_ctx
                .load(&*buf)
                .set_name("test")?
                .exec()?;
            
            Ok(())
        })?;
    }

    Ok(())
}
