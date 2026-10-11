<?php

declare(strict_types=1);

/** @param positive-int $n */
function steps(int $n): int
{
    return $n * 2;
}

/** @param int<0, max> $n */
function small(int $n): bool
{
    return $n < 10;
}

/** @param 1|2|3 $tier */
function level(int $tier): int
{
    return $tier * 100;
}

/**
 * @param non-empty-string $word
 * @return non-empty-string
 */
function shout(string $word): string
{
    return $word . '!';
}

/** @param numeric-string $digits */
function width(string $digits): int
{
    return strlen($digits);
}

/**
 * @param non-empty-list<int> $values
 * @return list{int, int}
 */
function ends(array $values): array
{
    return [$values[0], $values[count($values) - 1]];
}

$pair = ends([4, 5, 6]);
echo steps(3), ' ', level(2), ' ', shout('hey'), ' ', width('2026'), ' ', $pair[0], $pair[1], "\n";
if (small(4)) {
    echo "small\n";
}
