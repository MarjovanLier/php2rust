<?php

declare(strict_types=1);

final class Stops
{
    public static function value(string $text): int|false
    {
        return preg_match('/a/', $text);
    }

    public static function stored(string $text): bool
    {
        $found = preg_match('/a/', $text);
        return $found === 1;
    }

    public static function captured(string $text): bool
    {
        return preg_match('/(a)/', $text, $matches) === 1;
    }

    public static function named(string $text): bool
    {
        return preg_match(pattern: '/a/', subject: $text) === 1;
    }

    public static function againstFalse(string $text): bool
    {
        return preg_match('/a/', $text) === false;
    }

    public static function loose(string $text): bool
    {
        return preg_match('/a/', $text) == 1;
    }

    public static function two(string $text): bool
    {
        return preg_match('/a/', $text) !== 2;
    }

    public static function notString(int $number): bool
    {
        return preg_match('/1/', $number) === 1;
    }
}
