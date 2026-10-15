<?php

declare(strict_types=1);

final class Stock
{
    /**
     * @param array<string, int> $counts
     */
    public function __construct(private array $counts)
    {
    }

    public function count(string $item): int
    {
        return $this->counts[$item] ?? 0;
    }

    public function has(string $item): bool
    {
        return isset($this->counts[$item]);
    }

    /**
     * @return array<string, int>
     */
    public function counts(): array
    {
        return $this->counts;
    }
}

final class Tally
{
    /** @var array<int, int> */
    private array $seen = [];

    /** @var array<string, int> */
    private array $base = ['a' => 1, 'b' => 2];

    private int $total = 0;

    public function add(int $n): void
    {
        // The body changes the object that holds the map, so the loop reads a copy.
        foreach ($this->base as $v) {
            $this->total += $v + $n;
        }
    }

    public function total(): int
    {
        return $this->total + count($this->seen);
    }
}

/**
 * @param array<string, int> $m
 */
function withDefault(array $m = ['x' => 1]): int
{
    return count($m);
}

/**
 * @return array{name: string, tags: array<string, int>}
 */
function record(): array
{
    return ['name' => 'r', 'tags' => ['t' => 1]];
}

/**
 * @return list<array<string, int>>
 */
function rows(): array
{
    return [['a' => 1], ['b' => 2, 'c' => 3]];
}

/**
 * @param ?array<string, int> $m
 */
function maybe(?array $m): int
{
    if ($m === null) {
        return -1;
    }
    return count($m);
}

/**
 * @param array<string, array<string, int>> $m
 */
function nested(array $m): int
{
    $inner = $m['a'];
    return $m['a']['x'] + $inner['y'] + count($m['b']);
}

$stock = new Stock(['apple' => 3, 'pear' => 0]);
echo $stock->count('apple'), ' ', $stock->count('fig'), "\n";
echo $stock->has('pear') ? 'pear' : 'no pear', "\n";
echo count($stock->counts()), "\n";
$tally = new Tally();
$tally->add(1);
echo $tally->total(), "\n";
echo withDefault(), ' ', withDefault([]), "\n";
$r = record();
echo $r['name'], ' ', $r['tags']['t'], "\n";
echo count(rows()), ' ', rows()[1]['c'], "\n";
echo maybe(null), ' ', maybe(['a' => 1, 'b' => 2]), "\n";
echo nested(['a' => ['x' => 1, 'y' => 2], 'b' => ['z' => 3]]), "\n";
