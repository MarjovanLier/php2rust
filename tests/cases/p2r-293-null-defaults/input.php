<?php

declare(strict_types=1);

enum Size
{
    case Small;
    case Large;
}

function bump(int $n): int
{
    echo "bump called\n";
    return $n + 1;
}

function plusOne(?int $c): int
{
    if ($c === null) {
        $c = 0;
    }
    return $c + 1;
}

function show(?int $c): void
{
    if (null === $c) {
        $c = -1;
    }
    echo $c, "\n";
}

function onDemand(?int $c, int $d): int
{
    if ($c === null) {
        $c = bump($d);
    }
    return $c * 2;
}

function later(?int $c): int
{
    if ($c === null) {
        $c = 0;
    }
    $c += 5;
    return $c;
}

function ratio(?float $f): float
{
    if ($f === null) {
        $f = 0.5;
    }
    return $f * 2.0;
}

function flag(?bool $b): string
{
    if ($b === null) {
        $b = false;
    }
    return $b ? "yes" : "no";
}

function size(?Size $s): string
{
    if ($s === null) {
        $s = Size::Small;
    }
    return $s->name;
}

final class Box
{
    public function width(?int $w): int
    {
        if ($w === null) {
            $w = 10;
        }
        return $w * 3;
    }
}

function maybe(bool $b): ?int
{
    if ($b) {
        return 3;
    }
    return null;
}

function local(bool $b): int
{
    $c = maybe($b);
    if ($c === null) {
        $c = 7;
    }
    return $c;
}

echo plusOne(null), ' ', plusOne(4), "\n";
show(null);
show(8);
echo onDemand(null, 4), ' ', onDemand(2, 4), "\n";
echo later(null), ' ', later(1), "\n";
echo ratio(null) === 1.0 ? 'one' : 'other', ' ', ratio(2.0) === 4.0 ? 'four' : 'other', "\n";
echo flag(null), ' ', flag(true), "\n";
echo size(null), ' ', size(Size::Large), "\n";
echo (new Box())->width(null), ' ', (new Box())->width(2), "\n";
echo local(true), ' ', local(false), "\n";
