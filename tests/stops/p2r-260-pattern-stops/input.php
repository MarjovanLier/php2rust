<?php

declare(strict_types=1);

final class Stops
{
    public static function variable(string $pattern, string $text): bool
    {
        return preg_match($pattern, $text) === 1;
    }

    public static function outside(string $text): bool
    {
        return preg_match(\DateTimeInterface::ATOM, $text) === 1;
    }

    public static function joined(string $text): bool
    {
        return preg_match('/a' . 'b/', $text) === 1;
    }

    public static function unicode(string $text): bool
    {
        return preg_match('/a/u', $text) === 1;
    }

    public static function extended(string $text): bool
    {
        return preg_match('/a b/x', $text) === 1;
    }

    public static function byte(string $text): bool
    {
        return preg_match("/\t/", $text) === 1;
    }

    public static function unbalanced(string $text): bool
    {
        return preg_match('<a<>', $text) === 1;
    }

    public static function end(string $text): bool
    {
        return preg_match('/a$/', $text) === 1;
    }

    public static function lineStart(string $text): bool
    {
        return preg_match('/^a/m', $text) === 1;
    }

    public static function vertical(string $text): bool
    {
        return preg_match('/\v/', $text) === 1;
    }

    public static function boundary(string $text): bool
    {
        return preg_match('/\b{start}/', $text) === 1;
    }

    public static function nested(string $text): bool
    {
        return preg_match('/[a[b]]/', $text) === 1;
    }

    public static function operator(string $text): bool
    {
        return preg_match('/[a&&b]/', $text) === 1;
    }

    public static function posixOutside(string $text): bool
    {
        return preg_match('/[:alpha:]/', $text) === 1;
    }

    public static function posixRange(string $text): bool
    {
        return preg_match('/[[:digit:]-z]/', $text) === 1;
    }

    public static function twice(string $text): bool
    {
        return preg_match('/a**/', $text) === 1;
    }

    public static function afterAssertion(string $text): bool
    {
        return preg_match('/^*a/', $text) === 1;
    }

    public static function emptyRepeat(string $text): bool
    {
        return preg_match('/(?:a?|b)*c/', $text) === 1;
    }

    public static function lookAround(string $text): bool
    {
        return preg_match('/(?=a)a/', $text) === 1;
    }

    public static function backreference(string $text): bool
    {
        return preg_match('/(a)\1/', $text) === 1;
    }

    public static function subjectEnd(string $text): bool
    {
        return preg_match('/a\Z/', $text) === 1;
    }

    public static function badRange(string $text): bool
    {
        return preg_match('/[z-a]/', $text) === 1;
    }
}
