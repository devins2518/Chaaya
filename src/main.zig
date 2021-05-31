const std = @import("std");
const aakil = @import("aakil");

pub fn main() anyerror!void {
    std.log.info("All your codebase are belong to us.", .{});
    aakil.cpu.CPU.hello_world();
}
