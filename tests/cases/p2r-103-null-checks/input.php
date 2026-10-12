<?php

declare(strict_types=1);

function name(int $n): ?string
{
    if ($n > 0) {
        return 'ann';
    }
    return null;
}

function code(int $n): ?int
{
    if ($n > 0) {
        return $n * 10;
    }
    return null;
}

function same(?string $p, ?string $q): string
{
    if ($p === $q) {
        return "same\n";
    }
    return "different\n";
}

$a = name(1);
$b = name(0);
$c = code(2);
$d = code(0);
$n = 20;
if ($a !== null) {
    echo "a set\n";
}
if (null === $b) {
    echo "b null\n";
}
if ($a === 'ann') {
    echo "a is ann\n";
}
if ($c === $n) {
    echo "c is n\n";
}
if ($c !== $d) {
    echo "c and d differ\n";
}
echo same($a, 'ann'), same($b, null), same($a, $b);
