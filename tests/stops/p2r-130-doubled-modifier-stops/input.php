<?php

declare(strict_types=1);

// PHP rejects a modifier written twice and a second visibility when it compiles the file,
// and Mago reports none of them.
final final readonly class Point
{
    public public function __construct(public public int $x) {}

    final final public function twice(): int
    {
        return $this->x * 2;
    }
}

final class Counter
{
    private private int $count = 0;

    public static static function make(): Counter
    {
        return new Counter();
    }

    public function count(): int
    {
        return $this->count;
    }
}

// PHP rejects these constructors too.
readonly class Typed
{
    public function __construct(public int $x): void {}
}

readonly class Shared
{
    public static function __construct(public int $x) {}
}

// PHP rejects these modifiers too, and Mago reports none of them.
final class Modifiers
{
    private int $hidden = 0;

    final private int $sealed = 0;

    private public(set) int $wide = 0;

    public function __construct(abstract string $kind, private public(set) int $level) {}

    public readonly function twice(): int
    {
        return $this->level * 2;
    }

    public function add(final int $n): int
    {
        return $this->level + $n;
    }
}
