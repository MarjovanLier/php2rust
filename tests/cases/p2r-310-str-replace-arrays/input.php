<?php

declare(strict_types=1);

final class Text
{
    /**
     * @param string|string[] $search
     * @param string|string[] $replace
     */
    public static function strReplace(array|string $search, array|string $replace, string $subject): string
    {
        if ($subject === '') {
            return '';
        }
        if (is_string($search) && is_string($replace) && strlen($search) === 1) {
            return strtr($subject, [$search => $replace]);
        }
        return str_replace($search, $replace, $subject);
    }
}

/**
 * @return array<array-key, string>
 */
function two(string $a, string $b): array
{
    return ['k2' => $a, 'k1' => $b];
}

/**
 * @return array<array-key, string>
 */
function one(string $a): array
{
    return ['z' => $a];
}

/**
 * @param string|list<string> $search
 */
function listed(array|string $search, string $s): string
{
    return str_replace($search, '_', $s);
}

/**
 * @param list<string> $xs
 * @param array<string, string> $map
 */
function forms(string $s, array $xs, array $map): string
{
    $mid = 'b';
    return str_replace(['a', $mid], '-', $s) . '|'
        . str_replace($xs, ['1', '2'], $s) . '|'
        . str_replace($map, $xs, $s) . '|'
        . str_replace(['', 'b'], ['x', 'y'], $s) . '|'
        . str_replace(['a', 'b'], ['b', 'c'], $s) . '|'
        . str_replace(['é', 'c'], 'e', $s . 'é') . '|'
        . str_replace([], 'x', $s);
}

echo Text::strReplace('a', 'b', 'banana'), "\n";
echo Text::strReplace('an', 'AN', 'banana'), "\n";
echo Text::strReplace(two('a', 'n'), 'x', 'banana'), "\n";
echo Text::strReplace(two('a', 'n'), two('n', 'a'), 'banana'), "\n";
echo Text::strReplace(two('b', 'a'), one('1'), 'banana'), "\n";
echo Text::strReplace('', 'x', 'banana'), "\n";
echo '[', Text::strReplace('a', 'b', ''), "]\n";
echo forms('abcab', ['a', 'b', 'c'], ['x' => 'c', 'y' => 'a']), "\n";
echo listed(explode(',', 'a,n'), 'banana'), ' ', listed('na', 'banana'), "\n";
