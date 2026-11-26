<?php

declare(strict_types=1);

function nested(?int $c, bool $b): int
{
    if ($b) {
        if ($c === null) {
            $c = 0;
        }
    }
    return $c + 1;
}

function text(?string $s): string
{
    if ($s === null) {
        $s = 'guest';
    }
    return $s . '!';
}

function widened(?float $x): float
{
    if ($x === null) {
        $x = 0;
    }
    return $x * 2.0;
}

function chained(?int $c, bool $b): int
{
    if ($c === null || $b) {
        $c = 0;
    }
    return $c + 1;
}

function twoStatements(?int $c): int
{
    if ($c === null) {
        echo "default\n";
        $c = 0;
    }
    return $c + 1;
}

function unread(?int $c): void
{
    if ($c === null) {
        $c = 0;
    }
    echo "done\n";
}

function nullAgain(?int $c): int
{
    if ($c === null) {
        $c = 0;
    }
    $d = $c + 1;
    $c = null;
    return $d;
}

echo nested(null, true), text(null), chained(null, true), twoStatements(null), "\n";
echo widened(null) === 0.0 ? 'zero' : 'other', "\n";
unread(null);
echo nullAgain(null), "\n";
