<?php

declare(strict_types=1);

function floats(float|string $x): string
{
    return (string) $x;
}

/** @param array{a: int}|string $x */
function shaped(array|string $x): string
{
    return (string) $x;
}

/** @param array{a: int}|string $x */
function otherArm(array|string $x): string
{
    return is_scalar($x) ? 'scalar' : (string) $x;
}

/** @param array{a: int}|string $x */
function otherVariable(array|string $x, string $y): string
{
    return is_scalar($x) ? (string) $y : 'other';
}

final readonly class Holder
{
    public function __construct(public int|string $id) {}
}

function viaProperty(Holder $h): string
{
    return gettype($h->id);
}

function ownedArm(int|string $v): string
{
    $t = is_scalar($v) ? (string) $v : gettype($v);
    return $t;
}
