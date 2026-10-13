<?php

declare(strict_types=1);

abstract class Plain
{
    public function __construct(public int $x)
    {
    }
}

readonly class Promo
{
    public function __construct(public int &$a, public int ...$b)
    {
    }
}

readonly class Untyped
{
    public function __construct(public $a)
    {
    }
}

readonly class Body
{
    public function __construct(public int $a)
    {
        echo "made\n";
    }
}

readonly class Child extends Promo
{
}

readonly class Magic
{
    public function __construct(public int $a)
    {
    }

    public function __toString(): string
    {
        return 'm';
    }
}

readonly class Loop
{
    public function __construct(public Loop $next)
    {
    }
}

enum Nothing
{
}

readonly class Holder
{
    public function __construct(public Nothing $n)
    {
    }
}

function nested(): void
{
    readonly class Inner
    {
    }
}
