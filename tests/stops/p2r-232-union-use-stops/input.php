<?php

declare(strict_types=1);

function compare(int|string $a, int|string $b): bool
{
    return $a === $b;
}

function printed(int|string $a): void
{
    echo $a, "\n";
}

function narrowed(int|string $v, bool $b): int
{
    // Mago narrows `$v` here, but an or-chain binds nothing (P2R-252).
    if (!is_string($v) || $b) {
        return 0;
    }
    return strlen($v);
}

function joined(int|string $v): string
{
    return 'v=' . $v;
}

function condition(int|string|null $v): bool
{
    if ($v) {
        return true;
    }
    return false;
}

function counted(int|string $v): int
{
    return $v + 1;
}

function subject(int|string $v): int
{
    return match ($v) {
        1 => 1,
        default => 0,
    };
}

function cast(int|string $v): int
{
    return (int) $v;
}

final readonly class Leaf
{
    public function __construct(public string $name) {}

    public function label(): string
    {
        return $this->name;
    }
}

final readonly class Branch
{
    public function __construct(public string $name) {}

    public function label(): string
    {
        return $this->name;
    }
}

function property(Leaf|Branch $x): string
{
    return $x->name;
}

function method(Leaf|Branch $x): string
{
    return $x->label();
}

/** @param list<int>|string $v */
function looped(array|string $v): int
{
    foreach ($v as $item) {
        return $item;
    }
    return 0;
}

/** @param list<int>|string $v */
function element(array|string $v): int
{
    return $v[0];
}

function defaulted(int|string|null $v): int|string
{
    return $v ?? 0;
}

function plainCondition(int|string $v): bool
{
    if ($v) {
        return true;
    }
    return false;
}

function interpolated(int|string $v): string
{
    return "v=$v";
}

function divided(int|float $v): float
{
    return fdiv($v, 2.0);
}

function appended(int|string $u): string
{
    $t = $u;
    $t .= 'x';
    return gettype($t);
}
