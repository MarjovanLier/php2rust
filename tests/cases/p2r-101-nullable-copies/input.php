<?php

declare(strict_types=1);

function name(int $n): ?string
{
    if ($n > 0) {
        return 'ann';
    }
    return null;
}

function keep(?string $s): ?string
{
    return $s;
}

/**
 * @param ?list<int> $xs
 * @return ?list<int>
 */
function copyOf(?array $xs): ?array
{
    return $xs;
}

function show(?string $s): string
{
    if ($s === null) {
        return "none\n";
    }
    return "some\n";
}

/** @param ?list<int> $xs */
function known(?array $xs): string
{
    if ($xs === null) {
        return "no list\n";
    }
    return "list\n";
}

$a = name(1);
$b = $a;
echo show($a), show($b);
$c = keep($b);
echo show($c);
$d = $c;
echo show($d);
$l = copyOf([1, 2]);
echo known($l), known(copyOf(null));
