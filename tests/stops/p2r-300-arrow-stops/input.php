<?php

declare(strict_types=1);

final class Holder
{
    public function __construct(public int $base)
    {
    }

    /**
     * @param list<int> $xs
     * @return list<int>
     */
    public function shifted(array $xs): array
    {
        return array_map(fn(int $x): int => $x + $this->base, $xs);
    }
}

function positive(int $x): int
{
    if ($x < 0) {
        throw new InvalidArgumentException('negative');
    }
    return $x;
}

function apply(int $x): int
{
    return $x;
}

function maybe(int $x): ?int
{
    return $x > 1 ? $x : null;
}

/**
 * @param list<int> $xs
 * @return list<int>
 */
function forms(array $xs, string $label): array
{
    $stored = fn(int $x): int => $x;
    $fooBar = 1;
    $a = array_map(fn($x) => $x, $xs);
    $b = array_map(fn(int &$x): int => $x, $xs);
    $c = array_map(fn(int $x = 1): int => $x, $xs);
    $d = array_map(fn&(int $x): int => $x, $xs);
    $e = array_map(fn(int $x): int => strlen($label) + $x, $xs);
    $f = array_map(fn(int $x): int => $x++, $xs);
    $g = array_map(fn(int $x): int => positive($x), $xs);
    $h = array_map(fn(int $x): int => maybe($x) ?? throw new RuntimeException('none'), $xs);
    $i = array_map(fn(int $x): int => (fn(int $y): int => $y)($x), $xs);
    $j = array_map(static function (int $x): int {
        return $x;
    }, $xs);
    $k = array_map(fn(int $foo_bar): int => $foo_bar + $fooBar, $xs);
    return [apply(1), count($a), count($b), count($c), count($d), count($e), count($f), count($g),
        count($h), count($i), count($j), count($k), $stored(1)];
}
