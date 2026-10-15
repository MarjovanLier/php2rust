<?php

declare(strict_types=1);

/**
 * @return array<array-key, string>
 */
function make(): array
{
    return ['a'];
}

/**
 * @return string|string[]
 */
function either(bool $flag): array|string
{
    return $flag ? 'a' : make();
}

/**
 * @param list<string> $xs
 * @param list<int> $ns
 * @param int|list<string> $odd
 */
function forms(string $s, array $xs, array $ns, int|array $odd, bool $flag): string
{
    $a = str_replace('a', $xs, $s);
    $b = str_replace('a', ['x'], $s);
    $c = str_replace($xs, 'b', $xs);
    $d = str_replace(['a', 1], 'b', $s);
    $e = str_replace(['a', ...$xs], 'b', $s);
    $f = str_replace($ns, 'b', $s);
    $g = str_replace($odd, 'b', $s);
    $h = str_replace(either($flag), 'b', $s);
    $i = str_replace($xs, 'b', $s, $count);
    $j = str_replace([$s => 'a', 'k' => 'b'], ['1', '2'], $s);
    return $a . $b . $d . $e . $f . $g . $h . $i . count($c) . $count . $j;
}
