const std = @import("std");
pub const word = @bitSizeOf(usize);

//presumably using mulx is faster. than shifts?
fn mulh(a:usize, b:usize) [2]usize {
    const DWORD = std.meta.Int(.unsigned, 2*word);
    const x = @as(DWORD, a) * @as(DWORD, b);   
    return .{@intCast(x>>word), @truncate(x)};
}

pub fn packed_array(bits: comptime_int, size: comptime_int) void {
    return struct {
        data: usize,
        const S = @This();
        pub const Ind = std.math.IntFittingRange(0, size - 1);
        pub const Bit = std.math.IntFittingRange(0, word - 1);
        pub const Dat = std.meta.Int(.unsigned, bits);
        //log2 word
        pub const bit = @bitSizeOf(Bit);
        //number of data in a word
        pub const count = word / bits;
        comptime {
            if (1 << bit != word)
                @compileError("word size is not power of 2");
        }
        const Mask: usize = Dat.max;
        fn tobit(i: Ind) Bit {
            return @as(Bit, i) * bits;
        }
        const reciprocal = ((1<<word) + count - 1) / count;
        fn tobits2(i:Ind) struct {usize, Bit} {
            const b = mulh(reciprocal, i);
            return .{b[0], @intCast(b[1] >> (word - bit))};
        }
        const in_limit = a:{
            const largest = size / count;
            const x = tobits2(largest * count);
            const check_hi = x[0] == largest;
            const check_lo = x[1] + bits <= word;
            break :a check_hi and check_lo;
        };
        pub fn get(s: S, i: Ind) Dat {
            //todo:fix i*n
            return @truncate(s.data >> tobit(i));
        }
        pub fn zero(s: *S, i: Ind) void {
            s.*.data &= ~(Mask << tobit(i));
        }
        pub fn set1(s: *S, i: Ind, d: Dat) void {
            s.*.data |= (d << tobit(i));
        }
        pub fn set(s: *S, i: Ind, d: Dat) void {
            s.zero(i);
            s.set1(i, d);
        }
    };
}
