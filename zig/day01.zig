const std = @import("std");

pub fn main() !void {
    @setEvalBranchQuota(2_000_000);
    const part1, const part2 = comptime blk: {
        const input = @embedFile("inputs/day01/input");
        const nLines = std.mem.count(u8, input, &[_]u8{'\n'});
        var leftCol: [nLines]u32 = undefined;
        var rightCol: [nLines]u32 = undefined;
        var iter = std.mem.tokenizeAny(u8, input, &std.ascii.whitespace);
        for (0..nLines) |lineIdx| {
            leftCol[lineIdx] = try std.fmt.parseInt(u32, iter.next() orelse @compileError("bad line"), 10);
            rightCol[lineIdx] = try std.fmt.parseInt(u32, iter.next() orelse @compileError("bad line"), 10);
        }
        std.mem.sortUnstable(u32, &leftCol, {}, std.sort.asc(u32));
        std.mem.sortUnstable(u32, &rightCol, {}, std.sort.asc(u32));

        var absDiffSum = 0;
        for (leftCol, rightCol) |l, r| {
            absDiffSum += if (l > r) (l - r) else (r - l);
        }

        var weightedSum = 0;
        for (leftCol) |l| {
            // easy but slow because of quadratic runtime
            weightedSum += l * std.mem.count(u32, &rightCol, &.{l});
        }
        break :blk .{ absDiffSum, weightedSum };
    };
    std.debug.print("part 1: {}\npart2: {}\n", .{ part1, part2 });
}
