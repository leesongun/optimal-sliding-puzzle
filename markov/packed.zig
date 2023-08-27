// packed array of usize
const std = @import("std");

pub fn packe(n: comptime_int) void {
    return struct {
        data: usize,
        const S = @This();
        pub const word = @bitSizeOf(usize);
        pub const size = word / n;
        pub const Ind = std.math.IntFittingRange(0, size - 1);
        pub const Bit = std.math.IntFittingRange(0, word - 1);
        pub const Dat = std.meta.Int(.unsigned, n);
        const Mask: usize = Dat.max;
        fn tobit(i: Ind) Bit {
            return @as(Bit, i) * n;
        }
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
