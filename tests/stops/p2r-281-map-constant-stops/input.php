<?php

declare(strict_types=1);

final class Tables
{
    private const array EMPTY_KEY = ['' => 'x', 'a' => 'b'];

    private const array INT_KEY = [1 => 'one'];

    private const array NO_KEY = ['a', 'b'];

    private const array NOT_TEXT = ['a' => 1];

    private const array NONE = [];

    private const array WIDE = ['a' => 'A', "a" => 'B'];

    /** @var array<string, string> */
    private const array GOOD = ['x' => 'y'];

    public static function read(): string
    {
        return self::GOOD['x'];
    }

    /**
     * @return array<string, string>
     */
    public static function pass(): array
    {
        return self::GOOD;
    }

    public static function size(): int
    {
        return count(self::GOOD);
    }
}
