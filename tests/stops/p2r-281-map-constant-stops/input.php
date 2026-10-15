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

    private const ?array NULLABLE = ['a' => 'b'];

    private const iterable ITERABLE = ['a' => 'b'];

    private const array HEREDOC = [<<<EOT
        a
        EOT => 'b'];

    private const array NOWDOC = ['a' => <<<'EOT'
        b
        EOT];

    private const array FLOAT_KEY = [1.5 => 'b'];

    private const array CONSTANT_KEY = [PHP_EOL => 'b'];

    /** @var array<string, string> */
    private const array GOOD = ['x' => 'y'];

    public static function refused(string $text): string
    {
        return strtr($text, self::WIDE);
    }

    public static function has(): bool
    {
        return isset(self::GOOD['x']);
    }

    public static function each(): string
    {
        $out = '';
        foreach (self::GOOD as $value) {
            $out .= $value;
        }
        return $out;
    }

    public static function stored(): int
    {
        $copy = self::GOOD;
        return count($copy);
    }

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
