<?php

declare(strict_types=1);

final readonly class Plain
{
    #[Override]
    public function size(): int
    {
        return 1;
    }
}

/**
 * @implements ArrayAccess<string, string>
 */
final readonly class Marked implements ArrayAccess
{
    #[Override, Deprecated]
    public function offsetExists(mixed $offset): bool
    {
        return false;
    }

    #[SensitiveParameter]
    public function offsetGet(mixed $offset): string
    {
        return '';
    }

    #[Override]
    #[Override]
    public function offsetSet(mixed $offset, mixed $value): void {}

    public function offsetUnset(mixed $offset): void {}

    #[Override]
    public function other(): int
    {
        return 1;
    }
}
