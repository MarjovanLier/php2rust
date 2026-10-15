<?php

declare(strict_types=1);

final class Counter
{
    private int $n = 0;

    public function bump(): void
    {
        $this->n++;
    }
}

final class IndexMap
{
}

/**
 * @param array<string, int|string> $m
 */
function unionValue(array $m): int
{
    return count($m);
}

/**
 * @param array<string, mixed> $m
 */
function mixedValue(array $m): int
{
    return count($m);
}

/**
 * @param array<string, int>|string $m
 */
function unionMember(array|string $m): bool
{
    return is_string($m);
}

/**
 * @param array<string, Counter> $m
 */
function changing(array $m): int
{
    return count($m);
}

/**
 * @param array<class-string, int> $m
 */
function classKeys(array $m): int
{
    return count($m);
}

/**
 * @param array<string, int> $m
 */
function takeMap(array $m): int
{
    return count($m);
}

/**
 * @param list<int> $l
 */
function takeList(array $l): int
{
    return count($l);
}

/**
 * @return array<int, int>
 */
function intMap(): array
{
    return [1 => 2];
}

function passes(): int
{
    $list = [1, 2];
    $shape = ['a' => 1];
    return takeMap($shape) + takeMap($list) + takeList(intMap()) + takeMap(intMap());
}

/**
 * PHP keeps each value of a map as it is, so no map converts to a map of other values.
 *
 * @param array<string, int> $m
 * @return array<string, ?int>
 */
function nullableValues(array $m): array
{
    return $m;
}

/**
 * @param array<string, float> $m
 */
function takeFloats(array $m): int
{
    return count($m);
}

/**
 * @param array<string, int> $m
 * @return list<array<string, ?int>>
 */
function nested(array $m): array
{
    takeFloats($m);
    return [$m];
}
