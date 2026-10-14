<?php

declare(strict_types=1);

enum Mode
{
    case Fast;
    case Safe;

    public function label(string $prefix = 'mode', ?Mode $other = null): string
    {
        if ($other === null) {
            return $prefix;
        }
        return $prefix . '+';
    }

    public function same(Mode $other = self::Safe): bool
    {
        return $this === $other;
    }
}

final class Counter
{
    public function __construct(public int $count = 0, public string $name = "c", public Mode $mode = Mode::Safe) {}

    public function add(int $step = 1, float $scale = -0, ?int $cap = null): void
    {
        $this->count += $step;
        if ($cap !== null && $this->count > $cap) {
            $this->count = $cap;
        }
        if ($scale > 0.5) {
            echo "scaled\n";
        }
    }

    public static function make(int $start = -5, Mode $mode = Mode::Fast): Counter
    {
        return new Counter($start, "made", $mode);
    }
}

/**
 * @param list<Counter> $counters
 */
function bumpAll(array $counters = [], int $step = 1): int
{
    foreach ($counters as $counter) {
        $counter->add($step);
    }
    return count($counters);
}

/**
 * @param ?list<int> $xs
 */
function size(?array $xs = [], string|null $tag = null): int
{
    if ($xs === null) {
        return -1;
    }
    return count($xs);
}

/**
 * @param list<int> $xs
 */
function total(array $xs = [], int $bonus = 0, bool $twice = false): int
{
    $sum = $bonus;
    foreach ($xs as $x) {
        $sum += $x;
    }
    if ($twice) {
        return $sum * 2;
    }
    return $sum;
}

function scaled(float $f = 2): float
{
    return $f * 1.5;
}

echo total(), " ", total([1, 2]), " ", total(bonus: 3), " ", total([4], twice: true), "\n";
if (scaled() < scaled(f: 4.0)) {
    echo "less\n";
}
$c = new Counter(name: "x");
$c->add();
$c->add(step: 4, cap: 3);
$c->add(5, cap: 6);
echo $c->count, " ", $c->name, "\n";
$d = Counter::make();
$list = [new Counter()];
if (Mode::Safe->same() && !Mode::Fast->same()) {
    echo "same\n";
}
echo bumpAll(), " ", bumpAll($list), " ", size(), " ", size(null), "\n";
echo $d->count, " ", Mode::Fast->label(), " ", Mode::Safe->label(other: Mode::Fast), "\n";
