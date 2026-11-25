#!/usr/bin/env bash
# Runs the Kani or the Verus proofs under a memory cap and a time cap. Kani 0.68.0 and
# CBMC 6.11.0 have no memory option, and macOS refuses `ulimit -v`, so this script
# watches the run and ends it when it passes a cap.
#
# Use: scripts/prove.sh kani [cargo kani arguments, such as --harness NAME]
#      scripts/prove.sh verus
#
# Caps: PROVE_MAX_MB (default 4096) is the resident memory of every process in the run,
# the compilers included; PROVE_MAX_SECONDS (default 900) is the wall time. Verus also
# passes PROVE_SOLVER_MB (default 1024) to Z3, which enforces that cap itself.
#
# shortcut: the cap reads resident memory, which macOS lowers when it compresses memory
# under pressure; read `footprint` instead when a run passes the cap unseen on a Mac that
# is already short of memory.
set -euo pipefail

MAX_MB=${PROVE_MAX_MB:-4096}
MAX_SECONDS=${PROVE_MAX_SECONDS:-900}
SOLVER_MB=${PROVE_SOLVER_MB:-1024}

cd "$(dirname "$0")/.."
tool=${1:-}
shift || true
case "$tool" in
kani)
    case " $* " in
    *" -j"* | *" --jobs"*)
        echo "prove.sh: -j runs proofs at the same time; leave it out" >&2
        exit 2
        ;;
    esac
    # Two compiler jobs keep the build of the Mago crates under the cap. Kani 0.68.0
    # compiles with nightly-2026-08-21, where str::trim_prefix, which Mago 1.61 uses, is
    # not yet stable; drop the flag when a Kani release ships a newer nightly.
    command=(env CARGO_BUILD_JOBS=2
        RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-Zcrate-attr=feature(trim_prefix_suffix)"
        cargo kani "$@")
    ;;
verus)
    command=(verus verus/octal.rs --smt-option "memory_max_size=$SOLVER_MB" --num-threads 1 "$@")
    ;;
*)
    echo "use: scripts/prove.sh kani|verus [arguments]" >&2
    exit 2
    ;;
esac

# Job control gives the run its own process group, and one signal ends the group.
set -m
"${command[@]}" &
run=$!
# KILL follows TERM because a stopped process does not act on TERM; the script must
# not return while a solver still runs.
trap 'kill -- -"$run" 2>/dev/null || true; sleep 2; kill -KILL -- -"$run" 2>/dev/null || true' EXIT

peak=0
SECONDS=0
while kill -0 "$run" 2>/dev/null; do
    used=$(ps -Ao pgid=,rss= | awk -v group="$run" \
        '$1 == group { sum += $2 } END { print int(sum / 1024) }')
    if ((used > peak)); then
        peak=$used
    fi
    if ((used > MAX_MB || SECONDS > MAX_SECONDS)); then
        echo "prove.sh: stopped at ${used} MB after ${SECONDS} s (caps: ${MAX_MB} MB, ${MAX_SECONDS} s)" >&2
        exit 1
    fi
    # A Verus run ends within a second, so a slower sample misses its peak.
    sleep 0.2
done

status=0
wait "$run" || status=$?
trap - EXIT
echo "prove.sh: peak memory ${peak} MB in ${SECONDS} s"
exit "$status"
