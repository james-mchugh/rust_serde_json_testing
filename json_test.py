import orjson
import timeit

max_iters = 10_000_000

timer = timeit.Timer(
    """orjson.loads('{"foo": "bar"}')""",
    globals=globals(),
)

results = timer.repeat(7, max_iters)

min_duration_ns = min(results) / max_iters * 10**9
max_duration_ns = max(results) / max_iters * 10**9
print(f"Max duration {max_duration_ns:02}ns")
print(f"Min duration {min_duration_ns:02}ns")