<?php

declare(strict_types=1);

namespace Acme\Lib\Type\Deep;

final class Leaf
{
    public static function twice(int $n): int
    {
        return $n * 2;
    }
}
