pos: u64,
val: u64,

const Self = @This();
const Act = @import("action.zig");

//helper function
fn get(bitvec: u64, index: u4) u4 {
    return @truncate(bitvec >> (@as(u6, index) << 2));
}

fn inv(perm: u64) u64 {
    var res: u64 = 0;
    for (0..16) |i| {
        res |= i << (@as(u6, get(perm, i)) * 4);
    }
    return res;
}

pub fn init() Self {
    return .{
        .pos = 0xFEDC_BA98_7654_3210,
        .val = 0xFEDC_BA98_7654_3210,
    };
}

pub fn frompos(a: u64) Self {
    return .{
        .pos = a,
        .val = inv(a),
    };
}

pub fn verify(a: Self) bool {
    for (0..16) |i| {
        if (get(a.pos, get(a.val, i)) != i) return false;
    }
    return true;
}

pub fn sign(a: Self) u1 {
    var b = a;
    _ = b;
}

fn possible_moves(a: Self) u4 {
    const curpos = get(a.pos, 0);
    return @truncate(0x0000_0FF0_0FF0_0000 >> curpos); //todo:fix
}

fn act(a: *Self, b: Act) bool {
    _ = b;
    _ = a;
}
