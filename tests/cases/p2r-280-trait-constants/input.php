<?php

declare(strict_types=1);

trait Greek
{
    /** @var array<string, string> */
    private const array GREEK = ["\u{03b1}" => 'a', "\u{03b2}" => 'b', "\u{03b1}\u{03b2}" => 'ab!'];

    public const string NAME = 'greek';
}

trait Marks
{
    private const array MARKS = ['.' => '', ',' => ' ', '\\' => '/'];

    // A value that is also a key is not read again.
    private const array SWAP = ['a' => 'b', 'b' => 'a'];

    final public const int LIMIT = 3;
}

final class Cleaner
{
    use Greek, Marks;

    public const string VERSION = 'v1';

    public static function greek(string $text): string
    {
        return strtr($text, self::GREEK);
    }

    public static function marks(string $text): string
    {
        return strtr($text, static::MARKS);
    }

    public static function swap(string $text): string
    {
        return strtr($text, self::SWAP);
    }

    public static function label(): string
    {
        return self::NAME . ' ' . self::VERSION . ' ' . self::LIMIT;
    }
}

enum Level: string
{
    use Marks;

    case Low = 'low';

    public static function tidy(string $text): string
    {
        return strtr($text, self::MARKS);
    }
}

echo Cleaner::greek("\u{03b1}\u{03b2}\u{03b3} \u{03b2}\u{03b1}"), "\n";
echo Cleaner::marks('a.b,c\\d'), "\n";
echo '[', Cleaner::greek(''), '] [', Cleaner::marks(',.,'), '] [', Cleaner::greek("x\u{03b1}"), "]\n";
echo Cleaner::label(), ' ', Cleaner::swap('abba'), "\n";
echo Level::tidy('x.y,z'), ' ', Level::Low->value, ' ', Level::LIMIT, "\n";
