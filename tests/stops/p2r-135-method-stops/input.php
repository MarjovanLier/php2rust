<?php

declare(strict_types=1);

readonly class Node
{
    public function __construct(public int $x)
    {
    }

    public static function make(): int
    {
        return 1;
    }

    public function me(): static
    {
        return $this;
    }

    public function walk(): int
    {
        $next = new Node($this->x + 1);
        return $next->walk();
    }

    public function echoOf(Node $other): int
    {
        return $other->echoOf($this);
    }

    public function bad(): void
    {
        $this = new Node(1);
    }

    public function calls(): int
    {
        return self::make() + $this->missing();
    }
}
