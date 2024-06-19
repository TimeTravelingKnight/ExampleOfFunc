use helpermacros::memcopy;
use std::{ffi::{c_int, c_void}, slice};

mod helpermacros;
#[repr(C)]
struct GBAState {
    r0:u32,
    r1:u32,
    r2:u32,
    r3:u32,
    r4:u32,
    r5:u32,
    r6:u32,
    r7:u32,
    r8:u32,
    r9:u32,
    r10:u32,
    r11:u32,
    r12:u32,
    r13:u32,
    r14:u32,
    r15:u32,
    cpuFlags:u32,
    flagsImplicitUpdate:u32,
    pub memory:*mut u8
}

#[link(name = "chaudloader.dll")]
extern "C" {
    fn getRomFreeSpaceOffSetValueVer(gameVer:u8)->u32 ;
    
    fn updateRomFreeSpaceOffSetValueVer(gameVer:u8,newOffset:u32);
}

#[no_mangle]
pub extern "C" fn on_game_load(game:u32, gba_state:*mut GBAState) {
  
    if (game==9) {
        let x=unsafe{*((gba_state as *mut u8 ).wrapping_add(0x48) as *mut u64)} as usize;
        let y=(unsafe{getRomFreeSpaceOffSetValueVer(game as u8)}) as usize;
        let z=&[1;50];
        let size =50;
        memcopy!(x,y,size,z);
        unsafe{updateRomFreeSpaceOffSetValueVer(game as u8, (y+0x32) as u32)};
    }
}

#[no_mangle]
pub extern "C" fn luaopen_make2(_:c_void)-> c_int{
    return 0
}