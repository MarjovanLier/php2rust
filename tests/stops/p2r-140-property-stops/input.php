<?php

declare(strict_types=1);

final class Bad
{
    public $untyped = 1;
    public int $missing;
    public int $sum = 1 + 1;
    /** @var list<int> */
    public array $filled = [1, 2];
    public static int $shared = 0;
    public readonly int $fixed;
    var int $old = 0;
    public int $hooked = 0 {
        get => $this->hooked;
    }
}

abstract class Shape
{
}

final class Holder
{
    public const LIMIT = 3;
}
