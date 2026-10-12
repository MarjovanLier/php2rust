<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
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

function loud(string $s): string
{
    echo "[$s]";
    return $s;
}

function label(?string $s): string
{
    return $s ?? 'none';
}

function pick(?string $s, string $other): string
{
    return $s ?? $other;
}

/**
 * @param ?list<int> $xs
 * @return list<int>
 */
function orDefault(?array $xs): array
{
    return $xs ?? [1, 2];
}

$a = code(3);
$b = code(0);
echo $a ?? 0, ' ', $b ?? 7, "\n";
$n = 5;
echo $b ?? $n, "\n";
echo code(0) ?? code(0) ?? 9, "\n";
$s = name(1);
$t = name(0);
echo $s ?? 'none', ' ', $t ?? 'none', "\n";
echo strlen($t ?? 'abc'), "\n";
$u = $t ?? loud('x');
$v = $s ?? loud('y');
echo $u, ' ', $v, "\n";
echo label(null), ' ', pick($t, 'other'), "\n";
echo count(orDefault(null)), "\n";
