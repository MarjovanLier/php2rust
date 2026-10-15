<?php

declare(strict_types=1);

enum Unit: string
{
    case Metre = 'm';
    case Foot = 'ft';
}

/**
 * @param list<int> $xs
 * @return list<int>
 */
function shifted(array $xs, int $by, bool $flip, ?int $extra): array
{
    $offset = 2.5;
    return array_map(
        static fn(int $x): int => ($flip ? -$x : $x) + $by + ($extra ?? 0) + ($offset > 1.0 ? 1 : 0),
        $xs,
    );
}

/**
 * @param list<int> $xs
 * @return list<string>
 */
function labelled(array $xs, Unit $unit): array
{
    return array_map(fn(int $x): string => $x . $unit->value, $xs);
}

/**
 * @param list<int> $xs
 * @return list<int>
 */
function ones(array $xs): array
{
    return array_map(fn(int $x) => 1, $xs);
}

/**
 * @param list<string> $words
 * @return list<string>
 */
function kept(array $words): array
{
    $x = 'outer';
    $copies = array_map(fn(string $x): string => $x, $words);
    $copies[] = $x;
    return $copies;
}

/**
 * @param list<int> $xs
 * @return list<int>
 */
function keywords(array $xs): array
{
    return array_map(fn(int $type): int => 2, $xs);
}

/**
 * @param list<int> $xs
 */
function shown(array $xs): string
{
    return implode(',', array_map(fn(int $x): string => (string) $x, $xs));
}

echo shown(shifted([1, 2], 10, false, null)), "\n";
echo shown(shifted([1, 2], 10, true, 5)), "\n";
echo implode(',', labelled([3, 4], Unit::Foot)), "\n";
echo shown(ones([7, 8])), "\n";
echo shown(keywords([7, 8, 9])), "\n";
echo implode(',', kept(['a', 'b'])), "\n";
