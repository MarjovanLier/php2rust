<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n * 10;
    }
    return null;
}

function name(int $n): ?string
{
    if ($n > 0) {
        return 'ann';
    }
    return null;
}

function describe(?int $n): string
{
    if ($n !== null && $n > 25) {
        return "big\n";
    } elseif ($n !== null) {
        return "small $n\n";
    }
    return "none\n";
}

function greet(?string $s, bool $formal): string
{
    if ($formal && $s !== null) {
        return "Dear $s\n";
    }
    if ($s !== null) {
        return "Hi " . $s . "\n";
    }
    return "Hello\n";
}

echo describe(code(3)), describe(code(1)), describe(null);
echo greet(name(1), true), greet(name(0), true), greet('bob', false);
$s = name(1);
if ($s !== null) {
    echo strlen($s), "\n";
}
echo $s ?? 'none', "\n";
$total = 0;
foreach ([1, 2, 3] as $i) {
    $c = code($i - 2);
    if ($c !== null) {
        $total += $c;
    }
}
echo $total, "\n";
$a = code(1);
$b = code(2);
if ($a !== null && $b !== null) {
    echo $a + $b, "\n";
}
if ($a !== null) {
    echo "a set\n";
}
