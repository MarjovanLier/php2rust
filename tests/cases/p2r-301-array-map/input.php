<?php

declare(strict_types=1);

enum Size: string
{
    case Small = 's';
    case Large = 'l';
}

function shout(string $s): string
{
    return strtoupper($s) . '!';
}

function square(int $n): int
{
    return $n * $n;
}

function fullName(string $name): string
{
    return implode('-', array_map(ucwords(...), explode('-', strtolower($name))));
}

/**
 * @param list<int> $xs
 */
function numbers(array $xs): string
{
    return implode(',', array_map(fn(int $x): string => (string) $x, $xs));
}

/**
 * @param list<Size> $sizes
 */
function big(array $sizes): string
{
    $flags = array_map(fn(Size $s): bool => $s === Size::Large, $sizes);
    $out = '';
    foreach ($flags as $flag) {
        $out .= $flag ? 'y' : 'n';
    }
    return $out;
}

/**
 * @param list<float> $xs
 */
function total(array $xs): float
{
    $sum = 0.0;
    foreach (array_map(fn(float $x): float => $x * 2.0, $xs) as $x) {
        $sum += $x;
    }
    return $sum;
}

$words = [' Hello ', "\tWORLD\n", 'déjà vu'];
echo implode('|', array_map(trim(...), $words)), "\n";
echo implode('|', array_map(ltrim(...), $words)), "\n";
echo implode('|', array_map(rtrim(...), $words)), "\n";
echo implode('|', array_map(strtolower(...), $words)), "\n";
echo implode('|', array_map(strtoupper(...), $words)), "\n";
echo implode('|', array_map(ucfirst(...), ['abc', 'éa', ''])), "\n";
echo implode('|', array_map(lcfirst(...), ['ABC', 'Éa', ''])), "\n";
echo implode('|', array_map(ucwords(...), ['hello world', "a\tb", 'x-y'])), "\n";
echo implode('|', array_map(shout(...), ['a', 'bc'])), "\n";
echo numbers(array_map(square(...), [1, 2, 3])), "\n";
echo fullName('JEAN-LUC smith'), "\n";
echo big([Size::Small, Size::Large, Size::Large]), "\n";
echo total([1.5, 2.25]) > 7.0 ? 'yes' : 'no', "\n";
