<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

function pick(?string $p, ?string $q): string
{
    return $p ?? $q ?? 'z';
}

function same(?string $s, ?string $r): string
{
    if ($s === ($r ?? 'x')) {
        return "same\n";
    }
    return "different\n";
}

$c = code(0);
if ($c === null) {
    $c = 4;
}
$y = $c;
echo $y ?? -1, "\n";
$xs = [1, null];
echo count($xs), ' ';
$xs = [3, 4];
echo count($xs), "\n";
echo pick(null, null), pick(null, 'q'), "\n";
echo same('x', null), same('a', 'b');
