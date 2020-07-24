#![no_std]
#![feature(rustc_attrs)]

/// See [println!] for details
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        write!($crate::Stdout, $($arg)*).unwrap()
    };
}

/// A drop-in replacement for Rust's normal `println!` macro (albiet a very slow one).
/// 
/// If practical, [print_str!] and [print_int] are often more than ten times faster.
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        writeln!($crate::Stdout, $($arg)*).unwrap()
    }
}

// TODO:
/*#[macro_export]
macro_rules! print_direct {
    ($($tail:tt)*) => {
       concat!("tellraw @a [", print_direct!(@inner $($tail)*))
    };
    (@inner str $val:literal, $($tail:tt,)*) => {
        concat!("{\"text\":\"", $val, "\"}, ", print_direct!($($tail),*))
    };
    (@inner i32 $val:expr, $($tail:tt,)*) => {
        concat!("{\"score\": {\"name\": \"$0\", \"objective\": \"$obj\" } }")
    };
    (@inner) => {
        "]"
    };
}*/

/// Prints a string literal.
#[macro_export]
macro_rules! print_str {
    ($data:expr) => {
        $crate::print_raw($data.as_ptr(), $data.len())
    }
}

/// Allows inserting commands directly into the code that optionally take an input register.
/// 
/// - $obj expands to the scoreboard objective used by Langcraft.
/// - $0 expands to the argument provided.
/// 
/// With no arguments:
/// ```
/// insert_asm!("say hello world");
/// ```
/// 
/// With an input:
/// ```
/// let foo = 42;
/// insert_asm!("execute if score $0 $obj matches 42..42 run say Foo was 42", foo);
/// ```
#[macro_export]
macro_rules! insert_asm {
    ($asm:literal) => {
        insert_asm!($asm, 0)
    };
    ($asm:literal, $input:expr) => {
        insert_asm($asm.as_ptr(), $asm.len(), $input)
    };
}

/// Place a block at the turtle's current position.
/// 
/// For example, to place a cobblestone block at the coordinates 0 1 2:
/// ```
/// turtle_x(0);
/// turtle_y(1);
/// turtle_z(2);
/// turtle_set_raw!("minecraft:cobblestone");
/// ```
#[macro_export]
macro_rules! turtle_set_raw {
    ($block:literal) => {
        insert_asm!(concat!("execute at @e[tag=turtle] run setblock ~ ~ ~ ", $block))
    }
}

/// A limited subset of blocks for convenient usage.
#[repr(i32)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum McBlock {
    Air,
    Cobblestone,
    Granite,
    Andesite,
    Diorite,
    LapisBlock,
    IronBlock,
    GoldBlock,
    DiamondBlock,
    RedstoneBlock,
    EmeraldBlock,
}

impl Into<&'static str> for McBlock {
    fn into(self) -> &'static str {
        match self {
            McBlock::Air => "minecraft:air",
            McBlock::Cobblestone => "minecraft:cobblestone",
            McBlock::Granite => "minecraft:granite",
            McBlock::Andesite => "minecraft:andesite",
            McBlock::Diorite => "minecraft:diorite",
            McBlock::LapisBlock => "minecraft:lapis_block",
            McBlock::IronBlock => "minecraft:iron_block",
            McBlock::GoldBlock => "minecraft:gold_block",
            McBlock::DiamondBlock => "minecraft:diamond_block",
            McBlock::RedstoneBlock => "minecraft:redstone_block",
            McBlock::EmeraldBlock => "minecraft:emerald_block",
        }
    }
}

#[doc(hidden)]
pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            putc(b);
        }
        Ok(())
    }
}

/// Set the X coordinate of the turtle
pub fn turtle_x(x: i32) {
    unsafe {
        insert_asm!(
            "execute as @e[tag=turtle] store result entity @s Pos[0] double 1 run scoreboard players get $0 $obj",
            x
        );
    }
}

/// Set the Y coordinate of the turtle
pub fn turtle_y(y: i32) {
    unsafe {
        insert_asm!(
            "execute as @e[tag=turtle] store result entity @s Pos[1] double 1 run scoreboard players get $0 $obj",
            y
        );
    }
}

/// Set the Z coordinate of the turtle
pub fn turtle_z(z: i32) {
    unsafe {
        insert_asm!(
            "execute as @e[tag=turtle] store result entity @s Pos[2] double 1 run scoreboard players get $0 $obj",
            z
        );
    }
}

/// Write a character to `stdout`. No text is displayed until a newline is written.
pub fn putc(c: u8) {
    // TODO: Add checks for non-printing characters

    unsafe {
        insert_asm!(
            "scoreboard players operation %%temp0_putc $obj = $0 $obj",
            c as i32
        );
        insert_asm!("function stdout:putc", 0);
    }
}

/// Prints out a value to the chat
pub fn print_int(value: i32) {
    unsafe { print(value) }
}

extern "C" {
    #[doc(hidden)]
    #[rustc_args_required_const(0, 1)]
    pub fn insert_asm(data: *const u8, len: usize, input: i32);

    #[doc(hidden)]
    pub fn print_raw(data: *const u8, len: usize);

    #[doc(hidden)]
    pub fn print(value: i32);

    /// Sets the block at the turtle's position
    pub fn turtle_set(block: McBlock);

    /// Returns 1 if the block at the turtle's position matches the argument
    pub fn turtle_check(block: McBlock) -> bool;

    /// Returns the block at the turtle's position
    pub fn turtle_get() -> McBlock;

    /// Returns the char at the turtle's position
    pub fn turtle_get_char() -> u8;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
