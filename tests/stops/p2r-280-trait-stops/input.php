<?php

declare(strict_types=1);

trait WithMethod
{
    public const string A = 'a';

    public static function hello(): string
    {
        return 'hi';
    }
}

trait WithProperty
{
    public int $count = 0;
}

trait Uses
{
    use WithMethod;
}

trait Plain
{
    public const string B = 'b';
}

final class Conflicts
{
    use Plain {
        Plain::B as C;
    }
}

final class Unknown
{
    use Missing;
}

final class Clash
{
    use Plain;

    public const string B = 'b';
}

final class Twice
{
    use Plain, Plain;
}

trait First
{
    public const string SAME = 'first';
}

trait Second
{
    public const string SAME = 'second';
}

final class Both
{
    use First, Second;
}

trait Low
{
    public const string Low = 'low';
}

enum Level: string
{
    use Low;

    case Low = 'low';
}

trait Shape
{
    abstract public function area(): int;
}

trait Countable
{
    public const string D = 'd';
}

function local(): void
{
    trait Inner
    {
    }
}
