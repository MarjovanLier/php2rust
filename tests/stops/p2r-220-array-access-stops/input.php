<?php

declare(strict_types=1);

final readonly class Untyped implements ArrayAccess
{
    public function offsetExists(mixed $offset): bool
    {
        return false;
    }

    public function offsetGet(mixed $offset): mixed
    {
        return null;
    }

    public function offsetSet(mixed $offset, mixed $value): void {}

    public function offsetUnset(mixed $offset): void {}
}

/**
 * @implements ArrayAccess<string, int|string>
 */
final readonly class Either implements ArrayAccess
{
    public function offsetExists(mixed $offset): bool
    {
        return false;
    }

    public function offsetGet(mixed $offset): int|string
    {
        return 1;
    }

    public function offsetSet(mixed $offset, mixed $value): void {}

    public function offsetUnset(mixed $offset): void {}
}

/**
 * @implements ArrayAccess<string, float>
 */
final readonly class Measured implements ArrayAccess
{
    public function offsetExists(mixed $offset): bool
    {
        return false;
    }

    public function offsetGet(mixed $offset): float
    {
        return 1.5;
    }

    public function offsetSet(mixed $offset, mixed $value): void {}

    public function offsetUnset(mixed $offset): void {}
}

/**
 * @implements ArrayAccess<string, string>
 */
final readonly class Counted implements ArrayAccess, Countable
{
    public function count(): int
    {
        return 0;
    }

    public function offsetExists(mixed $offset): bool
    {
        return false;
    }

    public function offsetGet(mixed $offset): string
    {
        return '';
    }

    public function offsetSet(mixed $offset, mixed $value): void {}

    public function offsetUnset(mixed $offset): void {}
}

/**
 * @implements ArrayAccess<string, string>
 */
final readonly class Read implements ArrayAccess
{
    public function offsetExists(mixed $offset): bool
    {
        return true;
    }

    public function offsetGet(mixed $offset): string
    {
        return 'x';
    }

    public function offsetSet(mixed $offset, mixed $value): void {}

    public function offsetUnset(mixed $offset): void {}

    public function first(): string
    {
        return $this['a'];
    }
}
